use std::cmp::min;
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
            // If the guard wakes up past 1am, only count the minutes up to 1am
            let end_time = min(entry.timestamp.time(), NaiveTime::from_hms(1, 0, 0));
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
            let freq = minute_freq.entry(n).or_insert(0);
            *freq += 1;

            if *freq > max_freq {
                max_freq = *freq;
                max_minute = Some(key);
            }
        }
    }

    max_minute
}

fn find_best_guard_and_minute(entries: &mut [Entry]) -> Option<(Guard, i64)> {
    entries.sort_unstable();

    if let Some((guard, entries)) = find_sleepiest_guard(&entries) {
        if let Some(minute) = find_highest_freq_minute(&entries) {
            return Some((guard, minute));
        }
    }
    None
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

    if let Some((guard, minute)) = find_best_guard_and_minute(&mut entries) {
        println!("{:?} {}", guard, minute);
        println!("{}", guard.0 as i64 * minute);
    } else {
        println!("Unable to find sleepiest guard");
    }

    Ok(())
}

#[cfg(test)]
mod test_find_sleepiest_guard {
    use super::*;

    #[test]
    fn test_provided_example() -> Result<(), Box<dyn Error>> {
        // NOTE: these entries are already sorted
        let mut entries: Vec<Entry> = vec![
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
        ];

        let result = find_best_guard_and_minute(&mut entries);
        let expected = Some((Guard(10), 24));

        assert_eq!(result, expected);

        Ok(())
    }
}
