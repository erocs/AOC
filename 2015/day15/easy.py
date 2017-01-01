import re

RE = re.compile('^(\\w+): capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (-?\\d+)$')


class Ingredient(object):
  def __init__(self, dikt):
    self.props = dikt
    self.name = dikt['name']
    self.capacity = dikt['capacity']
    self.durability = dikt['durability']
    self.flavor = dikt['flavor']
    self.texture = dikt['texture']
    self.calories = dikt['calories']
    self.count = 1
    self.value = self.capacity + self.durability + self.flavor + self.texture
  def __str__(self):
    return '%s(Capacity %d,Durability %d,Flavor %d,Texture %d,Calories %d)' % (
        self.name, self.capacity, self.durability, self.flavor, self.texture, self.calories)

class Property(object):
  def __init__(self, name):
    self.name = name
    self.ingredients = []
  def add(self, ingredient):
    self.ingredients.append(ingredient)
  def total(self):
    total = 0
    for ing in self.ingredients:
      n = ing.props[self.name] * ing.count
      total += n
    return total

def result(props):
  total = 1
  for prop in props:
    t = prop.total()
    if t < 0:
      t = 0
    total *= t
  return total

with open("data.txt") as fin:
  props = {
      'capacity': Property('capacity'),
      'durability': Property('durability'),
      'flavor': Property('flavor'),
      'texture': Property('texture'),
#      'calories': Property('calories'),
      }
  ingredients = []
  for line in fin:
    line = line.strip()
    match = RE.match(line)
    if not match:
      print 'Bad line: ' + line
      continue
    ingredient = {
      'name': match.group(1),
      'capacity': int(match.group(2)),
      'durability': int(match.group(3)),
      'flavor': int(match.group(4)),
      'texture': int(match.group(5)),
      'calories': int(match.group(6)),
      }
    ingredient = Ingredient(ingredient)
    ingredients.append(ingredient)
    for p in props.itervalues():
      p.add(ingredient)
    print ingredient

  priorty = sorted(ingredients, key=lambda ing: ing.value, reverse=True)
  for n in xrange(4, 100):
    added = False
    for ing in priorty:
      ing.count += 1
      redo = False
      for p in props.itervalues():
        if p.total() <= 0:
          redo = True
          break
      if redo:
        ing.count -= 1
      else:
        added = True
        break
    if not added:
      priorty[0].count += 1
    print sum(ing.count for ing in ingredients), result(props.itervalues())
  print [(ing.name, ing.count) for ing in ingredients]
  print [(prop.name, prop.total()) for prop in props.itervalues()]






