use crate::result::{SplEval, SplResult};
use spl_ast::Unit;
use tokio::io::{AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, /* ChatMessageResponse, MessageRole,*/ request::ChatMessageRequest,},
        // tools::{ToolFunctionInfo, ToolInfo, ToolType},
    },
    // models::ModelOptions,
};

pub async fn generate<'a>(model: &'a str, input: &Unit<'a>) -> SplResult<'a> {
    if model.starts_with("ollama/") || model.starts_with("ollama_chat/") {
        let model = if model.starts_with("ollama/") {
            &model[7..]
        } else {
            &model[12..]
        };

        generate_ollama(model, input).await
    } else {
        todo!()
    }
}

async fn generate_ollama<'a>(model: &'a str, input: &Unit<'a>) -> SplResult<'a> {
    let ollama = Ollama::default();

    let input_messages: Vec<ChatMessage> = match input {
        Unit::Cross(v) | Unit::Plus(v) => {
            v.iter().map(|i| ChatMessage::user(i.to_string())).collect()
        }
        o => vec![ChatMessage::user(o.to_string())],
    };

    let (prompt, history_slice): (&ChatMessage, &[ChatMessage]) = match input_messages.split_last()
    {
        Some(x) => x,
        None => (&ChatMessage::user("".into()), &[]),
    };
    let history = Vec::from(history_slice);

    let req = ChatMessageRequest::new(model.into(), vec![prompt.clone()]);
    //        .options(options)
    // .format(ollama_rs::generation::parameters::FormatType::Json)
    //        .tools(tools);

    let mut stream = ollama
        .send_chat_messages_with_history_stream(
            ::std::sync::Arc::new(::std::sync::Mutex::new(history)),
            req,
        )
        .await?;

    let emit = true;

    // let mut last_res: Option<ChatMessageResponse> = None;
    let mut response_string = String::new();
    let mut stdout = stdout();
    if emit {
        stdout.write_all(b"\x1b[1mAssistant: \x1b[0m").await?;
    }
    while let Some(Ok(res)) = stream.next().await {
        if emit {
            stdout.write_all(b"\x1b[32m").await?; // green
            stdout.write_all(res.message.content.as_bytes()).await?;
            stdout.flush().await?;
            stdout.write_all(b"\x1b[0m").await?; // reset color
        }
        response_string += res.message.content.as_str();
        // last_res = Some(res);
    }
    if emit {
        stdout.write_all(b"\n").await?;
    }

    Ok(SplEval::String(response_string))
}
