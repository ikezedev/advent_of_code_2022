module Main (main) where

import Lib
import Text.Read (readMaybe)

main :: IO ()
main = print . map readMaybeInt . words =<< readFile "../rust_solutions/input/2022/day1.txt"

readMaybeInt :: String -> Maybe Int
readMaybeInt = readMaybe
