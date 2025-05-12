import { useEffect } from "react"
import { useNavigate } from "@tanstack/react-router"
import {
  CodeEditor,
  CodeEditorControl,
  Language,
} from "@patternfly/react-code-editor"

import PlayIcon from "@patternfly/react-icons/dist/esm/icons/play-icon"
import TopologyIcon from "@patternfly/react-icons/dist/esm/icons/project-diagram-icon"

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
    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate 4 candidate emails in parallel")
    (plus
     (repeat 4
             (g "ollama/granite3.2:2b"
                (user "write an introductory email for a job application, limited to at most 100 characters.")
                100 0.3)))

    (print "Done generating candidate emails. Now asking the model to select the best option from the candidates")
    (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"))
0 0.0)`

type Props = {
  setQuery(query: string): void
  onExecuteQuery(): void
  isDrawerOpen: boolean
}

export default function QueryEditor(props: Props) {
  const navigate = useNavigate()

  const customControls = [
    <CodeEditorControl
      key="play"
      icon={<PlayIcon />}
      aria-label="Execute query"
      tooltipProps={{ content: "Execute query" }}
      onClick={props.onExecuteQuery}
    />,

    <CodeEditorControl
      key="topology"
      icon={<TopologyIcon />}
      aria-label="Toggle Query Viewer"
      tooltipProps={{ content: "Toggle Query Viewer" }}
      onClick={() => {
        console.error("NNNN", props.isDrawerOpen)
        return navigate({ to: "/", search: { qv: !props.isDrawerOpen } })
      }}
    />,
  ]

  const { setQuery } = props
  useEffect(() => setQuery(initialQuery), [setQuery])

  return (
    <CodeEditor
      isCopyEnabled
      isDarkTheme
      isLineNumbersVisible={false}
      isMinimapVisible={false}
      code={initialQuery}
      headerMainContent="SPNL Query Editor"
      customControls={customControls}
      options={{ fontSize: 14, wordWrap: "on" }}
      onChange={props.setQuery}
      language={Language.clojure}
      onEditorDidMount={(editor) => {
        editor.layout()
      }}
      height="800px"
    />
  )
}
