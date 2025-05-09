import { Button } from "@patternfly/react-core"

import ConsoleIcon from "@patternfly/react-icons/dist/esm/icons/terminal-icon"
import TopologyIcon from "@patternfly/react-icons/dist/esm/icons/project-diagram-icon"

type Props = {
  title: "Console" | "Topology"
}

export default function Header(props: Props) {
  return (
    <div className="pf-v6-c-code-editor__header">
      <div className="pf-v6-c-code-editor__header-content">
        <div className="pf-v6-c-code-editor__controls">
          <Button
            icon={
              props.title === "Console" ? <ConsoleIcon /> : <TopologyIcon />
            }
            variant="plain"
          />
        </div>
        <div className="pf-v6-c-code-editor__header-main">{props.title}</div>
      </div>
    </div>
  )
}
