import { Slider as KSlider } from "@kobalte/core/slider"
import { createSignal, onCleanup, Show } from "solid-js"
import { clock } from "./audio-clock"
import { IconButton } from "./button"
import { Spinner } from "./spinner"

/** What a screen can do to a player from a command. */
export interface AudioControl {
  toggle: () => void
}

export interface AudioPlayerProps {
  /** A local URL (`rbxasset://`); the webview can't load remote media. */
  src: string
  /** Accessible name for the seek bar, like the sound's name. */
  label: string
  ref?: (control: AudioControl) => void
  class?: string
}

/** Play, pause and seek one sound. Loads nothing until played. */
export function AudioPlayer(props: AudioPlayerProps) {
  const [playing, setPlaying] = createSignal(false)
  const [waiting, setWaiting] = createSignal(false)
  const [failed, setFailed] = createSignal(false)
  const [time, setTime] = createSignal(0)
  const [length, setLength] = createSignal(Number.NaN)
  const [seeking, setSeeking] = createSignal<number>()
  let audio!: HTMLAudioElement

  const toggle = () => {
    if (failed()) return
    if (audio.paused) {
      setWaiting(true)
      audio.play().catch(() => setWaiting(false))
    } else audio.pause()
  }
  props.ref?.({ toggle })
  onCleanup(() => audio?.pause())

  return (
    <div data-component="audio-player" class={props.class} data-failed={failed() ? "" : undefined}>
      {/* biome-ignore lint/a11y/useMediaCaption: Roblox sounds come without captions. */}
      <audio
        ref={audio}
        src={props.src}
        preload="none"
        onPlay={() => setPlaying(true)}
        onPause={() => setPlaying(false)}
        onEnded={() => setPlaying(false)}
        onPlaying={() => setWaiting(false)}
        onWaiting={() => setWaiting(true)}
        onTimeUpdate={() => setTime(audio.currentTime)}
        onDurationChange={() => setLength(audio.duration)}
        onError={() => {
          setFailed(true)
          setWaiting(false)
          setPlaying(false)
        }}
      />
      <Show
        when={!waiting()}
        fallback={
          <span data-slot="spinner">
            <Spinner size="sm" />
          </span>
        }
      >
        <IconButton
          icon={playing() ? "pause" : "play"}
          label={playing() ? "Pause" : "Play"}
          disabled={failed()}
          onClick={toggle}
        />
      </Show>
      <Show when={!failed()} fallback={<span data-slot="error">Roblox won't send this sound.</span>}>
        <KSlider
          data-slot="seek"
          aria-label={props.label}
          value={[seeking() ?? time()]}
          minValue={0}
          maxValue={Number.isFinite(length()) ? length() : 1}
          step={0.1}
          disabled={!Number.isFinite(length())}
          onChange={(v) => setSeeking(v[0])}
          onChangeEnd={(v) => {
            audio.currentTime = v[0] ?? 0
            setTime(audio.currentTime)
            setSeeking(undefined)
          }}
          getValueLabel={({ values }) => clock(values[0] ?? 0)}
        >
          <KSlider.Track data-slot="track">
            <KSlider.Fill data-slot="fill" />
            <KSlider.Thumb data-slot="thumb">
              <KSlider.Input />
            </KSlider.Thumb>
          </KSlider.Track>
        </KSlider>
        <span data-slot="time">
          {clock(seeking() ?? time())} / {clock(length())}
        </span>
      </Show>
    </div>
  )
}
