import { useState } from "react"
import useLocalStorageState from "use-local-storage-state"

import {
  Select,
  SelectOption,
  SelectList,
  MenuToggle,
  type MenuToggleElement,
} from "@patternfly/react-core"

const demos = [
  {
    label: "Email Judge/Generator",
    description:
      "This demo is the simplest query, but does not generate great output",
    value: `(g "ollama/granite3.2:2b"
   (cross
    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate 4 candidate emails in parallel")
    (plus
     (repeat 4
             (g "ollama/granite3.2:2b"
                (user "write an introductory email for a job application, limited to at most 100 characters.")
                100 0.3)))

    (print "Done generating candidate emails. Now asking the model to select the best option from the candidates")
    (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department"))
0 0.0)`,
  },

  {
    label: "Improved Email Judge/Generator",
    description:
      "This demo generates better output, at the expense of a more complicated query",
    value: `(g "ollama/granite3.2:2b"
   (cross
    (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")

    (print "Generate 4 candidate emails in parallel")
    (plus
     (repeat 4
             (g "ollama/granite3.2:2b"
                (cross
                 (system "You compute an evaluation score from 0 to 100 that ranks given candidate introductory emails. Better emails are ones that mention specifics, such as names of people and companies. You present a list of the top 3 ordered by their rank showing the score and full content of each.")
                 (user "write an introductory email for a job application, limited to at most 100 characters.")

                 (user "My name is Shiloh. I am a data scientist with 10 years of experience and need an introductory email to apply for a position at IBM in their research department")
                 )

                100 0.2
                )
             )
     )
    )
   0 0.0
   )`,
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
    console.log("selected", value)

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
