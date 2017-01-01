charVal :: Char -> Int
charVal '(' = 1
charVal ')' = -1
charVal _   = 0

main = do
  f <- readFile "day1.txt"
  print (foldl (+) 0 (map charVal f))
