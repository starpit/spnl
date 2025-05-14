use clap::Parser;

use crate::args::Args;
use crate::demos::*;
use spnl::{
    from_str, pretty_print,
    run::{plan::plan, result::SpnlError, run},
};

mod args;
mod demos;

#[tokio::main]
async fn main() -> Result<(), SpnlError> {
    let args = Args::parse();
    let verbose = args.verbose;
    let show_only = args.show_query;

    let program = plan(&match args.demo {
        Some(Demo::Chat) => chat::demo(args),
        Some(Demo::Email) => email::demo(args),
        Some(Demo::Email2) => email2::demo(args),
        Some(Demo::Email3) => email3::demo(args),
        Some(Demo::SWEAgent) => sweagent::demo(args),
        None => {
            use std::io::prelude::*;
            let file = ::std::fs::File::open(args.file.clone().unwrap())?;
            let mut buf_reader = ::std::io::BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let mut tt = tinytemplate::TinyTemplate::new();
            tt.add_template("file", contents.as_str())?;
            let rendered = tt.render("file", &args)?;
            from_str(rendered.as_str())?
        }
    });

    if show_only {
        let _ = pretty_print(&program)?;
        return Ok(());
    } else if verbose {
        ptree::print_tree(&program)?;
    }

    run(&program, None).await.map(|res| {
        if res.to_string().len() > 0 {
            println!("{}", res);
        }
        Ok(())
    })?
}
