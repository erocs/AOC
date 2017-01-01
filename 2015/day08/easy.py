with open("day8.txt") as fin:
  uec = 0
  mc = 0
  for line in fin:
    line = line.strip()
    uec += len(line)
    ec = line[1:-1].decode('string_escape')
    mc += len(ec)
print uec, mc, uec - mc
# 1350
