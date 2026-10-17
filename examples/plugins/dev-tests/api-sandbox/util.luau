-- Loaded with require("util") to check in-folder modules.

return {
	label = function(ctx)
		return string.format(
			"%s / %s / %s",
			tostring(ctx.settings.choice),
			tostring(ctx.settings.flag),
			ctx.settings.note
		)
	end,
}
