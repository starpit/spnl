import { useState } from "react"
import useLocalStorageState from "use-local-storage-state"

import {
  Select,
  SelectOption,
  SelectList,
  MenuToggle,
  type MenuToggleElement,
} from "@patternfly/react-core"

import email from "../../spnl_cli/src/demos/email.lisp?raw"
import email2 from "../../spnl_cli/src/demos/email2.lisp?raw"
import email3 from "../../spnl_cli/src/demos/email3.lisp?raw"

const demos = [
  {
    label: "Email Judge/Generator",
    description:
      "This demo is the simplest query, but does not generate great output",
    value: email
      .replace(/\{n\}/g, "4")
      .replace(/\{model\}/g, "model")
      .replace(/\{temperature\}/g, "0.2")
      .replace(/\{max_tokens\}/g, "100"),
  },

  {
    label: "Improved Email Judge/Generator",
    description:
      "This demo generates better output, at the expense of a more complicated query",
    value: email2
      .replace(/\{n\}/g, "4")
      .replace(/\{model\}/g, "model")
      .replace(/\{temperature\}/g, "0.2")
      .replace(/\{max_tokens\}/g, "100"),
  },

  {
    label: "Policy-driven Email Generation",
    description: "This demonstrates using policies to guide email generation",
    value: email3
      .replace(/\{n\}/g, "4")
      .replace(/\{model\}/g, "model")
      .replace(/\{temperature\}/g, "0.2")
      .replace(/\{max_tokens\}/g, "100"),
  },
]

export default function DemoSelect() {
  const [isOpen, setIsOpen] = useState(false)
  const [selected, setSelected] = useLocalStorageState("spnl.demo.select", {
    defaultValue: demos[0].value,
  })

  const onToggleClick = () => {
    setIsOpen(!isOpen)
  }

  const onSelect = (
    _event: React.MouseEvent<Element, MouseEvent> | undefined,
    value: string | number | undefined,
  ) => {
    // eslint-disable-next-line no-console
    // console.log("selected", value)

    setSelected(value as string)
    setIsOpen(false)
  }

  const toggle = (toggleRef: React.Ref<MenuToggleElement>) => (
    <MenuToggle
      size="sm"
      ref={toggleRef}
      onClick={onToggleClick}
      isExpanded={isOpen}
    >
      {
        (demos.find((d) => d.value === selected) || { value: "internal error" })
          .label
      }
    </MenuToggle>
  )

  return (
    <Select
      isOpen={isOpen}
      selected={selected}
      onSelect={onSelect}
      onOpenChange={(isOpen) => setIsOpen(isOpen)}
      toggle={toggle}
      shouldFocusToggleOnSelect
    >
      <SelectList>
        {demos.map((demo) => (
          <SelectOption
            key={demo.value}
            value={demo.value}
            description={demo.description}
          >
            {demo.label}
          </SelectOption>
        ))}
      </SelectList>
    </Select>
  )
}
