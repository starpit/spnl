pub mod chat;
pub mod email;
pub mod email2;
pub mod email3;
pub mod gsm8k;
pub mod rag;
pub mod sweagent;

#[derive(clap::ValueEnum, Clone, Debug, serde::Serialize)]
#[clap(rename_all = "lowercase")]
pub enum Demo {
    Chat,
    Email,
    Email2,
    Email3,
    SWEAgent,
    GSM8k,
    #[cfg(feature = "rag")]
    Rag,
}
