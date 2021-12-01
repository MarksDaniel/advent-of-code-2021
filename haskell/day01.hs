main :: IO () 
main = do  
    f <- readFile "../input.txt" 
    let depths = map read $ lines f :: [Int]  
    print $ solve depths 1 -- Part One 
    print $ solve depths 3 -- Part Two 
    
solve :: Ord a => [a] -> Int -> Int
solve depths window = sum $ fromEnum <$> zipWith (<) depths (drop window depths)
