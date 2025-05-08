import { createRef, useEffect, useState } from "react"

import { Terminal } from "@xterm/xterm"
import { FitAddon } from "@xterm/addon-fit"
import { ClipboardAddon } from "@xterm/addon-clipboard"

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

      // for debugging:
      term.writeln(`\x1b[2mWaiting for first run\x1b[0m`)
    }
  }, [term, xtermRef])

  return <div ref={xtermRef} style={{ height: "400px" }} />
}
