import collections
import re

RE = re.compile('^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\\.')

DRUN = object()
DREST = object()

def simul(rdeer, tim):
  for i in xrange(tim + 1):
    for deer in rdeer.itervalues():
      deer['timer'] += 1
      if deer['state'] == DRUN:
        deer['dist'] += deer['speed']
        if deer['timer'] >= deer['dura']:
          deer['timer'] = 0
          deer['state'] = DREST
      elif deer['timer'] >= deer['rest']:
        deer['timer'] = 0
        deer['state'] = DRUN

def solve():
  tim = 2503
  rdeer = collections.defaultdict(dict)
  with open("day14.txt") as fin:
    for line in fin:
      line = line.strip()
      match = RE.match(line)
      if not match:
        raise Exception("Bad line: " + line)
      rdeer[match.group(1)]['speed'] = int(match.group(2))
      rdeer[match.group(1)]['dura'] = int(match.group(3))
      rdeer[match.group(1)]['rest'] = int(match.group(4))
      rdeer[match.group(1)]['state'] = DRUN
      rdeer[match.group(1)]['timer'] = 0
      rdeer[match.group(1)]['dist'] = 0
  simul(rdeer, tim)
  winner = sorted(rdeer.itervalues(), key=lambda d: d['dist'], reverse=True)[0]
  print winner
  # 2655

if __name__ == '__main__':
  solve()
