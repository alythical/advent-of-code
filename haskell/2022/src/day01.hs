import Data.List (sortBy)
import Data.Ord (Down (Down), comparing)
import System.IO

parse x = read x :: Int

parseElf x = sum $ map parse (takeWhile (/= "") x)

parseElves x =
  let parts = span (/= "") x
      elf = parseElf (fst parts)
   in case snd parts of
        [] -> [elf]
        _ -> elf : parseElves (tail (snd parts))

partOne x = maximum $ parseElves x

partTwo x = sum . take 3 . sortBy (comparing Down) $ parseElves x

main = do
  handle <- openFile "inputs/day01.txt" ReadMode
  contents <- hGetContents handle
  let elves = lines contents
  print (partOne elves)
  print (partTwo elves)
  hClose handle
