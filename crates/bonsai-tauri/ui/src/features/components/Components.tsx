import { createSignal, For, type ParentProps } from "solid-js"
import { applyAppearance, DEFAULT_SEEDS, derive, type Mode } from "~/lib/theme"
import {
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  Checkbox,
  Clamp,
  Collapsible,
  ColorPicker,
  type Column,
  Combobox,
  ContextMenu,
  DataTable,
  Dialog,
  Divider,
  EmojiPicker,
  EmptyState,
  Fact,
  FactGroup,
  Facts,
  Icon,
  IconButton,
  type IconName,
  Kbd,
  ListItem,
  MediaFrame,
  Mention,
  Menu,
  Meter,
  NavItem,
  PersonCard,
  Pips,
  RadioGroup,
  RailItem,
  Robux,
  SearchButton,
  SectionHeader,
  SegmentedControl,
  Select,
  SettingRow,
  SettingSwitch,
  SettingsGroup,
  Skeleton,
  Slider,
  Spinner,
  Stat,
  type StripTab,
  Swatch,
  Switch,
  showToast,
  TabPanel,
  TabStrip,
  Tabs,
  TextField,
  TextLink,
  ThemeCard,
  Thumbnail,
  Tile,
  ToggleButton,
  Tooltip,
  useConfirm,
  usePrompt,
  VerifiedBadge,
  Viewport,
  VirtualGrid,
  WindowControls,
} from "~/lib/ui"

const icons: IconName[] = [
  "home",
  "friends",
  "discover",
  "avatar",
  "marketplace",
  "inventory",
  "trades",
  "messages",
  "chat",
  "communities",
  "settings",
  "bell",
  "star",
  "heart",
  "thumbs-up",
  "play",
  "more",
  "refresh",
  "external",
  "robux",
  "dollar",
  "search",
  "server",
  "badge",
  "game",
  "sort-asc",
  "sort-desc",
  "video",
]

interface Row {
  id: string
  playing: number
  ping: number
}

const rows: Row[] = Array.from({ length: 300 }, (_, i) => ({
  id: (0x6758fdfd + i * 4099).toString(16),
  playing: (i * 7) % 51,
  ping: 1 + ((i * 13) % 140),
}))

const columns: Column<Row>[] = [
  {
    key: "players",
    label: "Players",
    width: "10rem",
    cell: (r) => <Meter class="w-full" label="Players" value={r.playing} max={50} tone="capacity" />,
  },
  { key: "ping", label: "Ping", width: "5rem", cell: (r) => `${r.ping} ms` },
  { key: "id", label: "Server", mono: true, cell: (r) => r.id },
  { key: "join", label: "", width: "4rem", align: "end", cell: () => <Button size="sm">Join</Button> },
]

const noop = () => {}

const buttonVariants = ["primary", "secondary", "ghost", "danger"] as const
const badgeVariants = ["neutral", "accent", "success", "warning", "danger"] as const

function Section(props: ParentProps<{ title: string }>) {
  return (
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-medium text-fg-muted">{props.title}</h2>
      <div class="flex flex-wrap items-center gap-3">{props.children}</div>
    </section>
  )
}

/** Dev gallery of our UI components. */
const galleryThemes = [
  { name: "Dark", detail: "Bonsai", tokens: derive("dark", DEFAULT_SEEDS.dark) },
  { name: "Light", detail: "Bonsai", tokens: derive("light", DEFAULT_SEEDS.light) },
  {
    name: "Ember",
    detail: "Warm, rounder",
    tokens: derive("dark", { ...DEFAULT_SEEDS.dark, background: "#1c1714", accent: "#d9612b", radius: 1.5 }),
  },
]

const galleryFonts = ["Inter", "Space Grotesk", "IBM Plex Sans", "Manrope", "Work Sans"]

