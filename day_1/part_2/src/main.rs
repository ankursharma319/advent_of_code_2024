use std::collections::HashMap;
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

fn solve<R: BufRead>(reader: R) -> u32 {
    let mut vec_a: Vec<u32> = Vec::new();
    let mut freq_b: HashMap<u32, u32> = HashMap::new();
    for l in reader.lines() {
        let l = l.unwrap();
        let mut words = l.split_whitespace();
        vec_a.push(u32::from_str(words.next().unwrap()).unwrap());
        let b = u32::from_str(words.next().unwrap()).unwrap();
        *freq_b.entry(b).or_default() += 1;
    }
    vec_a
        .iter()
        .map(|x: &u32| {
            return match freq_b.get(x) {
                Some(f) => x*f,
                None => 0,
            };
        })
        .sum()
}
