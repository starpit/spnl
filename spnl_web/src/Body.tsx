import { useCallback, useState } from "react"
import {
  Grid,
  GridItem,
  Card,
  CardExpandableContent,
  CardHeader,
  CardTitle,
  CardBody,
} from "@patternfly/react-core"

import QueryEditor from "./QueryEditor"
import Console from "./Console"

import { compile_query } from "spnl_wasm"

// Sigh, not sure how to get this automatically from Rust, yet. Vite seems to be getting in the way.
type User = { user: [string] }
type System = { system: [string] }
type Print = { print: [string] }
type Plus = { plus: Unit[] }
type Cross = { cross: Unit[] }
type Generate = { generate: [string, Unit, number, number] }
type Unit = Print | User | System | Plus | Cross | Generate

export default function Body() {
  const onExecuteQuery = useCallback((query: string) => {
    console.log("Compiling query", query)
    const p = JSON.parse(compile_query(query)) as Unit
    console.error("!!!!!", p)
  }, [])

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
    <Grid hasGutter>
      <GridItem span={6}>
        <Card isLarge isExpanded={isExpanded1}>
          <CardHeader onExpand={toggleExpanded1}>
            <CardTitle>Query Editor</CardTitle>
          </CardHeader>
          <CardExpandableContent>
            <CardBody>
              <QueryEditor onExecuteQuery={onExecuteQuery} />
            </CardBody>
          </CardExpandableContent>
        </Card>
      </GridItem>

      <GridItem span={6}>
        <Card isLarge isExpanded={isExpanded2}>
          <CardHeader onExpand={toggleExpanded2}>
            <CardTitle>Console</CardTitle>
          </CardHeader>
          <CardExpandableContent>
            <CardBody>
              <Console />
            </CardBody>
          </CardExpandableContent>
        </Card>
      </GridItem>
    </Grid>
  )
}
