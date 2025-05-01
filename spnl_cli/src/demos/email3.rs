use crate::args::Args;
use spnl_ast::{Unit, spnl};

// https://github.ibm.com/AI4BA/agentic-policy
pub fn demo(args: Args) -> Unit {
    let Args {
        model,
        n,
        temperature,
        max_tokens,
        ..
    } = args;

    let generate_one_candidate_email = spnl!(
        g model
          (cross (system (file "email3-generate-system-prompt.txt"))
                 (ask "Tell me about yourself:"))
          temperature max_tokens);

    let candidate_emails = spnl!(
        plusn n
            (format "Generate {n} candidate emails in parallel")
            generate_one_candidate_email
    );

    spnl!(g model (cross (system (file "email3-evaluate-system-prompt.txt")) candidate_emails))
}
