import md5

key = 'ckczppom'

i = 0
while True:
  h = md5.md5()
  h.update('%s%d' % (key, i))
  s = h.hexdigest()
  if s.startswith('00000'):
    break
  i += 1
print i, s

