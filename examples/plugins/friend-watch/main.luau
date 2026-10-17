-- Watchlist: get a desktop alert when specific people come online or join a game.
-- Add people from a user's context menu (Plugins > Watch / Unwatch).

local function watchlist(ctx)
	return ctx.store.get("watch") or {}
end

bonsai.command {
	id = "watch",
	title = "Watch user (alert when online)",
	keywords = { "watch", "alert", "online", "notify", "friend" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local list = watchlist(ctx)
		local id = tostring(arg)
		local u = ctx.user(id)
		list[id] = u.display_name or u.name
		ctx.store.set("watch", list)
		return { toast = "Watching " .. tostring(list[id]) }
	end,
}

bonsai.command {
	id = "unwatch",
	title = "Stop watching user",
	keywords = { "watch", "unwatch", "remove" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local list = watchlist(ctx)
		list[tostring(arg)] = nil
		ctx.store.set("watch", list)
		return { toast = "No longer watching " .. tostring(arg) }
	end,
}

bonsai.command {
	id = "list",
	title = "Show watchlist",
	keywords = { "watch", "list" },
	arg = "none",
	run = function(ctx)
		local names = {}
		for _, name in pairs(watchlist(ctx)) do
			table.insert(names, name)
		end
		table.sort(names)
		if #names == 0 then
			return { toast = "Watchlist is empty" }
		end
		return { toast = "Watching: " .. table.concat(names, ", ") }
	end,
}

bonsai.on("presence", function(ctx, changes)
	local list = watchlist(ctx)
	for _, c in ipairs(changes) do
		local name = list[tostring(c.id)]
		if name then
			local joined = c.status == "in_game"
			local came_online = c.status == "online" and c.was == "offline"
			if joined or (came_online and not ctx.settings.only_in_game) then
				local body = joined and ("is playing " .. tostring(c.location or "a game")) or "is online"
				return { alert = { title = name, body = body } }
			end
		end
	end
end)
