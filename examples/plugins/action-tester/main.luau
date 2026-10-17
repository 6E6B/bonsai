-- Tests the write paths. Use a throwaway account.
--   tier 1 runs at once, tier 2 opens the confirm dialog (or runs if automation is on),
--   refusals show the error text.

bonsai.command {
	id = "note",
	title = "Tier 1 via ctx.run: note on user (id)",
	keywords = { "write", "tier1", "note" },
	arg = "user",
	context = "user",
	run = function(ctx, arg)
		ctx.run("user.note", tostring(arg) .. " written by action-tester")
		return { toast = "note set" }
	end,
}

bonsai.command {
	id = "nickname-batch",
	title = "Tier 1 batch: nickname first 3 friends",
	keywords = { "write", "tier1", "nickname", "friends" },
	arg = "none",
	run = function(ctx)
		local out = {}
		for i, f in ipairs(ctx.friends()) do
			if i > 3 then
				break
			end
			table.insert(out, { id = "user.nickname", arg = tostring(f.id) .. " test-" .. i })
		end
		return { actions = out }
	end,
}

bonsai.command {
	id = "follow",
	title = "Tier 2 confirm: follow user (id)",
	keywords = { "write", "tier2", "follow" },
	arg = "user",
	context = "user",
	run = function(_, arg)
		return { actions = { { id = "user.follow", arg = tostring(arg) } } }
	end,
}

bonsai.command {
	id = "follow-run",
	title = "Tier 2 via ctx.run: follow user (id)",
	keywords = { "write", "tier2", "follow", "automation" },
	arg = "user",
	run = function(ctx, arg)
		local ok, err = pcall(ctx.run, "user.follow", tostring(arg))
		if ok then
			return { toast = "followed" }
		end
		return { toast = "refused: " .. tostring(err) }
	end,
}

bonsai.command {
	id = "refused",
	title = "Expect refusal: run an action not in the manifest",
	keywords = { "write", "refused", "test" },
	arg = "none",
	run = function(ctx)
		local ok, err = pcall(ctx.run, "user.block", "1")
		return { toast = (ok and "UNEXPECTED: ran" or "refused as expected: " .. tostring(err)) }
	end,
}
