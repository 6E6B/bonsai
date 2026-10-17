-- Dumps one read at a time and pages the fields through toasts.
-- Nil fields are omitted. Search the command bar for "Dump:".

local flatten = require("format")

local PAGE = 260

local show

local function remember(ctx, lines)
	ctx.store.set("lines", lines)
	return show(ctx, 1)
end

show = function(ctx, at)
	local lines = ctx.store.get("lines") or {}
	if #lines == 0 then
		return { toast = "Nothing dumped yet" }
	end
	at = math.clamp(at, 1, #lines)
	local parts = {}
	local n = 0
	local last = at
	while last <= #lines and #parts < 14 do
		local line = lines[last]
		local add = #line + (#parts > 0 and 1 or 0)
		if #parts > 0 and n + add > PAGE then
			break
		end
		table.insert(parts, line)
		n += add
		last += 1
	end
	ctx.store.set("at", at)
	ctx.store.set("next_at", last)
	local more = last <= #lines and "  (next page)" or ""
	return {
		toast = string.format("[%d-%d/%d]%s\n%s", at, last - 1, #lines, more, table.concat(parts, "\n")),
	}
end

local function dump(ctx, value, prefix)
	return remember(ctx, flatten(value, prefix))
end

bonsai.command {
	id = "user",
	title = "Dump: user",
	keywords = { "dump", "read", "user", "api" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local u = ctx.user(arg)
		if u == nil then
			return { toast = "user is nil" }
		end
		return dump(ctx, u, "user")
	end,
}

bonsai.command {
	id = "game",
	title = "Dump: game (place id)",
	keywords = { "dump", "read", "game", "place", "api" },
	arg = "game",
	context = "game",
	run = function(ctx, arg)
		local g = ctx.game(arg)
		if g == nil then
			return { toast = "no such place" }
		end
		return dump(ctx, g, "game")
	end,
}

bonsai.command {
	id = "item",
	title = "Dump: item (kind in settings)",
	keywords = { "dump", "read", "item", "catalog", "bundle", "api" },
	arg = "item",
	context = "item",
	run = function(ctx, arg)
		local kind = ctx.settings.item_kind
		local item = ctx.item(arg, kind)
		if item == nil then
			return { toast = "no such " .. kind }
		end
		return dump(ctx, item, kind)
	end,
}

bonsai.command {
	id = "friends",
	title = "Dump: friends",
	keywords = { "dump", "read", "friends", "api" },
	arg = "none",
	run = function(ctx)
		local rows = ctx.friends()
		local n = math.min(#rows, ctx.settings.rows)
		local slice = {}
		for i = 1, n do
			slice[i] = rows[i]
		end
		local lines = flatten(slice, "friends")
		table.insert(lines, 1, string.format("friends total=%d showing=%d", #rows, n))
		return remember(ctx, lines)
	end,
}

bonsai.command {
	id = "presence",
	title = "Dump: presence of friends",
	keywords = { "dump", "read", "presence", "api" },
	arg = "none",
	run = function(ctx)
		local ids = {}
		for _, f in ipairs(ctx.friends()) do
			table.insert(ids, f.id)
			if #ids >= ctx.settings.rows then
				break
			end
		end
		if #ids == 0 then
			return { toast = "No friends to ask about" }
		end
		return dump(ctx, ctx.presence(ids), "presence")
	end,
}

bonsai.command {
	id = "inventory",
	title = "Dump: inventory",
	keywords = { "dump", "read", "inventory", "api" },
	arg = "none",
	run = function(ctx)
		local which = ctx.settings.inventory
		local filter = { limit = ctx.settings.rows }
		if which ~= "collectibles" then
			filter.asset_type = tonumber(which)
		end
		local rows = ctx.inventory(filter)
		local lines = flatten(rows, "inventory")
		table.insert(lines, 1, "inventory " .. which .. " rows=" .. #rows)
		return remember(ctx, lines)
	end,
}

bonsai.command {
	id = "trades",
	title = "Dump: trades",
	keywords = { "dump", "read", "trades", "api" },
	arg = "none",
	run = function(ctx)
		local status = ctx.settings.trade_status
		local rows = ctx.trades { status = status, limit = ctx.settings.rows }
		local lines = flatten(rows, "trades")
		table.insert(lines, 1, "trades " .. status .. " rows=" .. #rows)
		return remember(ctx, lines)
	end,
}

bonsai.command {
	id = "requests",
	title = "Dump: friend requests",
	keywords = { "dump", "read", "requests", "friends", "api" },
	arg = "none",
	run = function(ctx)
		local rows = ctx.friend_requests { limit = ctx.settings.rows }
		local lines = flatten(rows, "requests")
		table.insert(lines, 1, "requests rows=" .. #rows)
		return remember(ctx, lines)
	end,
}

bonsai.command {
	id = "context",
	title = "Dump: ctx and settings",
	keywords = { "dump", "context", "settings", "api" },
	arg = "none",
	run = function(ctx)
		return dump(ctx, {
			account = ctx.account,
			now = ctx.now,
			plugin = bonsai.id,
			api = bonsai.api,
			settings = {
				item_kind = ctx.settings.item_kind,
				inventory = ctx.settings.inventory,
				trade_status = ctx.settings.trade_status,
				rows = ctx.settings.rows,
			},
		}, "ctx")
	end,
}

bonsai.command {
	id = "errors",
	title = "Dump: calls that should fail",
	keywords = { "dump", "error", "api" },
	arg = "none",
	run = function(ctx)
		local function bad(label, fn)
			local ok, err = pcall(fn)
			if ok then
				return label .. ": UNEXPECTED ok"
			end
			local text = tostring(err)
			local nl = string.find(text, "\n", 1, true)
			if nl then
				text = string.sub(text, 1, nl - 1)
			end
			text = string.gsub(text, "^runtime error: ", "")
			return label .. ": " .. text
		end
		local lines = {
			bad("user", function()
				ctx.user("nope")
			end),
			bad("item kind", function()
				ctx.item(1, "hat")
			end),
			bad("trades", function()
				ctx.trades { status = "nope" }
			end),
			bad("inventory", function()
				ctx.inventory { asset_type = 8, bundle_type = 1 }
			end),
		}
		return remember(ctx, lines)
	end,
}

bonsai.command {
	id = "next",
	title = "Dump: next page",
	keywords = { "dump", "page", "more" },
	arg = "none",
	run = function(ctx)
		local lines = ctx.store.get("lines") or {}
		local at = ctx.store.get("next_at") or (#lines + 1)
		if at > #lines then
			return { toast = "End of dump" }
		end
		return show(ctx, at)
	end,
}

bonsai.command {
	id = "prev",
	title = "Dump: previous page",
	keywords = { "dump", "page", "back" },
	arg = "none",
	run = function(ctx)
		local at = ctx.store.get("at") or 1
		if at <= 1 then
			return { toast = "Start of dump" }
		end
		return show(ctx, math.max(1, at - 8))
	end,
}
