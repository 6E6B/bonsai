/**
 * Every token a theme can set, in the order the editor shows them. Names are the CSS custom
 * properties in `~/lib/ui/styles/theme.css` without the leading `--`. `tokens.test.ts` fails when
 * this list and that file disagree, so a new token can't land without the editor knowing it.
 */
export type TokenKind = "color" | "shadow" | "length" | "font"

export type Token = {
  name: string
  group: string
  kind: TokenKind
  label: string
}

const t = (group: string, kind: TokenKind, rows: [name: string, label: string][]): Token[] =>
  rows.map(([name, label]) => ({ name, group, kind, label }))

export const TOKENS: readonly Token[] = [
  ...t("Surfaces", "color", [
    ["color-deep", "Window chrome"],
    ["color-canvas", "Page"],
    ["color-surface", "Cards and panels"],
    ["color-raised", "Popovers and dialogs"],
    ["color-hover", "Hover overlay"],
    ["color-active", "Pressed or selected overlay"],
    ["color-field", "Inputs"],
    ["color-control", "Secondary buttons"],
    ["color-control-hover", "Secondary buttons, hover"],
  ]),
  ...t("Text", "color", [
    ["color-fg", "Text"],
    ["color-fg-muted", "Muted text"],
    ["color-fg-faint", "Faint text"],
  ]),
  ...t("Lines", "color", [
    ["color-line", "Borders"],
    ["color-line-strong", "Strong borders"],
  ]),
  ...t("Accent", "color", [
    ["color-accent", "Accent"],
    ["color-accent-hover", "Accent, hover"],
    ["color-accent-soft", "Accent tint"],
    ["color-focus", "Focus ring"],
    ["color-on-accent", "Text on fills"],
    ["color-contrast", "Primary button"],
    ["color-verified", "Verified names"],
  ]),
  ...t("Status", "color", [
    ["color-success", "Success"],
    ["color-success-soft", "Success tint"],
    ["color-warning", "Warning"],
    ["color-warning-soft", "Warning tint"],
    ["color-danger", "Danger"],
    ["color-danger-hover", "Danger, hover"],
    ["color-danger-soft", "Danger tint"],
  ]),
  ...t("Overlays", "color", [
    ["color-scrim", "Behind dialogs"],
    ["color-media-chip", "Chips on images"],
    ["color-on-media", "Marks on images"],
    ["color-hover-on-fill", "Hover on fills"],
    ["color-pressed-on-fill", "Pressed on fills"],
  ]),
  ...t("Presence", "color", [
    ["color-presence-online", "Online"],
    ["color-presence-game", "In game"],
    ["color-presence-studio", "In Studio"],
    ["color-presence-offline", "Offline"],
  ]),
  ...t("Account tags", "color", [
    ["color-tag-red", "Red"],
    ["color-tag-orange", "Orange"],
    ["color-tag-yellow", "Yellow"],
    ["color-tag-green", "Green"],
    ["color-tag-blue", "Blue"],
    ["color-tag-purple", "Purple"],
    ["color-tag-pink", "Pink"],
  ]),
  ...t("Switches", "color", [
    ["color-track", "Switch track, off"],
    ["color-knob", "Switch knob"],
    ["color-knob-edge", "Switch knob edge"],
  ]),
  ...t("Shadows", "shadow", [
    ["shadow-control", "Buttons"],
    ["shadow-fill", "Filled buttons"],
    ["shadow-field", "Inputs"],
    ["shadow-media", "Thumbnails"],
    ["shadow-well", "Wells"],
    ["shadow-well-on", "Wells, on"],
    ["shadow-knob", "Switch knob"],
    ["shadow-key", "Keys"],
    ["shadow-raised", "Cards"],
    ["shadow-popover", "Popovers"],
    ["shadow-dialog", "Dialogs"],
    ["text-shadow-fill", "Text on fills"],
  ]),
  ...t("Type", "font", [
    ["font-sans", "Text font"],
    ["font-mono", "Code font"],
  ]),
  ...t("Type", "length", [
    ["text-xs", "Extra small"],
    ["text-xs--line-height", "Extra small, line"],
    ["text-sm", "Small"],
    ["text-sm--line-height", "Small, line"],
    ["text-base", "Body"],
    ["text-base--line-height", "Body, line"],
    ["text-lg", "Large"],
    ["text-lg--line-height", "Large, line"],
    ["text-xl", "Heading"],
    ["text-xl--line-height", "Heading, line"],
    ["text-2xl", "Title"],
    ["text-2xl--line-height", "Title, line"],
  ]),
  ...t("Shape", "length", [
    ["radius-xs", "Radius, extra small"],
    ["radius-sm", "Radius, small"],
    ["radius-md", "Radius, medium"],
    ["radius-lg", "Radius, large"],
    ["radius-full", "Radius, round"],
  ]),
]

/**
 * Properties in `theme.css` that themes don't set, and why. Density and motion are settings;
 * the sheens and the image drop shadow are tuned for every theme alike.
 */
export const UNTHEMED: Readonly<Record<string, string>> = {
  "control-sm": "density setting",
  "control-md": "density setting",
  "control-lg": "density setting",
  duration: "motion",
  "duration-slow": "motion",
  ease: "motion",
  "ease-draw": "motion",
  sheen: "same in every theme",
  "sheen-soft": "same in every theme",
  "sheen-field": "same in every theme",
  "sheen-track": "same in every theme",
  "sheen-track-on": "same in every theme",
  "sheen-knob": "same in every theme",
  "shadow-on-media": "a filter over images; same in every theme",
}

export const TOKEN: ReadonlyMap<string, Token> = new Map(TOKENS.map((x) => [x.name, x]))

export type Tokens = Record<string, string>
