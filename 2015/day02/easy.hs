import Data.List.Split

tok :: String -> (Int, Int, Int)
tok s = (read (xs !! 0) :: Int, read (xs !! 1) :: Int, read (xs !! 2) :: Int)
        where xs = splitOn "x" s

area :: (Int, Int, Int) -> Int
area xs@(w, h, d) =
    let
      wh = w * h
      wd = w * d
      hd = h * d
      sm' = if wh < wd then wh else wd
      sm = if hd < sm' then hd else sm'
    in 2 * (wh + wd + hd) + sm

main = do
  f <- readFile("data.txt")
  let ls = lines f
  print $ sum . map area $ map tok ls
