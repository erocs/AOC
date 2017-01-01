import collections
import itertools

def solve():
  cities = set()
  dists = collections.defaultdict(dict)
  with open('day9.txt') as fin:
    for line in fin:
      line = line.strip()
      idx = line.find(' to ')
      if idx < 0:
        raise Exception('Bad line: ' + line)
      idy = line.find(' = ')
      if idy < 0:
        raise Exception('Bad line: ' + line)
      t1 = line[:idx]
      t2 = line[idx+4:idy]
      cities.add(t1)
      cities.add(t2)
      d = int(line[idy+3:])
      if t1 < t2:
        dists[t1][t2] = d
      else:
        dists[t2][t1] = d
  best_dist = 0xFFFFFFFF
  for comb in itertools.permutations(cities, len(cities)):
    dist = 0
    i = 0
    while i < len(comb) - 1:
      if comb[i] < comb[i + 1]:
        t1 = comb[i]
        t2 = comb[i + 1]
      else:
        t1 = comb[i + 1]
        t2 = comb[i]
      d = dists[t1][t2]
      dist += d
      i += 1
    if dist < best_dist:
      best_dist = dist
  print best_dist
  # 251

if __name__ == '__main__':
  solve()
