charVal :: Char -> Int
charVal '(' = 1
charVal ')' = -1
charVal _   = 0

findBasement :: Int -> [(Int, Int)] -> Int
findBasement a (x@(i,b):xs) | c < 0     = i
                            | otherwise = findBasement c xs
                            where c = a + b

main = do
  f <- readFile "day1.txt"
  print $ findBasement 0 . zip [1..] $ map charVal f
