import type { WikiArticle, WikiBlock, WikiPageView, WikiSearchView, WikiTarget } from "./ipc/bindings"

/**
 * Made-up Roblox Wiki articles for the browser preview (`bun run dev`), where there is no
 * backend to fetch real ones. They match the games and friends in `sample.ts` (Burger Shift is
 * place 9001, pixelfox is user 101) so the panels on those pages have something to show. Every
 * name, item and event here is made up. The app never shows them.
 */

type Kind = "item" | "game" | "player" | "page"

interface Draft {
  title: string
  kind: Kind
  category: string
  target: WikiTarget | null
  revised?: number
  editors?: number
  extract: string
  infobox: { title?: string; facts: { label: string; value: string }[] }[]
  sections: {
    id: string
    title: string
    blocks: (
      | { type: "p"; text: string }
      | { type: "list"; items: string[] }
      | { type: "table"; columns: string[]; rows: string[][] }
      | { type: "note"; text: string }
    )[]
  }[]
  categories?: string[]
}

const DAY = 86_400_000
const ago = (days: number) => Date.now() - days * DAY

const drafts: Draft[] = [
  {
    title: "Burger Shift",
    kind: "game",
    category: "Experiences",
    target: { kind: "game", placeId: 9001 },
    revised: ago(1),
    editors: 38,
    extract:
      "Burger Shift is a restaurant simulator by [[Grill Studio]] where players take orders, cook and serve customers in a busy diner. It was released in April 2024 and passed one billion visits in 2026.",
    infobox: [
      {
        title: "Burger Shift",
        facts: [
          { label: "Developer", value: "Grill Studio" },
          { label: "Lead developer", value: "[[Pixel Fox]]" },
          { label: "Released", value: "April 12, 2024" },
          { label: "Genre", value: "Simulation, Roleplay" },
          { label: "Max players", value: "50" },
        ],
      },
      {
        title: "Records",
        facts: [
          { label: "Peak players", value: "212,400 (Big Grill 2025)" },
          { label: "Visits", value: "1.2B" },
        ],
      },
    ],
    sections: [
      {
        id: "gameplay",
        title: "Gameplay",
        blocks: [
          {
            type: "p",
            text: "Each round is a shift of ten minutes. Players pick a station (register, grill, fryer or drinks) and work together to serve as many customers as they can before closing. Tips are shared across the team and spent on upgrades for the diner.",
          },
          {
            type: "list",
            items: [
              "Register: take orders and hand out trays.",
              "Grill: cook patties without burning them.",
              "Fryer: keep fries and onion rings stocked.",
              "Drinks: fill cups and shakes.",
            ],
          },
        ],
      },
      {
        id: "history",
        title: "History",
        blocks: [
          {
            type: "p",
            text: "Burger Shift started as a weekend project by [[Pixel Fox]] and [[Lumen]] in late 2023. It was featured on the front page in its second week and reached 40,000 concurrent players within a month.",
          },
          {
            type: "p",
            text: "In 2025 the game hosted [[The Big Grill (2025)]], a two-week event that gave away the [[Golden Spatula]] and set the game's player record.",
          },
        ],
      },
      {
        id: "reception",
        title: "Reception",
        blocks: [
          {
            type: "p",
            text: "Players praised the game's cooperative design and quick rounds. Some criticised the late-game upgrade costs, which were lowered in update 3.2.",
          },
        ],
      },
      {
        id: "trivia",
        title: "Trivia",
        blocks: [
          {
            type: "list",
            items: [
              "The diner's sign was drawn by a player who won a fan art contest.",
              "The ceiling fan in the kitchen has spun since launch and is never reset between servers.",
              "Mr. Patty, the final customer of Night Shift, is voiced by the game's lead developer.",
            ],
          },
        ],
      },
    ],
    categories: ["Experiences", "Simulation experiences", "Front page experiences", "2024 experiences"],
  },
  {
    title: "Block Kart",
    kind: "game",
    category: "Experiences",
    target: { kind: "game", placeId: 9004 },
    revised: ago(9),
    editors: 17,
    extract:
      "Block Kart is a kart racing experience by Pit Lane. Players race on tracks built from bricks, collect power-ups and unlock karts with coins earned in races.",
    infobox: [
      {
        title: "Block Kart",
        facts: [
          { label: "Developer", value: "Pit Lane" },
          { label: "Released", value: "June 3, 2023" },
          { label: "Genre", value: "Racing" },
        ],
      },
    ],
    sections: [
      {
        id: "tracks",
        title: "Tracks",
        blocks: [
          {
            type: "table",
            columns: ["Track", "Cup", "Laps"],
            rows: [
              ["Baseplate Loop", "Starter", "3"],
              ["Lava Run", "Fire", "3"],
              ["Studless Speedway", "Classic", "5"],
            ],
          },
        ],
      },
    ],
  },
  {
    title: "Night Ferry",
    kind: "game",
    category: "Experiences",
    target: { kind: "game", placeId: 9003 },
    revised: ago(40),
    editors: 6,
    extract: "Night Ferry is a story experience by Harbor Lights about a ferry crossing that never reaches the shore.",
    infobox: [{ facts: [{ label: "Developer", value: "Harbor Lights" }] }],
    sections: [],
  },
  {
    title: "Golden Spatula",
    kind: "item",
    category: "Event items",
    target: { kind: "item", id: 1002 },
    revised: ago(12),
    editors: 11,
    extract:
      "The Golden Spatula is a back accessory that was free during [[The Big Grill (2025)]] in [[Burger Shift]]. Players earned it by serving 500 customers during the event.",
    infobox: [
      {
        title: "Golden Spatula",
        facts: [
          { label: "Type", value: "Back accessory" },
          { label: "Creator", value: "Roblox" },
          { label: "Released", value: "July 18, 2025" },
          { label: "Price", value: "Free (event)" },
          { label: "Obtainable", value: "No" },
        ],
      },
      { title: "Event", facts: [{ label: "Event", value: "[[The Big Grill (2025)]]" }] },
    ],
    sections: [
      {
        id: "obtaining",
        title: "How it was obtained",
        blocks: [
          {
            type: "list",
            items: [
              "Serve 500 customers in [[Burger Shift]] during the event.",
              "Orders served in private servers counted.",
              "The badge for the quest is still awarded, but the item is no longer granted.",
            ],
          },
        ],
      },
      {
        id: "trivia",
        title: "Trivia",
        blocks: [
          {
            type: "list",
            items: [
              "It is the first event item whose quest could be completed in a private server.",
              "An early model had a burger on the blade; it was removed before release.",
            ],
          },
        ],
      },
    ],
    categories: ["Event items", "Back accessories", "2025 items", "Burger Shift"],
  },
  {
    title: "Frostbite Crown",
    kind: "item",
    category: "Limited items",
    target: { kind: "item", id: 1001 },
    revised: ago(2),
    editors: 24,
    extract:
      "The Frostbite Crown is a limited hat released by Roblox for the 2019 winter sale. Only 2,000 copies were sold, and it is now one of the more traded limiteds.",
    infobox: [
      {
        title: "Frostbite Crown",
        facts: [
          { label: "Type", value: "Hat" },
          { label: "Creator", value: "Roblox" },
          { label: "Released", value: "December 20, 2019" },
          { label: "Original price", value: "R$ 25,000" },
          { label: "Copies", value: "2,000" },
          { label: "Limited", value: "Yes (unique)" },
        ],
      },
    ],
    sections: [
      {
        id: "history",
        title: "History",
        blocks: [
          {
            type: "p",
            text: "The crown went on sale on December 20, 2019 and sold out in under four minutes. Its resale price doubled within the first week.",
          },
          {
            type: "note",
            text: "Prices on the wiki are written by editors and may be out of date. Bonsai shows live RAP and Rolimons values on the item page.",
          },
        ],
      },
      {
        id: "variants",
        title: "Variants",
        blocks: [
          {
            type: "table",
            columns: ["Item", "Year", "Copies"],
            rows: [
              ["Frostbite Crown", "2019", "2,000"],
              ["Frostbite Crown (Midnight)", "2021", "1,200"],
            ],
          },
        ],
      },
    ],
    categories: ["Limited items", "Hats", "Winter items", "2019 items"],
  },
  {
    title: "Traffic Cone Hat",
    kind: "item",
    category: "Hats",
    target: { kind: "item", id: 1003 },
    revised: ago(120),
    editors: 3,
    extract: "The Traffic Cone Hat is a hat sold in the Avatar Shop since 2012. It is one of the most worn free hats.",
    infobox: [{ facts: [{ label: "Type", value: "Hat" }] }],
    sections: [],
  },
  {
    title: "Pixel Fox",
    kind: "player",
    category: "Notable players",
    target: { kind: "user", id: 101 },
    revised: ago(5),
    editors: 9,
    extract:
      "pixelfox, known as Pixel Fox, is a developer and the lead of Grill Studio. They are best known for creating [[Burger Shift]] with [[Lumen]].",
    infobox: [
      {
        title: "Pixel Fox",
        facts: [
          { label: "Username", value: "pixelfox" },
          { label: "Joined", value: "2016" },
          { label: "Known for", value: "[[Burger Shift]]" },
          { label: "Communities", value: "Grill Studio (owner)" },
        ],
      },
    ],
    sections: [
      {
        id: "career",
        title: "Career",
        blocks: [
          {
            type: "p",
            text: "Pixel Fox began making obbies in 2017 and joined several studios as a scripter before starting Grill Studio in 2023.",
          },
        ],
      },
      {
        id: "games",
        title: "Experiences",
        blocks: [
          {
            type: "table",
            columns: ["Experience", "Year", "Role"],
            rows: [
              ["[[Burger Shift]]", "2024", "Lead developer"],
              ["Pizza Rush", "2021", "Scripter"],
            ],
          },
        ],
      },
    ],
    categories: ["Notable players", "Developers"],
  },
  {
    title: "Lumen",
    kind: "player",
    category: "Notable players",
    target: { kind: "user", id: 104 },
    revised: ago(30),
    editors: 4,
    extract: "lumen_dev, known as Lumen, is a builder who co-created [[Burger Shift]].",
    infobox: [{ facts: [{ label: "Username", value: "lumen_dev" }] }],
    sections: [],
  },
  {
    title: "The Big Grill (2025)",
    kind: "page",
    category: "Events",
    target: null,
    revised: ago(60),
    editors: 14,
    extract:
      "The Big Grill was a two-week event in [[Burger Shift]] in July 2025. It gave away the [[Golden Spatula]] and drew a record 212,400 players.",
    infobox: [
      {
        title: "The Big Grill",
        facts: [
          { label: "Dates", value: "July 18 – August 1, 2025" },
          { label: "Prize", value: "[[Golden Spatula]]" },
        ],
      },
    ],
    sections: [],
    categories: ["Events", "2025 events"],
  },
  {
    title: "Grill Studio",
    kind: "page",
    category: "Communities",
    target: null,
    extract: "Grill Studio is the community that develops [[Burger Shift]]. It was founded by [[Pixel Fox]] in 2023.",
    infobox: [],
    sections: [],
  },
]

