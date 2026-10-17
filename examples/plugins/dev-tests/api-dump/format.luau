-- Flattens a read result into short lines for the toast pager.

local MAX_LIST = 8
local MAX_TEXT = 72

local function shorten(s)
	if #s <= MAX_TEXT then
		return s
	end
	return string.sub(s, 1, MAX_TEXT - 3) .. "..."
end

local function scalar(v)
	local ty = type(v)
	if ty == "string" then
		return shorten(string.format("%q", v))
	end
	return tostring(v)
end

local function is_list(t)
	local n = 0
	for k in pairs(t) do
		if type(k) ~= "number" or k < 1 or k % 1 ~= 0 then
			return false
		end
		n += 1
	end
	return n == #t
end

local function flatten(value, prefix, out, depth)
	if depth > 5 then
		table.insert(out, prefix .. " = ...")
		return
	end
	if type(value) ~= "table" then
		table.insert(out, prefix .. " = " .. scalar(value))
		return
	end
	if next(value) == nil then
		table.insert(out, prefix .. " = {}")
		return
	end
	if is_list(value) then
		table.insert(out, string.format("%s (%d)", prefix, #value))
		for i, v in ipairs(value) do
			if i > MAX_LIST then
				table.insert(out, string.format("%s[%d..] %d more", prefix, i, #value - i + 1))
				break
			end
			flatten(v, prefix .. "[" .. i .. "]", out, depth + 1)
		end
		return
	end
	local keys = {}
	for k in pairs(value) do
		table.insert(keys, k)
	end
	table.sort(keys, function(a, b)
		return tostring(a) < tostring(b)
	end)
	for _, k in ipairs(keys) do
		flatten(value[k], prefix .. "." .. tostring(k), out, depth + 1)
	end
end

return function(value, prefix)
	local out = {}
	flatten(value, prefix, out, 0)
	return out
end
