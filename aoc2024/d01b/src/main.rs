const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    println!("{INPUT_STR}");

    let (mut line1, mut line2) = INPUT_STR
        .split('\n')
        .map(|l| l.split(' '))
        .map(|l| l.filter(|c| c != &"").collect::<Vec<&str>>())
        .filter(|a| !a.is_empty())
        .map(|l| (l[0].parse::<i64>().unwrap(), l[1].parse::<i64>().unwrap()))
        .fold((vec![], vec![]), |(mut acc1, mut acc2), (a, b)| {
            acc1.push(a);
            acc2.push(b);
            (acc1, acc2)
        });

    line1.sort();
    line2.sort();

    let total_diff = (line1)
        .iter()
        .map(|n| {
            line2
                .iter()
                .fold(0, |acc, m| if m == n { acc + m } else { acc })
        })
        .fold(0, |acc, n| acc + n);

    println!("{total_diff:?}");
}
