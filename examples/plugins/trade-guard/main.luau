-- Screens inbound trade partners for throwaway or banned accounts and summarizes collectibles.

local DAY = 86400

local function flag_partner(ctx, partner)
	local ok, u = pcall(ctx.user, partner.id)
	if not ok or not u then
		return nil
	end
	if u.banned then
		return "banned"
	end
	if u.created and ctx.now - u.created < ctx.settings.min_age * DAY then
		return math.floor((ctx.now - u.created) / DAY) .. " days old"
	end
	return nil
end

local function suspicious_trades(ctx)
	local out = {}
	for _, t in ipairs(ctx.trades { status = "inbound", limit = 25 }) do
		if t.partner then
			local reason = flag_partner(ctx, t.partner)
			if reason then
				table.insert(out, { trade = t, reason = reason })
			end
		end
	end
	return out
end

bonsai.on("trade", function(ctx, _)
	local hits = suspicious_trades(ctx)
	if #hits == 0 then
		return
	end
	if ctx.settings.alert_only then
		local lines = {}
		for _, h in ipairs(hits) do
			local who = h.trade.partner.name or tostring(h.trade.partner.id)
			table.insert(lines, "@" .. who .. ": " .. h.reason)
		end
		return {
			alert = {
				title = #hits .. " suspicious inbound trade(s)",
				body = table.concat(lines, "\n"),
			},
		}
	end
	local actions = {}
	for _, h in ipairs(hits) do
		table.insert(actions, { id = "trade.decline", arg = tostring(h.trade.id) })
	end
	return { actions = actions }
end)

bonsai.command {
	id = "review",
	title = "Review inbound trades",
	keywords = { "trade", "inbound", "decline", "scam", "alt" },
	arg = "none",
	run = function(ctx)
		local hits = suspicious_trades(ctx)
		if #hits == 0 then
			return { toast = "No suspicious inbound trades" }
		end
		local actions = {}
		for _, h in ipairs(hits) do
			table.insert(actions, { id = "trade.decline", arg = tostring(h.trade.id) })
		end
		return { actions = actions }
	end,
}

bonsai.command {
	id = "rap",
	title = "Summarize user's collectibles",
	keywords = { "inventory", "collectibles", "limiteds", "rap", "worth", "trade" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local items = ctx.inventory { user = arg, limit = 200 }
		if #items == 0 then
			return { toast = "No collectibles found (or inventory is private)" }
		end
		local total_rap = 0
		local on_hold = 0
		local top_name, top_rap = nil, 0
		for _, item in ipairs(items) do
			local r = item.rap or 0
			total_rap += r
			if item.on_hold then
				on_hold += 1
			end
			if r > top_rap then
				top_rap = r
				top_name = item.name
			end
		end
		local top = top_name and string.format(". Top: %s (%d RAP)", top_name, top_rap) or ""
		return {
			toast = string.format(
				"%d collectibles, %d total RAP (%d on hold)%s",
				#items,
				total_rap,
				on_hold,
				top
			),
		}
	end,
}
