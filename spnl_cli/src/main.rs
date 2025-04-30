use clap::Parser;

use crate::args::Args;
use crate::demos::*;
use spnl_run::{result::SpnlError, run};

mod args;
mod demos;

#[tokio::main]
async fn main() -> Result<(), SpnlError> {
    let args = Args::parse();
    let program = match args.demo {
        Demo::Chat => chat::demo(args),
        Demo::Email => email::demo(args),
        Demo::Email2 => email2::demo(args),
        Demo::Email3 => email3::demo(args),
    };

    run(&program, None).await.map(|res| {
        if res.to_string().len() > 0 {
            println!("{:?}", res);
        }
        Ok(())
    })?
}