const slug = (title: string) => title.replace(/\s+/g, "_")

const view = (d: Draft): WikiArticle => {
  const title = d.kind === "item" ? `Catalog:${d.title}` : d.title
  return {
    title,
    wiki: "roblox",
    source: "Roblox Wiki",
    name: d.title,
    kind: d.kind,
    extract: d.extract,
    lead: [{ type: "p", text: d.extract }],
    sections: d.sections.map((s) => ({
      id: s.id,
      title: s.title,
      level: 2,
      blocks: s.blocks.map((b): WikiBlock => (b.type === "list" ? { ...b, ordered: false } : b)),
    })),
    infoboxTitle: d.infobox[0]?.title ?? "",
    infobox: d.infobox.flatMap((g) => g.facts.map((f) => ({ key: "", ...f }))),
    image: null,
    categories: d.categories ?? [d.category],
    target: d.target,
    wikis: [],
    url: `https://roblox.fandom.com/wiki/${slug(title)}`,
    revision: 1,
  }
}

export const sampleArticles: WikiArticle[] = drafts.map(view)

const page = (article: WikiArticle | undefined): WikiPageView => ({
  enabled: true,
  loading: false,
  error: null,
  fetchedAt: ago(0.2),
  article: article ?? null,
  source: "Roblox Wiki",
  license: "CC BY-SA 3.0",
})

