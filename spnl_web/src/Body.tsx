import { useCallback, useEffect, useState } from "react"
import {
  Grid,
  GridItem,
  Card,
  CardExpandableContent,
  CardHeader,
  CardTitle,
  CardBody,
  HelperText,
  HelperTextItem,
} from "@patternfly/react-core"

import Header from "./Header"
import Console from "./Console"
import Topology from "./Topology"
import QueryEditor from "./QueryEditor"

import run from "./run"

import { compile_query } from "spnl_wasm"

export default function Body() {
  const [unit, setUnit] = useState<null | Unit>(null)
  const [query, setQuery] = useState<null | string>(null)
  const [compilationError, setCompilationError] = useState<null | Error>(null)

  useEffect(() => {
    if (!query) {
      setUnit(null)
    } else {
      try {
        setUnit(JSON.parse(compile_query(query)) as import("./Unit").Unit)
        setCompilationError(null)
      } catch (err) {
        console.error(err)
        setCompilationError(err)
      }
    }
  }, [query, setUnit])

  const onExecuteQuery = useCallback(() => run(unit), [unit])

  const [isExpanded1, setIsExpanded1] = useState(true)
  const [isExpanded2, setIsExpanded2] = useState(true)
  const toggleExpanded1 = useCallback(
    () => setIsExpanded1((v) => !v),
    [setIsExpanded1],
  )
  const toggleExpanded2 = useCallback(
    () => setIsExpanded2((v) => !v),
    [setIsExpanded2],
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
            <Console />
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
