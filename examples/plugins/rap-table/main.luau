-- A view: the account's collectibles by RAP, with a filter. Shows how a command returns a tree
-- of Bonsai's components and gets the user's input back in `event`.

local ui = bonsai.ui

local function draw(ctx, min)
	local rows, total = {}, 0
	for _, item in ctx.inventory { limit = 500 } do
		local rap = item.rap or 0
		if rap >= min then
			table.insert(rows, { item.name or "?", rap })
			total += rap
		end
	end
	table.sort(rows, function(a, b)
		return a[2] > b[2]
	end)
	return ui.column {
		ui.row {
			ui.stat { label = "Items", value = #rows },
			ui.stat { label = "Total RAP", value = total },
		},
		ui.row {
			ui.input { key = "min", label = "Minimum RAP", value = min, action = "filter" },
			ui.button { label = "Filter", action = "filter", style = "primary" },
		},
		ui.table { columns = { "Item", { label = "RAP", align = "end" } }, rows = rows },
	}
end

bonsai.command {
	id = "show",
	title = "Collectibles by RAP",
	keywords = { "rap", "value", "limiteds", "collectibles", "inventory" },
	run = function(ctx)
		if ctx.account == 0 then
			return { toast = "Allow this plugin to use the account in Settings > Plugins." }
		end
		return { view = draw(ctx, 0) }
	end,
	event = function(ctx, e)
		return { view = draw(ctx, tonumber(e.values.min) or 0) }
	end,
}