export default function Components() {
  const [height, setHeight] = createSignal(100)
  const [swatch, setSwatch] = createSignal("var(--color-accent)")
  const [picked, setPicked] = createSignal("#3d7dff")
  const [tint, setTint] = createSignal("#3d7dff33")
  const [themeCard, setThemeCard] = createSignal("Dark")
  const [sort, setSort] = createSignal("Popular")
  const [font, setFont] = createSignal("Inter")
  const [fontQuery, setFontQuery] = createSignal("")
  const [open, setOpen] = createSignal(false)
  const [emojiOpen, setEmojiOpen] = createSignal(false)
  const [emoji, setEmoji] = createSignal("🙂")
  const [tab, setTab] = createSignal("servers")
  const [strip, setStrip] = createSignal("b")
  const [stripTabs, setStripTabs] = createSignal<StripTab[]>([
    { key: "p", title: "Trades", icon: "trades", pinned: true },
    { key: "a", title: "Home", icon: "home" },
    { key: "b", title: "Tile Cats", icon: "game" },
    { key: "c", title: "A very long profile name that truncates", icon: "user" },
  ])
  const [seg, setSeg] = createSignal("ping")
  const [fav, setFav] = createSignal(false)
  const prompt = usePrompt()
  const confirm = useConfirm()

  return (
    <div class="flex max-w-4xl flex-col gap-10 p-6">
      <div class="flex items-center justify-between">
        <h1 class="text-xl font-medium">Components</h1>
        <Select
          aria-label="Theme"
          size="sm"
          options={["system", "dark", "light"]}
          defaultValue="system"
          onChange={(v) => applyAppearance(v as Mode)}
        />
      </div>

      <Section title="Button">
        <For each={buttonVariants}>{(v) => <Button variant={v}>{v}</Button>}</For>
      </Section>
      <Section title="Button sizes and states">
        <Button size="sm">Small</Button>
        <Button>Medium</Button>
        <Button size="lg">Large</Button>
        <Button icon="plus">With icon</Button>
        <Button loading>Loading</Button>
        <Button disabled>Disabled</Button>
        <IconButton icon="copy" label="Copy" />
        <IconButton icon="search" label="Search" variant="secondary" />
      </Section>

      <Section title="Avatar">
        <Avatar size="sm" name="Builderman" />
        <Avatar name="Roblox" />
        <Avatar size="lg" name="Bonsai App" />
      </Section>

      <Section title="Badge">
        <For each={badgeVariants}>{(v) => <Badge variant={v}>{v}</Badge>}</For>
      </Section>

      <Section title="Kbd">
        <Kbd keys={["Ctrl", "K"]} />
        <Kbd keys={["G", "H"]} />
      </Section>

      <Section title="Emoji picker">
        <EmojiPicker
          label="Emoji"
          placement="bottom-start"
          open={emojiOpen()}
          onOpenChange={setEmojiOpen}
          onPick={setEmoji}
        />
        <span class="text-2xl">{emoji()}</span>
      </Section>

      <Section title="Theme card">
        <For each={galleryThemes}>
          {(t) => (
            <ThemeCard
              class="w-40"
              name={t.name}
              detail={t.detail}
              tokens={t.tokens}
              selected={themeCard() === t.name}
              onClick={() => setThemeCard(t.name)}
            />
          )}
        </For>
      </Section>

      <Section title="Text field">
        <div class="flex w-full max-w-md flex-col gap-4">
          <TextField label="Display name" placeholder="Builderman" description="Shown to other players." />
          <TextField label="User ID" value="156" readOnly />
          <TextField label="Invalid" value="bad value" error="Names can't contain spaces." />
          <TextField size="lg" icon="search" aria-label="Search" placeholder="Search" />
        </div>
      </Section>

      <Section title="Select">
        <Select label="Sort" options={["Popular", "Top rated", "Newest"]} value={sort()} onChange={setSort} />
        <Select size="sm" aria-label="Small" options={["One", "Two"]} placeholder="Small" />
      </Section>

      <Section title="Combobox">
        <Combobox
          label="Font"
          placeholder="Pick a font"
          searchPlaceholder="Search fonts"
          options={galleryFonts
            .filter((f) => f.toLowerCase().includes(fontQuery().toLowerCase()))
            .map((f) => ({ value: f, label: f, detail: "Sans Serif" }))}
          value={font()}
          onSearch={setFontQuery}
          onChange={setFont}
        />
        <Combobox size="sm" aria-label="Loading" options={[]} placeholder="Opens to a loading list" loading />
      </Section>
      <Section title="Switch and checkbox">
        <Switch defaultChecked>Show online friends first</Switch>
        <Switch disabled>Disabled</Switch>
        <Switch layout="row" class="w-72" description="Label first, switch at the end. For settings lists.">
          Row layout
        </Switch>
        <Checkbox defaultChecked>Remember me</Checkbox>
        <Checkbox description="Only for this account.">Hide offline</Checkbox>
      </Section>

      <Section title="Slider and swatch">
        <Slider
          class="w-56"
          label="Height"
          value={height()}
          min={90}
          max={105}
          onChange={setHeight}
          format={(v) => `${v}%`}
        />
        <For each={["var(--color-accent)", "var(--color-warning)", "var(--color-danger)"]}>
          {(c) => <Swatch color={c} label={c} selected={swatch() === c} onClick={() => setSwatch(c)} />}
        </For>
      </Section>

      <Section title="Color picker">
        <ColorPicker class="w-56" label="Accent" value={picked()} onChange={setPicked} />
        <ColorPicker class="w-56" label="Tint" alpha value={tint()} onChange={setTint} />
      </Section>

      <Section title="Radio">
        <RadioGroup
          label="Privacy"
          defaultValue="friends"
          options={["Everyone", "Friends", "No one"].map((l) => ({ value: l.toLowerCase(), label: l }))}
        />
        <RadioGroup label="Horizontal" orientation="horizontal" defaultValue="A" options={["A", "B", "C"]} />
      </Section>

      <Section title="Toast">
        <Button onClick={() => showToast("Copied link")}>Plain</Button>
        <Button onClick={() => showToast({ title: "Saved", description: "Outfit saved.", variant: "success" })}>
          Success
        </Button>
        <Button
          onClick={() =>
            showToast({
              title: "Rate limited (catalog.roblox.com)",
              variant: "warning",
              countdownSecs: 10,
              key: "catalog.roblox.com",
            })
          }
        >
          Warning
        </Button>
        <Button onClick={() => showToast({ title: "Failed", description: "Server error.", variant: "error" })}>
          Error
        </Button>
      </Section>

      <Section title="Tooltip">
        <Tooltip content="Join this server">
          <Button>Hover me</Button>
        </Tooltip>
        <Tooltip content="Command bar" keys={["Ctrl", "K"]} placement="bottom">
          <IconButton icon="search" label="Command bar" />
        </Tooltip>
      </Section>

      <Section title="Dialog">
        <Button onClick={() => setOpen(true)}>Open dialog</Button>
        <Dialog open={open()} onOpenChange={setOpen} title="Leave group" description="You can rejoin later." size="sm">
          <div class="flex justify-end gap-2 p-4">
            <Button variant="ghost" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button variant="danger" onClick={() => setOpen(false)}>
              Leave
            </Button>
          </div>
        </Dialog>
      </Section>

      <Section title="Card and divider">
        <Card class="flex w-64 flex-col gap-2 p-3">
          <span class="font-medium">Card</span>
          <Divider />
          <span class="text-fg-muted">Muted text inside a card.</span>
        </Card>
      </Section>

      <Section title="Settings group">
        <div class="flex w-[28rem] flex-col gap-6">
          <SettingsGroup
            title="Playing"
            description="Saved on this computer for every account."
            actions={
              <Button size="sm" variant="ghost">
                Reset
              </Button>
            }
          >
            <SettingRow
              label="Launch Roblox with"
              description="Changed from the default: a dot and a reset button."
              changed
              onReset={() => {}}
            >
              <Select
                class="w-40"
                aria-label="Launch Roblox with"
                value="default"
                options={[
                  { value: "default", label: "System default" },
                  { value: "custom", label: "Custom command" },
                ]}
              />
            </SettingRow>
            <SettingSwitch label="Trade offers" description="A row that is on or off." checked />
          </SettingsGroup>
          <SettingsGroup title="Accounts" meta={1} list>
            <ListItem leading={<Avatar name="Builder" />} title="Builder" subtitle="@builderman" />
          </SettingsGroup>
        </div>
      </Section>

      <Section title="Facts and clamp">
        <Facts class="w-64">
          <FactGroup title="Experience">
            <Fact label="Genre">Roleplay</Fact>
            <Fact label="Created">Apr 27, 2024</Fact>
          </FactGroup>
          <FactGroup title="Servers">
            <Fact label="Server size">12</Fact>
            <Fact label="Private servers">Allowed</Fact>
          </FactGroup>
        </Facts>
        <Clamp lines={3} class="w-64">
          <p>
            Run a burger joint with your friends. Take orders, flip patties and keep the line moving. New drive-thru,
            seasonal menu and a rebalanced tip system in this update.
          </p>
        </Clamp>
      </Section>

      <Section title="Icon (Phosphor)">
        <For each={icons}>
          {(n) => (
            <Tooltip content={n}>
              <Icon name={n} size={18} />
            </Tooltip>
          )}
        </For>
      </Section>

      <Section title="Avatar presence and count">
        <Avatar name="Online" presence="online" />
        <Avatar name="In game" presence="game" />
        <Avatar name="Studio" presence="studio" />
        <Avatar name="Offline" presence="offline" />
        <Avatar size="lg" name="Unread" count={6} />
        <Avatar size="xl" name="Profile" presence="game" />
      </Section>

      <Section title="Window controls">
        <div class="flex h-10 bg-deep">
          <WindowControls onMinimize={noop} onMaximize={noop} onClose={noop} />
        </div>
      </Section>

      <Section title="Rail item and nav item">
        <div class="flex flex-col bg-deep py-2">
          <RailItem label="Active account" active color="blue" onClick={noop}>
            <Avatar size="lg" name="Main" count={2} />
          </RailItem>
          <RailItem label="Alt" color="pink" onClick={noop}>
            <Avatar size="lg" name="Alt" />
          </RailItem>
        </div>
        <div class="flex w-52 flex-col gap-0.5 bg-deep p-2">
          <NavItem label="Home" icon="home" active shortcut="g h" onClick={noop} />
          <NavItem label="Friends" icon="friends" count={2} onClick={noop} />
          <NavItem label="Trades" icon="trades" onClick={noop} />
        </div>
        <div class="flex flex-col gap-0.5 bg-deep p-2">
          <NavItem compact label="Home" icon="home" active onClick={noop} />
          <NavItem compact label="Messages" icon="messages" count={3} onClick={noop} />
        </div>
      </Section>

      <Section title="Search button">
        <SearchButton
          class="max-w-md"
          placeholder="Search, paste a link, or type > for commands"
          keys={["Ctrl", "K"]}
          onClick={noop}
        />
      </Section>

      <Section title="Tab strip">
        <TabStrip
          class="w-full bg-deep px-1 pt-1"
          active={strip()}
          onSelect={setStrip}
          onClose={noop}
          onMove={(key, to) => {
            const rest = stripTabs().filter((t) => t.key !== key)
            const moved = stripTabs().find((t) => t.key === key)
            if (moved) setStripTabs([...rest.slice(0, to), moved, ...rest.slice(to)])
          }}
          tabs={stripTabs()}
        />
      </Section>

      <Section title="Tabs (sub-tabs)">
        <Tabs
          class="w-full"
          aria-label="Example"
          value={tab()}
          onChange={setTab}
          items={[
            { value: "servers", label: "Servers" },
            { value: "store", label: "Store" },
            { value: "requests", label: "Requests", count: 3 },
            { value: "admin", label: "Admin", disabled: true },
          ]}
        >
          <TabPanel value="servers">Servers panel</TabPanel>
          <TabPanel value="store">Store panel</TabPanel>
          <TabPanel value="requests">Requests panel</TabPanel>
        </Tabs>
      </Section>

      <Section title="Segmented control and toggle button">
        <SegmentedControl
          aria-label="Sort"
          value={seg()}
          onChange={setSeg}
          options={[
            { value: "friends", label: "Friends" },
            { value: "ping", label: "Ping" },
            { value: "players", label: "Players" },
          ]}
        />
        <SegmentedControl aria-label="Rig" size="sm" value="R15" onChange={noop} options={["R6", "R15"]} />
        <ToggleButton icon="star" pressedIcon="star-fill" pressed={fav()} onChange={setFav}>
          Favorite
        </ToggleButton>
        <ToggleButton icon="thumbs-up" pressedIcon="thumbs-up-fill" pressed onChange={noop}>
          21.9K
        </ToggleButton>
      </Section>

      <Section title="Menu and context menu">
        <Menu
          items={[
            { label: "Open profile", icon: "user", onSelect: noop },
            { label: "Message", icon: "chat", shortcut: "Ctrl+M", onSelect: noop },
            {
              kind: "submenu",
              label: "Join as",
              icon: "play",
              items: [
                { label: "Alt 1", onSelect: noop },
                { label: "Alt 2", onSelect: noop },
              ],
            },
            { kind: "separator" },
            { kind: "heading", label: "Danger" },
            { label: "Block", icon: "block", danger: true, onSelect: noop },
          ]}
        >
          Open menu
        </Menu>
        <Menu icon="more" label="More" items={[{ label: "Copy ID", icon: "copy", onSelect: noop }]} />
        <ContextMenu items={[{ label: "Copy ID", icon: "copy", onSelect: noop }]}>
          <Card class="px-3 py-2 text-fg-muted">Right-click me</Card>
        </ContextMenu>
      </Section>

      <Section title="Prompt and confirm">
        <Button
          onClick={async () => showToast(String(await prompt({ title: "Rename", label: "Label", initial: "Main" })))}
        >
          Prompt
        </Button>
        <Button
          variant="danger"
          onClick={async () =>
            showToast(String(await confirm({ title: "Remove account?", confirm: "Remove", danger: true })))
          }
        >
          Confirm
        </Button>
      </Section>

      <Section title="Alert">
        <div class="flex w-full flex-col gap-2">
          <Alert title="You're offline">Showing saved data.</Alert>
          <Alert variant="warning" title="Session expired" actions={<Button size="sm">Paste new cookie</Button>}>
            Main needs a new cookie.
          </Alert>
          <Alert variant="danger" title="Captcha needed">
            Solve it on the website, then retry.
          </Alert>
        </div>
      </Section>

      <Section title="Section header">
        <div class="flex w-full flex-col gap-2">
          <SectionHeader title="Page title" size="page" actions={<Button size="sm">Action</Button>} />
          <SectionHeader title="Section" meta="61 servers" />
          <SectionHeader title="In game" meta={2} size="group" />
        </div>
      </Section>

      <Section title="List item">
        <Card class="flex w-full max-w-md flex-col p-1">
          <ListItem
            leading={<Avatar name="Pixel Fox" presence="game" />}
            title="Pixel Fox"
            subtitle="Burger Shift"
            onClick={noop}
            actionsOnHover
            actions={<IconButton size="sm" icon="play" label="Join" />}
          />
          <ListItem
            leading={<Avatar name="Mossy" />}
            title="Unread message"
            subtitle="group event saturday"
            meta="2h ago"
            unread
            selected
            onClick={noop}
          />
          <ListItem
            leading={<Avatar name="Quiet" presence="offline" />}
            title="Offline friend"
            subtitle="Offline"
            dim
            onClick={noop}
          />
          <ListItem
            leading={<Avatar name="Brick" />}
            title="Request"
            subtitle="Not clickable, with actions"
            actions={
              <>
                <Button size="sm" variant="primary">
                  Accept
                </Button>
                <Button size="sm">Decline</Button>
              </>
            }
          />
        </Card>
      </Section>

      <Section title="Person card">
        <div class="grid w-full grid-cols-[repeat(auto-fill,minmax(13rem,1fr))] gap-2">
          <PersonCard
            name="Tilework"
            presence="game"
            subtitle="Tile Cats"
            onClick={noop}
            action={<IconButton size="sm" icon="play" label="Join" onClick={noop} />}
          />
          <PersonCard name="Seraphon" presence="online" subtitle="Online" onClick={noop} />
        </div>
      </Section>

      <Section title="Thumbnail and tile">
        <Thumbnail class="w-16" alt="Square" />
        <Thumbnail class="w-40" alt="Wide" shape="wide" />
        <MediaFrame class="w-64" index={0} count={3} onStep={() => {}} caption="Trailer" captionIcon="video">
          <Thumbnail alt="Media frame" shape="wide" />
        </MediaFrame>
        <Viewport class="w-40" tabindex={0} aria-label="Viewport">
          <span class="absolute right-2 bottom-2">
            <Spinner size="sm" />
          </span>
        </Viewport>
        <div class="grid w-full grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-3">
          <Tile title="Tile Cats" subtitle="1,037 playing" onClick={noop} />
          <Tile
            title="Neon Crown"
            subtitle={<Robux amount={250} />}
            fallback="avatar"
            badge={<Badge variant="success">Limited</Badge>}
            onClick={noop}
          />
          <Tile title="Being worn" subtitle={<Robux amount={null} />} fallback="avatar" selected onClick={noop} />
        </div>
      </Section>

      <Section title="Stat, meter, robux, link">
        <Card class="grid w-full grid-cols-5 gap-4 p-3">
          <Stat label="Playing">1,037</Stat>
          <Stat label="Visits">7.3M</Stat>
          <Stat label="Balance" size="lg">
            <Robux amount={12400} />
          </Stat>
          <Stat label="Creator">
            <span class="flex items-center gap-1">
              <TextLink tone="verified" onClick={noop}>
                Tilework
              </TextLink>
              <VerifiedBadge />
            </span>
          </Stat>
          <Stat label="Fake creator">
            <span class="flex items-center gap-1">
              <TextLink tone="danger" onClick={noop}>
                Roblox°°°
              </TextLink>
              <Icon name="alert" class="text-danger" />
            </span>
          </Stat>
        </Card>
        <span class="flex items-center gap-1">
          by{" "}
          <TextLink tone="name" onClick={noop}>
            Chiidoku
          </TextLink>{" "}
          · <TextLink onClick={noop}>roblox.com/games/1</TextLink>
        </span>
        <span>
          Made with <Mention text="@Chiidoku" name="Chiidoku" onClick={noop} /> and{" "}
          <Mention text="@Tilework" name="Tilework" verified onClick={noop} />, thanks{" "}
          <Mention text="@Looking" name="Looking" />
        </span>
        <span class="rounded-lg bg-accent px-3 py-1.5 text-on-accent">
          Inside a bubble:{" "}
          <TextLink tone="inherit" onClick={noop}>
            roblox.com/games/1
          </TextLink>
        </span>
        <Meter class="w-40" label="Players" value={17} max={50} tone="capacity" />
        <Meter class="w-40" label="Players" value={46} max={50} tone="capacity" />
        <Meter class="w-40" label="Players" value={50} max={50} tone="capacity" />
        <span class="flex items-center gap-3">
          <For each={[1, 2, 3, 4, 5]}>{(n) => <Pips label="Demand" value={n} max={5} tone="scale" />}</For>
          <Pips label="Signal" value={2} max={4} />
        </span>
      </Section>

      <Section title="Collapsible">
        <Collapsible title="About" defaultOpen>
          A logic puzzle about placing cats on a grid.
        </Collapsible>
      </Section>

      <Section title="Skeleton and empty state">
        <div class="flex items-center gap-3">
          <Skeleton class="size-10 rounded-full" />
          <div class="flex flex-col gap-1.5">
            <Skeleton class="h-3 w-32" />
            <Skeleton class="h-3 w-20" />
          </div>
        </div>
        <EmptyState icon="friends" title="Nobody here yet" description="Friends you add show up here.">
          <Button icon="user-plus">Add friend</Button>
        </EmptyState>
      </Section>

      <Section title="Data table (300 rows, virtual)">
        <DataTable
          class="w-full"
          aria-label="Servers"
          columns={columns}
          rows={rows}
          rowKey={(r) => r.id}
          onRowClick={noop}
        />
      </Section>

      <Section title="Virtual grid (500 tiles)">
        <VirtualGrid
          class="w-full"
          items={Array.from({ length: 500 }, (_, i) => i)}
          minItemWidth={96}
          estimateRowHeight={130}
        >
          {(i) => <Tile title={`Item ${i + 1}`} fallback="avatar" onClick={noop} />}
        </VirtualGrid>
      </Section>

      <Section title="Spinner">
        <Spinner size="sm" />
        <Spinner />
        <Spinner size="lg" />
      </Section>
    </div>
  )
}
