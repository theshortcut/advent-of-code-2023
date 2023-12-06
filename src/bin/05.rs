use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac {
    seeds: HashSet<u64>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let seeds: HashSet<u64> = parts
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();
        let seed_to_soil = RangeMap::new(parts.next().unwrap(), seeds.clone());
        let soil_to_fertilizer = RangeMap::new(
            parts.next().unwrap(),
            seed_to_soil.0.values().copied().collect(),
        );
        let fertilizer_to_water = RangeMap::new(
            parts.next().unwrap(),
            soil_to_fertilizer.0.values().copied().collect(),
        );
        let water_to_light = RangeMap::new(
            parts.next().unwrap(),
            fertilizer_to_water.0.values().copied().collect(),
        );
        let light_to_temperature = RangeMap::new(
            parts.next().unwrap(),
            water_to_light.0.values().copied().collect(),
        );
        let temperature_to_humidity = RangeMap::new(
            parts.next().unwrap(),
            light_to_temperature.0.values().copied().collect(),
        );
        let humidity_to_location = RangeMap::new(
            parts.next().unwrap(),
            temperature_to_humidity.0.values().copied().collect(),
        );
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

#[derive(Debug)]
struct RangeMap(HashMap<u64, u64>);

impl RangeMap {
    fn new(input: &str, sources: HashSet<u64>) -> Self {
        let mut map = HashMap::new();
        let ranges: Vec<(u64, u64, u64)> = input
            .lines()
            .skip(1)
            .filter_map(|l| {
                l.split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
            })
            .collect();
        sources.iter().for_each(|source| {
            if let Some((dest_start, source_start, _)) =
                ranges.iter().find(|&(_, source_start, length)| {
                    source >= source_start && source < &(source_start + length)
                })
            {
                let delta = source - source_start;
                map.insert(*source, dest_start + delta);
            } else {
                map.insert(*source, *source);
            }
        });
        Self(map)
    }

    fn get(&self, k: &u64) -> u64 {
        match self.0.get(k) {
            Some(v) => *v,
            _ => *k,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::new(input);
    almanac
        .seeds
        .iter()
        .map(|seed| {
            let soil = almanac.seed_to_soil.get(seed);
            let fertilizer = almanac.soil_to_fertilizer.get(&soil);
            let water = almanac.fertilizer_to_water.get(&fertilizer);
            let light = almanac.water_to_light.get(&water);
            let temperature = almanac.light_to_temperature.get(&light);
            let humidity = almanac.temperature_to_humidity.get(&temperature);
            almanac.humidity_to_location.get(&humidity)
        })
        .min()
}

#[derive(Debug)]
struct Almanac2 {
    seeds: Vec<Range<u64>>,
    seed_to_soil: RangeMap2,
    soil_to_fertilizer: RangeMap2,
    fertilizer_to_water: RangeMap2,
    water_to_light: RangeMap2,
    light_to_temperature: RangeMap2,
    temperature_to_humidity: RangeMap2,
    humidity_to_location: RangeMap2,
}

impl Almanac2 {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .tuples()
            .map(|(start, length)| start..(start + length))
            .collect();
        let seed_to_soil = RangeMap2::new(parts.next().unwrap());
        let soil_to_fertilizer = RangeMap2::new(parts.next().unwrap());
        let fertilizer_to_water = RangeMap2::new(parts.next().unwrap());
        let water_to_light = RangeMap2::new(parts.next().unwrap());
        let light_to_temperature = RangeMap2::new(parts.next().unwrap());
        let temperature_to_humidity = RangeMap2::new(parts.next().unwrap());
        let humidity_to_location = RangeMap2::new(parts.next().unwrap());
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

#[derive(Debug)]
struct RangeMap2(Vec<(Range<u64>, Range<u64>)>);

impl RangeMap2 {
    fn new(input: &str) -> Self {
        let ranges = input
            .lines()
            .skip(1)
            .filter_map(|l| {
                l.split(' ')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
            })
            .map(|(dest_start, source_start, length)| {
                (
                    dest_start..dest_start + length,
                    source_start..source_start + length,
                )
            })
            .collect();
        Self(ranges)
    }

    fn get_source(&self, v: &u64) -> u64 {
        if let Some((dest, source)) = self.0.iter().find(|(d, _)| d.contains(v)) {
            let delta = v - dest.start;
            source.start + delta
        } else {
            *v
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac2::new(input);
    ((0 as u64)..(200000000 as u64))
        .into_par_iter()
        .filter(|location| {
            let humidity = almanac.humidity_to_location.get_source(location);
            let temperature = almanac.temperature_to_humidity.get_source(&humidity);
            let light = almanac.light_to_temperature.get_source(&temperature);
            let water = almanac.water_to_light.get_source(&light);
            let fertilizer = almanac.fertilizer_to_water.get_source(&water);
            let soil = almanac.soil_to_fertilizer.get_source(&fertilizer);
            let seed = almanac.seed_to_soil.get_source(&soil);
            almanac.seeds.iter().any(|r| r.contains(&seed))
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
