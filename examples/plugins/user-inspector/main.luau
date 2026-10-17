-- One-line trust summary for anyone: account age, social counts and red flags.
-- Use it from a user's context menu before accepting a trade or friend request.

local DAY = 86400

local function age(ctx, created)
	if not created then
		return "unknown age"
	end
	local days = math.floor((ctx.now - created) / DAY)
	if days >= 365 then
		return string.format("%.1f years old", days / 365)
	end
	return days .. " days old"
end

bonsai.command {
	id = "inspect",
	title = "Inspect user",
	keywords = { "user", "trust", "age", "scam", "check", "alt" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		local u = ctx.user(arg)
		if not u then
			return { toast = "No such user" }
		end
		local flags = {}
		if u.banned then
			table.insert(flags, "BANNED")
		end
		if u.created and ctx.now - u.created < 30 * DAY then
			table.insert(flags, "new account")
		end
		if u.friends == 0 then
			table.insert(flags, "no friends")
		end
		if u.followers and u.following and u.followers == 0 and u.following > 50 then
			table.insert(flags, "follows many, followed by none")
		end
		if not u.description or u.description == "" then
			table.insert(flags, "empty bio")
		end
		local verdict = #flags == 0 and "no red flags" or ("flags: " .. table.concat(flags, ", "))
		return { toast = string.format("%s (@%s): %s, %s friends, %s followers. %s%s",
			tostring(u.display_name), tostring(u.name), age(ctx, u.created),
			tostring(u.friends or "?"), tostring(u.followers or "?"),
			verdict, u.verified and " (verified)" or "") }
	end,
}
