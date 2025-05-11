import { useCallback, useEffect, useState } from "react"
import {
  Grid,
  GridItem,
  HelperText,
  HelperTextItem,
} from "@patternfly/react-core"

import Header from "./Header"
import Console, { type RunState } from "./Console"
import Topology from "./Topology"
import QueryEditor from "./QueryEditor"

import { compile_query } from "spnl_wasm"

export default function Body() {
  const [unit, setUnit] = useState<null | import("./Unit").Unit>(null)
  const [query, setQuery] = useState<null | string>(null)
  const [compilationError, setCompilationError] = useState<null | Error>(null)

  const [runState, setRunState] = useState<RunState>("idle")
  const onRunComplete = useCallback(
    (success: boolean) => setRunState(success ? "success" : "error"),
    [setRunState],
  )

  useEffect(() => {
    if (!query) {
      setUnit(null)
    } else {
      try {
        setCompilationError(null)
        setUnit(JSON.parse(compile_query(query)) as import("./Unit").Unit)
      } catch (err) {
        console.error(err)
        setCompilationError(err as Error)
      }
    }
  }, [query, setUnit])

  const onExecuteQuery = useCallback(
    () => setRunState("running"),
    [setRunState],
  )

  return (
    <>
      <HelperText component="ul" style={{ marginBottom: "1em" }}>
        <HelperTextItem>
          Welcome to the SPNL Playground. Edit your query on the left, then
          click Run to execute it.
        </HelperTextItem>
        {compilationError && (
          <HelperTextItem component="li" variant="error">
            Compilation error: {compilationError.message}
          </HelperTextItem>
        )}
      </HelperText>

      <Grid hasGutter>
        <GridItem span={4}>
          <QueryEditor setQuery={setQuery} onExecuteQuery={onExecuteQuery} />
        </GridItem>

        <GridItem span={4}>
          <div className="pf-v6-c-code-editor">
            <Header title="Console" />
            <div className="pf-v6-c-code-editor__main">
              <div className="pf-v6-c-code-editor__code">
                <Console
                  runState={runState}
                  query={unit}
                  onComplete={onRunComplete}
                />
              </div>
            </div>
          </div>
        </GridItem>

        <GridItem span={4}>
          <div className="pf-v6-c-code-editor">
            <Header title="Topology" />
            <div className="pf-v6-c-code-editor__main">
              <div className="pf-v6-c-code-editor__code">
                <Topology unit={unit} />
              </div>
            </div>
          </div>
        </GridItem>
      </Grid>
    </>
  )
}
