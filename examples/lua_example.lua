module = {}

local entry = {}
function entry.read (index)
    if index == 1 then
        return "Hello, World!"
    elseif index == 2 then
        return "This is Lua!"
    end
end

function module.entry()
    return entry
end
