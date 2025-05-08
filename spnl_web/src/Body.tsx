import { Flex, FlexItem, Card, CardHeader, CardTitle, CardBody } from "@patternfly/react-core"

import QueryEditor from "./QueryEditor"
import Console from "./Console"

export default function Body() {
  return (
    <Flex hasGutter>
      <FlexItem flex={{default: "flex_1"}}>
        <Card isPlain>
          <CardHeader><CardTitle>Query Editor</CardTitle></CardHeader>
          <CardBody><QueryEditor/></CardBody>
        </Card>
      </FlexItem>
        
      <FlexItem>
        <Card isPlain>
          <CardHeader><CardTitle>Console</CardTitle></CardHeader>
          <CardBody><Console/></CardBody>
        </Card>
        </FlexItem>
    </Flex>
  )
}

