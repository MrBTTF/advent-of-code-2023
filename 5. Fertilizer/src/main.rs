use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    dest: u64,
    source: u64,
    length: u64,
}

impl Range {
    fn new(source: u64, length: u64) -> Self {
        Range {
            dest: 0,
            source,
            length,
        }
    }

    fn substract_range(self, r: Range) -> (Option<Range>, Vec<Self>) {
        let v_start = self.source;
        let v_end = self.source + self.length - 1;
        let range_start = r.source;
        let range_end: u64 = r.source + r.length - 1;
        let intersection = if range_start > v_end || range_end < v_start {
            None
        } else {
            Some((v_start.max(range_start), (v_end).min(range_end)))
        };

        let Some(intersection) = intersection else {
            return (None, vec![self]);
        };

        println!(
            "Range: ({},{}) ({},{}) {:#?}",
            v_start, v_end, range_start, range_end, intersection
        );
        let mut unmapped = vec![];

        if intersection.0 > v_start {
            unmapped.push(Range::new(v_start, intersection.0 - v_start));
        }
        if v_end > intersection.1 {
            unmapped.push(Range::new(intersection.1 + 1, v_end - intersection.1));
        }
        (
            Some(Range::new(
                r.dest + intersection.0 - range_start,
                intersection.1 - intersection.0 + 1,
            )),
            unmapped,
        )
    }
}

impl TryFrom<&[u64]> for Range {
    type Error = String;

    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            let err = format!("Can't convert to range, length: {}", value.len());
            Err(err)
        } else {
            Ok(Self {
                dest: value[0],
                source: value[1],
                length: value[2],
            })
        }
    }
}

fn convert(map: Vec<Range>, values: Vec<Range>) -> Vec<Range> {
    let mut result: HashSet<Range> = HashSet::new();

    let mut unmapped = values.clone();
    for range in map.iter() {
        let mut new_unmapped = vec![];
        for v in unmapped.iter() {
            let (mapped, u) = v.substract_range(*range);
            if let Some(mapped) = mapped {
                result.insert(mapped);
            }
            new_unmapped.extend(u.iter());
        }
        unmapped = new_unmapped.clone();
        new_unmapped.clear();
    }
    result.extend(unmapped);
    println!("Result:");
    println!("{:?}", result);

    Vec::from_iter(result)
}

fn parse_values(s: &str) -> Vec<Range> {
    s.split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| Range::new(chunk[0].parse().unwrap(), chunk[1].parse().unwrap()))
        .collect()
}

fn parse_range(s: &str) -> Range {
    let range = s
        .split_whitespace()
        .take(3)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    range.as_slice().try_into().unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    // let mut sum: u64 = 0;
    let lines: Vec<&str> = contents.lines().collect();

    let mut values: Vec<Range> = parse_values(lines[0]);

    println!("Seeds: {:?}", values);

    let mut it = lines.iter().skip(2);
    while let Some(_) = it.next() {
        let mut map: Vec<Range> = vec![];

        for range in it.by_ref() {
            if range.is_empty() {
                break;
            }
            if range.starts_with('#') {
                continue;
            }
            map.push(parse_range(range));
        }
        // map.sort_by_key(|a| a.source);

        // println!("Locations: {:?}", values);
        values = convert(map, values);
    }

    println!("Locations: {:?}", values);
    let mut min_location = std::u64::MAX;
    for v in values {
        println!("Locations: {:?}", v.source);
        if v.source < min_location {
            min_location = v.source;
        }
    }
    println!("Minimal location: {}", min_location);

    // println!("Sum of all of the part numbers: {sum}");
}

//53 56  61 69  81 94

//46 49   54 62   74 87

//45 55   78 80   82 85    90 98

#[cfg(test)]
mod test_main {

    use super::*;

    #[test]
    fn test_convert_seeds_to_soil() {
        let values = parse_values("seeds: 79 14 55 13");
        let map = "50 98 2
        52 50 48"
            .lines()
            .map(|line| parse_range(line))
            .collect();
        let result = convert(map, values);
        assert_eq!(
            result,
            vec![
                Range {
                    dest: 0,
                    source: 81,
                    length: 14
                },
                Range {
                    dest: 0,
                    source: 57,
                    length: 13
                }
            ]
        );
    }

    #[test]
    fn test_convert_soil_to_fertilizer() {
        let values = parse_values("seeds: 81 14 57 13");
        let map = "0 15 37
        37 52 2
        39 0 15"
            .lines()
            .map(|line| parse_range(line))
            .collect();
        let result = convert(map, values);
        assert_eq!(
            result,
            vec![
                Range {
                    dest: 0,
                    source: 81,
                    length: 14
                },
                Range {
                    dest: 0,
                    source: 57,
                    length: 13
                }
            ]
        );
    }
}
