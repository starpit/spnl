use indicatif::{MultiProgress, ProgressBar};
use tokio::io::{AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

use crate::{Generate, Query, run::result::SpnlResult};

use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, /* ChatMessageResponse, MessageRole,*/ request::ChatMessageRequest,},
        // tools::{ToolFunctionInfo, ToolInfo, ToolType},
    },
    models::ModelOptions,
};

pub async fn generate(
    model: &str,
    input: &Query,
    max_tokens: &Option<i32>,
    temp: &Option<f32>,
    m: Option<&MultiProgress>,
    prepare: bool,
) -> SpnlResult {
    if prepare {
        todo!()
    }

    let ollama = Ollama::default();
    let input_messages: Vec<ChatMessage> = messagify(input);

    let (prompt, history_slice): (&ChatMessage, &[ChatMessage]) = match input_messages.split_last()
    {
        Some(x) => x,
        None => (&ChatMessage::user("".into()), &[]),
    };
    let history = Vec::from(history_slice);

    let req = ChatMessageRequest::new(model.into(), vec![prompt.clone()]).options(
        ModelOptions::default()
            .temperature(temp.unwrap_or_default())
            .num_predict(if let Some(max_tokens) = max_tokens {
                *max_tokens
            } else {
                -1
            }),
    );
    // .format(ollama_rs::generation::parameters::FormatType::Json)
    //        .tools(tools);

    let mut stream = ollama
        .send_chat_messages_with_history_stream(
            ::std::sync::Arc::new(::std::sync::Mutex::new(history)),
            req,
        )
        .await?;

    let quiet = m.is_some();
    let mut pb = m.map(|m| {
        m.add(if let Some(max_tokens) = max_tokens {
            ProgressBar::new(*max_tokens as u64)
        } else {
            ProgressBar::no_length()
        })
    });

    let mut stdout = stdout();
    /* if !quiet {
        stdout.write_all(b"\x1b[1mUser: \x1b[0m").await?;
        stdout.write_all(prompt.content.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
    } */

    // let mut last_res: Option<ChatMessageResponse> = None;
    let mut response_string = String::new();
    if !quiet {
        stdout.write_all(b"\x1b[1mAssistant: \x1b[0m").await?;
    }
    while let Some(Ok(res)) = stream.next().await {
        if !quiet {
            stdout.write_all(b"\x1b[32m").await?; // green
            stdout.write_all(res.message.content.as_bytes()).await?;
            stdout.flush().await?;
            stdout.write_all(b"\x1b[0m").await?; // reset color
        } else if let Some(pb) = pb.as_mut() {
            pb.inc(res.message.content.len() as u64)
        }
        response_string += res.message.content.as_str();
        // last_res = Some(res);
    }
    if !quiet {
        stdout.write_all(b"\n").await?;
    }

    if m.is_some() {
        Ok(Query::User(response_string))
    } else {
        Ok(Query::Generate(Generate {
            model: format!("ollama/{model}"),
            input: Box::new(Query::User(response_string)),
            max_tokens: *max_tokens,
            temperature: *temp,
            accumulate: None,
        }))
    }
}

fn messagify(input: &Query) -> Vec<ChatMessage> {
    match input {
        Query::Cross(v) | Query::Plus(v) => v.iter().flat_map(messagify).collect(),
        Query::System(s) => vec![ChatMessage::system(s.clone())],
        o => vec![ChatMessage::user(o.to_string())],
    }
}

#[cfg(feature = "rag")]
pub async fn embed(
    embedding_model: &str,
    data: &crate::run::with::embed::EmbedData,
) -> Result<Vec<Vec<f32>>, crate::run::result::SpnlError> {
    use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

    let docs = match data {
        crate::run::with::embed::EmbedData::Vec(v) => v,
        crate::run::with::embed::EmbedData::Query(u) => &messagify(u)
            .into_iter()
            .map(|m| m.content)
            .collect::<Vec<_>>(),
    };

    let request = GenerateEmbeddingsRequest::new(embedding_model.to_string(), docs.clone().into());

    let ollama = Ollama::default();
    Ok(ollama.generate_embeddings(request).await?.embeddings)
}
