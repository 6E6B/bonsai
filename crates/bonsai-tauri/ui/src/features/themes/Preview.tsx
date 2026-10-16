import { createMemo, createSignal, type JSX, onCleanup, onMount, Show } from "solid-js"
import type { Base, Tokens } from "~/lib/theme"
import {
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  Checkbox,
  IconButton,
  Kbd,
  ListItem,
  Meter,
  NavItem,
  Robux,
  SectionHeader,
  SegmentedControl,
  Switch,
  TextField,
} from "~/lib/ui"
import { styleRules, tokensAt } from "./inspect"

const noop = () => {}

/**
 * The app drawn in the draft's tokens, with the real components. The tokens go on the frame as
 * custom properties, so everything inside reads them instead of the app's. With `inspecting`,
 * hovering outlines an element and clicking reports the tokens that paint it.
 */
export function Preview(props: {
  tokens: Tokens
  base: Base
  view: "app" | "parts"
  inspecting: boolean
  onPick: (tokens: string[]) => void
}) {
  let frame: HTMLDivElement | undefined
  const [box, setBox] = createSignal<{ x: number; y: number; w: number; h: number }>()
  const vars = createMemo(() => {
    const out: JSX.CSSProperties = { "color-scheme": props.base }
    for (const [name, value] of Object.entries(props.tokens)) out[`--${name}`] = value
    return out
  })
  // Read once per inspect session; the app's stylesheets don't change while it runs.
  let rules: CSSStyleRule[] | undefined
  const target = (e: MouseEvent) => (e.target instanceof Element && frame?.contains(e.target) ? e.target : undefined)
  const onMove = (e: MouseEvent) => {
    const el = target(e)
    if (!el || !frame) return setBox(undefined)
    const r = el.getBoundingClientRect()
    const f = frame.getBoundingClientRect()
    setBox({ x: r.left - f.left, y: r.top - f.top, w: r.width, h: r.height })
  }
  const onClick = (e: MouseEvent) => {
    if (!props.inspecting) return
    e.preventDefault()
    e.stopPropagation()
    const el = target(e)
    if (!el || !frame) return
    rules ??= styleRules(Array.from(document.styleSheets))
    props.onPick(tokensAt(el, frame, rules))
  }
  // Capture, so a click while inspecting never reaches the button under it.
  const onHover = (e: MouseEvent) => props.inspecting && onMove(e)
  const onLeave = () => setBox(undefined)
  onMount(() => {
    frame?.addEventListener("click", onClick, true)
    frame?.addEventListener("mousemove", onHover)
    frame?.addEventListener("mouseleave", onLeave)
  })
  onCleanup(() => {
    frame?.removeEventListener("click", onClick, true)
    frame?.removeEventListener("mousemove", onHover)
    frame?.removeEventListener("mouseleave", onLeave)
    rules = undefined
  })

  return (
    <div
      ref={frame}
      data-inspecting={props.inspecting ? "" : undefined}
      class="relative flex min-h-[32rem] flex-1 overflow-hidden rounded-lg border border-line bg-canvas font-sans text-base text-fg"
      classList={{ "cursor-crosshair": props.inspecting }}
      style={vars()}
    >
      <Show when={props.view === "app"} fallback={<Parts />}>
        <AppView />
      </Show>
      <Show when={props.inspecting && box()}>
        {(b) => (
          <div
            aria-hidden="true"
            class="pointer-events-none absolute rounded-xs border-2 border-focus"
            style={{ left: `${b().x}px`, top: `${b().y}px`, width: `${b().w}px`, height: `${b().h}px` }}
          />
        )}
      </Show>
    </div>
  )
}

