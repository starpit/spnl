import { useCallback } from "react"
import {
  Flex,
  FlexItem,
  Card,
  CardHeader,
  CardTitle,
  CardBody,
} from "@patternfly/react-core"

import QueryEditor from "./QueryEditor"
import Console from "./Console"

export default function Body() {
  const onExecuteQuery = useCallback((query: string) => {}, [])

  return (
    <Flex>
      <FlexItem flex={{ default: "flex_1" }}>
        <Card isPlain>
          <CardHeader>
            <CardTitle>Query Editor</CardTitle>
          </CardHeader>
          <CardBody>
            <QueryEditor onExecuteQuery={onExecuteQuery} />
          </CardBody>
        </Card>
      </FlexItem>

      <FlexItem>
        <Card isPlain>
          <CardHeader>
            <CardTitle>Console</CardTitle>
          </CardHeader>
          <CardBody>
            <Console />
          </CardBody>
        </Card>
      </FlexItem>
    </Flex>
  )
}
