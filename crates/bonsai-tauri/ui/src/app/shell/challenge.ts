/** Names of the places a two-step code comes from, by rbx-core's method key. */
export const METHOD_LABELS: Record<string, string> = {
  authenticator: "Authenticator app",
  email: "Email",
}

/** The digits of a typed or pasted code, at most six ("123 456" → "123456"). */
export function codeDigits(text: string): string {
  return text.replace(/\D/g, "").slice(0, 6)
}

/** The digits of a typed account PIN, at most four. */
export function pinDigits(text: string): string {
  return text.replace(/\D/g, "").slice(0, 4)
}
