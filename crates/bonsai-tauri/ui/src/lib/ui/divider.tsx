import { Separator as KSeparator } from "@kobalte/core/separator"
import type { ComponentProps } from "solid-js"

export interface DividerProps extends ComponentProps<"hr"> {
  orientation?: "horizontal" | "vertical"
}

export function Divider(props: DividerProps) {
  return <KSeparator data-component="divider" {...props} />
}
