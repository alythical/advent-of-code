import Data.Bifunctor (bimap)
import System.IO

withinRange range n = n >= fst range && n <= snd range

allWithinRange left right = all (withinRange right) [fst left .. snd left]

anyWithinRange left right = any (withinRange right) [fst left .. snd left]

convert x n = read (drop n x) :: Int

intRange raw n = bimap (`convert` n) (`convert` 1) $ span (/= '-') raw

parse line = bimap (`intRange` 0) (`intRange` 1) $ span (/= ',') line

countRanges ranges f = length . filter id $ map (\(fst, snd) -> f fst snd || f snd fst) ranges

partOne ranges = countRanges ranges allWithinRange

partTwo ranges = countRanges ranges anyWithinRange

main = do
  handle <- openFile "inputs/day04.txt" ReadMode
  contents <- hGetContents handle
  let ranges = map parse $ lines contents
  print $ partOne ranges
  print $ partTwo ranges
  hClose handle