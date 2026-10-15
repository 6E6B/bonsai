/* @refresh reload */
import { render } from "solid-js/web"
import "./index.css"
import { App } from "~/app/App"
import { setTheme } from "~/lib/ui"

setTheme("system")

const root = document.getElementById("root")
if (root) render(() => <App />, root)
