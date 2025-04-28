use crate::args::Args;
use spl_ast::{Unit, spl};

pub fn demo(args: Args) -> Unit {
    let Args {
        model, temperature, ..
    } = args;

    spl!(loop (g model (cross "Chatting" (system "You are a helpful chat bot") (ask "❯")) temperature))
}
