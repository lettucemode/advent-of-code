local hylib = require 'hylib'
local md5 = require 'md5'

io.input('../inputs/p11.txt')
local input = io.read('*a')
local CHIP = 'M'
local GENERATOR = 'G'

function parse_state(desc)
    local state = {}
    local floor = 1
    for line in desc:splitlines() do
        state[floor] = {}
        local gens = hylib.iter_to_table(line:gmatch(' (%a+) generator'))
        for k, v in pairs(gens) do
            table.insert(state[floor], v:sub(1,1):upper()..GENERATOR)
        end
        local chips = hylib.iter_to_table(line:gmatch(' (%a+)-compat'))
        for k, v in pairs(chips) do
            table.insert(state[floor], v:sub(1,1):upper()..CHIP)
        end
        floor = floor + 1
    end
    return state
end

function print_floors(floors)
    for i = #floors, 1, -1 do
        print(i, table.concat(floors[i], '..'))
    end
end

function get_floors_string(floors)
    local str = ''
    for i = #floors, 1, -1 do
        str = str..i..'    '..table.concat(floors[i], '..')..'\n'
    end
    return str
end

function get_encoded_string(encoded)
    local str = ''
    for k, v in pairs(encoded) do
        str = str..k..'::'..v.CHIP..'/'..v.GENERATOR..'/'..v.ELEVATOR..'\n'
    end
    return str
end

function is_chip(chip)
    return chip:sub(2,2) == CHIP
end

function is_gen(gen)
    return gen:sub(2,2) == GENERATOR
end

function is_paired(gen, chip)
    return is_gen(gen) and gen:sub(1,1) == chip:sub(1,1)
end

function get_element(gen_or_chip)
    return gen_or_chip:sub(1,1)
end

function is_legal(floors)
    for i, floor in ipairs(floors) do
        for chip in hylib.where(floor, is_chip) do
            if hylib.any(floor, is_gen) and
               not hylib.any(floor, is_paired, chip) then
                return false
            end
        end
    end
    return true
end

function is_solved(floors)
    for i = 1, #floors - 1 do
        if #floors[i] > 0 then return false end
    end
    return true
end

function get_legal_moves(state)
    local floors_moving_to = { math.max(1, state.elevator - 1), math.min(#state.floors, state.elevator + 1) }
    local new_states = {}
    for f = 1, 2 do
        
        -- don't move to the same floor
        if floors_moving_to[f] == state.elevator then goto continue end

        -- if dest floor and floors below it are empty, skip it
        local _, first_nonempty_floor = hylib.where(state.floors, function(flo) return #flo > 0 end)()
        if first_nonempty_floor > floors_moving_to[f] then goto continue end

        -- brute force single item move
        for k, v in pairs(state.floors[state.elevator]) do
            local new_floors = hylib.deepcopy(state.floors)
            table.insert(new_floors[floors_moving_to[f]], v)
            table.remove(new_floors[state.elevator], k)
            if is_legal(new_floors) then
                table.insert(new_states, { floors=new_floors, 
                                            elevator=floors_moving_to[f],
                                            num_moves=state.num_moves+1})
            end
        end

        -- brute force double item move
        for k1, v1 in pairs(state.floors[state.elevator]) do
            for k2, v2 in pairs(state.floors[state.elevator]) do
                if k1 == k2 then goto inner_continue end
                local new_floors = hylib.deepcopy(state.floors)
                table.insert(new_floors[floors_moving_to[f]], v1)
                table.insert(new_floors[floors_moving_to[f]], v2)
                table.remove(new_floors[state.elevator], math.max(k1, k2))
                table.remove(new_floors[state.elevator], math.min(k1, k2))
                if is_legal(new_floors) then
                    table.insert(new_states, { floors=new_floors, 
                                                elevator=floors_moving_to[f],
                                                num_moves=state.num_moves+1})
                end
                ::inner_continue::
            end
        end

        ::continue::
    end

    -- give em back
    return new_states
end

function codify(floors, elevator)
    local elements = {}
    local codified = {}
    for i = 1, #floors do
        table.sort(floors[i])
        for j = 1, #floors[i] do
            local el = get_element(floors[i][j])
            if not elements[el] then
                codified[#codified+1] = {}
                elements[el] = #codified
            end
            if is_chip(floors[i][j]) then
                codified[elements[el]].CHIP = i
            else
                codified[elements[el]].GENERATOR = i
            end
            codified[elements[el]].ELEVATOR = elevator
        end
    end
    return codified
end

function is_equivalent(code1, code2)
    local copy = hylib.deepcopy(code2)
    for k, v1 in pairs(code1) do
        local _, idx = hylib.where(copy, 
            function(v2) return v1.CHIP == v2.CHIP and 
                                v1.GENERATOR == v2.GENERATOR and 
                                v1.ELEVATOR == v2.ELEVATOR end)()
        if not idx then return false end
        table.remove(copy, idx)
    end
    return true
end

function not_seen_yet(priors, floors, elevator)
    local codified = codify(floors, elevator)
    local hash = md5.sum(get_encoded_string(codified))
    local found = priors[hash] ~= nil
    if not found then
        priors[hash] = 1
    end
    return not found
end

local start_floors = parse_state(input)
local all_states = {{floors=start_floors, elevator=1, num_moves=0}}
local already_seen = {codify(start_floors)}
local done = false
local step = 0
while not done do
    print(string.format('states at step #%d --> ', step), #all_states)
    if #all_states == 0 then break end

    local next_states = {}
    for _, state in pairs(all_states) do
        local new_states = hylib.filter(get_legal_moves(state), 
            function(s) return not_seen_yet(already_seen, s.floors, s.elevator) end)
        for _, new_move in pairs(new_states) do
            if is_solved(new_move.floors) then 
                print(' SOLUTION --> ', new_move.num_moves)
                done = true
            else
                table.insert(next_states, new_move)
            end
            if done then break end
        end
        if done then break end
    end
    if done then break end
    
    all_states = next_states
    step = step + 1
end