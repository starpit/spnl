use crate::args::Args;
use spnl_ast::{Unit, spnl};

pub fn demo(args: Args) -> Unit {
    let Args {
        model,
        n,
        temperature,
        max_tokens,
        ..
    } = args;

    spnl!(
        g model
         (cross "Ask the model to select the best option from the candidates"
          (system (format "You compute an evaluation score that ranks {n} given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You always explain your thinking by presenting a list of the top 3 ordered by their rank, and finish by showing me the best one."))

          (plusn n (format "Generate {n} candidate emails in parallel")
           (g model
            (format "write an introductory email for a job application, limited to at most {max_tokens} characters. use your imagination, go wild.")
            temperature max_tokens)
          )

          "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"
         )
    )
}
