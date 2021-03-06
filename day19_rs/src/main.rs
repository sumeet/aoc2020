use arcstr::core::fmt::Binary;
use arcstr::Substr;
use binary_heap_plus::BinaryHeap;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::cmp::Reverse;
use std::iter::once;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
enum Rule {
    Ref(usize),
    MatchChar(char),
    Or([Arc<Rule>; 2]),
    Series(Vec<Arc<Rule>>),
}

fn parse_rule(rulestring: &str) -> Arc<Rule> {
    let rule = if rulestring.contains("|") {
        let (left, right) = rulestring.split(" | ").collect_tuple().unwrap();
        Rule::Or([parse_rule(left), parse_rule(right)])
    } else if rulestring.contains(" ") {
        Rule::Series(rulestring.split(" ").map(parse_rule).collect())
    } else if rulestring.contains('"') {
        Rule::MatchChar(rulestring.chars().nth(1).unwrap()) // for "b", takes out b
    } else {
        Rule::Ref(rulestring.parse().unwrap())
    };
    Arc::new(rule)
}

#[derive(Debug, Clone)]
enum MatchResult {
    Remaining(Substr),
    RuleDidNotMatch,
    Branches((Substr, Substr)),
}

impl MatchResult {
    fn is_full_match(&self) -> bool {
        match self {
            MatchResult::Remaining(substr) => substr.is_empty(),
            MatchResult::RuleDidNotMatch => false,
            MatchResult::Branches(_) => unimplemented!(),
        }
    }
}

struct QItem {
    remaining: Substr,
    series_index: usize,
}

