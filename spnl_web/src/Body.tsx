import { useCallback, useEffect, useState } from "react"
import {
  Grid,
  GridItem,
  Card,
  CardExpandableContent,
  CardHeader,
  CardTitle,
  CardBody,
} from "@patternfly/react-core"

import Console from "./Console"
import Topology from "./Topology"
import QueryEditor from "./QueryEditor"

import { compile_query } from "spnl_wasm"

export default function Body() {
  const [unit, setUnit] = useState<null | Unit>(null)
  const [query, setQuery] = useState<null | string>(null)

  useEffect(() => {
    if (query === null) {
      setUnit(null)
    } else {
      setUnit(JSON.parse(compile_query(query)) as import("./Unit").Unit)
    }
  }, [query, setUnit])

  const onExecuteQuery = useCallback(() => {
    console.log("Execute query", query)
  }, [query])

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
      <GridItem span={8}>
        <QueryEditor setQuery={setQuery} onExecuteQuery={onExecuteQuery} />
      </GridItem>

      <GridItem span={4}>
        <Topology unit={unit} />
      </GridItem>

      <GridItem span={12}>
        <Console />
      </GridItem>
    </Grid>
  )
}
