import { Select as KSelect } from "@kobalte/core/select"
import { createMemo, Show } from "solid-js"
import { Icon } from "./icon"
import { type NormalOption, normalize, type Option } from "./options"

export interface SelectProps {
  options: Option[]
  value?: string
  defaultValue?: string
  onChange?: (value: string) => void
  placeholder?: string
  label?: string
  /** Accessible name when there is no visible label. */
  "aria-label"?: string
  size?: "sm" | "md" | "lg"
  disabled?: boolean
  class?: string
}

export function Select(props: SelectProps) {
  const options = createMemo(() => normalize(props.options))
  const find = (v: string | undefined) => (v === undefined ? undefined : (options().find((o) => o.value === v) ?? null))
  return (
    <KSelect<NormalOption>
      data-component="select"
      data-size={props.size ?? "md"}
      class={props.class}
      options={options()}
      optionValue="value"
      optionTextValue="label"
      optionDisabled="disabled"
      value={find(props.value)}
      defaultValue={find(props.defaultValue) ?? undefined}
      onChange={(o) => o && props.onChange?.(o.value)}
      placeholder={props.placeholder}
      disabled={props.disabled}
      gutter={4}
      itemComponent={(p) => (
        <KSelect.Item item={p.item} data-slot="item">
          <KSelect.ItemLabel>{p.item.rawValue.label}</KSelect.ItemLabel>
          <KSelect.ItemIndicator data-slot="check">
            <Icon name="check" size={14} />
          </KSelect.ItemIndicator>
        </KSelect.Item>
      )}
    >
      <Show when={props.label}>
        <KSelect.Label data-slot="field-label">{props.label}</KSelect.Label>
      </Show>
      <KSelect.Trigger data-slot="trigger" aria-label={props["aria-label"]}>
        <KSelect.Value<NormalOption> data-slot="value">{(s) => s.selectedOption().label}</KSelect.Value>
        <KSelect.Icon data-slot="chevron">
          <Icon name="chevron-down" size={14} />
        </KSelect.Icon>
      </KSelect.Trigger>
      <KSelect.Portal>
        <KSelect.Content data-component="select-content">
          <KSelect.Listbox data-slot="listbox" />
        </KSelect.Content>
      </KSelect.Portal>
    </KSelect>
  )
}
