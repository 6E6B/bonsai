/* @refresh reload */
import { render } from "solid-js/web"
import "./index.css"
import { App } from "~/app/App"
import { applyAppearance } from "~/lib/theme"

applyAppearance("system")

// Suppress the webview menu without interrupting custom context-menu handlers.
document.addEventListener("contextmenu", (event) => event.preventDefault())

const root = document.getElementById("root")
if (root) render(() => <App />, root)
