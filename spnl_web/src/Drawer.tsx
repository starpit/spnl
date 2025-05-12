import { useMemo } from "react"
import {
  Button,
  Card,
  CardBody,
  CardHeader,
  CardTitle,
  Divider,
} from "@patternfly/react-core"

import Topology from "./Topology"

import CloseIcon from "@patternfly/react-icons/dist/esm/icons/times-icon"

type Props = {
  close(): void
  unit: null | import("./Unit").Unit
}

export default function Drawer({ close, unit }: Props) {
  const actions = useMemo(
    () => ({
      actions: <Button variant="plain" onClick={close} icon={<CloseIcon />} />,
    }),
    [close],
  )

  return (
    <Card isPlain isLarge>
      <CardHeader actions={actions}>
        <CardTitle>Query Viewer</CardTitle>
      </CardHeader>
      <Divider />
      <CardBody>{unit && <Topology unit={unit} />}</CardBody>
    </Card>
  )
}
