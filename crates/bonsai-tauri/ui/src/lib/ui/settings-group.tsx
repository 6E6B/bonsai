import { type JSX, type ParentProps, Show } from "solid-js"
import { SectionHeader } from "./section-header"
import { Switch } from "./switch"

export interface SettingsGroupProps extends ParentProps {
  /** Heading above the panel. Leave out for the first group under a page title. */
  title?: string
  meta?: JSX.Element
  /** Buttons on the right of the heading, like Reset. */
  actions?: JSX.Element
  /** A line under the heading saying what the group is for or where it is saved. */
  description?: string
  /** For a panel of `ListItem`s: no side padding and no lines between rows. */
  list?: boolean
  class?: string
}

/** A titled panel of settings rows with a line between each. Children are `SettingRow`s,
 * `SettingSwitch`es or any block that should sit between lines. */
export function SettingsGroup(props: SettingsGroupProps) {
  return (
    <section data-component="settings-group" class={props.class}>
      <Show when={props.title}>{(t) => <SectionHeader title={t()} meta={props.meta} actions={props.actions} />}</Show>
      <Show when={props.description}>
        <p data-slot="description">{props.description}</p>
      </Show>
      <div data-slot="body" data-list={props.list ? "" : undefined}>
        {props.children}
      </div>
    </section>
  )
}

export interface SettingRowProps extends ParentProps {
  label: string
  description?: JSX.Element
  /** `stacked` puts the control under the label at full width: sliders, long fields. */
  layout?: "inline" | "stacked"
  class?: string
}

/** One setting: label and description on the left, its value or control on the right. */
export function SettingRow(props: SettingRowProps) {
  return (
    <div data-component="setting-row" data-layout={props.layout ?? "inline"} class={props.class}>
      <div data-slot="text">
        <span data-slot="label">{props.label}</span>
        <Show when={props.description}>
          <span data-slot="description">{props.description}</span>
        </Show>
      </div>
      <div data-slot="control">{props.children}</div>
    </div>
  )
}

export interface SettingSwitchProps {
  label: string
  description?: string
  checked?: boolean
  disabled?: boolean
  onChange?: (checked: boolean) => void
}

/** A setting that is on or off, laid out like a `SettingRow`. */
export function SettingSwitch(props: SettingSwitchProps) {
  return (
    <Switch
      layout="row"
      description={props.description}
      checked={props.checked}
      disabled={props.disabled}
      onChange={props.onChange}
    >
      {props.label}
    </Switch>
  )
}
