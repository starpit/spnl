use clap::Parser;

use crate::args::Args;
use crate::demos::*;
use spl_run::{result::SplError, run};

mod args;
mod demos;

#[tokio::main]
async fn main() -> Result<(), SplError> {
    let args = Args::parse();
    let program = match args.demo {
        Demo::Chat => chat::demo(args),
        Demo::Email => email::demo(args),
    };

    run(&program, None).await.map(|res| {
        if res.to_string().len() > 0 {
            println!("{:?}", res);
        }
        Ok(())
    })?
}
