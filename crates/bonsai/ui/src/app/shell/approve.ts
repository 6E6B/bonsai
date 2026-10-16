/** A typed or pasted quick sign-in code as Roblox takes it: letters and digits only,
 * upper-cased, at most six ("abc 123" → "ABC123"). */
export function quickCode(text: string): string {
  return text
    .replace(/[^a-z0-9]/gi, "")
    .toUpperCase()
    .slice(0, 6)
}
