import type { WikiArticle, WikiSource } from "./wiki"

/**
 * Made-up wiki articles for the wiki prototype. They match the games and friends in `sample.ts`
 * (Burger Shift is place 9001, pixelfox is user 101) so the panels on those pages have
 * something to show. Every name, item and event here is made up.
 */

const DAY = 86_400_000
// Relative to now so "edited 5 hours ago" reads right whenever the prototype runs.
const NOW = Date.now()
const ago = (days: number) => NOW - days * DAY

const CC = "CC BY-SA 3.0"

export const sources: WikiSource[] = [
  { key: "roblox", name: "Roblox Wiki", host: "roblox.fandom.com", license: CC },
  { key: "burgershift", name: "Burger Shift Wiki", host: "burgershift.fandom.com", license: CC, placeId: 9001 },
]

type Draft = Omit<WikiArticle, "slug" | "categories" | "editors" | "revised"> &
  Partial<Pick<WikiArticle, "categories" | "editors" | "revised">>

const article = (a: Draft): WikiArticle => ({
  slug: a.title.replace(/\s+/g, "_"),
  categories: [a.category],
  editors: 4,
  revised: ago(3),
  ...a,
})

const roblox: WikiArticle[] = [
  article({
    source: "roblox",
    title: "Burger Shift",
    kind: "game",
    category: "Experiences",
    target: { kind: "game", placeId: 9001 },
    revised: ago(1),
    editors: 38,
    extract:
      "Burger Shift is a restaurant simulator by [[Grill Studio]] where players take orders, cook and serve customers in a busy diner. It was released in April 2024 and passed one billion visits in 2026. It has its own fan wiki, the [[Burger Shift Wiki]].",
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
  }),
  article({
    source: "roblox",
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
  }),
  article({
    source: "roblox",
    title: "Night Ferry",
    kind: "game",
    category: "Experiences",
    target: { kind: "game", placeId: 9003 },
    revised: ago(40),
    editors: 6,
    extract: "Night Ferry is a story experience by Harbor Lights about a ferry crossing that never reaches the shore.",
    infobox: [{ facts: [{ label: "Developer", value: "Harbor Lights" }] }],
    sections: [],
  }),
  article({
    source: "roblox",
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
  }),
  article({
    source: "roblox",
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
  }),
  article({
    source: "roblox",
    title: "Traffic Cone Hat",
    kind: "item",
    category: "Hats",
    target: { kind: "item", id: 1003 },
    revised: ago(120),
    editors: 3,
    extract: "The Traffic Cone Hat is a hat sold in the Avatar Shop since 2012. It is one of the most worn free hats.",
    infobox: [{ facts: [{ label: "Type", value: "Hat" }] }],
    sections: [],
  }),
  article({
    source: "roblox",
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
  }),
  article({
    source: "roblox",
    title: "Lumen",
    kind: "player",
    category: "Notable players",
    target: { kind: "user", id: 104 },
    revised: ago(30),
    editors: 4,
    extract: "lumen_dev, known as Lumen, is a builder who co-created [[Burger Shift]].",
    infobox: [{ facts: [{ label: "Username", value: "lumen_dev" }] }],
    sections: [],
  }),
  article({
    source: "roblox",
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
  }),
  article({
    source: "roblox",
    title: "Grill Studio",
    kind: "page",
    category: "Communities",
    target: null,
    extract: "Grill Studio is the community that develops [[Burger Shift]]. It was founded by [[Pixel Fox]] in 2023.",
    infobox: [],
    sections: [],
  }),
]

