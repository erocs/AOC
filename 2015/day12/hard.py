import json

def is_num(s):
  if isinstance(s, int):
    return True
  for ch in s:
    if ch < '0' or ch > '9':
      return False
  return True

def sum_js(js):
  sum = 0
  if isinstance(js, list):
    for ele in js:
      sum += sum_js(ele)
    return sum
  elif isinstance(js, dict):
    for ele in js.itervalues():
      if ele == 'red':
        return 0
      sum += sum_js(ele)
    return sum
  elif is_num(js):
    return int(js)
  else:
    return 0

def solve():
  with open('day12.txt') as fin:
    js = json.load(fin)
  print sum_js(js)

if __name__ == '__main__':
  solve()
  # 96852
