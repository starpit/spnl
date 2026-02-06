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

### 2. **No Batching in Prefill Phase**
**Location:** `model.rs` lines 60-63

**Problem:**
```rust
// Prefill phase - process all prompt tokens at once
let prompt_len = tokens.len();
let input = Tensor::new(&tokens[..], config.device)?.unsqueeze(0)?;
let _logits = model.forward_pass(&input, 0)?;
```

The prefill processes all tokens but could be optimized with batching strategies.

**Solution:** 
- Process prompt tokens in optimized batch sizes (e.g., 128-256 tokens at a time)
- Use separate code paths for prefill vs decode phases
- Consider chunked prefill for very long prompts

**Expected Improvement:** 1.2-1.5x speedup for long prompts

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

### 5. **KV Cache Recreation**
**Location:** `llama.rs` line 89

**Problem:**
```rust
fn clear_cache(&mut self) {
    // Reinitialize cache for Llama model - expensive!
    self.cache = Cache::new(true, self.dtype, &self.config, &self.device).ok();
}
```

**Solution:** 
- Don't recreate cache, just reset/clear it
- Implement `cache.reset()` method instead of full recreation
- Keep cache allocated between generations

**Expected Improvement:** 1.1-1.2x speedup

---

## Implementation Priority

- [x] 1. **Model Caching** (Highest Impact) - 3-4x improvement
- [ ] 2. **Flash Attention** (High Impact) - 2-3x improvement
- [x] 3. **Reduce GPU-CPU Syncs** (Medium Impact) - 1.3-1.5x improvement
- [ ] 4. **Optimize KV Cache** (Medium Impact) - 1.1-1.2x improvement
- [x] 5. **Batch Prefill** (Medium Impact) - 1.2-1.5x improvement

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