const burgershift: WikiArticle[] = [
  article({
    source: "burgershift",
    title: "Burger Shift Wiki",
    kind: "page",
    category: "Main",
    target: { kind: "game", placeId: 9001 },
    revised: ago(0.2),
    editors: 112,
    extract:
      "Welcome to the Burger Shift Wiki, the guide to every station, ingredient, customer and update. Start with [[Stations]], or read the [[Night Shift]] guide.",
    infobox: [],
    sections: [],
  }),
  article({
    source: "burgershift",
    title: "Stations",
    kind: "page",
    category: "Gameplay",
    target: null,
    revised: ago(2),
    editors: 21,
    extract: "There are four stations in the diner. Each one needs a different skill and earns a share of the tips.",
    infobox: [
      {
        facts: [
          { label: "Stations", value: "4" },
          { label: "Unlock", value: "All from level 1" },
        ],
      },
    ],
    sections: [
      {
        id: "list",
        title: "List of stations",
        blocks: [
          {
            type: "table",
            columns: ["Station", "Tip share", "Upgrade"],
            rows: [
              ["Register", "20%", "Second till"],
              ["[[Grill]]", "35%", "Double grill"],
              ["Fryer", "25%", "Auto basket"],
              ["Drinks", "20%", "Shake machine"],
            ],
          },
        ],
      },
    ],
  }),
  article({
    source: "burgershift",
    title: "Grill",
    kind: "page",
    category: "Equipment",
    target: null,
    revised: ago(4),
    editors: 9,
    extract:
      "The Grill cooks patties. Each patty has to be flipped once and taken off before it burns. Upgrading it adds a second grill and a timer light.",
    infobox: [
      {
        title: "Grill",
        facts: [
          { label: "Station", value: "Kitchen" },
          { label: "Cook time", value: "6 s per side" },
          { label: "Burns after", value: "11 s" },
          { label: "Upgrade cost", value: "4,000 tips" },
        ],
      },
    ],
    sections: [
      {
        id: "tips",
        title: "Tips",
        blocks: [
          {
            type: "list",
            items: [
              "Flip when the edge turns brown, not when the timer light comes on.",
              "Keep two patties going at once once you have the Double Grill.",
              "Burnt patties still count for the [[Night Shift]] quota.",
            ],
          },
        ],
      },
    ],
  }),
  article({
    source: "burgershift",
    title: "Night Shift",
    kind: "page",
    category: "Game modes",
    target: null,
    revised: ago(1),
    editors: 16,
    extract:
      "Night Shift is the hard mode, unlocked at level 20. Customers arrive faster, orders are longer and the last customer is [[Mr. Patty]].",
    infobox: [
      {
        title: "Night Shift",
        facts: [
          { label: "Unlocked at", value: "Level 20" },
          { label: "Length", value: "15 min" },
          { label: "Boss", value: "[[Mr. Patty]]" },
        ],
      },
    ],
    sections: [
      {
        id: "strategy",
        title: "Strategy",
        blocks: [
          {
            type: "p",
            text: "Put your best player on the [[Grill]]. Keep one player on the register at all times; walk-outs end the shift early.",
          },
        ],
      },
    ],
  }),
  article({
    source: "burgershift",
    title: "Mr. Patty",
    kind: "page",
    category: "Customers",
    target: null,
    revised: ago(6),
    editors: 7,
    extract:
      "Mr. Patty is the final customer of [[Night Shift]]. He orders twelve burgers at once and leaves after 90 seconds.",
    infobox: [
      {
        title: "Mr. Patty",
        facts: [
          { label: "Appears in", value: "[[Night Shift]]" },
          { label: "Patience", value: "90 s" },
          { label: "Reward", value: "Night Owl badge" },
        ],
      },
    ],
    sections: [],
  }),
  article({
    source: "burgershift",
    title: "Ingredients",
    kind: "page",
    category: "Items",
    target: null,
    revised: ago(8),
    editors: 12,
    extract: "Every order is built from these ingredients. New ones arrive with updates.",
    infobox: [],
    sections: [
      {
        id: "list",
        title: "List",
        blocks: [
          {
            type: "table",
            columns: ["Ingredient", "Station", "Added"],
            rows: [
              ["Bun", "Register", "Launch"],
              ["Patty", "[[Grill]]", "Launch"],
              ["Cheese", "[[Grill]]", "Launch"],
              ["Onion rings", "Fryer", "Update 1.4"],
              ["Milkshake", "Drinks", "Update 2.0"],
            ],
          },
        ],
      },
    ],
  }),
  article({
    source: "burgershift",
    title: "Updates",
    kind: "page",
    category: "Updates",
    target: null,
    revised: ago(0.2),
    editors: 30,
    extract: "The update log. The newest update is 3.4, released this week.",
    infobox: [],
    sections: [
      {
        id: "log",
        title: "Update log",
        blocks: [
          {
            type: "table",
            columns: ["Version", "Date", "Changes"],
            rows: [
              ["3.4", "Oct 2026", "Drive-thru window, two new customers"],
              ["3.2", "Mar 2026", "Cheaper late upgrades"],
              ["3.0", "Jul 2025", "[[Night Shift]], The Big Grill event"],
              ["2.0", "Nov 2024", "Drinks station, milkshakes"],
            ],
          },
        ],
      },
    ],
  }),
]

export const articles: WikiArticle[] = [...roblox, ...burgershift]
