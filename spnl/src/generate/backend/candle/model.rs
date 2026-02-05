use candle_core::{Device, Tensor};
use indicatif::ProgressBar;
use tokenizers::Tokenizer;

/// Callback function type for streaming tokens as they are generated
pub type TokenCallback = Box<dyn FnMut(&str) -> anyhow::Result<()> + Send>;

/// Configuration for text generation
pub struct GenerateConfig<'a> {
    pub device: &'a Device,
    pub max_tokens: usize,
    pub temperature: f64,
    pub tokenizer: &'a Tokenizer,
    pub progress_bar: Option<&'a ProgressBar>,
}

/// Trait for model-specific forward pass operations
pub trait ModelForward {
    /// Perform forward pass and return logits
    fn forward_pass(&mut self, input: &Tensor, position: usize) -> anyhow::Result<Tensor>;

    /// Get maximum position embeddings for this model
    fn max_position_embeddings(&self) -> usize;

    /// Clear or reset the model's KV cache (if applicable)
    fn clear_cache(&mut self);
}

/// Trait for all candle-based language models
pub trait CandleModel: Send {
    /// Generate text from a prompt
    fn generate(
        &mut self,
        tokens: &[u32],
        config: GenerateConfig,
        token_callback: Option<&mut TokenCallback>,
    ) -> anyhow::Result<String>;

    /// Get the model's EOS token ID
    fn eos_token_id(&self, tokenizer: &Tokenizer) -> u32;
}

/// Shared generation logic for all models
/// This eliminates code duplication across model implementations
pub fn generate_text<M: ModelForward>(
    model: &mut M,
    tokens: &[u32],
    eos_token: u32,
    config: GenerateConfig,
    mut token_callback: Option<&mut TokenCallback>,
) -> anyhow::Result<String> {
    let mut tokens = tokens.to_vec();

    // Pre-allocate with estimated capacity
    let mut generated_tokens = Vec::with_capacity(config.max_tokens);

    // Clear KV cache before starting generation
    model.clear_cache();

    // Prefill phase - process all prompt tokens at once
    let prompt_len = tokens.len();
    let input = Tensor::new(&tokens[..], config.device)?.unsqueeze(0)?;
    let _logits = model.forward_pass(&input, 0)?;

    // Pre-allocate single-token input buffer for reuse (reduces allocations)
    let mut token_buffer = [0u32; 1];

    // Generation phase - one token at a time
    for index_pos in 0..config.max_tokens {
        let start_pos = prompt_len + index_pos;

        // Check if we're exceeding max_position_embeddings
        if start_pos >= model.max_position_embeddings() {
            break;
        }

        // Reuse token buffer instead of creating new tensor each iteration
        token_buffer[0] = tokens[tokens.len() - 1];
        let input = Tensor::new(&token_buffer[..], config.device)?.unsqueeze(0)?;
        let logits = model.forward_pass(&input, start_pos)?;

        let logits = logits.squeeze(0)?;
        let last_token_logits = if logits.dims().len() == 2 {
            logits.get(logits.dim(0)? - 1)?
        } else {
            logits
        };

        // Apply temperature and sample
        // Note: to_scalar() forces GPU->CPU sync, but necessary for sampling
        let next_token = if config.temperature > 0.0 && config.temperature != 1.0 {
            let logits = (last_token_logits / config.temperature)?;
            let probs = candle_nn::ops::softmax(&logits, 0)?;
            probs.argmax(0)?.to_scalar::<u32>()?
        } else {
            // Skip softmax for greedy sampling
            last_token_logits.argmax(0)?.to_scalar::<u32>()?
        };

        if next_token == eos_token {
            break;
        }

        tokens.push(next_token);
        generated_tokens.push(next_token);

        // Stream token if callback is provided
        if let Some(callback) = token_callback.as_mut() {
            let token_text = config
                .tokenizer
                .decode(&[next_token], false)
                .map_err(|e| anyhow::anyhow!("Token decoding failed: {}", e))?;
            callback(&token_text)?;
        }

        // Update progress bar per token
        if let Some(pb) = config.progress_bar {
            pb.inc(1);
        }
    }

    // Decode all generated tokens at once
    let generated_text = config
        .tokenizer
        .decode(&generated_tokens, false)
        .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))?;

    Ok(generated_text)
}

// Made with Bob
