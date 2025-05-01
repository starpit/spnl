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

    let generate_system_prompt = spnl!(file "email3-generate-system-prompt.txt");
    let evaluate_system_prompt = spnl!(file "email3-evaluate-system-prompt.txt");

    let generate_user_prompt = spnl!(ask "Tell me about yourself" "My name is Greg. I am a data scientist with 10 years of experience applying for a position at IBM in their research department");

    let generate_one_candidate_email = spnl!(g model (cross (system generate_system_prompt) generate_user_prompt) temperature max_tokens);
    let candidate_emails = spnl!(plusn n (desc (format "Generate {n} candidate emails in parallel")) generate_one_candidate_email);

    spnl!(g model (cross (system evaluate_system_prompt) candidate_emails))
}
