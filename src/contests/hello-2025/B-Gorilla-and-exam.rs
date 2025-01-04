use std::{
    cmp::{self},
    collections::HashMap,
    io::{self, BufWriter, Read, Write},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    num: u64,
    count: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.count
            .cmp(&other.count)
            .then_with(|| self.num.cmp(&other.num))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Error reading line");

    let mut lines = input.lines();
    let t: usize = lines
        .next()
        .expect("error reading t")
        .trim()
        .parse()
        .expect("Error converting t");

    let mut out = BufWriter::new(io::stdout().lock());

    for _ in 0..t {
        let nums: Vec<i32> = lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        // get k
        let n = nums[0];
        let mut k: i32 = nums[1];

        let mut num_map: HashMap<u64, i32> = HashMap::with_capacity(n as usize);

        // add the num: with their count in map
        for num in lines
            .next()
            .expect("Error reading line")
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
        {
            num_map.entry(num).and_modify(|p| *p += 1).or_insert(1);
        }

        let mut vec: Vec<State> = Vec::with_capacity(num_map.len());
        for (num, count) in num_map.into_iter() {
            vec.push(State { num, count });
        }
        vec.sort(); // sort based on occurrences count

        let mut result = 0;
        let mut l: i32 = 0;
        let mut r = vec.len() as i32 - 1;

        // Iterate from start to end, to find how many flat nums can be replaced
        // such that, the removal of them becomes fast, to make the array empy.
        while l <= r {
            // Inner loop only runs once till the K becomes 0. Basically getting
            // the min occurrences items from array and keep consuming k till it
            // is possible, and move the l pointer, if some item is consumed.
            while k > 0 {
                let smallest = vec[l as usize];
                if k >= smallest.count {
                    // All occurences of this min repated number can be replaced
                    vec[l as usize].count = 0;
                    l += 1;
                } else {
                    // All occurences cannot be converted, so update this nums' count
                    // to be prevCount - conversions available.
                    vec[l as usize].count -= k;
                }
                k -= smallest.count;
            }
            r -= 1;
            result += 1;
        }

        writeln!(out, "{}", result).expect("Error writing");
    }

    out.flush().ok();
}
