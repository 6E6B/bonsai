-- Store, settings, command args, open, and require. No Roblox reads or writes.
-- Search the command bar for "Sandbox:".

local util = require("util")

bonsai.command {
	id = "context",
	title = "Sandbox: show ctx and settings",
	keywords = { "sandbox", "settings", "context", "api" },
	arg = "none",
	run = function(ctx)
		return {
			toast = string.format(
				"account=%s now=%s plugin=%s api=%s\n%s",
				tostring(ctx.account),
				tostring(ctx.now),
				tostring(bonsai.id),
				tostring(bonsai.api),
				util.label(ctx)
			),
		}
	end,
}

bonsai.command {
	id = "store",
	title = "Sandbox: round-trip the store",
	keywords = { "sandbox", "store", "storage", "api" },
	arg = "none",
	run = function(ctx)
		ctx.store.set("text", "hello")
		ctx.store.set("flag", true)
		ctx.store.set("n", 7)
		ctx.store.set("list", { "a", 2, false })
		ctx.store.set("map", { name = "x", nested = { ok = true } })
		local got = {
			text = ctx.store.get("text"),
			flag = ctx.store.get("flag"),
			n = ctx.store.get("n"),
			list = ctx.store.get("list"),
			map = ctx.store.get("map"),
		}
		ctx.store.set("text", nil)
		local deleted = ctx.store.get("text") == nil
		local ok = got.text == "hello"
			and got.flag == true
			and got.n == 7
			and got.list[1] == "a"
			and got.list[3] == false
			and got.map.nested.ok == true
			and deleted
		return { toast = ok and "store round-trip ok" or "store round-trip FAILED" }
	end,
}

bonsai.command {
	id = "text",
	title = "Sandbox: echo text",
	keywords = { "sandbox", "arg", "text", "api" },
	arg = "text",
	run = function(_, arg)
		return { toast = "text: " .. tostring(arg) }
	end,
}

bonsai.command {
	id = "account",
	title = "Sandbox: echo account arg",
	keywords = { "sandbox", "arg", "account", "api" },
	arg = "account",
	run = function(ctx, arg)
		return { toast = string.format("arg account=%s ctx.account=%s", tostring(arg), tostring(ctx.account)) }
	end,
}

local function open_cmd(id, title, kind, arg)
	bonsai.command {
		id = id,
		title = title,
		keywords = { "sandbox", "open", kind, "api" },
		arg = arg,
		context = kind,
		run = function(_, id)
			return { open = { kind = kind, id = tonumber(id) } }
		end,
	}
end

open_cmd("open-user", "Sandbox: open user", "user", "user")
open_cmd("open-game", "Sandbox: open game", "game", "game")
open_cmd("open-item", "Sandbox: open item", "item", "item")
open_cmd("open-group", "Sandbox: open group", "group", "text")

bonsai.command {
	id = "late",
	title = "Sandbox: register a command after load",
	keywords = { "sandbox", "error", "api" },
	arg = "none",
	run = function()
		local ok, err = pcall(function()
			bonsai.command {
				id = "too-late",
				title = "Should not register",
				run = function() end,
			}
		end)
		if ok then
			return { toast = "UNEXPECTED: registered after load" }
		end
		return { toast = tostring(err) }
	end,
}

bonsai.command {
	id = "timeout",
	title = "Sandbox: trip the 50 ms time limit",
	keywords = { "sandbox", "timeout", "limit", "error", "api" },
	arg = "none",
	run = function()
		while true do
		end
	end,
}

bonsai.command {
	id = "memory",
	title = "Sandbox: trip the 32 MB memory cap",
	keywords = { "sandbox", "memory", "limit", "error", "api" },
	arg = "none",
	run = function()
		local s = string.rep("x", 64 * 1024 * 1024)
		return { toast = tostring(#s) }
	end,
}

