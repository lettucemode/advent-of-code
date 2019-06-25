local hylib = require 'hylib'

io.input('../inputs/p7.txt')
local input = io.read('*a')

function has_abba(str)
    for i = 1, #str - 3 do
        if str:sub(i,i) == str:sub(i+3,i+3) and 
            str:sub(i,i) ~= str:sub(i+1,i+1) and
            str:sub(i+1,i+1) == str:sub(i+2,i+2) then
            return true
        end
    end
    return false
end

function supports_tls(str)
    for s in str:gmatch('%[%a+%]') do
        if has_abba(s) then
            return false
        end
    end
    no_brackets = str:gsub('(%[%a+%])', '|')
    if has_abba(no_brackets) then
        return true
    end
    return false
end

function has_aba(str, prev_i)
    for i = prev_i or 1, #str - 2 do
        if str:sub(i,i) == str:sub(i+2,i+2) and 
            str:sub(i,i) ~= str:sub(i+1,i+1) then
            return true, str:sub(i,i), str:sub(i+1,i+1), i
        end
    end
    return false, '', '', 0
end

function has_bab(str, a, b)
    for i = 1, #str - 2 do
        if str:sub(i,i) == b and 
            str:sub(i+1,i+1) == a and 
            str:sub(i+2,i+2) == b then
            return true
        end
    end
    return false
end

function supports_ssl(str)
    local result = true
    local i = 0
    while result do
        no_brackets = str:gsub('(%[%a+%])', '||')
        result, a, b, i = has_aba(no_brackets, i+1)
        if not result then
            return false
        end
        for s in str:gmatch('%[%a+%]') do
            if has_bab(s, a, b) then
                return true
            end
        end
    end
end

local tls_count = 0
local ssl_count = 0
for line in input:splitlines() do
    if supports_tls(line) then
        tls_count = tls_count + 1
    end
    if supports_ssl(line) then
        ssl_count = ssl_count + 1
    end
end

print(tls_count, ssl_count)