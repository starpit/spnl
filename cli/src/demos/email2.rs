use crate::args::Args;
use spnl::{Query, spnl};

pub fn demo(args: Args) -> Query {
    let Args {
        model,
        n,
        temperature,
        max_tokens,
        ..
    } = args;

    spnl!(
        g model
         (cross
          (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

          (print (format "Generate {n} candidate emails in parallel"))
          (plus (repeat n
           (g model
            (cross
             (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")
             (format "write an introductory email for a job application, limited to at most {max_tokens} characters.")

             "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"
            )

            temperature max_tokens
           ))
          )
         )
    )
}
