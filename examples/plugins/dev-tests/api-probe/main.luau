-- Exercises every read in the plugin API. Each command catches errors and reports them.

local function probe(name, fn)
	local ok, res = pcall(fn)
	if ok then
		return name .. ": ok " .. tostring(res)
	end
	return name .. ": FAIL " .. tostring(res)
end

local function count(t)
	if t == nil then
		return "nil"
	end
	return tostring(#t) .. " rows"
end

bonsai.command {
	id = "user",
	title = "Probe: read a user (id)",
	keywords = { "test", "user" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local u = ctx.user(arg)
		print("user", u.id, u.name)
		return { toast = string.format("%s (@%s) created=%s friends=%s banned=%s",
			tostring(u.display_name), tostring(u.name), tostring(u.created),
			tostring(u.friends), tostring(u.banned)) }
	end,
}

bonsai.command {
	id = "game",
	title = "Probe: read a game (place id)",
	keywords = { "test", "game", "place" },
	arg = "game",
	context = "game",
	run = function(ctx, arg)
		local g = ctx.game(arg)
		if g == nil then
			return { toast = "no such place" }
		end
		return { toast = string.format("%s (universe %s)", tostring(g.name), tostring(g.universe_id)) }
	end,
}

bonsai.command {
	id = "item",
	title = "Probe: read an item (asset id)",
	keywords = { "test", "item", "catalog" },
	arg = "item",
	context = "item",
	run = function(ctx, arg)
		local i = ctx.item(arg)
		return { toast = "item name: " .. tostring(i.name) }
	end,
}

bonsai.command {
	id = "all",
	title = "Probe: run every read",
	keywords = { "test", "all", "reads", "smoke" },
	arg = "none",
	run = function(ctx)
		local limit = ctx.settings.limit
		local lines = {}
		local function add(s)
			table.insert(lines, s)
			print(s)
		end
		add("account=" .. tostring(ctx.account) .. " now=" .. tostring(ctx.now))
		add(probe("user(1)", function() return ctx.user(1).name end))
		add(probe("game(920587237)", function() local g = ctx.game(920587237); return g and g.name end))
		add(probe("item(1818)", function() return ctx.item(1818).name end))
		add(probe("presence({1})", function() return count(ctx.presence({ 1 })) end))
		add(probe("friends()", function() return count(ctx.friends()) end))
		add(probe("inventory", function() return count(ctx.inventory { limit = limit }) end))
		add(probe("trades inbound", function() return count(ctx.trades { limit = limit }) end))
		add(probe("trades completed", function() return count(ctx.trades { status = "completed", limit = limit }) end))
		add(probe("friend_requests", function() return count(ctx.friend_requests { limit = limit }) end))
		add(probe("store", function()
			ctx.store.set("last_probe", ctx.now)
			return ctx.store.get("last_probe")
		end))

		local fails = 0
		for _, l in ipairs(lines) do
			if string.find(l, "FAIL", 1, true) then
				fails += 1
			end
		end
		local summary = table.concat(lines, "\n")
		if ctx.settings.verbose then
			return { alert = { title = "API probe: " .. fails .. " failed", body = summary } }
		end
		return { toast = "API probe: " .. fails .. " failed" }
	end,
}
