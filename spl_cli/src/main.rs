use crate::demos::*;
use spl_run::{result::SplError, run};

mod args;
mod demos;

#[tokio::main]
async fn main() -> Result<(), SplError> {
    let program = email::demo();

    run(&program, None).await.map(|res| {
        if res.to_string().len() > 0 {
            println!("{:?}", res);
        }
        Ok(())
    })?
}
