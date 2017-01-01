import re

RE = re.compile('^([\w ]+) (\d+),(\d+) through (\d+),(\d+)$')

def newB(dim):
  return [[0 for _ in xrange(dim)] for _ in xrange(dim)]

def getB(arr, dim, idx):
  x = idx % dim
  y = idx / dim
  return arr[x][y]

def setB(arr, dim, idx, val):
  x = idx % dim
  y = idx / dim
  arr[x][y] = val

def onB(arr, dim, idx):
  val = getB(arr, dim, idx)
  setB(arr, dim, idx, val + 1)

def offB(arr, dim, idx):
  val = getB(arr, dim, idx)
  if val:
    setB(arr, dim, idx, val - 1)

def togB(arr, dim, idx):
  val = getB(arr, dim, idx)
  setB(arr, dim, idx, val + 2)

def r(f, arr, dim, x1, y1, x2, y2):
  for y in xrange(y1, y2+1):
    idx = y * dim + x1
    for x in xrange(x1, x2+1):
      f(arr, dim, idx)
      idx += 1

def s(arr, dim):
  t = 0
  for idx in xrange(dim * dim):
    val = getB(arr, dim, idx)
    t += val
  return t

def puzzle():
  arr = newB(1000)
  print 'start'
  x = 0
  y = 0
  
  ON = 'turn on'
  OFF = 'turn off'
  TOG = 'toggle'

  with open("day6.txt") as fin:
    for line in fin:
      match = RE.match(line)
      if not match:
        print 'NO MATCH ' + line
      x1 = int(match.group(2))
      y1 = int(match.group(3))
      x2 = int(match.group(4))
      y2 = int(match.group(5))
      if ON in match.group(1):
        r(onB, arr, 1000, x1, y1, x2, y2)
        print ON, (x2 - x1 + 1) * (y2 - y1 + 1), s(arr, 1000)
      elif OFF in match.group(1):
        r(offB, arr, 1000, x1, y1, x2, y2)
        print OFF, (x2 - x1 + 1) * (y2 - y1 + 1), s(arr, 1000)
      elif TOG in match.group(1):
        r(togB, arr, 1000, x1, y1, x2, y2)
        print TOG, (x2 - x1 + 1) * (y2 - y1 + 1), s(arr, 1000)
      else:
        print 'NO CMD ' + line
  
  print s(arr, 1000)
  # 14110788


def test():
  arr = newB(32)
  print s(arr, 32), 0
  r(onB, arr, 32, 0, 0, 31, 31)
  print s(arr, 32), 32 * 32
  r(offB, arr, 32, 0, 0, 31, 30)
  print s(arr, 32), 32
  r(togB, arr, 32, 0, 30, 31, 31)
  print s(arr, 32), 160

if __name__ == '__main__':
  puzzle()
  # test()
