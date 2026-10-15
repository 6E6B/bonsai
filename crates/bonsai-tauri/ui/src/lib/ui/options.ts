/** An option for Select and RadioGroup. A plain string is its own value and label. */
export type Option = string | { value: string; label: string; disabled?: boolean }

export interface NormalOption {
  value: string
  label: string
  disabled: boolean
}

export function normalize(options: Option[]): NormalOption[] {
  return options.map((o) =>
    typeof o === "string" ? { value: o, label: o, disabled: false } : { ...o, disabled: o.disabled ?? false },
  )
}
