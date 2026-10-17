-- Screens incoming friend requests for throwaway and banned accounts.
-- New request: declines matches if automation is on for the account, otherwise it
-- alerts and you confirm with "Review pending requests".

local DAY = 86400

local function why(ctx, req)
	local ok, u = pcall(ctx.user, req.id)
	if not ok then
		return nil
	end
	if ctx.settings.decline_banned and u.banned then
		return "banned"
	end
	if u.created and ctx.now - u.created < ctx.settings.min_age * DAY then
		return "account is " .. math.floor((ctx.now - u.created) / DAY) .. " days old"
	end
	if u.friends and u.friends < ctx.settings.min_friends then
		return "only " .. u.friends .. " friends"
	end
	return nil
end

local function suspicious(ctx)
	local out = {}
	for _, req in ipairs(ctx.friend_requests { limit = 50 }) do
		local reason = why(ctx, req)
		if reason then
			table.insert(out, { req = req, reason = reason })
		end
	end
	return out
end

bonsai.on("friend_request", function(ctx, e)
	local hits = suspicious(ctx)
	if #hits == 0 then
		return
	end
	local actions = {}
	local lines = {}
	for _, h in ipairs(hits) do
		table.insert(actions, { id = "user.decline_friend", arg = tostring(h.req.id) })
		table.insert(lines, (h.req.name or h.req.id) .. ": " .. h.reason)
	end
	return {
		actions = actions,
		alert = { title = #hits .. " suspicious friend request(s)", body = table.concat(lines, "\n") },
	}
end)

bonsai.command {
	id = "review",
	title = "Review pending requests",
	keywords = { "friend", "requests", "decline", "spam", "alt" },
	arg = "none",
	run = function(ctx)
		local hits = suspicious(ctx)
		if #hits == 0 then
			return { toast = "No suspicious requests" }
		end
		local actions = {}
		for _, h in ipairs(hits) do
			table.insert(actions, { id = "user.decline_friend", arg = tostring(h.req.id) })
		end
		return { actions = actions }
	end,
}
