use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};

mod entry;

use chrono::{NaiveTime, Timelike};
use entry::{Entry, Event, Guard};

fn find_sleepiest_guard(entries: &[Entry]) -> Option<(Guard, Vec<(i64, NaiveTime)>)> {
    let mut current_guard: Option<&Guard> = None;
    let mut guards: HashMap<&Guard, Vec<(i64, NaiveTime)>> = HashMap::new();
    let mut asleep_at = None;

    for entry in entries {
        if let Event::BeginsShift(guard) = &entry.event {
            current_guard = Some(guard);
            asleep_at = None;
        } else if Event::FallsAsleep == entry.event {
            asleep_at = Some(entry.timestamp.time());
        } else if Event::WakesUp == entry.event {
            let duration = guards.entry(current_guard.unwrap()).or_insert(vec![]);

            let target_time = asleep_at.unwrap();
            let end_time = entry.timestamp.time();
            let delta = end_time - asleep_at.unwrap();

            // Store number of minutes + starting time
            duration.push((delta.num_minutes(), target_time));
            asleep_at = None;
        }
    }

    let mut max_minutes = 0;
    let mut max_guard = None;

    // find the sleepiest guard
    for (guard, entries) in guards.iter() {
        let total = entries.iter().map(|(x, _)| x).sum();
        if total >= max_minutes {
            max_minutes = total;
            max_guard = Some(*guard);
        }
    }

    if let Some(guard) = max_guard {
        Some((*guard, guards.remove(guard).unwrap()))
    } else {
        None
    }
}

fn find_highest_freq_minute(entries: &[(i64, NaiveTime)]) -> Option<i64> {
    let mut minute_freq: HashMap<i64, i64> = HashMap::new();
    let mut max_minute: Option<i64> = None;
    let mut max_freq: i64 = 0;

    for (minutes, start_time) in entries {
        for n in 0..*minutes {
            let key = n + start_time.minute() as i64;
            let freq = minute_freq.entry(key).or_insert(0);
            *freq += 1;

            if *freq > max_freq {
                max_freq = *freq;
                max_minute = Some(key);
            }
        }
    }

    max_minute
}

fn strategy_1(entries: &[Entry]) -> Option<(Guard, i64)> {
    if let Some((guard, entries)) = find_sleepiest_guard(&entries) {
        if let Some(minute) = find_highest_freq_minute(&entries) {
            return Some((guard, minute));
        }
    }
    None
}

fn strategy_2(entries: &[Entry]) -> Option<(Guard, i64)> {
    let mut minute_freq: HashMap<(Guard, i64), i32> = HashMap::new();
    let mut current_guard = None;
    let mut asleep_at = None;
    let mut max_freq = 0;
    let mut max_entry = None;

    for entry in entries {
        if let Event::BeginsShift(guard) = entry.event {
            current_guard = Some(guard);
            asleep_at = None;
        } else if Event::FallsAsleep == entry.event {
            asleep_at = Some(entry.timestamp);
        } else if Event::WakesUp == entry.event {
            let duration = entry.timestamp - asleep_at.unwrap();
            for n in 0..duration.num_minutes() {
                let key = asleep_at.unwrap().minute() as i64 + n;

                let freq = minute_freq
                    .entry((current_guard.unwrap(), key))
                    .or_insert(0);
                *freq += 1;

                if *freq > max_freq {
                    max_freq = *freq;
                    max_entry = Some((current_guard.unwrap(), key));
                }
            }
        }
    }

    max_entry
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;

    let contents = contents;
    let mut entries = vec![];

    for line in contents.lines() {
        let entry: Entry = line.parse()?;
        entries.push(entry);
    }

    entries.sort_unstable();

    if let Some((guard, minute)) = strategy_1(&entries) {
        println!("Strategy 1: {}", guard.0 as i64 * minute);
    } else {
        println!("Unable to find result for strategy 1");
    }

    if let Some((guard, minute)) = strategy_2(&entries) {
        println!("Strategy 2: {}", guard.0 as i64 * minute);
    } else {
        println!("Unable to find result for strategy 2");
    }

    Ok(())
}

#[cfg(test)]
mod test_find_highest_freq_minute {
    use super::*;

    #[test]
    fn test_empty() {
        let result = find_highest_freq_minute(&mut []);
        assert_eq!(result, None);
    }

    #[test]
    fn test_correct_output() {
        assert_eq!(NaiveTime::from_hms(0, 5, 0).minute(), 5);

        let entries = vec![
            (10, NaiveTime::from_hms(0, 5, 0)),
            (5, NaiveTime::from_hms(0, 0, 0)),
            (4, NaiveTime::from_hms(0, 6, 0)),
        ];

        let result = find_highest_freq_minute(&entries);

        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_take_first_in_ties() {
        let entries = vec![
            (10, NaiveTime::from_hms(0, 0, 0)),
            (5, NaiveTime::from_hms(0, 10, 0)),
            (4, NaiveTime::from_hms(0, 15, 0)),
        ];

        let result = find_highest_freq_minute(&entries);

        assert_eq!(result, Some(0));
    }
}

#[cfg(test)]
mod test_strategies {
    use super::*;
    use entry::EntryError;

    fn get_example() -> Result<Vec<Entry>, EntryError> {
        // NOTE: these entries are already sorted
        Ok(vec![
            "[1518-11-01 00:00] Guard #10 begins shift".parse()?,
            "[1518-11-01 00:05] falls asleep".parse()?,
            "[1518-11-01 00:25] wakes up".parse()?,
            "[1518-11-01 00:30] falls asleep".parse()?,
            "[1518-11-01 00:55] wakes up".parse()?,
            "[1518-11-01 23:58] Guard #99 begins shift".parse()?,
            "[1518-11-02 00:40] falls asleep".parse()?,
            "[1518-11-02 00:50] wakes up".parse()?,
            "[1518-11-03 00:05] Guard #10 begins shift".parse()?,
            "[1518-11-03 00:24] falls asleep".parse()?,
            "[1518-11-03 00:29] wakes up".parse()?,
            "[1518-11-04 00:02] Guard #99 begins shift".parse()?,
            "[1518-11-04 00:36] falls asleep".parse()?,
            "[1518-11-04 00:46] wakes up".parse()?,
            "[1518-11-05 00:03] Guard #99 begins shift".parse()?,
            "[1518-11-05 00:45] falls asleep".parse()?,
            "[1518-11-05 00:55] wakes up".parse()?,
        ])
    }

    #[test]
    fn test_strategy_1() -> Result<(), Box<dyn Error>> {
        let entries = get_example()?;

        let result = strategy_1(&entries);
        let expected = Some((Guard(10), 24));

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_strategy_2() -> Result<(), Box<dyn Error>> {
        let entries = get_example()?;

        let result = strategy_2(&entries);
        let expected = Some((Guard(99), 45));

        assert_eq!(result, expected);

        Ok(())
    }
}
