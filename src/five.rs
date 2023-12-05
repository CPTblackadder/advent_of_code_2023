use std::{cmp::max, ops::Range};

use crate::helpers::get_contents;

struct RangeMap {
    // Store special ranges
    ranges: Vec<(u64, u64, u64)>,
}

impl RangeMap {
    fn create(lines: &[&str]) -> Self {
        let range = Vec::new();
        let mut me = Self { ranges: range };
        for line in lines {
            let mut l = line.split(" ");
            let dest = l.next().unwrap().parse::<u64>().unwrap();
            let source = l.next().unwrap().parse::<u64>().unwrap();
            let length = l.next().unwrap().parse::<u64>().unwrap();
            me.insert_range(source, dest, length);
        }
        // Ensure ranges sorted by source value
        me.ranges.sort_by(|x, y| x.0.cmp(&y.0));
        me
    }
    fn insert_range(&mut self, source_start: u64, dest_start: u64, length: u64) {
        // Check if there is an overlapping range already
        for (x, y, l) in &self.ranges {
            let (x, y, l) = (*x, *y, *l);
            if x < source_start + length && source_start < x + l {
                panic!("Source ranges overlap");
            }
            if y < dest_start + length && dest_start < y + l {
                panic!("Destination ranges overlap");
            }
        }
        self.ranges.push((source_start, dest_start, length));
    }

    fn get_value(&self, key: u64) -> u64 {
        for (source_start, dest_start, length) in &self.ranges {
            if source_start <= &key && key < source_start + length {
                return key - source_start + dest_start;
            }
        }
        key
    }

    // Given a range produce a list of ranges that this mapping would map values in the input range to
    fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut new_ranges = vec![];
        let mut last_end = range.start;
        // Ranges are sorted with respect to source value
        for (x, y, l) in &self.ranges {
            let (source, dest, length) = (*x, *y, *l);
            if source + length < range.start || range.end <= source {
                continue;
            }
            if last_end < source {
                assert!(last_end < source);
                new_ranges.push(last_end..source);
            }
            if range.end < source + length {
                assert!(range.end > source);
                assert!(dest < (dest + range.end) - source);
                let d = dest + ((max(source, last_end)) - source);
                new_ranges.push(d..((dest + range.end) - source));
                return new_ranges;
            } else {
                assert!(dest < dest + length);
                let d = dest + ((max(source, last_end)) - source);
                new_ranges.push(d..dest + length);
                last_end = source + length;
            }
        }
        if last_end < range.end {
            new_ranges.push(last_end..range.end);
        }
        new_ranges
    }
}

pub fn run_task() {
    let contents = get_contents("five".to_owned());
    let lines: Vec<&str> = contents.lines().collect();
    let seeds = lines[0]
        .split(&[':', ' '])
        .map(|x| x.trim().parse::<u64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap());
    let seed_to_soil = RangeMap::create(&lines[3..18]);
    let soil_to_fertilizer = RangeMap::create(&lines[21..38]);
    let fertilizer_to_water = RangeMap::create(&lines[41..81]);
    let water_to_light = RangeMap::create(&lines[83..99]);
    let light_to_temperature = RangeMap::create(&lines[101..140]);
    let temperature_to_humidity = RangeMap::create(&lines[143..180]);
    let humidity_to_location = RangeMap::create(&lines[184..218]);

    println!(
        "{:?}",
        seeds
            .map(|x| seed_to_soil.get_value(x))
            .map(|x| soil_to_fertilizer.get_value(x))
            .map(|x| fertilizer_to_water.get_value(x))
            .map(|x| water_to_light.get_value(x))
            .map(|x| light_to_temperature.get_value(x))
            .map(|x| temperature_to_humidity.get_value(x))
            .map(|x| humidity_to_location.get_value(x))
            .min()
            .unwrap()
    );

    let seed_ranges = lines[0]
        .split(&[':', ' '])
        .map(|x| x.trim().parse::<u64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .fold(
            (None, Vec::new()),
            |(part_range, mut ranges), y| match part_range {
                Some(n) => {
                    assert!(n < n + y);
                    ranges.push(n..n + y);
                    (None, ranges)
                }
                None => (Some(y), ranges),
            },
        );

    assert_eq!(seed_ranges.0, None);

    let best_seed: u64 = seed_ranges
        .1
        .into_iter()
        .map(|x| seed_to_soil.map_range(x))
        .flatten()
        .map(|x| soil_to_fertilizer.map_range(x))
        .flatten()
        .map(|x| fertilizer_to_water.map_range(x))
        .flatten()
        .map(|x| water_to_light.map_range(x))
        .flatten()
        .map(|x| light_to_temperature.map_range(x))
        .flatten()
        .map(|x| temperature_to_humidity.map_range(x))
        .flatten()
        .map(|x| humidity_to_location.map_range(x))
        .flatten()
        .fold(u64::MAX, |x, y| if y.start < x { y.start } else { x });
    println!("{:?}", best_seed);
}
