import { For, Show } from "solid-js"
import { menuItem, run, shortcutOf } from "~/lib/commands/registry"
import { dateTime } from "~/lib/format"
import type { Account, VoiceBanView } from "~/lib/ipc/bindings"
import { activeVoiceBans, clock, useNowSecs } from "~/lib/moderation"
import { shell } from "~/lib/store"
import { accountName, streamerUi } from "~/lib/streamer"
import { Badge, Divider, Icon, Menu, Robux, redacting, Spinner, TextLink, Tooltip } from "~/lib/ui"

/** Voice bans shown one by one before the rest fold into "+N". */
const SHOWN_BANS = 3

/** The bottom line: who you are acting as, balance, background work, connectivity. */
export function StatusBar(props: { zoom: number }) {
  const active = () => shell.accounts.find((a) => a.id === shell.active)
  return (
    <footer
      data-window-corner="bottom"
      class="flex h-7 shrink-0 items-center gap-3 border-t border-line bg-deep px-3 text-sm text-fg-muted"
    >
      <Show when={active()} fallback={<span>No account</span>}>
        {(a) => (
          <>
            <TextLink tone="muted" onClick={() => run("nav.profile")}>
              <span class="text-fg">{accountName(a())}</span>
              <Show when={!redacting()}> @{a().name}</Show>
            </TextLink>
            <Show when={shell.robux !== null}>
              <Divider orientation="vertical" class="my-1.5" />
              <Robux amount={shell.robux ?? 0} private />
            </Show>
          </>
        )}
      </Show>
      <Show when={shell.jobs > 0}>
        <span class="flex items-center gap-1.5">
          <Spinner size="sm" />
          {shell.jobs} running
        </span>
      </Show>
      <Show when={shell.offline}>
        <Badge variant="warning">Offline</Badge>
      </Show>
      <VoiceBans />
      <div class="ml-auto flex items-center gap-3">
        <StreamerPill />
        <Show when={props.zoom !== 1}>
          <TextLink tone="muted" onClick={() => run("view.zoom_reset")}>
            {Math.round(props.zoom * 100)}%
          </TextLink>
        </Show>
        <TextLink tone="muted" onClick={() => run("help.shortcuts")}>
          <span class="text-fg">{shortcutOf("help.shortcuts") ?? "?"}</span> shortcuts
        </TextLink>
      </div>
    </footer>
  )
}

type Banned = Account & { voiceBan: VoiceBanView }

const banName = (a: Banned) => accountName(a)
const banLeft = (a: Banned, now: number) => (a.voiceBan.until === null ? "no end date" : clock(a.voiceBan.until - now))

const banTip = (a: Banned) =>
  a.voiceBan.until === null
    ? `${banName(a)} is voice banned. Roblox didn't say when it ends.`
    : `${banName(a)} is voice banned until ${dateTime(a.voiceBan.until * 1000)}.`

/** A live countdown for every account with a voice ban. Clicking one switches to it. */
function VoiceBans() {
  const now = useNowSecs(1000)
  const bans = () => activeVoiceBans(shell.accounts, now())
  const shown = () => bans().slice(0, SHOWN_BANS)
  const rest = () => bans().slice(SHOWN_BANS)
  return (
    <Show when={bans().length > 0}>
      <span class="flex min-w-0 items-center gap-3">
        <For each={shown()}>
          {(a) => (
            <Tooltip content={banTip(a)}>
              <TextLink tone="muted" onClick={() => run("account.switch", String(a.id))}>
                <span class="flex items-center gap-1">
                  <span class="text-warning">
                    <Icon name="voice-banned" size={14} />
                  </span>
                  <span class="text-fg">{banName(a)}</span>
                  <span class="tabular-nums">{banLeft(a, now())}</span>
                </span>
              </TextLink>
            </Tooltip>
          )}
        </For>
        <Show when={rest().length > 0}>
          <Tooltip
            content={
              <span class="flex flex-col">
                <For each={rest()}>
                  {(a) => (
                    <span>
                      {banName(a)} {banLeft(a, now())}
                    </span>
                  )}
                </For>
              </span>
            }
          >
            <span>+{rest().length} voice banned</span>
          </Tooltip>
        </Show>
      </span>
    </Show>
  )
}

/** Shown while streamer mode is on. Opens a menu to peek, turn it off or change it. */
function StreamerPill() {
  return (
    <Show when={shell.streamer.on}>
      <Menu
        variant="ghost"
        size="sm"
        placement="top-start"
        items={[
          menuItem("streamer.reveal", { icon: "eye", disabled: streamerUi.revealing }),
          menuItem("streamer.toggle", { label: "Turn off streamer mode", arg: "off" }),
          menuItem("nav.settings", { label: "Streamer mode settings", arg: "streamer", icon: "settings" }),
        ]}
      >
        <span data-streamer-pill class="flex items-center gap-1.5">
          <Icon name="video" size={14} />
          <Show when={streamerUi.revealing} fallback="Streamer mode">
            Showing hidden content
          </Show>
        </span>
      </Menu>
    </Show>
  )
}
