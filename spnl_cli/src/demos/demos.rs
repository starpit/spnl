#[derive(clap::ValueEnum, Clone, Debug, serde::Serialize)]
pub enum Demo {
    Chat,
    Email,
    Email2,
    Email3,
}
