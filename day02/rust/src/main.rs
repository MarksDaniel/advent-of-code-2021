fn solve (contents: &'static str) -> (usize, usize, usize) {
    let commands = contents.lines();
    let res = commands.fold((0, 0, 0), |(h, d, a), line| {
        let mut split = line.split(' ');
        let (dir, amount) = (split.next().unwrap(), split.next().unwrap());
        match (dir, amount.parse::<usize>().unwrap()) {
        ("forward", v) => (h + v, d + a * v, a),
        ("down", v)    => (h, d, a + v),
        ("up", v)      => (h, d, a - v),
        _              => unreachable!(),
        }
    });
    return res;
}

fn main() {
    let contents = include_str!("../../input.txt");
    let (h, d, a) = solve(contents);
    println!("{}", h * a); // Part One 
    println!("{}", h * d); // Part Two 
}