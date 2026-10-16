import type { ComponentProps } from "solid-js"

/** A square frame with the same background as a `Thumbnail`, for content drawn on top of
 * it, like the 3D avatar canvas. Children are positioned against it. */
export function Viewport(props: ComponentProps<"div">) {
  return <div data-component="viewport" {...props} />
}