fn matches_rule(
    all_rules: Arc<Vec<Arc<Rule>>>,
    rule: Arc<Rule>,
    string_to_match: Substr,
) -> MatchResult {
    match &*rule {
        Rule::Ref(inner_rule_index) => matches_rule(
            Arc::clone(&all_rules),
            Arc::clone(&all_rules[*inner_rule_index]),
            string_to_match.substr(..),
        ),
        Rule::MatchChar(c) => {
            if string_to_match.chars().next() == Some(*c) {
                MatchResult::Remaining(string_to_match.substr(1..))
            } else {
                MatchResult::RuleDidNotMatch
            }
        }
        Rule::Or(inner_rules) => {
            // let start_time = SystemTime::now();
            // let timeout = Duration::from_millis(10);
            let matches = inner_rules
                .iter()
                .map(|inner_rule| {
                    matches_rule(
                        Arc::clone(&all_rules),
                        Arc::clone(&inner_rule),
                        string_to_match.substr(..),
                    )
                })
                .filter_map(|result| match result {
                    MatchResult::Remaining(remaining) => Some(remaining),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if matches.len() == 2 {
                MatchResult::Branches((matches[0].substr(..), matches[1].substr(..)))
            } else if matches.len() == 1 {
                MatchResult::Remaining(matches[0].substr(..))
            } else {
                MatchResult::RuleDidNotMatch
            }
        }
        Rule::Series(inner_rules) => {
            let mut maxheap: BinaryHeap<_, _> =
                BinaryHeap::new_by_key(|qitem: &QItem| qitem.series_index);
            maxheap.push(QItem {
                remaining: string_to_match,
                series_index: 0,
            });
            while let Some(qitem) = maxheap.pop() {
                if qitem.series_index == inner_rules.len() {
                    // TODO: come back here?
                    return MatchResult::Remaining(qitem.remaining);
                }

                let rule = &inner_rules[qitem.series_index];
                let inner_match = matches_rule(
                    Arc::clone(&all_rules),
                    Arc::clone(rule),
                    qitem.remaining.substr(..),
                );
                match inner_match {
                    MatchResult::Remaining(substr) => maxheap.push(QItem {
                        remaining: substr,
                        series_index: qitem.series_index + 1,
                    }),
                    MatchResult::Branches((substr_a, substr_b)) => {
                        for substr in once(substr_a).chain(once(substr_b)).into_iter() {
                            maxheap.push(QItem {
                                remaining: substr,
                                series_index: qitem.series_index + 1,
                            });
                        }
                    }
                    MatchResult::RuleDidNotMatch => (),
                }
            }
            MatchResult::RuleDidNotMatch
        }
    }
}

fn main() {
    let mut part = 1;
    let mut rule_lines = vec![];
    let mut messages = vec![];
    for line in INPUT.lines() {
        match part {
            1 => {
                if line.is_empty() {
                    part = 2;
                } else {
                    rule_lines.push(line);
                }
            }
            2 => messages.push(Substr::from(line)),
            _ => unimplemented!(),
        }
    }

    let mut all_rules = rule_lines
        .iter()
        .map(|line| line.split(": ").collect_tuple().unwrap())
        .sorted_by_key(|(key, _)| key.parse::<usize>().unwrap())
        .map(|(_, rule_string)| parse_rule(rule_string))
        .collect_vec();

    // part 2 modifications
    all_rules[8] = parse_rule("42 | 42 8");
    all_rules[11] = parse_rule("42 31 | 42 11 31");

    for (i, rule) in all_rules.iter().enumerate() {
        println!("rule{} = @{{ {} }}", i, to_peg(rule));
    }

    // let all_rules = Arc::new(all_rules);
    //
    // let rule_to_match = Arc::clone(&all_rules[0]);
    // let count = messages
    //     .into_par_iter()
    //     .filter(|message| {
    //         let result = matches_rule(
    //             Arc::clone(&all_rules),
    //             Arc::clone(&rule_to_match),
    //             message.substr(..),
    //         );
    //         result.is_full_match()
    //     })
    //     .count();
    // dbg!(count);
}

fn to_peg(rule: &Rule) -> String {
    match rule {
        Rule::Ref(rulenum) => format!("rule{}", rulenum),
        Rule::MatchChar(c) => format!("\"{}\"", c),
        Rule::Or([rule_a, rule_b]) => format!("({} | {})", to_peg(rule_a), to_peg(rule_b)),
        Rule::Series(rules) => {
            let series = rules
                .iter()
                .map(|inner_rule| to_peg(inner_rule))
                .join(" ~ ");
            format!("({})", series)
        }
    }
}

const INPUT: &'static str = r#"28: 106 129
105: 97 20 | 83 91
124: 10 20 | 35 91
122: 128 20 | 41 91
94: 77 91 | 124 20
114: 20 91 | 20 20
89: 20 20
82: 106 102
110: 109 91 | 60 20
1: 91 20
56: 91 121 | 20 61
8: 42
52: 102 91 | 129 20
115: 85 20 | 51 91
50: 91 107 | 20 2
49: 7 20 | 5 91
62: 106 106
118: 4 91 | 101 20
27: 91 129 | 20 60
68: 18 91 | 99 20
99: 60 91 | 89 20
33: 100 20 | 122 91
2: 106 60
75: 17 106
67: 93 91 | 65 20
96: 20 48 | 91 52
107: 62 20 | 44 91
102: 20 20 | 91 91
55: 35 20 | 72 91
38: 27 91 | 36 20
37: 20 113 | 91 67
109: 20 20 | 106 91
108: 20 87 | 91 71
5: 91 91
92: 126 91 | 95 20
64: 91 1 | 20 5
95: 79 91 | 85 20
78: 20 5
9: 37 91 | 53 20
31: 91 69 | 20 47
21: 89 20 | 62 91
42: 20 86 | 91 56
120: 23 20 | 5 91
93: 60 91 | 114 20
66: 91 1
43: 91 12 | 20 65
4: 91 50 | 20 115
129: 91 20 | 106 91
15: 120 91 | 14 20
23: 91 91 | 91 20
72: 20 23 | 91 44
127: 129 91 | 40 20
53: 105 20 | 68 91
69: 118 91 | 16 20
100: 91 111 | 20 2
123: 20 84 | 91 60
86: 104 20 | 9 91
106: 91 | 20
97: 91 7 | 20 102
35: 7 91 | 23 20
13: 109 91 | 129 20
65: 20 109
7: 20 91 | 91 20
0: 8 11
10: 91 44 | 20 84
117: 20 23
126: 20 123 | 91 19
88: 20 21 | 91 26
12: 91 62 | 20 129
48: 91 5 | 20 5
98: 20 81 | 91 45
14: 20 1 | 91 62
34: 91 17 | 20 13
40: 20 91
51: 62 20 | 114 91
128: 1 20 | 44 91
131: 125 20 | 98 91
85: 91 1 | 20 44
32: 91 59 | 20 119
63: 6 20 | 75 91
60: 91 20 | 20 106
73: 44 106
90: 20 129 | 91 23
6: 117 91 | 58 20
19: 5 20 | 109 91
58: 89 20 | 1 91
11: 42 31
45: 7 91 | 44 20
44: 91 106 | 20 20
70: 91 24 | 20 34
80: 91 39 | 20 76
91: "b"
36: 20 102 | 91 23
83: 1 20 | 60 91
103: 91 80 | 20 29
116: 38 20 | 74 91
25: 91 54 | 20 49
26: 109 91 | 89 20
39: 114 20 | 102 91
17: 20 1 | 91 89
30: 20 5 | 91 40
76: 20 109 | 91 23
130: 60 106
22: 91 55 | 20 88
112: 15 91 | 43 20
79: 5 20 | 1 91
41: 23 91
57: 112 91 | 94 20
119: 20 123 | 91 65
111: 7 91 | 89 20
54: 89 91 | 114 20
20: "a"
74: 91 78 | 20 48
101: 91 25 | 20 96
81: 7 91 | 60 20
84: 20 20 | 91 20
16: 103 91 | 22 20
71: 110 91 | 128 20
121: 32 20 | 116 91
125: 28 20 | 66 91
46: 5 91 | 5 20
24: 20 83 | 91 64
59: 91 83 | 20 46
77: 127 91 | 90 20
47: 20 3 | 91 57
104: 91 70 | 20 108
61: 33 91 | 131 20
3: 91 92 | 20 63
29: 82 91 | 30 20
18: 84 91 | 89 20
113: 82 20 | 130 91
87: 73 91 | 13 20

bbabaabaaabaabbbabbbbbbb
aababaaabaaaaaaabbabbaaaabbababbabbbabaa
abbbabbabbaababababaaaaa
abbaaaabaaabbbbababaaabbbababaaabaaaaababbaababb
abaabbaababababbbbbbababbbbbbaababaaaabbabbbbbbaabaaaaaaaabbbbbb
aabbaaabaaabaaaaaababaaa
aaabbabbaabbbbaababbbabaaabbbaabbaababbb
baaabaabababbbabbbababab
aaaaabaaaabababaabaaabaabbbaaaabbbabbaaaabbaabaabbbabaaababbbbba
bbaabbaababaaaabaabbabbb
bbaabaaabbaaabaaaaaabbaaaaababaa
aaaaababaaabbbabbabbbabbabbbabbababaaaaaabaaaaaa
bbaabbaaaabababaabbbbaaa
babbababbbbbbaabbababbbbbbaababaabaabbaaaabbaaaababaabba
abbbabbabaaabaaaaaabbaabbabaabaabbaaaaba
aaabababaabaaabababbabaaabbbbabb
baababbabaaababbbabbbaaaababbbabaaaaaaaa
babbaabaababbabbabbaabaa
aaabbaabbaabaaabaaaabaab
aaabaaaaaaaaabbbaaaabbba
aaaabbaabbababbabaabaaabaaaaaababaaaaaaa
aabbbbaaaaaaabaaabbaaabb
aaababbaabaababbabbaabbb
aabaabaaaaababbabaaaaabb
aabaabbaabaaabaaabbbbaab
aabbbabbbabbaabababaabaa
abaabababbabbaaabbabbbab
babbbababaabbaabababbbbbbabbaababbaababababaabbb
ababaaabbaaabaabababababaabbbabbaaababbbbbbabbab
aabaabaabbbaababaababbababbbabaabbabbbab
ababababaaaaaabaaaabbabbaabbbabbbaaabaabbbbabbbbbaaabbab
aabbaababababbbbabbbbbba
baabaababbabbbbbbaabbaba
ababbbbbaabbaabaabbaaabaababbabbbaaabbbbbbaaaabb
abaababbaabaabbaaabaabaababbabbbbbbaabaa
bbbabbabaabbbbbabaabaabb
aaaabbbbbaabaaabaaabbbbbbbabbaabaabbbabbabbbbaabbbbaabbb
bbabababaaaaabaaabbbaaab
babbabbabbbbbbababaabbab
abaabaaaaaabbbbbaaaaabbabbbaaaabbbbaabab
bbabaabaabaabaabababbabaababaaaaabbaababaabbabbabbabbabb
abababababbabaaaabbbaaab
bbababababaaabbaaaaababb
abbaaaaabbbabbbabbabbabaabbababbaaabbabbbbbaabaa
aaababaaaaaaababbaaaaabbaaaababbbbbbaabbbababaabaaaaaaaaaabababaaabaabbbbbbbaaababbbbabb
bbaababaaabbababbaabbbaa
aabbbbbabbbabbabbbaaabba
aaaaabbbaababababbbbabbb
abbbaaaabaaaaabbabababba
aabbaaabbaaaababbabaabab
aaabbaaababababbbabbbabbbbbaaaab
bbbabbbbbabbbabaabbbbbba
baaabbbabaaaabbbbababaababbbaababbaaabaaabababaababaabbb
ababbaaaababaaabbbbababa
abaababbbbbbbbababbabbaa
bbabababbbaababaabaaaaaa
babaaaababbbabbbbbaababbababbbaabbaababb
aabbabaaaababbbabbbbbabb
aaaabbababbabaaaabaaabbbbbaabbabababbaabaabababaaabaaaab
aabaabaabbbbbbababaababbbaabaabb
aabbaaabbbaabbabaabbaaaaabababba
aabbaaaaabaabaabbaaabbaa
aaabbabbaaaaaabababaabaa
aaababbaaababbbaaabbaabaabaaabab
abaaaabaaaaabbaaabbbaabb
aaabbaabbbabbaabaabaabab
bbbbaaabaaaaabbbbbbbaaaa
aaabbbaabbabaaaaababaaaa
bbbbbaaaaaabbbbabbaabaaabaaabaaababaaabbabbaababbabaabaaaaababaaabbabbaa
baabbaabbaabaaaaaababaaa
aaaabbababaabaaabbbbabaa
abbaaaabababbaababaaaaab
ababababbabbaababaabbbba
aaaaabbababbaababbabbaaa
abaabaaaabaabaaabbbbbabb
bbaaabaaabaabbbabbbbabbababaabaa
babaaaababaaabbaabbbbaba
aaababbabaaabbbababbbababbbabbabbbaabbba
bbbbbaaaaabbbbbababbbbab
bababbbaababaaabbaaaaababaaabaababababaaababbbaa
abaabbbabbaaababbabaabaa
baaaaabaaaaabbbbbabababa
aaaabbbaabbbaabbabbabbaabbaabaaa
aababbbbbbaabbaaabaaaaab
babababbbbabaaabbbbbaaaa
abaabbbababbbabbbaabbbabaaabaabbabaaaaab
aabbabaaaababbbaaaaaaaaa
abaaabbbbababaaaaabbabaaabbbaaaaabbaaaaa
babbbababbabbaababbbaaab
aabbbbbabbbbbabababbaabb
baabababbaaababbbbbaabba
ababbbbbabaaabbaabbaabab
bbaabbabbaabababbaaaabaa
aaabbbabbbbabbaaaaababbbbabbbaababbbbbbb
bababbababbbbbaabbabaaaaaabaabbbbaaabbbaaaabbbabaaaababbbbbaaabbbbabbaaa
aabbbabbbaabbaaaabbbabbabbaabbbbbaaabbbb
bbaabbabababaaabbababbaa
aaabbbbbabbaaabaaabbbaba
aaaabaaabbbabbabbbaabbabbbbbaabbabbbabab
abaabbbbbababbbbbbaaaabb
bbabbaabaabababababababbbbbbbabb
aabbababaabbbaaaabaaabbbabbbabbbbaabbababbabbabbbababaaa
babbabbbaaabbbaabbabbbbbabbaabaa
aaaaaabbaabbbabbbabbaaaa
baabaabaabbabababbabaaab
abbaaababbbbaabaaaababaa
babbaaaaabababaababbabbaaaabbabbaaabababbbbbbbaa
bbaaababaaaaaaababaabbab
abaaabaabaababbabbabbbab
baabbaaaaaababbbaababbbbabbbbabb
bbbbaababbbbabbaabaaabab
bbaaaabbbbabbbaaaabbbaaaabbaaaaaabbaabab
baabbaabbbabababaaabaaaaaaabaaabaaababaa
abababaaabaababbbbabaabb
baabbaabaabbaabaabbabbbbaaabbaaaababaaabbabbabbbbaababaababbbbabbbbaaabb
bbbabbababaabbaabaaabaabababaabb
abaabbaabbbbabbabaabbbbb
ababbaabbababbabaabaabbbabaaaababaaaaababbbbabaaaaabaabbbaaaaabb
bbabbbbbaaabaaaabbabbbbbaabbaababbbbbbbbbaabaabb
bbaabbbbbaaabbbaabbbbbba
aaabbbbabbbabbbabbbaaaaa
ababbaaabbbabbbaababaaba
ababbbabbbbaaaaaabbbbbba
baababbaabbababaabbbaaaa
babbbaaababbbababbbaabaa
ababbbabaabbabaaaabaabab
aabaabbbabaaabbbaaaababa
aaabbbbaaabaabbaabbbbaba
abbaabbabaaababaaababbbabbaabbbbbababbbbabbbbbab
aababbbababbbaaaababbabbbabaabbaabbbbabb
aabbbabbbaaabaabaababbbaabbababb
aaabbbbbabaaabbbbbabbaabaaababaaababbbbababbbbababbaabab
aaaabbbbbbbbbbaabbaaaabaaaabbbaaabbbabbbbbababbabaababaabaaabaabaaabbbbbaaabbaba
ababbaaaaaaabbabbbababaa
bbaaaaaabaabaabaaabbababbbabbabb
baababbababbaaaabbabaababababaab
abaaaabaaabbbbaaabbabaaabaababbababbabbbbabaabba
babbbabbaabaaabbabbbabab
babbbabbbaabaaabababbaba
bbbbababbbaaababbaaaababaaababaa
babaaabaaaaaabbbbabababbbbbbbabb
bbbbbaaaabbbaaaabbbabbbaabababaaabababbababaaaaaabbbbaab
bbaababaaaaaabbbababbabbaaabbabaababbbbabbaabbba
bbbbaabaabbaaababaaaaabb
bbaaaaabbaaabababaaaabaa
aababbbaababababbaaaaaaa
babaaaabaababababbbbbbaa
abaabaabaaababbbabbbaaba
aabaabbbbbaabbaababaabba
ababababbbaabababbabbbaa
baabaaabbabbaaaaaabbbbab
aabbaaabbbbabbbbbbbbababbbaabbabbbaaaaabbbabaaabbaabbbabaaaababbaabbabba
babbaabaabaaabbbbbabaaaaabaaaaaabbbaabab
aaaaababaabbbbbaaaababbabbaabbabaabbaaaaaaaabbbabaaaabbbababbbaaabbaabaa
aaaaabaaaaabbbaaabababbbbbabaabb
ababaaabbbabaababaaabbbaaabaabbaabbaaaabbaaabaabbbabaaab
bababaaabbababbabbababaa
aaaaabbbabaabbaabbbaabbb
babababbaaabaaaababaaaaa
aabbaababbbabbaaaabaabaaabbaaaababbabaab
abaaaabbbabaaabaabbbbbbb
baaababbbabbaabaabbbabab
baabbaabbaaababaabaabbbbaaabaaab
bbaabaaaaabaabaaabbbbaaa
babbbabbabaabaaabbabbbaa
aabaabaabaaabbbabaaaaabb
bbaaabbaabaaabbbbbaabbbabbaaaababaaaaabbabbabaabbababbbbbabbabbabaababab
abaaabbbbbaabbaaaaaaabaabbaabababbbaaaaa
aaaaabbbbabbaaaaabaabbaabaababbbaababbab
baaabbbaaabbaaabaabaabbbaabaabab
bbbabbabbabbabaabbbaabba
bbbbababbbaabaaababababbbababaaaabbbaaba
abbbabbabababbabababaabb
baabaaaabbaaabaababaabab
aaabaaaababbabbabaabaabb
babaaabaabbbabbabbbbbbababaaaabbaaaaabbaabbaaaaabbabbaaabbbbabaa
bbaabbabbaaababaabbbabaa
aaabaaaaaaaabaaabbbabbabbbaaaaba
abaaaababaaabaaaabbbbabb
aaababbbbabbbbbbabbababb
babababbaabbbaabbbbaabaa
ababbaaaaaaaabbbbabaaaaa
abbabbbbbaaaaaabbaababbb
bababbabbbbabbbaaaaaaabbbabbbaaabaabbaaaabaaaababbbbbabbabbbbabbabbbbbba
bababbabaaabbabaaaaaaaaa
ababbababbabbbaaaaaaabbbbbaaaabb
abbaabbaaabbababbbbbbaaababababbabbaabbaababbaba
bbbbbabaabbbabbabbbaabbb
ababababbaababbaaabbbaba
ababababaabbabababbaaabb
aaabbbaaaaabbabaabbaabaa
aaabbabbbbaaaaabaababbbbbaaaaabb
aabbbabbbaaaaaabababaaaa
ababbaabaaabababbbbababb
aababbbaaaaaaabaabbbbabb
babbabaaaaaabbaabaabaabb
aaaabaaaaabbabaaaabaaaab
aabbbaabaaaabaabababbbaabbbbbaaa
ababbbbbabaaabaaaabbabbaabbbabbb
aaaaaabaababababbabaabaa
babaabbbbbbbaabbabbabaab
abababbbbbabaababaabbbab
abbaaabaabaabaabbbabbaaa
aaabaababbabaababbbaabab
aabbbbaaaabbababababababaabbaaba
baaababaaabbaababaabbbaa
aababbbbbabbbbabbbbbbbbabbabaabababaaaaaaaaaaaabbabbaabbbbbbbabbabbabaaa
babababbbababaaabaabbbba
babbbaaabaaababababbbaab
bbaaabababaaaaaabbaaabbb
aabaabbaababbaababbaaabb
bbabaaaabbaabbaaaaaaaaaa
baaaaaabbbbabbabbbbbabaa
baaaababaaabbbabbabbbbbbbabbbaaaabaaaaab
babbbabbbaaabbbababbaaaaaaabababbbabaabbbbaaaaba
aabaabaabaaababbbabaabba
aaababababbabababaaaabba
baaabaaaaaaaababbaaaabbaabbaaaaaaaaabbaabaaaaaababbbbbababbbaaaabbbbbaaabbbaaaba
baababababaabbaaaabaabaabbaabaababbbabab
abaababbbabababbbaababbaaaabbabaababaaaaabaaabab
abaaabbbbbaaaaaaabbbbbba
bbaabababbaabbabbbaabbbb
abaaabaabababbbbabaaaaab
babbaababaaabbbbbbaaababaabababbaaaaabbbaaabaaabbabababbbbbbabbaabaababbaabaabbb
abbbbbaaabaaaabbbbaabbba
aaabbbbbaabaabbbaaaaababbbabaaabaabaababbbaaaababaabbbaababaabaa
bbabbaabbaaabbbaabbbbbbbabababbaabbaaaaaaababbbbbaabbaaabaaaabbbbbaaabbabbaabaabaaaababa
bbbbababbbbbababbaaaaaababaaabaabbaaabbb
aaabbabbaaaaabbabbbaaabb
aabbbbbaaaaabbbbbabbabbb
aabaaababbbabbabbbbbaaababbbbabb
ababbbbbaabbbbbbaaabbbabaabbbbaaabaaabaababaaabaaaabbbabbabbbaaabbbababb
aaaaabbaabaabaabbabaaabaaabaaabababbbbba
baabaaababbaabbabbabaaab
babbaabaaabaaababbbabbbaaaaabaaababaaababaaaabbbbaaabbaabbbaaababbabaaab
aaabbababbabaaaaaabaaabbaaababaabbbaaaaa
aabbabaaaabababaaabaabab
aaabbbbaaabababaababbbaa
babbaababbabaaaaaabbbbbb
abaabbbaabbabbbbaabbbbbabaababbb
bbaaabaaabbaabbabbbbaaaa
ababbbbbbaabaababaababaa
bbbbbbabbbbbababbaaaaaaa
baabababaabaaababbbabaaa
babababbbabbbaaaaabbbaba
bbbbbabaabaaabbabaaabbbaabbbaaaa
abbababaabbbabbababbbbab
babaaabababbaaababbbbbabbabaabbbaabbaabbbbababaaababbabb
abbaaabababbbabbbbaababb
bbabaababbbbabbabaabaaaaabbaabbabaabaaaababbbbabaabbbaaabaaabbbb
baabbaaaabababbbbabbbaaababbbaab
aabbbabbbbaaabababbabbbbaababbaabaaabbab
abbaaaabaabbaaaaaabaabaaabaabaaaabbaabbbabbbaaba
aaababbaaaaaababaababbab
baaababaaaabbbaabaabbbba
aabbbbbabbaaaaaaaaabaaaaabbabbabbaabbaba
ababababababbaabbabaaaaa
aabbbbaababbaaaababbbbbaabbabbabbbaababbabbbbbaababbabaabaabaaaababbbbba
bbabbbbbabbbbbaabbabaaab
bbaaabbbbbbbabaabbbbabaa
abaaababababbabaaababaaaabbbabaa
abbbabbaaaababbaababaabb
baabbaabbabbaababbabaabb
aabbbbaaaaaaabbabbabbbab
aaabbbbbabaaaabbbbabababbbbbaaaa
bbbbaaabbababbbabbbabbbbbbabababbbbbabbabbaabaabbbbbabaababaabba
bbaaaaaaabbabaaaabaaababbbabbbaaaaabbbbaaaabaaaababbabbabbbabaaaaabaaaabbaaabaaa
babbabbabbabbaabaaabaaaabaaaabaa
baaabaaaaaaabbabbabaabab
ababababaaabbbbabbbabaaa
baaaababaaaabbabbbbaaabb
bbbbbabaababbabbabaababaabbbaabb
aabbaaabbaaaababababbbaa
babaaabababbaababbabbbbbababbaaaabbabbbbaaaabaab
bbbbbbababaabbaabbbbabababbbbbbb
abababbbbbbbaaabaaaabaab
bbbbbbababaabbbabbbbbbba
babbabaababbbaaabaabbabb
aaabbaaaaabaabaabbababbb
bbaababaabaababbabbbbbba
aaaaaabbbbbbabbaaaaabbbaaababaab
aaabbbababababaaaabbaaaabbbaaaaaababbbbaabbbbaaaabbabaabaababbaa
bbabbbbbbaabaaabbbbabaab
ababbabbabbaaababaabbbbb
ababababbaabaaabbababbaa
baabaababbbabbaabaaaaaaa
baaaababaabbbbbababababa
aabbbbaaabbaaababbbbaabb
bbababaaabaababbbbbbbabaaabbaaabbaaabbabababbbbabaabbbbbbabaabbb
bbbbababbaaabbbaaabbaaaaaaaaababbbabaaababbbaaba
aaaaaabbaaaaababaaabaabb
bbbabbbbabbaabbababbbaaabbbaabaa
babbbbbbbabbabbaaabbbbbaaaabbabbbaabaaabbbaababb
ababbabbaaababbababbabbabaaaababababaaabbaabbbbb
abbbaabaabaaaaabbbabbbaabaabbabbbbaaaaaaabababab
abababbbaaaabbabaaaaabaabaabbaba
baaabababaaaabbabbaaababbbbababbaabbbababaaabbbaaabaaabbbbabaaaaabbbaaba
ababbbabaabbbbbaaaababaa
abbababbabbabbabaaabaabbabbabbaaaabbbbab
aaaabaaabbababababbabbab
aaabbbbababbababbaaaaabb
abababaaabbabbbbabbbbaab
babaaabaaababbbbabbbaaab
aaaaababbabbbabbabbaabbabaaaabbb
babababbaaaaaabbbbabbaba
abaaabbaabaabbbbbabababa
aaabbbbabaaabbbaababababbaabbabbbabbaabb
aaaababbbaaaaaaaababbbabaaaaaaabaaaaaabbaabbabbbbabababa
abaabbaabababbbaabaaaaababababaabbbbaabaabababaa
aabbbabbbbaaaaaababbbabbbaaaabbb
bbbbaabaaabbaababbbabbbabbbaaaba
aabbaabaaabbaaabbabbaaab
aabababababaaabbabaaabaaaaaaaababbabbbbbbbbababababbaabbbaabbbbb
babbabaaaaabbbbababbaabaaaabbbbabbaababaaaaabaabaababaaaabbbbbab
abbaabbaaabbababbbbaaabb
abaaabbbbabaaabababaabba
bbbbababaaabaaaaaabbbabbbbbaabab
bbaabbaabbabbbbbabbabbab
babaaaaaabaabaaaaaababbaaaabbabbbaababbbaaabbabbaaabbabbaaaabaabbaabbbababbaabaaababbaab
aaaaabbaaabbaaaaaababbaa
abaaaabababbbbaaaaaabbbabaaabbaa
babaaabaababaaabbabbaaab
baababbaababbaabbbaababaabaaaababaabbabbaababbaa
bbbabbaaaaababbaabababba
aabbababbbaabbbbaabababababaabab
babbabaaabbaaaababaaaabbbabbabaabbbbaaabbbabbbbaaabbbbbb
aaaabbabaaabbbabaabaabbaaaabbabbbabbaabaabbabbabbaaabbbbabaabbab
bbabaaaaaabbbaababaaabab
abaaabbbabbabbbbbaababbb
bbaaaaabbbaaabaabbbaaaaa
abaabaabbaaabbbabaabbbab
aabbabaababbaababbaaaaabababbababbbaabaa
aabaabbbaaabbbabaabbabba
aabbaaabaabaaabaaabbbaba
baabbaaaaaabbabbbabaaaaa
babbababbaaaabbaaabbabbbabbbbbbbababaaaa
bbaabababaaababbabbabbab
baaaababaaabbaabaaabbaabbaabbaaaaabaabaabbaaaabb
aabbbbbabaaabaaabbbaabba
baaababbaabaaabbaababaaa
aabbbbabbbbbaaabbabbbabbbbbabbababaabaababbabbbbbaabaaaaabbbabbababbbababababbba
abbabbbbaaaabaaababababaaabbbbababbabbaaabaabbab
babbbababbbabbbaaabbaaabaabaaabababaababbbabbbaaababaaaa
abaabaaabbbbbabbabaabababaabbababbbabaaa
baaaaabaaaaabbabaaaabbaabbbaabaaabbaaaaa
ababbaaabbaaababaabbaababbababaabbaabbba
aabbaaaababaaabbbbaababababbbabbaaababbaaaaaaaaa
aaababbbbbbbaababaaaaababababbabbaabbbaa
bbbbbaaaaaabbbbbaaabbbaabbaabbababbabbaaaaaababb
baabababbbabbbbbbabaaabbaaaaaaaababaabab
bbabbbbaaabbabaaaabbbaaabbbbabbaabaaaaaaaababbbaaabbbababaaabbbb
bbbabbbabbbabbaaaababaaa
abaaabaaabbbabbabbbbbabb
ababaaabbaaaaaaaaabbaabbababaabbabababaaaabbbaababbabbbbbababbabbabaaaaa
ababbaabbabbabaaabbbbabb
abababaabbaabbaabaaaaaabaabbabba
bbbbbaaaaabbaaabbaababbb
baaaaaabaaabbaaabaaabaaaaaaababa
babbbaaaababbaaaaaaabaab
abbaabbaabbabbabbbbabbababaabaababbaabbaabaabbabbabbaababbabbabaaababaab
aaabbabaaaababbaabbbbaab
baabbaaaaabbbbbabaaaaaaa
aaababbbaababababbaaabaabbaaaaaabbbbbababbabbbabaabbbaba
abbbabbaabaaabbabbaaabba
bbbaaaaabaaabababaaabbaabbbbaababaaabbaaabbaaaaaabbaabbaaaabbabaabaaaaaaabaaaabb
bbbabbbbaabaabbaabbbaabb
abaabbbbaaaaabaabbbaabba
abbabaaababbbabbbbbbbabb
baabbaaaaabaabbbbabbbabbbbaaabab
aaabbababaabaaaabbbbaaabbbbbbaaabaaaabba
baaabaabbababbbababaaabbbabaabbaaababaab"#;
