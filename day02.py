import sys

with open(sys.argv[1], 'r') as f:
  part1 = 0
  part2 = 0
  for aa, bb in (x.split('-') for x in f.read().strip().split(",")):
    b = int(bb)
    a = int(aa)
    seen = set()
    for reps in range(2, len(bb) + 1):
      l = len(aa) // reps
      if l * reps != len(aa):
        v = int('1' + '0' * l)
      else:
        v = int(aa[:l])
      while True:
        bad = int(str(v)*reps)
        v += 1
        if bad < a or bad in seen:
          continue
        if bad > b:
          break
        if reps == 2:
          part1 += bad
        part2 += bad
        seen.add(bad)
  print("part1:", part1)
  print("part2:", part2)