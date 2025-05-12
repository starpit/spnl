import useLocalStorageState from "use-local-storage-state"
import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type KeyboardEvent,
} from "react"

import {
  Button,
  Select,
  SelectOption,
  type SelectProps,
  SelectList,
  MenuToggle,
  type MenuToggleElement,
  type SelectOptionProps,
  TextInputGroup,
  TextInputGroupMain,
  TextInputGroupUtilities,
} from "@patternfly/react-core"

import { prebuiltAppConfig } from "@mlc-ai/web-llm"

import TimesIcon from "@patternfly/react-icons/dist/esm/icons/times-icon"

const width = { width: "25em" }
const NO_RESULTS = "no results"

const createItemId = (value: string) =>
  `select-typeahead-${value.replace(" ", "-")}`

const initialSelectOptions = prebuiltAppConfig.model_list
  .map((m) => ({
    value: m.model_id,
    children: m.model_id,
    isDisabled: false,
    isAriaDisabled: false,
    description: `Context window: ${m.overrides?.context_window_size ?? "unknown"}. VRAM required: ${m.vram_required_MB}MB`,
  }))
  .sort((a, b) => a.value.localeCompare(b.value))

export default function ModelDownloader() {
  const [isOpen, setIsOpen] = useState(false)
  const [selected, setSelected] = useLocalStorageState("spnl.model.default", {
    defaultValue: "TinyLlama-1.1B-Chat-v0.4-q4f32_1-MLC-1k",
  })
  const [inputValue, setInputValue] = useState(selected)
  const [filterValue, setFilterValue] = useState<string>("")
  const [selectOptions, setSelectOptions] =
    useState<SelectOptionProps[]>(initialSelectOptions)
  const [focusedItemIndex, setFocusedItemIndex] = useState<number | null>(null)
  const [activeItemId, setActiveItemId] = useState<string | null>(null)
  const textInputRef = useRef<HTMLInputElement>(undefined)

  const onToggleClick = useCallback(() => {
    setIsOpen((isOpen) => !isOpen)
  }, [setIsOpen])

  const resetActiveAndFocusedItem = useCallback(() => {
    setFocusedItemIndex(null)
    setActiveItemId(null)
  }, [setFocusedItemIndex, setActiveItemId])

  const closeMenu = useCallback(() => {
    setIsOpen(false)
    resetActiveAndFocusedItem()
  }, [setIsOpen, resetActiveAndFocusedItem])

  const selectOption = useCallback(
    (value: string | number, content: string | number) => {
      setInputValue(String(content))
      setFilterValue("")
      setSelected(String(value))

      closeMenu()
    },
    [closeMenu, setInputValue, setFilterValue, setSelected],
  )

  const onSelect = useCallback<Required<SelectProps>["onSelect"]>(
    (_event, value) => {
      if (value && value !== NO_RESULTS) {
        const optionText = selectOptions.find(
          (option) => option.value === value,
        )?.children
        selectOption(value, optionText as string)
      }
    },
    [selectOption, selectOptions],
  )

  useEffect(() => {
    let newSelectOptions: SelectOptionProps[] = initialSelectOptions

    // Filter menu items based on the text input value when one exists
    if (filterValue) {
      newSelectOptions = initialSelectOptions.filter((menuItem) =>
        String(menuItem.children)
          .toLowerCase()
          .includes(filterValue.toLowerCase()),
      )

      // When no options are found after filtering, display 'No results found'
      if (!newSelectOptions.length) {
        newSelectOptions = [
          {
            isAriaDisabled: true,
            children: `No results found for "${filterValue}"`,
            value: NO_RESULTS,
          },
        ]
      }

      // Open the menu when the input value changes and the new value is not empty
      if (!isOpen) {
        setIsOpen(true)
      }
    }

    setSelectOptions(newSelectOptions)
  }, [isOpen, filterValue])

  const setActiveAndFocusedItem = useCallback(
    (itemIndex: number) => {
      setFocusedItemIndex(itemIndex)
      const focusedItem = selectOptions[itemIndex]
      setActiveItemId(createItemId(focusedItem.value))
    },
    [setFocusedItemIndex, selectOptions, setActiveItemId],
  )

  const onTextInputChange = useCallback(
    (_event: React.FormEvent<HTMLInputElement>, value: string) => {
      setInputValue(value)
      setFilterValue(value)

      resetActiveAndFocusedItem()

      if (value !== selected) {
        setSelected("")
      }
    },
    [
      setInputValue,
      setFilterValue,
      resetActiveAndFocusedItem,
      selected,
      setSelected,
    ],
  )

  const onInputClick = useCallback(() => {
    if (!isOpen) {
      setIsOpen(true)
    } else if (!inputValue) {
      closeMenu()
    }
  }, [isOpen, setIsOpen, inputValue, closeMenu])

  const handleMenuArrowKeys = useCallback(
    (key: string) => {
      let indexToFocus = 0

      if (!isOpen) {
        setIsOpen(true)
      }

      if (selectOptions.every((option) => option.isDisabled)) {
        return
      }

      if (key === "ArrowUp") {
        // When no index is set or at the first index, focus to the last, otherwise decrement focus index
        if (focusedItemIndex === null || focusedItemIndex === 0) {
          indexToFocus = selectOptions.length - 1
        } else {
          indexToFocus = focusedItemIndex - 1
        }

        // Skip disabled options
        while (selectOptions[indexToFocus].isDisabled) {
          indexToFocus--
          if (indexToFocus === -1) {
            indexToFocus = selectOptions.length - 1
          }
        }
      }

      if (key === "ArrowDown") {
        // When no index is set or at the last index, focus to the first, otherwise increment focus index
        if (
          focusedItemIndex === null ||
          focusedItemIndex === selectOptions.length - 1
        ) {
          indexToFocus = 0
        } else {
          indexToFocus = focusedItemIndex + 1
        }

        // Skip disabled options
        while (selectOptions[indexToFocus].isDisabled) {
          indexToFocus++
          if (indexToFocus === selectOptions.length) {
            indexToFocus = 0
          }
        }
      }

      setActiveAndFocusedItem(indexToFocus)
    },
    [focusedItemIndex, isOpen, selectOptions, setActiveAndFocusedItem],
  )

  const onInputKeyDown = useCallback(
    (event: KeyboardEvent<HTMLInputElement>) => {
      const focusedItem =
        focusedItemIndex !== null ? selectOptions[focusedItemIndex] : null

      switch (event.key) {
        case "Enter":
          if (
            isOpen &&
            focusedItem &&
            focusedItem.value !== NO_RESULTS &&
            !focusedItem.isAriaDisabled
          ) {
            selectOption(focusedItem.value, focusedItem.children as string)
          }

          if (!isOpen) {
            setIsOpen(true)
          }

          break
        case "ArrowUp":
        case "ArrowDown":
          event.preventDefault()
          handleMenuArrowKeys(event.key)
          break
      }
    },
    [
      focusedItemIndex,
      selectOption,
      selectOptions,
      isOpen,
      setIsOpen,
      handleMenuArrowKeys,
    ],
  )

  const onClearButtonClick = useCallback(() => {
    setSelected("")
    setInputValue("")
    setFilterValue("")
    resetActiveAndFocusedItem()
    textInputRef?.current?.focus()
  }, [
    setSelected,
    setInputValue,
    setFilterValue,
    resetActiveAndFocusedItem,
    textInputRef,
  ])

  const toggle = (toggleRef: React.Ref<MenuToggleElement>) => (
    <MenuToggle
      ref={toggleRef}
      onClick={onToggleClick}
      isExpanded={isOpen}
      variant="typeahead"
      style={width}
    >
      <TextInputGroup isPlain>
        <TextInputGroupMain
          value={inputValue}
          onClick={onInputClick}
          onChange={onTextInputChange}
          onKeyDown={onInputKeyDown}
          id="typeahead-select-input"
          autoComplete="off"
          innerRef={textInputRef}
          placeholder="Select a model"
          {...(activeItemId && { "aria-activedescendant": activeItemId })}
          role="combobox"
          isExpanded={isOpen}
          aria-controls="select-typeahead-listbox"
        />

        <TextInputGroupUtilities
          {...(!inputValue ? { style: { display: "none" } } : {})}
        >
          <Button
            variant="plain"
            onClick={onClearButtonClick}
            aria-label="Clear input value"
            icon={<TimesIcon />}
          />
        </TextInputGroupUtilities>
      </TextInputGroup>
    </MenuToggle>
  )

  return (
    <Select
      isScrollable
      variant="typeahead"
      isOpen={isOpen}
      selected={selected}
      onSelect={onSelect}
      onOpenChange={(isOpen) => setIsOpen(isOpen)}
      toggle={toggle}
    >
      <SelectList>
        {selectOptions.map((option, idx) => (
          <SelectOption
            key={option.value || option.children}
            value={option.value}
            description={option.description}
            isFocused={focusedItemIndex === idx}
            id={createItemId(option.value)}
          >
            {option.value}
          </SelectOption>
        ))}
      </SelectList>
    </Select>
  )
}
