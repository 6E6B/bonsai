/** An authenticator key in groups of four, easier to read and type: `ABCD EFGH IJ`. */
export function groupKey(key: string): string {
  return (
    key
      .replace(/\s+/g, "")
      .match(/.{1,4}/g)
      ?.join(" ") ?? ""
  )
}
