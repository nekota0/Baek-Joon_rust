use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no data");

    let mut iter = input.trim().split_whitespace();
    let length: usize = iter.next().expect("no data").parse().expect("no data");
    let counts: usize = iter.next().expect("no data").parse().expect("no data");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("no data");

    let numbers: Vec<i32> = input2.trim()
        .split(',')
        .map(|s| s.trim().parse().expect("cant change"))
        .collect();
    
    assert_eq!(length, numbers.len());
    let mut vec: Vec<i32> = numbers;

    for _i in 0..counts {
        for j in 0..(vec.len()-1) {
            vec[j] = vec[j + 1] - vec[j];
        }
        vec.pop();
    }

    let stringified: Vec<String> = vec.iter().map(|&x| x.to_string()).collect();
    let solution = stringified.join(",");
    println!("{}", solution);
}