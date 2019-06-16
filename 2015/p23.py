import aoc

input = aoc.get_input(23)


def get_jump_value(part):
    if part.startswith('+'):
        return int(part.lstrip('+'))
    elif part.startswith('-'):
        return int(part.rstrip('-'))
    return 0


program = [line for line in input.splitlines()]
registers = {'a': 1, 'b': 0}
instruction_pointer = 0

while instruction_pointer < len(program):
    instruction = program[instruction_pointer]
    parts = instruction.split(' ')

    if parts[0] == 'inc':
        registers[parts[1]] += 1
        instruction_pointer += 1
    elif parts[0] == 'hlf':
        registers[parts[1]] /= 2
        instruction_pointer += 1
    elif parts[0] == 'tpl':
        registers[parts[1]] *= 3
        instruction_pointer += 1
    elif parts[0] == 'jmp':
        instruction_pointer += get_jump_value(parts[1])
    elif parts[0] == 'jie':
        if registers[parts[1].rstrip(',')] % 2 == 0:
            instruction_pointer += get_jump_value(parts[2])
        else:
            instruction_pointer += 1
    elif parts[0] == 'jio':
        if registers[parts[1].rstrip(',')] == 1:
            instruction_pointer += get_jump_value(parts[2])
        else:
            instruction_pointer += 1

print(registers)
