# Candle Backend Performance Optimization Plan

## Current Performance Issue
The Candle backend is **5x slower** than Ollama for text generation.

## Critical Performance Issues Identified

### 1. **Model Reloading Per Request** ⚠️ HIGHEST IMPACT
**Location:** `mod.rs` lines 120-127, 300-307

**Problem:**
```rust
// Each worker loads the model from disk for EVERY request
let (mut model, tokenizer, device, _dtype) = match load_model(&item.model_path) {
    Ok(m) => m,
    Err(e) => {
        eprintln!("Failed to load model: {}", e);
        continue;
    }
};
```

**Impact:** Models are multi-GB files. Loading from disk on every request is extremely expensive.

**Solution:** Implement a model pool/cache that keeps models loaded in memory:
```rust
// Pseudo-code
struct ModelPool {
    models: HashMap<String, Arc<Mutex<Box<dyn CandleModel>>>>,
}

impl ModelPool {
    fn get_or_load(&mut self, path: &str) -> Result<Arc<Mutex<Box<dyn CandleModel>>>> {
        if !self.models.contains_key(path) {
            let model = load_model(path)?;
            self.models.insert(path.to_string(), Arc::new(Mutex::new(model)));
        }
        Ok(Arc::clone(self.models.get(path).unwrap()))
    }
}
```

**Expected Improvement:** 3-4x speedup for repeated requests

---

### 2. **No Batching in Prefill Phase** ❌ NOT BENEFICIAL
**Location:** `model.rs` lines 60-63

**Problem:**
```rust
// Prefill phase - process all prompt tokens at once
let prompt_len = tokens.len();
let input = Tensor::new(&tokens[..], config.device)?.unsqueeze(0)?;
let _logits = model.forward_pass(&input, 0)?;
```

**Attempted Solution:**
- Tried chunked processing (256 tokens at a time)
- Added tensor reuse across chunks

**Result:** ❌ **No performance improvement**
- Chunking added overhead from multiple forward passes
- Lost parallelism benefit of single-pass processing
- Memory copies added CPU overhead
- **Conclusion:** Single-pass prefill is already optimal

**Current Status:** Reverted to original single-pass approach

---

### 3. **Inefficient Token-by-Token Processing**
**Location:** `model.rs` lines 69-120

**Problems:**
- Creates new tensors every iteration (line 79)
- Multiple GPU-CPU synchronizations per token (lines 94, 97)
- `to_scalar()` forces expensive synchronization

```rust
// Current code - creates new tensor each iteration
token_buffer[0] = tokens[tokens.len() - 1];
let input = Tensor::new(&token_buffer[..], config.device)?.unsqueeze(0)?;

// Sampling forces GPU->CPU sync
let next_token = if config.temperature > 0.0 && config.temperature != 1.0 {
    let logits = (last_token_logits / config.temperature)?;
    let probs = candle_nn::ops::softmax(&logits, 0)?;
    probs.argmax(0)?.to_scalar::<u32>()?  // GPU->CPU sync here
} else {
    last_token_logits.argmax(0)?.to_scalar::<u32>()?  // And here
};
```

**Solution:**
- Reuse tensor objects instead of creating new ones
- Batch multiple operations before syncing
- Use `argmax_keepdim()` to avoid unnecessary syncs
- Consider GPU-based sampling when possible

**Expected Improvement:** 1.3-1.5x speedup

---

### 4. **Flash Attention Disabled**
**Location:** `llama.rs` line 42

**Problem:**
```rust
Config {
    // ...
    use_flash_attn: false,  // ❌ Flash Attention disabled
}
```

**Solution:** Enable Flash Attention when GPU supports it:
```rust
use_flash_attn: device.is_cuda() && supports_flash_attention(),
```

**Expected Improvement:** 2-3x speedup for attention operations (significant for long contexts)

---

### 5. **Position-Independent KV Cache for Cross-Generate Reuse** 🚀 ADVANCED
**Applies to:** All RoPE-based models (Llama, Qwen, Mistral, etc.)
**Location:** All model implementations, candle-transformers upstream

**Current Problem:**
```rust
// In candle-transformers (all RoPE models)
let k = self.apply_rotary_emb(&k, index_pos, cache)?;  // RoPE applied
cache.kvs[block_idx] = Some((k.clone(), v.clone()));   // Cached WITH position
```

RoPE is applied **before** caching, so cached K tensors have position embeddings baked in. This prevents reusing cache across different positions.

**Use Case:**
- Generate 1: "What is the capital of France?" → "Paris"
- Generate 2: "When I visit **Paris**, I will eat quiche"
- Want to reuse K/V tensors for "Paris" even though it's at different positions

