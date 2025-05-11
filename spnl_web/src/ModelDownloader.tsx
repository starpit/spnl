import { useState } from "react"
import {
  Select,
  SelectOption,
  SelectList,
  SelectGroup,
  MenuToggle,
  type MenuToggleElement,
  Divider,
} from "@patternfly/react-core"

import { prebuiltAppConfig } from "@mlc-ai/web-llm"

export default function ModelDownloader() {
  const [isOpen, setIsOpen] = useState(false)
  const [model, setModel] = useState("TinyLlama-1.1B-Chat-v0.4-q4f32_1-MLC-1k")

  const onToggleClick = () => {
    setIsOpen(!isOpen)
  }

  const onSelect = (
    _event: React.MouseEvent<Element, MouseEvent> | undefined,
    value: string | number | undefined,
  ) => {
    // eslint-disable-next-line no-console
    console.log("selected", value)

    setModel(value as string)
    setIsOpen(false)
  }

  const toggle = (toggleRef: React.Ref<MenuToggleElement>) => (
    <MenuToggle
      ref={toggleRef}
      onClick={onToggleClick}
      isExpanded={isOpen}
      style={
        {
          width: "200px",
        } as React.CSSProperties
      }
    >
      {model}
    </MenuToggle>
  )

  const groups = prebuiltAppConfig.model_list
    .map((m) => {
      const idx = m.model_id.indexOf("-")
      const group = m.model_id.slice(0, idx)
      const label = m.model_id.slice(idx + 1)
      console.error("!!!!!", group, label, m.model_id)
      return {
        group,
        label,
        model: m.model_id,
        description: `Context window: ${m.overrides?.context_window_size ?? "unknown"}. VRAM required: ${m.vram_required_MB}MB`,
      }
    })
    .reduce(
      (G, item) => {
        if (!(item.group in G)) {
          G[item.group] = []
        }
        G[item.group].push(item)
        return G
      },
      {} as Record<
        string,
        { group: string; label: string; model: string; description: string }[]
      >,
    )
  console.error("!!!!!!!", groups)

  return (
    <Select
      id="single-grouped-select"
      isScrollable
      isOpen={isOpen}
      selected={model}
      onSelect={onSelect}
      onOpenChange={(isOpen) => setIsOpen(isOpen)}
      toggle={toggle}
      shouldFocusToggleOnSelect
    >
      {Object.entries(groups).map(([group, items]) => (
        <>
          <SelectGroup label={group}>
            <SelectList>
              {items.map((item) => (
                <SelectOption value={item.model} description={item.description}>
                  {item.label}
                </SelectOption>
              ))}
            </SelectList>
          </SelectGroup>
          <Divider />
        </>
      ))}
    </Select>
  )
}
