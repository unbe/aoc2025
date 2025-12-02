import sys

with open(sys.argv[1], 'r') as f:
  pos = 50
  part1 = 0
  part2 = 0
  for line in f:
    sign = 1 if line[0] == 'R' else -1
    step = int(line[1:])
    oldpos = pos
    next = pos + sign * step
    pos = next % 100
    rounds = abs(next - pos) // 100
    if (pos == 0 and sign > 0) or (oldpos == 0 and sign < 0):
      rounds -= 1
    part2 += rounds
    if pos == 0:
      part1 += 1
      part2 += 1
  print("part1:", part1)
  print("part2:", part2)