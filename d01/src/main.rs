fn main() {
    // * part 1
    let input = include_str!("./input.txt");

    let lines = input.split("\n\n");

    let mut lines_parse: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    // let answer_1 = lines_parse.iter().max().unwrap();

    lines_parse.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", lines_parse[0]);

    // * part 2
    println!("Part 2: {:?}", lines_parse[0] + lines_parse[1] + lines_parse[2]);
    
    println!("Part 2 (iterator): {:?}", lines_parse.iter().take(3).fold(0, |acc, x| acc + x));
    

}