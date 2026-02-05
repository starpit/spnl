use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;

use super::config::{GenericConfig, detect_architecture};
use super::download::download_model_files;
use super::{
    CandleModel, LlamaGenericConfig, LlamaModelWrapper, Qwen2GenericConfig, Qwen2ModelWrapper,
    Qwen3GenericConfig, Qwen3ModelWrapper, Qwen3MoeGenericConfig, Qwen3MoeModelWrapper,
};

/// Load a model from HuggingFace Hub
pub fn load_model(
    model_id: &str,
) -> anyhow::Result<(Box<dyn CandleModel>, Tokenizer, Device, DType)> {
    // Determine device (Metal on macOS if available, otherwise CPU)
    let device = Device::new_metal(0).unwrap_or(Device::Cpu);

    // Use F16 for faster inference on Metal/GPU, F32 as fallback
    let dtype = if device.is_metal() {
        DType::F16
    } else {
        DType::F32
    };

    // Download all necessary files
    let (tokenizer_path, config_path, filenames) = download_model_files(model_id)?;

    // Load tokenizer
    let tokenizer = Tokenizer::from_file(tokenizer_path)
        .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

    // Load and parse config
    let config_str = std::fs::read_to_string(config_path)?;
    let generic_config: GenericConfig = serde_json::from_str(&config_str)?;

    // Detect architecture
    let architecture = detect_architecture(&generic_config)?;

    // Load weights
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };

    // Load the appropriate model based on architecture
    let model: Box<dyn CandleModel> = match architecture.as_str() {
        arch if arch.contains("Llama") => {
            let llama_config = LlamaGenericConfig {
                hidden_size: generic_config.hidden_size,
                intermediate_size: generic_config.intermediate_size,
                vocab_size: generic_config.vocab_size,
                num_hidden_layers: generic_config.num_hidden_layers,
                num_attention_heads: generic_config.num_attention_heads,
                num_key_value_heads: generic_config.num_key_value_heads,
                rms_norm_eps: generic_config.rms_norm_eps,
                rope_theta: generic_config.rope_theta,
                max_position_embeddings: generic_config.max_position_embeddings,
                bos_token_id: generic_config.bos_token_id,
                eos_token_id: generic_config.eos_token_id,
            };
            let config = llama_config.into();
            Box::new(LlamaModelWrapper::load(vb, config, device.clone(), dtype)?)
        }
        arch if arch.contains("Qwen2") => {
            let qwen2_config = Qwen2GenericConfig {
                vocab_size: generic_config.vocab_size,
                hidden_size: generic_config.hidden_size,
                intermediate_size: generic_config.intermediate_size,
                num_hidden_layers: generic_config.num_hidden_layers,
                num_attention_heads: generic_config.num_attention_heads,
                num_key_value_heads: generic_config.num_key_value_heads,
                max_position_embeddings: generic_config.max_position_embeddings,
                sliding_window: generic_config.sliding_window,
                max_window_layers: generic_config.max_window_layers,
                tie_word_embeddings: generic_config.tie_word_embeddings,
                rope_theta: generic_config.rope_theta,
                rms_norm_eps: generic_config.rms_norm_eps,
                use_sliding_window: generic_config.use_sliding_window,
                hidden_act: generic_config.hidden_act,
            };
            let config = qwen2_config.into();
            Box::new(Qwen2ModelWrapper::load(vb, config)?)
        }
        arch if arch.contains("Qwen3") => {
            // Check if this is a MoE model by looking for num_experts field
            if generic_config.num_experts.is_some() {
                // This is a Qwen3 MoE model (e.g., A3B)
                let qwen3_moe_config = Qwen3MoeGenericConfig {
                    vocab_size: generic_config.vocab_size,
                    hidden_size: generic_config.hidden_size,
                    intermediate_size: generic_config.intermediate_size,
                    num_hidden_layers: generic_config.num_hidden_layers,
                    num_attention_heads: generic_config.num_attention_heads,
                    num_key_value_heads: generic_config.num_key_value_heads,
                    max_position_embeddings: generic_config.max_position_embeddings,
                    sliding_window: generic_config.sliding_window,
                    max_window_layers: generic_config.max_window_layers,
                    tie_word_embeddings: generic_config.tie_word_embeddings,
                    rope_theta: generic_config.rope_theta,
                    rms_norm_eps: generic_config.rms_norm_eps,
                    use_sliding_window: generic_config.use_sliding_window,
                    hidden_act: generic_config.hidden_act,
                    attention_bias: generic_config.attention_bias,
                    head_dim: generic_config.head_dim,
                    num_experts_per_tok: generic_config.num_experts_per_tok,
                    num_experts: generic_config.num_experts,
                    moe_intermediate_size: generic_config.moe_intermediate_size,
                    decoder_sparse_step: generic_config.decoder_sparse_step,
                    norm_topk_prob: generic_config.norm_topk_prob,
                };
                let config = qwen3_moe_config.into();
                Box::new(Qwen3MoeModelWrapper::load(vb, config)?)
            } else {
                // Standard Qwen3 model
                let qwen3_config = Qwen3GenericConfig {
                    vocab_size: generic_config.vocab_size,
                    hidden_size: generic_config.hidden_size,
                    intermediate_size: generic_config.intermediate_size,
                    num_hidden_layers: generic_config.num_hidden_layers,
                    num_attention_heads: generic_config.num_attention_heads,
                    num_key_value_heads: generic_config.num_key_value_heads,
                    max_position_embeddings: generic_config.max_position_embeddings,
                    sliding_window: generic_config.sliding_window,
                    max_window_layers: generic_config.max_window_layers,
                    tie_word_embeddings: generic_config.tie_word_embeddings,
                    rope_theta: generic_config.rope_theta,
                    rms_norm_eps: generic_config.rms_norm_eps,
                    use_sliding_window: generic_config.use_sliding_window,
                    hidden_act: generic_config.hidden_act,
                    attention_bias: generic_config.attention_bias,
                    head_dim: generic_config.head_dim,
                };
                let config = qwen3_config.into();
                Box::new(Qwen3ModelWrapper::load(vb, config)?)
            }
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported architecture: {}. Currently supported: Llama, Qwen2, Qwen3",
                architecture
            ));
        }
    };

    Ok((model, tokenizer, device, dtype))
}

// Made with Bob
