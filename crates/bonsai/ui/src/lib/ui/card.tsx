import { type ComponentProps, splitProps } from "solid-js"

export interface CardProps extends ComponentProps<"div"> {
  /** Inside another card: the control fill, so it stands out from the card around it. */
  nested?: boolean
}

/** A bordered panel. Pad and lay it out with Tailwind. */
export function Card(props: CardProps) {
  const [local, rest] = splitProps(props, ["nested"])
  return <div data-component="card" data-nested={local.nested ? "" : undefined} {...rest} />
}