function AppView() {
  return (
    <>
      <aside class="flex w-44 flex-none flex-col gap-0.5 bg-deep p-2">
        <NavItem label="Home" icon="home" onClick={noop} />
        <NavItem label="Friends" icon="friends" active count={3} onClick={noop} />
        <NavItem label="Discover" icon="discover" onClick={noop} />
        <NavItem label="Marketplace" icon="marketplace" onClick={noop} />
        <NavItem label="Trades" icon="trades" count={1} onClick={noop} />
      </aside>
      <div class="flex min-w-0 flex-1 flex-col gap-3 overflow-auto p-4">
        <SectionHeader
          title="Friends"
          meta="3 online of 4"
          actions={<SegmentedControl size="sm" aria-label="Show" value="all" onChange={noop} options={opts} />}
        />
        <Card class="flex flex-col p-1">
          <ListItem
            leading={<Avatar name="Ada" size="sm" presence="game" />}
            title="Ada"
            subtitle="Playing Tower Run"
            meta={<Badge variant="success">In game</Badge>}
            actions={<Button size="sm">Join</Button>}
            selected
          />
          <ListItem
            leading={<Avatar name="Ben" size="sm" presence="online" />}
            title="Ben"
            subtitle="Online"
            unread
            onClick={noop}
          />
          <ListItem
            leading={<Avatar name="Cy" size="sm" presence="studio" />}
            title="Cy"
            subtitle="In Studio"
            onClick={noop}
          />
          <ListItem
            leading={<Avatar name="Dee" size="sm" presence="offline" />}
            title="Dee"
            subtitle="Last online 2 days ago"
            dim
            onClick={noop}
          />
        </Card>
        <div class="flex flex-wrap items-center gap-2">
          <Button variant="primary" size="sm">
            Send trade
          </Button>
          <Button size="sm" icon="messages">
            Message
          </Button>
          <Button variant="ghost" size="sm">
            Cancel
          </Button>
          <Button variant="danger" size="sm">
            Unfriend
          </Button>
          <IconButton size="sm" icon="more" label="More" />
          <Robux amount={12400} />
        </div>
        <div class="flex flex-wrap items-center gap-3">
          <TextField size="sm" icon="search" aria-label="Search" placeholder="Search friends" class="w-48" />
          <Switch checked onChange={noop}>
            Notify
          </Switch>
          <Checkbox checked onChange={noop}>
            Hide offline
          </Checkbox>
        </div>
        <Meter label="Server" value={18} max={20} tone="capacity" class="w-60" />
        <Alert title="Trade accepted" variant="success">
          Your items are on the way.
        </Alert>
        <Alert title="Cookie expires soon" variant="warning" />
      </div>
    </>
  )
}

const opts = [
  { value: "all", label: "All" },
  { value: "online", label: "Online" },
]

/** Raised things the app shows in portals (dialogs, menus, toasts), drawn in place. */
function Parts() {
  return (
    <div class="flex flex-1 flex-col gap-4 overflow-auto p-4">
      <div class="flex flex-wrap items-start gap-4">
        <div class="flex w-72 flex-col gap-3 rounded-lg bg-raised p-4 shadow-dialog">
          <div class="font-semibold text-lg">Remove friend?</div>
          <div class="text-fg-muted text-sm">You'd have to send a new request to be friends again.</div>
          <div class="flex justify-end gap-2">
            <Button size="sm" variant="ghost">
              Cancel
            </Button>
            <Button size="sm" variant="danger">
              Remove
            </Button>
          </div>
        </div>
        <div class="flex w-48 flex-col gap-0.5 rounded-md bg-raised p-1 text-sm shadow-popover">
          <div class="rounded-sm bg-hover px-2 py-1">Copy ID</div>
          <div class="px-2 py-1">Open in browser</div>
          <div class="px-2 py-1 text-danger">Block</div>
          <div class="flex justify-between px-2 py-1 text-fg-faint">
            Disabled <Kbd keys={["Ctrl", "B"]} />
          </div>
        </div>
        <div class="flex w-64 items-center gap-2 rounded-md bg-raised p-3 text-sm shadow-popover">
          <Avatar name="Ada" size="sm" presence="online" />
          <div class="flex-1">Ada sent you a trade</div>
        </div>
      </div>
      <Card class="flex flex-col gap-3 p-4">
        <div class="flex flex-wrap gap-2">
          <Badge variant="accent">Accent</Badge>
          <Badge variant="success">Success</Badge>
          <Badge variant="warning">Warning</Badge>
          <Badge variant="danger">Danger</Badge>
          <Badge>Neutral</Badge>
        </div>
        <div class="flex flex-wrap gap-2">
          <Badge variant="red">Red</Badge>
          <Badge variant="orange">Orange</Badge>
          <Badge variant="yellow">Yellow</Badge>
          <Badge variant="green">Green</Badge>
          <Badge variant="blue">Blue</Badge>
          <Badge variant="purple">Purple</Badge>
          <Badge variant="pink">Pink</Badge>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          <Button size="sm" variant="primary" disabled>
            Disabled
          </Button>
          <Button size="sm" disabled>
            Disabled
          </Button>
          <Kbd keys={["Ctrl", "K"]} />
          <span class="font-mono text-sm">mono 0123</span>
        </div>
        <Alert title="New version ready" variant="info" />
        <Alert title="Couldn't load the inventory" variant="danger" />
      </Card>
    </div>
  )
}
