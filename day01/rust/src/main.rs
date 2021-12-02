fn solve(depths: &[usize], window: usize) -> usize {
    depths.iter().zip(depths.iter().skip(window)).filter(|(x, y)| x < y).count()
}

fn main() {
    let depths: Vec<usize> = include_str!("../../input.txt").lines()
                            .map(|line|line.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    println!("{}", solve(&depths, 1)); // Part One
    println!("{}", solve(&depths, 3)); // Part Two 
}

