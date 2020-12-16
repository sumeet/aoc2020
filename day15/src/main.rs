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

    let target = 2020;
    for insertion_index in where_to_insert..target - 1 {
        let next_num = match map[prev] {
            None => 0,
            Some(prev_use) => insertion_index - prev_use as usize,
        };
        map[prev] = Some(insertion_index as u32);
        prev = next_num;
    }
    //     let next_num = match map[prev as usize] {
    //         None => 0,
    //         Some(prev_index) => {
    //             println!("there was a prev index for {}: {}", prev, prev_index);
    //             current_index as u32 - prev_index
    //         }
    //     };
    //     println!("next num: {}", next_num);
    //     prev = next_num;
    //     map[prev as usize] = Some(current_index as u32 - 1);
    // }
    println!("last one was: {}", prev);
}
