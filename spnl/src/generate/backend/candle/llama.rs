use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::llama::{Cache, Config, Llama, LlamaEosToks};
use serde::Deserialize;
use tokenizers::Tokenizer;

use super::model::{CandleModel, ModelForward};

#[derive(Debug, Deserialize)]
pub struct LlamaGenericConfig {
    pub hidden_size: usize,
    pub intermediate_size: usize,
    pub vocab_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub num_key_value_heads: Option<usize>,
    pub rms_norm_eps: Option<f64>,
    pub rope_theta: Option<f64>,
    pub max_position_embeddings: Option<usize>,
    pub bos_token_id: Option<u32>,
    pub eos_token_id: Option<u32>,
}

impl From<LlamaGenericConfig> for Config {
    fn from(config: LlamaGenericConfig) -> Self {
        Config {
            hidden_size: config.hidden_size,
            intermediate_size: config.intermediate_size,
            vocab_size: config.vocab_size,
            num_hidden_layers: config.num_hidden_layers,
            num_attention_heads: config.num_attention_heads,
            num_key_value_heads: config
                .num_key_value_heads
                .unwrap_or(config.num_attention_heads),
            rms_norm_eps: config.rms_norm_eps.unwrap_or(1e-6),
            rope_theta: config.rope_theta.unwrap_or(10000.0) as f32,
            max_position_embeddings: config.max_position_embeddings.unwrap_or(2048),
            bos_token_id: Some(config.bos_token_id.unwrap_or(1)),
            eos_token_id: Some(LlamaEosToks::Single(config.eos_token_id.unwrap_or(2))),
            rope_scaling: None,
            tie_word_embeddings: false,
            use_flash_attn: false,
        }
    }
}

pub struct LlamaModelWrapper {
    model: Llama,
    config: Config,
    cache: Option<Cache>,
    device: Device,
    dtype: DType,
}

impl LlamaModelWrapper {
    pub fn load(
        vb: VarBuilder,
        config: Config,
        device: Device,
        dtype: DType,
    ) -> anyhow::Result<Self> {
        let model = Llama::load(vb, &config)?;
        Ok(Self {
            model,
            config,
            cache: None,
            device,
            dtype,
        })
    }
}

impl ModelForward for LlamaModelWrapper {
    fn forward_pass(&mut self, input: &Tensor, position: usize) -> anyhow::Result<Tensor> {
        // Llama requires a cache object, so we need to handle it specially
        if let Some(cache) = &mut self.cache {
            Ok(self.model.forward(input, position, cache)?)
        } else {
            anyhow::bail!("Cache not initialized for Llama model")
        }
    }

    fn max_position_embeddings(&self) -> usize {
        self.config.max_position_embeddings
    }

    fn clear_cache(&mut self) {
        // Reinitialize cache for Llama model
        self.cache = Cache::new(true, self.dtype, &self.config, &self.device).ok();
    }
}

impl CandleModel for LlamaModelWrapper {
    fn generate(
        &mut self,
        tokens: &[u32],
        config: super::model::GenerateConfig,
        token_callback: Option<&mut super::model::TokenCallback>,
    ) -> anyhow::Result<String> {
        let eos_token = self.eos_token_id(config.tokenizer);
        super::model::generate_text(self, tokens, eos_token, config, token_callback)
    }

    fn eos_token_id(&self, tokenizer: &Tokenizer) -> u32 {
        tokenizer.token_to_id("</s>").unwrap_or(2)
    }
}

// Made with Bob