const norm = (t: string) => t.replace(/_/g, " ").trim().toLowerCase()

/** The sample article called `title`, by its wiki title or its name. */
export const samplePage = (title: string) =>
  page(sampleArticles.find((a) => norm(a.title) === norm(title) || norm(a.name) === norm(title)))

/** The sample article about an item, experience or player. */
export const sampleAbout = (kind: string, id: number) =>
  page(
    sampleArticles.find((a) => {
      const t = a.target
      if (!t) return false
      if (t.kind === "game") return kind === "game" && t.placeId === id
      return (kind === "item" ? "item" : kind === "user" ? "user" : "") === t.kind && t.id === id
    }),
  )

export function sampleSearch(query: string, wiki = "roblox"): WikiSearchView {
  const q = norm(query)
  const days = [1, 3, 6, 12, 20, 40]
  return {
    wiki,
    enabled: true,
    loading: false,
    error: null,
    hits:
      q && wiki === "roblox"
        ? sampleArticles
            .filter((a) => norm(a.name).includes(q) || a.extract.toLowerCase().includes(q))
            .map((a, i) => ({
              title: a.title,
              name: a.name,
              kind: a.kind === "item" ? "item" : "page",
              snippet: a.extract.replace(/\[\[(?:[^\]|]+\|)?([^\]]+)\]\]/g, "$1"),
              edited: new Date(ago(days[i % days.length])).toISOString(),
            }))
        : [],
  }
}
