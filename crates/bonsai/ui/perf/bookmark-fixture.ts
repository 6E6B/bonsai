// Synthetic backend state for exercising bookmark command handlers in the browser build.
import { bookmarkIdentity } from "../src/lib/bookmarks"
import type { BookmarkList, Invocation } from "../src/lib/ipc/bindings"
import { browserShell } from "../src/lib/sample"

const enabled = new URLSearchParams(location.search).has("bookmarks")
export const bookmarkFixture: BookmarkList = {
  account: browserShell.active ?? 0,
  items: enabled
    ? [
        {
          id: 1,
          title: "Industrial Outpost",
          path: "/games/1000?tab=servers",
          kind: "experience",
          status: "1,240 playing",
          thumbnail: null,
        },
        { id: 2, title: "Night Shift", path: "/games/1001", kind: "experience", status: "86 playing", thumbnail: null },
        { id: 3, title: "Builder", path: "/users/100", kind: "person", status: "In game", thumbnail: null },
        {
          id: 4,
          title: "Limited accessories",
          path: "/marketplace?sales=limited",
          kind: "view",
          status: "",
          thumbnail: null,
        },
      ]
    : [],
}
let next = 5
export function bookmarkInvocation(inv: Invocation) {
  const raw = inv.arg ?? ""
  if (inv.id === "bookmark.toggle") {
    const target = JSON.parse(raw) as { path: string; title: string; fallback: string }
    const at = bookmarkFixture.items.findIndex((b) => bookmarkIdentity(b.path) === bookmarkIdentity(target.path))
    if (at >= 0) bookmarkFixture.items.splice(at, 1)
    else
      bookmarkFixture.items.push({
        id: next++,
        path: target.path,
        title: target.title || target.fallback,
        kind: "view",
        status: "",
        thumbnail: null,
      })
  } else if (inv.id === "bookmark.remove") {
    bookmarkFixture.items = bookmarkFixture.items.filter((b) => b.id !== Number(raw))
  } else if (inv.id === "bookmark.rename") {
    const space = raw.indexOf(" ")
    const bookmark = bookmarkFixture.items.find((b) => b.id === Number(raw.slice(0, space)))
    if (bookmark) bookmark.title = raw.slice(space + 1)
  }
  return { status: "ok" as const, data: null }
}
