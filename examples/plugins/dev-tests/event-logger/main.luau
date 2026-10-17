-- Listens to every event, logs it to per-account storage, and alerts.
-- Run "Event logger: Show log" to see what was recorded.

local EVENTS = { "presence", "friend_request", "trade", "message", "chat", "game_update" }

local function describe(name, e)
	local parts = {}
	if type(e) == "table" then
		if e.new then
			table.insert(parts, "new=" .. tostring(e.new) .. " total=" .. tostring(e.total))
		elseif e.title then
			table.insert(parts, tostring(e.title))
		elseif e.from then
			table.insert(parts, "from=" .. tostring(e.from))
		else
			table.insert(parts, tostring(#e) .. " changes")
		end
	end
	return name .. " " .. table.concat(parts, " ")
end

for _, name in ipairs(EVENTS) do
	bonsai.on(name, function(ctx, e)
		local log = ctx.store.get("log") or {}
		local line = describe(name, e)
		table.insert(log, { at = ctx.now, text = line })
		while #log > ctx.settings.keep do
			table.remove(log, 1)
		end
		ctx.store.set("log", log)
		print(line)
		if ctx.settings.alerts then
			return { alert = { title = "Event: " .. name, body = line } }
		end
	end)
end

bonsai.command {
	id = "show",
	title = "Show log",
	keywords = { "events", "history" },
	arg = "none",
	run = function(ctx)
		local log = ctx.store.get("log") or {}
		if #log == 0 then
			return { toast = "No events yet" }
		end
		local out = {}
		for i = math.max(1, #log - 4), #log do
			table.insert(out, log[i].text)
		end
		return { toast = #log .. " events. Latest:\n" .. table.concat(out, "\n") }
	end,
}

bonsai.command {
	id = "clear",
	title = "Clear log",
	keywords = { "events", "reset" },
	arg = "none",
	run = function(ctx)
		ctx.store.set("log", nil)
		return { toast = "Log cleared" }
	end,
}
