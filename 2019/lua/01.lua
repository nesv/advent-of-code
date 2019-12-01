-- Read the input from STDIN.
weights = {}
while true do
    local line = io.read()
    if line == nil then break end
    weights[#weights+1] = tonumber(line)
end

function calc_weight(n)
    if n == 0 then return 0 end
    return math.floor(n / 3) - 2
end

-- Part 1.
sum = 0
for i = 1, #weights do
    sum = sum + calc_weight(weights[i])
end
print(sum)

-- Part 2.
total = 0
for i = 1, #weights do
    local w = calc_weight(weights[i])
    local current = w
    while true do
        local x = calc_weight(current)
        if x <= 0 then break end
        w = w + x
        current = x
    end
    total = total + w
end
print(total)