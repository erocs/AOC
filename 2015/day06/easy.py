import array
import re

RE = re.compile('^([\w ]+) (\d+),(\d+) through (\d+),(\d+)$')

def newB(size):
  arr = array.array('L')
  size = size >> 5
  size += 1
  arr.extend(0 for _ in xrange(size))
  return arr

def getB(arr, idx):
  bit = idx & 0x1F
  idx = idx >> 5
  val = 1 if (arr[idx] & (1 << bit)) != 0 else 0
  # print "getB(%d,%d) -> %d" % (idx, bit, val)
  return val

def setB(arr, idx, val):
  bit = idx & 0x1F
  idx = idx >> 5
  orig = arr[idx]
  if val:
    arr[idx] = orig | (1 << bit)
  else:
    arr[idx] = orig & ~(1 << bit)
  # print "setB(%d,%d) -> %d" % (idx, bit, val)

def onB(arr, idx):
  setB(arr, idx, 1)

def offB(arr, idx):
  setB(arr, idx, 0)

def togB(arr, idx):
  if getB(arr, idx):
    setB(arr, idx, 0)
  else:
    setB(arr, idx, 1)

def r(f, arr, dim, x1, y1, x2, y2):
  # print "r(%s, %d, %d, %d, %d, %d)" % (f.__name__, dim, x1, y1, x2, y2)
  for y in xrange(y1, y2+1):
    idx = y * dim + x1
    for x in xrange(x1, x2+1):
      f(arr, idx)
      idx += 1

def c(arr, dim):
  count = 0
  for idx in xrange(dim * dim):
    if getB(arr, idx):
      count += 1
  return count

def puzzle():
  arr = newB(1000 * 1000)
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
        print ON, (x2 - x1 + 1) * (y2 - y1 + 1), c(arr, 1000)
      elif OFF in match.group(1):
        r(offB, arr, 1000, x1, y1, x2, y2)
        print OFF, (x2 - x1 + 1) * (y2 - y1 + 1), c(arr, 1000)
      elif TOG in match.group(1):
        r(togB, arr, 1000, x1, y1, x2, y2)
        print TOG, (x2 - x1 + 1) * (y2 - y1 + 1), c(arr, 1000)
      else:
        print 'NO CMD ' + line
  
  print c(arr, 1000)
  # 377891


def test():
  arr = newB(32 * 32)
  print c(arr, 32), 0
  r(onB, arr, 32, 0, 0, 31, 31)
  print c(arr, 32), 32 * 32
  r(offB, arr, 32, 0, 0, 31, 30)
  print c(arr, 32), 32
  r(togB, arr, 32, 0, 30, 31, 31)
  print c(arr, 32), 32

if __name__ == '__main__':
  puzzle()
  # test()
