import { createRef, useEffect, useState } from "react"
import { Button, Tooltip } from "@patternfly/react-core"

import { Terminal } from "@xterm/xterm"
import { FitAddon } from "@xterm/addon-fit"
import { ClipboardAddon } from "@xterm/addon-clipboard"

import CopyIcon from "@patternfly/react-icons/dist/esm/icons/copy-icon"

import "@xterm/xterm/css/xterm.css"

export default function Console() {
  const xtermRef = createRef<HTMLDivElement>()
  const [term, setTerm] = useState<null | Terminal>(null)

  // Why a two-stage useEffect? Otherwise: cannot read properties of
  // undefined (reading 'dimensions')
  // See https://stackoverflow.com/a/78116690/5270773
  useEffect(() => {
    const term = new Terminal({
      fontFamily:
        '"Red Hat Mono", RedHatMono, "Courier New", Courier, monospace',
      convertEol: true,
    })
    setTerm(term)

    // for debugging:
    term.writeln(`\x1b[2mWaiting for first run\x1b[0m`)

    return () => {
      if (term) {
        term.dispose()
      }
    }
  }, [])

  useEffect(() => {
    if (term && xtermRef.current) {
      const fitAddon = new FitAddon()
      term.loadAddon(fitAddon)
      const clipboardAddon = new ClipboardAddon()
      term.loadAddon(clipboardAddon)

      term.open(xtermRef.current)
      fitAddon.fit()
      // term.focus()
    }
  }, [term, xtermRef])

  return (
    <div>
      <div className="pf-v6-c-code-editor">
        <div className="pf-v6-c-code-editor__header">
          <div className="pf-v6-c-code-editor__header-content">
            <div className="pf-v6-c-code-editor__controls">
              <Tooltip content="Copy to clipboard">
                <Button icon={<CopyIcon />} variant="plain" />
              </Tooltip>
            </div>
            <div className="pf-v6-c-code-editor__header-main">Console</div>
          </div>
        </div>

        <div ref={xtermRef} />
      </div>
    </div>
  )
}
