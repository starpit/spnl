import { useCallback, useEffect, useState } from "react"
import {
  Grid,
  GridItem,
  HelperText,
  HelperTextItem,
  Page,
  PageSection,
} from "@patternfly/react-core"

import Drawer from "./Drawer"
import Header from "./Header"
import Masthead from "./Masthead"
import QueryEditor from "./QueryEditor"
import Console, { type RunState } from "./Console"

import { compile_query } from "spnl_wasm"

import "@patternfly/react-core/dist/styles/base.css"

export type BodyProps = {
  /** Show topology */
  qv: boolean
}

export default function Body(props: BodyProps) {
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
    <Page
      masthead={<Masthead />}
      isNotificationDrawerExpanded={!!unit && props.qv}
      notificationDrawer={<Drawer unit={unit} />}
    >
      <PageSection>
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
          <GridItem span={7}>
            <QueryEditor
              isDrawerOpen={props.qv}
              setQuery={setQuery}
              onExecuteQuery={onExecuteQuery}
            />
          </GridItem>

          <GridItem span={5}>
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
        </Grid>
      </PageSection>
    </Page>
  )
}