**Proposed Solution: Cache Pre-RoPE Tensors**

Modify candle-transformers to cache **pre-RoPE** K tensors:

```rust
// Compute K and V projections
let k_pre_rope = self.k_proj.forward(x)?;
let v = self.v_proj.forward(x)?;

// Cache pre-RoPE tensors (position-independent!)
cache.kvs[block_idx] = Some((k_pre_rope.clone(), v.clone()));

// Apply RoPE only when using cached tensors for attention
if let Some((cache_k_pre, cache_v)) = &cache.kvs[block_idx] {
    // Apply RoPE to cached K based on NEW positions in current context
    let cache_k = self.apply_rotary_emb(cache_k_pre, new_position, cache)?;
    
    // Apply RoPE to new K
    let k = self.apply_rotary_emb(&k_pre_rope, index_pos, cache)?;
    
    // Concatenate
    k = Tensor::cat(&[cache_k, &k], 2)?;
    v = Tensor::cat(&[cache_v, &v], 2)?;
}
```

**Key Insight:** Pre-RoPE K tensors are position-independent. We can apply RoPE with ANY position when we use them!

**Implementation Steps:**

1. **Modify candle-transformers** (all RoPE models):
   - Cache K **before** applying RoPE
   - Apply RoPE lazily during attention
   - No need to store positions - tensors are position-independent!

2. **Add token-level cache management** in spnl:
   ```rust
   // Map tokens to their pre-RoPE K/V tensors
   struct TokenCache {
       cache: HashMap<u32, (Tensor, Tensor)>,  // token_id → (k_pre_rope, v)
   }
   ```

3. **Extend API** to support cache reuse:
   ```rust
   pub fn generate_with_token_cache(
       &mut self,
       tokens: &[u32],
       token_cache: &TokenCache,  // Reusable pre-RoPE tensors
       config: GenerateConfig,
   ) -> Result<String>
   ```

4. **Cache lookup logic**:
   - For each token in new prompt, check if it's in token_cache
   - If found, reuse pre-RoPE K/V and apply RoPE for current position
   - If not found, compute normally and add to cache

**Expected Improvement:**
- **2-5x speedup** for prompts with significant token overlap
- Especially beneficial for:
  - Templated prompts with variable insertions
  - Multi-step reasoning with shared context
  - Batch processing with common prefixes/suffixes

**Challenges:**
- Requires candle-transformers fork or upstream contribution
- Slightly increased memory (pre-RoPE vs post-RoPE tensors are same size)
- Need token matching/hashing system
- Must update all RoPE-based model implementations

**Alternative: Upstream Contribution**
- Propose to candle-transformers as opt-in feature
- Config flag: `cache_pre_rope: bool`
- Would benefit entire community
- Works for all RoPE models automatically

**Status:** 📋 Design phase - feasible and model-agnostic

---

## Implementation Priority

- [x] 1. **Model Caching** (Highest Impact) - 3-4x improvement
- [ ] 2. **Flash Attention** (High Impact) - 2-3x improvement
- [x] 3. **Reduce GPU-CPU Syncs** (Medium Impact) - 1.3-1.5x improvement
- [ ] 4. **Optimize KV Cache** (Medium Impact) - 1.1-1.2x improvement
- [~] 5. **Batch Prefill** (Attempted - No benefit) - Reverted

**Combined Expected Improvement:** 5-10x speedup (matching or exceeding Ollama)

---

## Additional Optimizations to Consider

### 6. **Speculative Decoding**
- Generate multiple tokens in parallel
- Verify with full model
- Can provide 2-3x speedup

### 7. **Continuous Batching**
- Process multiple requests simultaneously
- Better GPU utilization
- Requires architectural changes

### 8. **Quantization**
- Use INT8/INT4 quantization
- Reduces memory bandwidth requirements
- 1.5-2x speedup with minimal quality loss

### 9. **Kernel Fusion**
- Fuse multiple operations into single kernels
- Reduces memory transfers
- Requires custom CUDA kernels

---

## Benchmarking Plan

1. Measure baseline performance
2. Implement model caching - measure improvement
3. Enable Flash Attention - measure improvement
4. Optimize GPU-CPU syncs - measure improvement
5. Optimize KV cache - measure improvement
6. Optimize prefill batching - measure improvement

Track tokens/second for:
- Short prompts (< 100 tokens)
- Medium prompts (100-500 tokens)
- Long prompts (> 500 tokens)

---

## References

- Ollama architecture: Model pooling, Flash Attention, optimized sampling
- vLLM: Continuous batching, PagedAttention
- Flash Attention paper: 2-3x speedup for attention operations
- Candle documentation: Best practices for tensor operations