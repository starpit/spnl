use spl_ast::spl;
use spl_run::{result::SplError, run};

#[tokio::main]
async fn main() -> Result<(), SplError> {
    // let program = spl!(g "ollama/granite3.2:2b" (cross (file "/tmp/foo") "Tell me a story"));
    let program = spl!(g "ollama/granite3.2:2b"
                       (cross "Please pick the shortest of the following"
                        (plus
                         (g "ollama/granite3.2:2b" "write an introductory email")
                         (g "ollama/granite3.2:2b" "write an introductory email")
                         (g "ollama/granite3.2:2b" "write an introductory email")
                         (g "ollama/granite3.2:2b" "write an introductory email")
                         (g "ollama/granite3.2:2b" "write an introductory email")
                        )
                       )
    );

    println!("{:?} -> {:?}", program, run(&program).await?);
    Ok(())
}
