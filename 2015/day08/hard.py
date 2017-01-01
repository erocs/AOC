def encod(s):
  result = []
  for ch in s:
    if ch == '"':
      result.append('\\"')
    elif ch == '\\':
      result.append('\\\\')
    else:
      result.append(ch)
  return ''.join(result)

with open("day8.txt") as fin:
  uec = 0
  mc = 0
  for line in fin:
    line = line.strip()
    mc += len(line)
    ec = '"%s"' % encod(line)
    uec += len(ec)
print uec, mc, uec - mc
# 2085
