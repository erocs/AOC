inp = 'hepxcrrq'

alpbet = 'abcdefghjkmnpqrstuvwxyz'
_inc = {alpbet[i]:alpbet[i+1] for i in xrange(len(alpbet)-1)}
_inc['z'] = 'a'
invld = 'iol'

def valid(s):
  if len(s) != 8:
    return False
  l1 = len(s) - 1
  l2 = len(s) - 2
  p1 = None
  p2 = None
  trip = False
  for i in xrange(len(s)):
    # if s[i] in invld:
    #   return False
    if i < l1:
      if not p2 and s[i] == s[i + 1]:
        if not p1:
          p1 = s[i]
        elif p1 != s[i]:
          p2 = s[i]
      if not trip and i < l2 and s[i:i+3] in alpbet:
        trip = True
  return p1 and p2 and trip

def incr(s):
  carry = 1
  res = []
  for i in xrange(len(s)-1, -1, -1):
    if carry:
      ch = _inc[s[i]]
    else:
      ch = s[i]
    res.append(ch)
    if ch != 'a':
      carry = 0
  return ''.join(res[::-1])

def solve():
  c = 0
  s = inp
  while True:
    s = incr(s)
    if valid(s):
      print s
      c += 1
      if c == 2:
        break

if __name__ == '__main__':
  solve()
  # hepxxyzz
  # heqaabcc
