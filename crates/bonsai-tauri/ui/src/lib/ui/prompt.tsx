import { createSignal, onMount } from "solid-js"
import { Button } from "./button"
import { useDialog } from "./dialog"
import { TextField } from "./text-field"

export interface PromptOptions {
  title: string
  label: string
  initial?: string
  placeholder?: string
  /** Text of the submit button. Default "Save". */
  submit?: string
  /** Lets an empty answer through, for clearing a value. */
  allowEmpty?: boolean
  maxLength?: number
}

export interface ConfirmOptions {
  title: string
  description?: string
  /** Text of the confirm button, a verb: "Remove", "Leave". */
  confirm: string
  danger?: boolean
}

/** Asks for one line of text in the app dialog. Resolves to null when cancelled. */
export function usePrompt() {
  const dialog = useDialog()
  return (opts: PromptOptions) =>
    new Promise<string | null>((resolve) => {
      let done = false
      const finish = (v: string | null) => {
        if (done) return
        done = true
        dialog.close()
        resolve(v)
      }
      dialog.show(() => <PromptForm {...opts} onDone={finish} />, {
        title: opts.title,
        size: "sm",
        onClose: () => finish(null),
      })
    })
}

function PromptForm(props: PromptOptions & { onDone: (v: string | null) => void }) {
  const [value, setValue] = createSignal(props.initial ?? "")
  let input: HTMLInputElement | undefined
  onMount(() => input?.select())
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        props.onDone(value().trim())
      }}
    >
      <TextField
        ref={input}
        label={props.label}
        placeholder={props.placeholder}
        maxLength={props.maxLength}
        value={value()}
        onChange={setValue}
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone(null)}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={!props.allowEmpty && value().trim() === ""}>
          {props.submit ?? "Save"}
        </Button>
      </div>
    </form>
  )
}

/** Asks the user to confirm a destructive or hard-to-undo action. */
export function useConfirm() {
  const dialog = useDialog()
  return (opts: ConfirmOptions) =>
    new Promise<boolean>((resolve) => {
      let done = false
      const finish = (v: boolean) => {
        if (done) return
        done = true
        dialog.close()
        resolve(v)
      }
      dialog.show(
        () => (
          <div class="flex justify-end gap-2 p-4">
            <Button variant="ghost" onClick={() => finish(false)}>
              Cancel
            </Button>
            <Button variant={opts.danger ? "danger" : "primary"} onClick={() => finish(true)} autofocus>
              {opts.confirm}
            </Button>
          </div>
        ),
        { title: opts.title, description: opts.description, size: "sm", onClose: () => finish(false) },
      )
    })
}
