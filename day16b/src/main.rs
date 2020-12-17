use itertools::Itertools;
use std::collections::HashMap;
use std::ops::RangeInclusive;

type Ranges = [RangeInclusive<usize>; 2];

fn main() {
    let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    let mut section = 1;
    let mut ranges_by_field: HashMap<&str, Ranges> = HashMap::new();
    let mut our_ticket: Vec<usize> = vec![];
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
            2 => our_ticket = line.split(",").map(|s| s.parse().unwrap()).collect(),
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

    let ticket_length = our_ticket.len();
    let valid_tickets = &valid_tickets;

    let hm = ranges_by_field
        .iter()
        .map(move |(field_name, [range_a, range_b])| {
            (
                field_name,
                (0..ticket_length)
                    .map(move |field_index| {
                        valid_tickets
                            .iter()
                            .enumerate()
                            .filter_map(move |(ticket_id, ticket)| {
                                let field_value = ticket[field_index];
                                if range_a.contains(&field_value) || range_b.contains(&field_value)
                                {
                                    Some(ticket_id)
                                } else {
                                    None
                                }
                            })
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    let hm = hm
        .iter()
        .map(|(field_name, fields)| {
            (
                field_name,
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(field_id, matching_ticket_ids)| {
                        if matching_ticket_ids.len() == ticket_length {
                            Some(field_id)
                        } else {
                            None
                        }
                    })
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();
    println!("{:?}", hm);
}
