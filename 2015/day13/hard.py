import collections
import itertools
import re

RE = re.compile('^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)\.$')

def table(peeps, hippiness):
  best_hippiness = -100000
  for comb in itertools.permutations(peeps, len(peeps)):
    hippy = 0
    i = 0
    while i < len(comb) - 1:
      p1 = comb[i]
      p2 = comb[i + 1]
      hl = hippiness[p1][p2]
      hr = hippiness[p2][p1]
      hippy += hl + hr
      i += 1
    p1 = comb[0]
    hl = hippiness[p1][p2]
    hr = hippiness[p2][p1]
    hippy += hl + hr
    if hippy > best_hippiness:
      best_hippiness = hippy
  return best_hippiness

def solve():
  peeps = set()
  hippiness = collections.defaultdict(dict)
  with open('day13.txt') as fin:
    for line in fin:
      line = line.strip()
      match = RE.match(line)
      if not match:
        raise Exception('Bad line: ' + line)
      per = match.group(1)
      amt = int(match.group(3))
      if match.group(2) == 'lose':
        amt *= -1
      oth = match.group(4)
      hippiness[per][oth] = amt
      peeps.add(per)
      peeps.add(oth)
  for p in peeps:
    hippiness[p]['me'] = 0
    hippiness['me'][p] = 0
  peeps.add('me')
  print table(peeps, hippiness)
  # 725

if __name__ == '__main__':
  solve()
