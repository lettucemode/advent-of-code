import aoc
import json

input = aoc.get_input(12)

# # # # # #
# part 2
# # # # # #


def recurse(data, nums):
    if type(data) in (list, tuple):
        for i in range(len(data)):
            recurse(data[i], nums)
    elif type(data) is dict:
        for k in data.keys():
            if data[k] == 'red':
                return
        for k in data.keys():
            recurse(data[k], nums)
    elif str(data).lstrip('-').isnumeric():
        nums.append(data)


data = json.load(
    open(f'd:\\source\\repos\\advent-of-code\\2015\\inputs\\p12.txt'))
nums = []
recurse(data, nums)
print(sum(nums))


# # # # # #
# part 1
# # # # # #
# nums = []
# numStr = ''

# for i in range(len(input)):
#     if input[i].isnumeric() or input[i] == '-':
#         numStr += input[i]
#     elif numStr != '':
#         num = int(numStr)
#         nums.append(num)
#         numStr = ''

# print(nums)
# print(sum(nums))
