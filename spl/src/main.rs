use spl_ast::spl;
use spl_run::{result::SplError, run};

#[tokio::main]
async fn main() -> Result<(), SplError> {
    //let program = spl!(g "ollama/granite3.2:2b" (cross "Sample cross" (file "/tmp/foo") "Tell me a story" (ask "What is your question")));
    let program = spl!(
        g "ollama/granite3.2:2b"
            (cross "Ask the model to select the best option from the candidates"
             "Select exactly one of the following candidate email messages, judging based on shortest length"
             (plus "Generate candidate emails in parallel"
              (g "ollama/granite3.2:2b" "write an introductory email, limited to at most 200 characters" 400)
              (g "ollama/granite3.2:2b" "write an introductory email, limited to at most 200 characters" 400)
              (g "ollama/granite3.2:2b" "write an introductory email, limited to at most 200 characters" 400)
              (g "ollama/granite3.2:2b" "write an introductory email, limited to at most 200 characters" 400)
              (g "ollama/granite3.2:2b" "write an introductory email, limited to at most 200 characters" 400)
             )
            )
    );

    let res = run(&program, None).await?.to_string();
    if res.len() > 0 {
        println!("{}", res);
    }
    Ok(())
}
