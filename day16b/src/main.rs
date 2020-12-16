use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::RangeInclusive;

type Ranges = [RangeInclusive<usize>; 2];

fn main() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    let mut section = 1;
    let mut ranges_by_field: HashMap<&str, Ranges> = HashMap::new();
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        if line.trim() == "your ticket:" {
            section = 2;
            continue;
        }
        if line.trim() == "nearby tickets:" {
            section = 3;
            continue;
        }
        if line.trim() == "" {
            continue;
        }

        match section {
            1 => {
                let (field_name, ranges) = line.trim().split(": ").collect_tuple().unwrap();
                let (range1, range2) = ranges
                    .split(" or ")
                    .map(|range_str| {
                        let (lo, hi) = range_str.split("-").collect_tuple().unwrap();
                        RangeInclusive::new(lo.parse().unwrap(), hi.parse().unwrap())
                    })
                    .collect_tuple()
                    .unwrap();
                ranges_by_field.insert(field_name, [range1, range2]);
            }
            2 => {}
            3 => nearby_tickets.push(line.split(",").map(|s| s.parse().unwrap()).collect()),
            _ => unimplemented!(),
        }
    }

    let all_ranges = || ranges_by_field.values().flatten();
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|all_values_in_ticket| {
            all_values_in_ticket
                .iter()
                .all(|value| all_ranges().any(|range| range.contains(value)))
        })
        .collect();

    // .filter(|value| all_ranges().all(|r| !r.contains(value)))
    // .sum();
    println!("{:?}", valid_tickets);
}
