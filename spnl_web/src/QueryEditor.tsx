import { useState } from "react"
import { CodeEditor, CodeEditorControl } from "@patternfly/react-code-editor"

import PlayIcon from "@patternfly/react-icons/dist/esm/icons/play-icon"

/*const initialQuery = `(g "{model}"
   (cross
    (print "Ask the model to select the best option from the candidates")

    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate {n} candidate emails in parallel")

    (plus
     (repeat {n}
             (g "{model}"
                (user "write an introductory email for a job application, limited to at most {max_tokens} characters.")
                {max_tokens} {temperature})))

    (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"))
0 0.0)`*/

const initialQuery = `(g "ollama/granite3.2:2b"
   (cross
    (print "Ask the model to select the best option from the candidates")

    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate 4 candidate emails in parallel")

    (plus
     (repeat 4
             (g "ollama/granite3.2:2b"
                (user "write an introductory email for a job application, limited to at most {max_tokens} characters.")
                100 0.3)))

    (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"))
0 0.0)`

type Props = {
  onExecuteQuery(query: string): void
}

export default function QueryEditor(props: Props) {
  const [query, setQuery] = useState(initialQuery)

  const customControls = (
    <CodeEditorControl
      icon={<PlayIcon />}
      aria-label="Execute query"
      tooltipProps={{ content: "Execute query" }}
      onClick={props.onExecuteQuery}
      isVisible={query !== ""}
    />
  )

  return (
    <CodeEditor
      isCopyEnabled
      isDarkTheme
      isLineNumbersVisible
      isMinimapVisible={false}
      code={initialQuery}
      customControls={customControls}
      options={{ fontSize: 14, wordWrap: "on" }}
      onChange={setQuery}
      language="clojure"
      onEditorDidMount={(editor, _monaco) => {
        editor.layout()
      }}
      height="500px"
    />
  )
}
