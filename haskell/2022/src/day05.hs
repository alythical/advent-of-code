import System.IO
import Data.Char (isDigit)
import Data.List (foldl')

data Instruction = Instruction
  { count :: Int,
    from :: Int,
    to :: Int
  }
  deriving (Show)

crateInit =
  [ ['N', 'Z'],
    ['D', 'C', 'M'],
    ['P']
  ]

-- i don't want to subject myself to unnecessary pain tyvm
actualCrateInit =
  [ ['T', 'R', 'D', 'H', 'Q', 'N', 'P', 'B'],
    ['V', 'T', 'J', 'B', 'G', 'W'],
    ['Q', 'M', 'V', 'S', 'D', 'H', 'R', 'N'],
    ['C', 'M', 'N', 'Z', 'P'],
    ['B', 'Z', 'D'],
    ['Z', 'W', 'C', 'V'],
    ['S', 'L', 'Q', 'V', 'C', 'N', 'Z', 'G'],
    ['V', 'N', 'D', 'M', 'J', 'G', 'L'],
    ['G', 'C', 'Z', 'F', 'M', 'P', 'T']
  ]

parse x = read x :: Int

parseInstruction instr = parseInstruction' $ parseInstruction'' instr

parseInstruction' nums = Instruction (head nums) (head (tail nums) - 1) (head (tail $ tail nums) - 1)

parseInstruction'' instr =
  let rest = dropWhile (not . isDigit) instr
      n = takeWhile isDigit rest
   in parse n : case dropWhile isDigit rest of
        "" -> []
        r -> parseInstruction'' r

craneMover crates instr =
  let moving = take (count instr) (crates !! from instr)
      src = drop (count instr) (crates !! from instr)
      dst = reverse moving ++ crates !! to instr
   in updateArray crates [] 0 (from instr) (to instr) src dst

enhancedCraneMover crates instr =
  let moving = take (count instr) (crates !! from instr)
      src = drop (count instr) (crates !! from instr)
      dst = moving ++ crates !! to instr
   in updateArray crates [] 0 (from instr) (to instr) src dst

updateArray crates arr idx from to src dst
  | idx >= length crates = arr
  | idx == from = updateArray crates (arr ++ [src]) (idx + 1) from to src dst
  | idx == to = updateArray crates (arr ++ [dst]) (idx + 1) from to src dst
  | otherwise = updateArray crates (arr ++ [crates !! idx]) (idx + 1) from to src dst

partOne instrs = map head $ foldl' craneMover actualCrateInit instrs

partTwo instrs = map head $ foldl' enhancedCraneMover actualCrateInit instrs

main = do
  handle <- openFile "inputs/day05.txt" ReadMode
  contents <- hGetContents handle
  let instrs = map parseInstruction (lines contents)
  print $ partOne instrs
  print $ partTwo instrs
  hClose handle