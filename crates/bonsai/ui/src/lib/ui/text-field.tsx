import { TextField as KTextField } from "@kobalte/core/text-field"
import { type ComponentProps, Show, splitProps } from "solid-js"
import { Icon, type IconName } from "./icon"

export interface TextFieldProps extends Omit<ComponentProps<"input">, "value" | "onChange" | "onInput" | "size"> {
  /** A text area that grows with its content (capped at about five lines). */
  multiline?: boolean
  label?: string
  description?: string
  /** Marks the field invalid and shows this message under it. */
  error?: string
  /** Icon inside the field, before the text. Search bars pass `"search"`. */
  icon?: IconName
  size?: "sm" | "md" | "lg"
  value?: string
  defaultValue?: string
  onChange?: (value: string) => void
}

/** A text input with an optional label, description and error. Other props go to the `<input>`;
 * use `onChange` instead of `onInput`. */
export function TextField(props: TextFieldProps) {
  const [root, field, input] = splitProps(
    props,
    ["class", "size", "multiline"],
    [
      "label",
      "description",
      "error",
      "icon",
      "value",
      "defaultValue",
      "onChange",
      "disabled",
      "readOnly",
      "required",
      "name",
    ],
  )
  return (
    <KTextField
      data-component="text-field"
      data-size={root.size ?? "md"}
      data-multiline={root.multiline ? "" : undefined}
      class={root.class}
      value={field.value}
      defaultValue={field.defaultValue}
      onChange={field.onChange}
      validationState={field.error ? "invalid" : "valid"}
      disabled={field.disabled}
      readOnly={field.readOnly}
      required={field.required}
      name={field.name}
    >
      <Show when={field.label}>
        <KTextField.Label data-slot="field-label">{field.label}</KTextField.Label>
      </Show>
      <div data-slot="control">
        <Show when={field.icon}>{(name) => <Icon name={name()} size={14} />}</Show>
        <Show
          when={root.multiline}
          fallback={<KTextField.Input data-slot="input" {...(input as Record<string, unknown>)} />}
        >
          <KTextField.TextArea
            data-slot="input"
            autoResize
            submitOnEnter={false}
            {...(input as Record<string, unknown>)}
          />
        </Show>
      </div>
      <Show when={field.description}>
        <KTextField.Description data-slot="field-description">{field.description}</KTextField.Description>
      </Show>
      <KTextField.ErrorMessage data-slot="field-error">{field.error}</KTextField.ErrorMessage>
    </KTextField>
  )
}
