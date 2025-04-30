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

    spnl!(
        g model
         (cross
          (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and always show the full content of each candidate email.")

          (plusn n (desc (format "Generate {n} candidate emails in parallel"))
           (g model
            (cross
             (system "You are IBM Sales Assistant, an expert in writing emails for IBM sellers to help in prospecting.

You MUST strictly adhere to the following guidelines. Pay attention to each of the following guideline attributes. You must include all these guideline attributes in the email if mentioned below (subject, greeting, signatures, etc.) and the guideline attributes also should adhere to its list of requirements mentioned. But allow the user to override the guidelines in your response if they explicitly ask in their query. Be professional and don't use asterisks, emojis, links, or any other symbols in the email.

The guidelines are:
{guidelines}

Email should start with a Subject: ....

Just give me the email text. Add a new line between each of these segments. Don't include any other words, text, or comments.")

             "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"
            )

            temperature max_tokens
           )
          )
         )
    )
}
