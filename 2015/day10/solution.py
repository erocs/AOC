import itertools

inp = '3113322113'

def rle(s):
  result = []
  for k, g in itertools.groupby(s):
    g = list(g)
    lg = len(g)
    result.append('%d%s' % (lg, k))
  return ''.join(result)

s = inp
for _ in xrange(50):
  s = rle(s)
print len(s)
# 329356
# For hard, loop 50 instead of 40
# 4666278
