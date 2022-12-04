const PUZZLE_INPUT : &str = include_str!("../input.txt");


fn main() {
    let assignment_pairs = PUZZLE_INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            let first_num_pair = first.split("-").map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
            let second_num_pair = second.split("-").map(|n| n.parse().unwrap()).collect::<Vec<i32>>();
            let pairs = ((first_num_pair[0], first_num_pair[1]), (second_num_pair[0], second_num_pair[1]));
            //println!("{:?}", pairs);
            pairs

        })
        .filter(|(f, s)| { 
            (s.0 >= f.0 && s.0 <= f.1) || (f.0 >= s.0 && f.0 <= s.1)
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();


    println!("Number of complete overlapping pairs: {}", assignment_pairs.len());
}
