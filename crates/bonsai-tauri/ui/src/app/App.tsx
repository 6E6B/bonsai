import { Route, Router } from "@solidjs/router"
import { lazy } from "solid-js"
import { DialogProvider, Toaster } from "~/lib/ui"
import { Layout } from "./Layout"

const Home = lazy(() => import("~/features/home/Home"))
const Friends = lazy(() => import("~/features/friends/Friends"))
const Discover = lazy(() => import("~/features/discover/Discover"))
const Game = lazy(() => import("~/features/game/Game"))
const Profile = lazy(() => import("~/features/profile/Profile"))
const AvatarEditor = lazy(() => import("~/features/avatar/AvatarEditor"))
const Marketplace = lazy(() => import("~/features/marketplace/Marketplace"))
const ItemPage = lazy(() => import("~/features/marketplace/ItemPage"))
const PassPage = lazy(() => import("~/features/marketplace/PassPage"))
const Inventory = lazy(() => import("~/features/inventory/Inventory"))
const Trades = lazy(() => import("~/features/trades/Trades"))
const Values = lazy(() => import("~/features/values/Values"))
const Transactions = lazy(() => import("~/features/transactions/Transactions"))
const Messages = lazy(() => import("~/features/messages/Messages"))
const Chat = lazy(() => import("~/features/chat/Chat"))
const Notifications = lazy(() => import("~/features/notifications/Notifications"))
const Communities = lazy(() => import("~/features/communities/Communities"))
const Community = lazy(() => import("~/features/communities/Community"))
const Search = lazy(() => import("~/features/search/Search"))
const Settings = lazy(() => import("~/features/settings/Settings"))
const Components = lazy(() => import("~/features/components/Components"))
const Overview = lazy(() => import("~/features/overview/Overview"))

export function App() {
  return (
    <DialogProvider>
      <Router root={Layout}>
        <Route path="/" component={Home} />
        <Route path="/friends" component={Friends} />
        <Route path="/discover" component={Discover} />
        <Route path="/games/:id" component={Game} />
        <Route path="/users/:id" component={Profile} />
        <Route path="/avatar" component={AvatarEditor} />
        <Route path="/marketplace" component={Marketplace} />
        <Route path="/catalog/:id" component={ItemPage} />
        <Route path="/bundles/:id" component={ItemPage} />
        <Route path="/passes/:id" component={PassPage} />
        <Route path="/inventory/:user?" component={Inventory} />
        <Route path="/trades" component={Trades} />
        <Route path="/values" component={Values} />
        <Route path="/transactions" component={Transactions} />
        <Route path="/messages" component={Messages} />
        <Route path="/chat" component={Chat} />
        <Route path="/notifications" component={Notifications} />
        <Route path="/communities" component={Communities} />
        <Route path="/communities/:id" component={Community} />
        <Route path="/search" component={Search} />
        <Route path="/settings/:section?" component={Settings} />
        <Route path="/components" component={Components} />
        <Route path="/overview" component={Overview} />
      </Router>
      <Toaster />
    </DialogProvider>
  )
}
