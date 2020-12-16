use std::collections::HashMap;

fn main() {
    let input = "2,0,1,9,5,19";
    let mut map : Vec<Option<u32>> = vec![None; 30000000];


    let parsed = input.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (tail, head) = parsed.split_last().unwrap();

    let mut prev = *tail;
    let mut where_to_insert = 0;

    for (i, num) in head.iter().enumerate() {
        map[*num] = Some(i as u32);
        where_to_insert = 1 + i;
    }

    let target = 30000000;
    for insertion_index in where_to_insert..target - 1 {
        let next_num = match map[prev] {
            None => 0,
            Some(prev_use) => insertion_index - prev_use as usize,
        };
        map[prev] = Some(insertion_index as u32);
        prev = next_num;
    }
    println!("last one was: {}", prev);
}
