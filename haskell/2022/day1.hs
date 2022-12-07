import Data.List

main = do
    content <- readFile "input.txt"
    putStr "Part 1: "
    print . part1 . parse $ content
    putStr "Part 2: "
    print . part2 . parse $ content

part1 :: [[Int]] -> Int
part1 xs = maximum (map sum xs)

part2 :: [[Int]] -> Int
part2 xs = sum . take 3 $ sortBy (flip compare) (map sum xs)

parse :: String -> [[Int]]
parse s = map (map read) (splitList (lines s))

splitList :: [String] -> [[String]]
splitList s = case dropWhile (== "") s of
    [] -> []
    s' -> w : splitList s''
        where (w, s'') = break (== "") s'
