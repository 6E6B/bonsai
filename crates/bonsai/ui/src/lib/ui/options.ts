/** An option for Select and RadioGroup. A plain string is its own value and label. */
export type Option = string | { value: string; label: string; disabled?: boolean; hint?: string }

export interface NormalOption {
  value: string
  label: string
  disabled: boolean
  /** Muted text after the label, like the prefix that picks it (SegmentedControl only). */
  hint?: string
}

export function normalize(options: Option[]): NormalOption[] {
  return options.map((o) =>
    typeof o === "string" ? { value: o, label: o, disabled: false } : { ...o, disabled: o.disabled ?? false },
  )
}
