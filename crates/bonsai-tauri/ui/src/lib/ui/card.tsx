import type { ComponentProps } from "solid-js"

/** A bordered panel. Pad and lay it out with Tailwind. */
export function Card(props: ComponentProps<"div">) {
  return <div data-component="card" {...props} />
}
