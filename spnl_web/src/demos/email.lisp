(g "ollama/granite3.2:2b"
   (cross
    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate 4 candidate emails in parallel")
    (plus
     (repeat 4
             (g "ollama/granite3.2:2b"
                (user "write an introductory email for a job application, limited to at most 100 characters.")
                100 0.3)))

    (print "Done generating candidate emails. Now asking the model to select the best option from the candidates")
    (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"))
0 0.0)
