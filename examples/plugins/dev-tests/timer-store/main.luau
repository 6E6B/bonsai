-- Tests bonsai.every (the shortest allowed interval is 60 s), storage and settings.

bonsai.every(60, function(ctx)
	local n = (ctx.store.get("ticks") or 0) + 1
	ctx.store.set("ticks", n)
	local text = string.format("%s #%d", ctx.settings.label, n)
	print(text)
	if ctx.settings.mode == "toast" then
		return { toast = text }
	elseif ctx.settings.mode == "alert" then
		return { alert = { title = "Timer", body = text } }
	end
end)

bonsai.command {
	id = "ticks",
	title = "Show tick count",
	keywords = { "timer", "storage" },
	arg = "none",
	run = function(ctx)
		return { toast = "ticks: " .. tostring(ctx.store.get("ticks") or 0) }
	end,
}

bonsai.command {
	id = "reset",
	title = "Reset tick count",
	keywords = { "timer", "storage" },
	arg = "none",
	run = function(ctx)
		ctx.store.set("ticks", nil)
		return { toast = "reset" }
	end,
}

bonsai.command {
	id = "echo",
	title = "Echo text",
	keywords = { "arg", "text" },
	arg = "text",
	run = function(_, arg)
		return { toast = "you said: " .. tostring(arg) }
	end,
}

bonsai.command {
	id = "open-self",
	title = "Open user 1",
	keywords = { "open", "navigate" },
	arg = "none",
	run = function()
		return { open = { kind = "user", id = 1 } }
	end,
}
