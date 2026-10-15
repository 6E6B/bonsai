import { Button as KButton } from "@kobalte/core/button"
import { Icon } from "./icon"

export interface WindowControlsProps {
  onMinimize: () => void
  onMaximize: () => void
  onClose: () => void
}

/** Minimize, maximize and close for the custom title bar. Full height of the bar, flush with
 * the window's right edge. */
export function WindowControls(props: WindowControlsProps) {
  return (
    <div data-component="window-controls">
      <KButton data-slot="button" aria-label="Minimize" onClick={() => props.onMinimize()}>
        <Icon name="minimize" size={14} />
      </KButton>
      <KButton data-slot="button" aria-label="Maximize" onClick={() => props.onMaximize()}>
        <Icon name="maximize" size={12} />
      </KButton>
      <KButton data-slot="button" data-close="" aria-label="Close" onClick={() => props.onClose()}>
        <Icon name="x" size={14} />
      </KButton>
    </div>
  )
}
