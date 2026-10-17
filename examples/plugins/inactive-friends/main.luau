-- Finds friends you haven't seen online for a long time and offers to remove them.
-- Bonsai can only see presence while it runs, so it records "last seen" every 5 minutes
-- and counts a friend as inactive only after the plugin has watched them for the full period.

local DAY = 86400

bonsai.every(300, function(ctx)
	local seen = ctx.store.get("seen") or {}
	if ctx.store.get("since") == nil then
		ctx.store.set("since", ctx.now)
	end
	local current = {}
	for _, f in ipairs(ctx.friends()) do
		current[tostring(f.id)] = true
		if f.status ~= "offline" then
			seen[tostring(f.id)] = ctx.now
		end
	end
	for id in pairs(seen) do
		if not current[id] then
			seen[id] = nil -- no longer a friend
		end
	end
	ctx.store.set("seen", seen)
end)

local function inactive(ctx)
	local seen = ctx.store.get("seen") or {}
	local since = ctx.store.get("since") or ctx.now
	local cutoff = ctx.now - ctx.settings.days * DAY
	if since > cutoff then
		return nil, math.ceil((since - cutoff) / DAY)
	end
	local out = {}
	for _, f in ipairs(ctx.friends()) do
		local last = seen[tostring(f.id)]
		if f.status == "offline" and (last == nil or last < cutoff) then
			table.insert(out, f)
		end
	end
	return out
end

bonsai.command {
	id = "review",
	title = "Review inactive friends",
	keywords = { "friends", "cleanup", "inactive", "unfriend", "old" },
	arg = "none",
	run = function(ctx)
		local list, wait = inactive(ctx)
		if list == nil then
			return { toast = "Still collecting data. Ask again in about " .. wait .. " days." }
		end
		if #list == 0 then
			return { toast = "No inactive friends" }
		end
		local out = {}
		for _, f in ipairs(list) do
			table.insert(out, { id = "user.unfriend", arg = tostring(f.id) })
		end
		return { actions = out }
	end,
}

bonsai.command {
	id = "count",
	title = "Count inactive friends",
	keywords = { "friends", "inactive", "stats" },
	arg = "none",
	run = function(ctx)
		local list, wait = inactive(ctx)
		if list == nil then
			return { toast = "Collecting data for " .. wait .. " more days" }
		end
		return { toast = #list .. " friends inactive for " .. ctx.settings.days .. "+ days" }
	end,
}
