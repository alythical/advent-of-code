import Data.Char (isAsciiLower)
import GHC.Base (ord)
import System.IO

valueOf x = if isAsciiLower x then 26 - (122 - ord x) else 52 - (90 - ord x)

containsItem rucksack item = if item `elem` rucksack then Just item else Nothing

toBool x = case x of
  Just _ -> True
  Nothing -> False

toTuple lst = (head lst, head (tail lst), head (tail (tail lst)))

check x = toBool . containsItem x

parts rucksack = splitAt (div (length rucksack) 2) rucksack

byThree rucksacks =
  take 3 rucksacks : case drop 3 rucksacks of
    [] -> []
    r -> byThree r

inCommon (rucksack, other) = head $ filter (check other) rucksack

inCommonThree (rucksack, first, second) = head $ filter (\x -> check first x && check second x) rucksack

partOne rucksacks = sum $ map (valueOf . inCommon) rucksacks

partTwo rucksacks = sum $ map (valueOf . inCommonThree) rucksacks

main = do
  handle <- openFile "inputs/day03.txt" ReadMode
  contents <- hGetContents handle
  let rucksacks = map parts (lines contents)
  print (partOne rucksacks)
  let rucksacks = map toTuple $ byThree (lines contents)
  print (partTwo rucksacks)
  hClose handle