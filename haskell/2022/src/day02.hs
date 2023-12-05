import Data.Bifunctor (bimap)
import Distribution.Utils.String (trim)
import System.IO

data Choice = R | P | S deriving (Show, Eq)

choiceScore o = case o of
  R -> 1
  P -> 2
  S -> 3

score (o, m) =
  if o == m
    then 3 + choiceScore m
    else case (m, o) of
      (R, S) -> choiceScore m + 6
      (P, R) -> choiceScore m + 6
      (S, P) -> choiceScore m + 6
      _ -> choiceScore m

choices = map choiceScore [R, P, S]

rescore (o, w) =
  let opponent = choiceScore o
   in case w of
        S -> (choices !! mod opponent 3) + 6
        P -> 3 + choiceScore o
        R -> choices !! mod (opponent + 1) 3

parseChar c = case trim c of
  "A" -> R
  "B" -> P
  "C" -> S
  "X" -> R
  "Y" -> P
  "Z" -> S

parse line = let rule = splitAt 1 line in bimap parseChar parseChar rule

partOne x = sum $ map (score . parse) x

partTwo x = sum $ map (rescore . parse) x

main = do
  handle <- openFile "inputs/day02.txt" ReadMode
  contents <- hGetContents handle
  let rules = lines contents
  print (partOne rules)
  print (partTwo rules)
  hClose handle