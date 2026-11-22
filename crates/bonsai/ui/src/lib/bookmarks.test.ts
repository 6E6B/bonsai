import { describe, expect, it } from "vitest"
import type { BookmarkView } from "~/lib/ipc/bindings"
import { bookmarkFolders, bookmarkIdentity, bookmarkPath, bookmarkRows, findBookmark, groupKey } from "./bookmarks"

const bookmarks: BookmarkView[] = [
  {
    id: 1,
    path: "/games/1?tab=servers",
    title: "Doors",
    kind: "experience",
    thumbnail: null,
    status: "12 playing",
    folder: "",
    tags: [],
  },
  {
    id: 2,
    path: "/users/2",
    title: "Builder",
    kind: "person",
    thumbnail: null,
    status: "Online",
    folder: "",
    tags: [],
  },
  {
    id: 3,
    path: "/games/3",
    title: "Doors Two",
    kind: "experience",
    thumbnail: null,
    status: "",
    folder: "",
    tags: [],
  },
  {
    id: 4,
    path: "/chat?conversation=1",
    title: "Chat",
    kind: "view",
    thumbnail: null,
    status: "",
    folder: "",
    tags: [],
  },
]
describe("bookmarks", () => {
  it("groups by kind and preserves order inside a group", () => {
    expect(
      bookmarkRows(bookmarks, "").map((r) => (r.type === "header" ? [groupKey(r.group), r.count] : r.bookmark.id)),
    ).toEqual([["experience", 2], 1, 3, ["person", 1], 2, ["view", 1], 4])
  })
  it("filters all title/path words and reveals matching collapsed groups", () => {
    expect(bookmarkRows(bookmarks, "DOORS two", { experience: true })).toEqual([
      { type: "header", group: { folder: null, kind: "experience" }, count: 1 },
      { type: "bookmark", bookmark: bookmarks[2] },
    ])
    expect(bookmarkRows(bookmarks, "missing")).toEqual([])
    expect(bookmarkRows(bookmarks, "", { experience: true }).filter((r) => r.type === "bookmark")).toHaveLength(2)
  })
  it("puts folders first, mixes kinds inside them and filters by tag", () => {
    const filed: BookmarkView[] = [
      { ...bookmarks[0], folder: "zombies", tags: ["Horror"] },
      { ...bookmarks[1], folder: "Alpha", tags: [] },
      { ...bookmarks[3], folder: "zombies", tags: ["work"] },
      bookmarks[2],
    ]
    expect(bookmarkFolders(filed)).toEqual(["Alpha", "zombies"])
    expect(
      bookmarkRows(filed, "").map((r) => (r.type === "header" ? [groupKey(r.group), r.count] : r.bookmark.id)),
    ).toEqual([["folder:Alpha", 1], 2, ["folder:zombies", 2], 1, 4, ["experience", 1], 3])
    expect(bookmarkRows(filed, "", { "folder:zombies": true }).filter((r) => r.type === "bookmark")).toHaveLength(2)
    const ids = (q: string) => bookmarkRows(filed, q).flatMap((r) => (r.type === "bookmark" ? [r.bookmark.id] : []))
    expect(ids("#hor")).toEqual([1])
    expect(ids("#or")).toEqual([])
    expect(ids("work")).toEqual([4])
    expect(ids("zombies")).toEqual([1, 4])
  })
  it("matches entity sub-tabs while keeping private view queries", () => {
    expect(findBookmark(bookmarks, "/games/1")).toBe(bookmarks[0])
    expect(findBookmark(bookmarks, "/chat?conversation=2")).toBeUndefined()
    expect(bookmarkIdentity("/games/1/private?tab=servers")).toBe("/games/1/private?tab=servers")
  })
  it("resolves menu targets and rejects remote URLs", () => {
    expect(bookmarkPath("game:123")).toBe("/games/123")
    expect(bookmarkPath("group:123")).toBe("/communities/123")
    expect(bookmarkPath("/settings/privacy")).toBe("/settings/privacy")
    expect(bookmarkPath("https://roblox.com/games/123")).toBeUndefined()
    expect(bookmarkPath("//example.com")).toBeUndefined()
  })
})
