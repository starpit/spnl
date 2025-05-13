import { useEffect } from "react"
import { useNavigate } from "@tanstack/react-router"
import useLocalStorageState from "use-local-storage-state"

import {
  CodeEditor,
  CodeEditorControl,
  Language,
} from "@patternfly/react-code-editor"

import PlayIcon from "@patternfly/react-icons/dist/esm/icons/play-icon"
import TopologyIcon from "@patternfly/react-icons/dist/esm/icons/project-diagram-icon"

type Props = {
  setQuery(query: string): void
  onExecuteQuery(): void
  isDrawerOpen: boolean
}

export default function QueryEditor(props: Props) {
  const navigate = useNavigate()
  const [initialQuery] = useLocalStorageState<string>("spnl.demo.select")

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
      onClick={() => navigate({ to: "/", search: { qv: !props.isDrawerOpen } })}
    />,

    /*<CodeEditorControl
      key="demo"
      icon={<DemoSelect />}
      aria-label="Demo Select"
      tooltipProps={{ content: "Select a demo" }}
      onClick={() => {}}
    />,*/
  ]

  const { setQuery } = props
  useEffect(() => {
    if (initialQuery) setQuery(initialQuery)
  }, [setQuery, initialQuery])

  return (
    <CodeEditor
      isCopyEnabled
      isDarkTheme
      isLineNumbersVisible={false}
      isMinimapVisible={false}
      code={initialQuery || ""}
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
