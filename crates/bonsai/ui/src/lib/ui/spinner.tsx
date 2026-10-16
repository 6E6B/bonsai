import { type ComponentProps, splitProps } from "solid-js"

export interface SpinnerProps extends ComponentProps<"span"> {
  size?: "sm" | "md" | "lg"
}

export function Spinner(props: SpinnerProps) {
  const [local, rest] = splitProps(props, ["size"])
  return <span data-component="spinner" data-size={local.size ?? "md"} role="status" aria-label="Loading" {...rest} />
}
