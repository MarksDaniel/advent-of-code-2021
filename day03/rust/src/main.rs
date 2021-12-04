const LINE_SIZE: usize = 12;

fn solve1(diagnostics: &[i32]) -> i32 {
    let mut gamma_rate = 0; 
    let mut epsilon_rate = 0;
    let mut n = 0;
    while n < LINE_SIZE {
        let (zeroes, ones): (Vec<i32>, Vec<i32>) 
            = diagnostics.iter().partition(|&x| (x & 1 << n) > 0);
            if zeroes.len() > ones.len() {
                gamma_rate |= 1 << n;
            } else {
                epsilon_rate |= 1 << n;
            }
        n += 1;
    }
    return gamma_rate * epsilon_rate;
}

fn solve2(diagnostics: &[i32]) -> i32 {
    let oxygen_generator_rating = apply_bit_criteria(diagnostics, |zeroes, ones| zeroes >= ones);
    let co2_scrubber_rating = apply_bit_criteria(diagnostics, |zeroes, ones| zeroes < ones);
    return oxygen_generator_rating * co2_scrubber_rating;
}

fn apply_bit_criteria<F: Fn(usize, usize) -> bool>(diagnostics: &[i32], is_greater: F) -> i32 {
    let mut diagnostics = diagnostics.to_vec();
    for n in (0..LINE_SIZE).rev() {
        let (ones, zeroes): (Vec<i32>, Vec<i32>) = diagnostics.iter().partition(|&x| (x & 1 << n) > 0);
        if is_greater(zeroes.len(), ones.len()) {
            diagnostics = zeroes;
        } else {
            diagnostics = ones;
        }
        if let [life_support_rating] = *diagnostics {
            return life_support_rating;
        }
    }
    return -1;
}

fn main() {
    let diagnostics: Vec<i32> = include_str!("../../input.txt").lines()
                   .map(|line| i32::from_str_radix(line, 2).unwrap())
                   .collect();
    println!("{}", solve1(&diagnostics)); // Part One
    println!("{}", solve2(&diagnostics)); // Part Two
}