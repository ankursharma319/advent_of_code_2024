use std::io::BufRead;
use std::str::FromStr;

fn main() -> () {
    println!("Hello, world!");

    let example = "3   4
4   3
2   5
1   3
3   9
3   3
";
    let example = std::io::BufReader::new(std::io::Cursor::new(example));
    println!("Examples answer is {}", solve(example));

    let file = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap());
    // println!("{:#?}", content);
    let answer = solve(file);
    println!("Problems answer is {}", answer);
}

fn solve<R: BufRead>(reader: R) -> i32 {
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();
    for l in reader.lines() {
        let l = l.unwrap();
        let mut words = l.split_whitespace();
        vec_a.push(i32::from_str(words.next().unwrap()).unwrap());
        vec_b.push(i32::from_str(words.next().unwrap()).unwrap());
    }
    vec_a.sort();
    vec_b.sort();
    return vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(x, y):(&i32, &i32)| {
            return i32::abs(x-y);
        })
        .sum();
}
