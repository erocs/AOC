import re

RE_READ = re.compile('^([\w\d]+)$')
RE_NOT = re.compile('^NOT ([\w\d]+)$')
RE_EXPR = re.compile('^([\w\d]+) (\w+) ([\w\d]+)$')

ops = {}

IUNK = object()
IREAD = object()
INOT = object()
IEXPR = object()

def reset():
  ops = {}

def isConst(s):
  for ch in s:
    if '0' > ch or '9' < ch:
      return False
  return True

def opAnd(a, b):
  return a & b

def opOr(a, b):
  return a | b

def opLshift(a, b):
  return a << b

def opRshift(a, b):
  return a >> b

class Op(object):
  _ops = {
      'AND': opAnd,
      'OR': opOr,
      'LSHIFT': opLshift,
      'RSHIFT': opRshift,
      }

  def __init__(self, definition):
    self.definition = definition.strip()
    self.val = None
    self.in_type = IUNK
    self.in_op = None
    self.inw = []
    self._parse()
    self._register()
    self.try_solve()

  def _register(self):
    if self.outw in ops:
      raise Exception('Wire already registered: ' + self.outw)
    ops[self.outw] = self

  def _parse(self):
    idx = self.definition.find(' -> ')
    if idx < 0:
      raise Exception('Parse Error: ' + self.definition)
    self.in_expr = self.definition[:idx]
    self.outw = self.definition[idx + 4:]
    match = RE_READ.match(self.in_expr)
    if match:
      self.in_type = IREAD
      self.inw.append(match.group(1))
      return
    match = RE_NOT.match(self.in_expr)
    if match:
      self.in_type = INOT
      self.inw.append(match.group(1))
      return
    match = RE_EXPR.match(self.in_expr)
    if match:
      self.in_type = IEXPR
      self.in_op = match.group(2)
      self.inw.append(match.group(1))
      self.inw.append(match.group(3))
      return
    raise Exception('Invalid in_expr: ' + self.in_expr)

  def try_solve(self):
    if self.val is not None:
      return self.val
    if self.in_type == IREAD:
      if isConst(self.inw[0]):
        self.val = int(self.inw[0])
        return self.val
      a = ops.get(self.inw[0], None)
      if a:
        self.val = a.try_solve()
      else:
        return None
      return self.val
    if self.in_type == INOT:
      if isConst(self.inw[0]):
        val = int(self.inw[0])
      else:
        a = ops.get(self.inw[0], None)
        if a:
          val = a.try_solve()
        else:
          return None
      if val is not None:
        self.val = 0b1111111111111111 - val
        return self.val
      return None
    if self.in_type != IEXPR:
      raise Exception('Unknown in_type for ' + self.outw)
    if isConst(self.inw[0]):
      a = int(self.inw[0])
    else:
      a = ops.get(self.inw[0], None)
      if a:
        a = a.try_solve()
      if a is None:
        return None
    if isConst(self.inw[1]):
      b = int(self.inw[1])
    else:
      b = ops.get(self.inw[1], None)
      if b:
        b = b.try_solve()
      if b is None:
        return None
    op = self._ops[self.in_op]
    self.val = op(a, b)
    return self.val

def main():
  with open('day7.txt') as fin:
    for line in fin:
      Op(line)
  print ops['a'].try_solve()
  # 956
  # For hard, just change wire b in day7.txt to 956 and rerun
  # 40149

def test():
  e = Op('NOT d -> e')
  d = Op('NOT c -> d')
  c = Op('a OR b -> c')
  a = Op('7 -> a')
  b = Op('4 -> b')
  print e.try_solve()

if __name__ == '__main__':
  main()
  # test()
