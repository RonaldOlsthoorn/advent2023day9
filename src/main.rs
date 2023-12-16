
use std::{io::{BufReader, BufRead}, fs::File, collections::VecDeque};


struct Sequence {
    data: VecDeque<i32>
}

impl Sequence {

    fn predict_next(&self) -> i32 {

        let mut derivatives: Vec<VecDeque<i32>> = vec![self.data.clone()];

        while !difference(&derivatives[derivatives.len() - 1]).iter().all(|e| *e==0) {
            let new_seq = difference(&derivatives[derivatives.len() - 1]);
            derivatives.push(new_seq);
        }

        for i in (1..derivatives.len()).rev() {
            let diff = derivatives[i - 1][derivatives[i - 1].len() - 1] + derivatives[i][derivatives[i].len() - 1];
            derivatives[i - 1].push_back(diff);
        }

        return derivatives[0][derivatives[0].len() - 1];        
    }

    fn predict_previous(&self) -> i32 {

        let mut derivatives: Vec<VecDeque<i32>> = vec![self.data.clone()];

        while !difference(&derivatives[derivatives.len() - 1]).iter().all(|e| *e==0) {
            let new_seq = difference(&derivatives[derivatives.len() - 1]);
            derivatives.push(new_seq);
        }

        for i in (1..derivatives.len()).rev() {
            let diff = derivatives[i - 1][0] - derivatives[i][0];
            derivatives[i - 1].push_front(diff);
        }

        return derivatives[0][0];        
    }
}

fn difference(sequence: &VecDeque<i32>) -> VecDeque<i32> {
    
    let mut res = VecDeque::new();

    for i in 0..(sequence.len() - 1) {
        let window: Vec<i32> = sequence.iter().skip(i).take(2).map(|v| *v).collect();
        res.push_back(window[1] - window[0]);
    }
    res

}


fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let sequences: Vec<Sequence> = lines.iter().map(|line| Sequence{data: line.split_whitespace().map(|s| s.parse().unwrap()).collect()}).collect();

    println!("Part 1 {}", sequences.iter().map(|s| s.predict_next()).sum::<i32>());
    println!("Part 2 {}", sequences.iter().map(|s| s.predict_previous()).sum::<i32>());

}