use chrono::NaiveDateTime;
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Guard(i32);

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    BeginsShift(Guard),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq)]
pub struct Entry {
    pub timestamp: NaiveDateTime,
    pub event: Event,
}

#[derive(Debug, PartialEq)]
pub enum EntryError {
    Chrono(chrono::ParseError),
    Missing(String),
    InvalidEvent(String),
}

impl EntryError {
    fn is_missing(message: &str) -> Self {
        EntryError::Missing(String::from(message))
    }
}

impl From<chrono::ParseError> for EntryError {
    fn from(error: chrono::ParseError) -> Self {
        EntryError::Chrono(error)
    }
}

impl std::fmt::Display for EntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EntryError::Chrono(e) => write!(f, "Chrono Error: {}", e),
            EntryError::Missing(m) => write!(f, "Missing Error: {}", m),
            EntryError::InvalidEvent(m) => write!(f, "Invalid Event: {}", m),
        }
    }
}

impl Error for EntryError {}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.timestamp == other.timestamp && self.event == other.event
    }
}

impl std::str::FromStr for Entry {
    type Err = EntryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split("]");

        let timestamp = tokens.next().ok_or(EntryError::is_missing("timestamp"))?;
        let message = tokens
            .next()
            .ok_or(EntryError::is_missing("message"))?
            .trim();

        // note the extra [ in front of the format string
        let timestamp = NaiveDateTime::parse_from_str(&timestamp, "[%Y-%m-%d %H:%M")?;

        let event = if message == "wakes up" {
            Event::WakesUp
        } else if message == "falls asleep" {
            Event::FallsAsleep
        } else if message.ends_with("begins shift") {
            let message = message.replace(" begins shift", "");
            let mut tokens = message.split("#");
            tokens.next();
            let guard = tokens.next().ok_or(EntryError::InvalidEvent(String::from(
                "Invalid shift message",
            )))?;

            let guard: i32 = match guard.parse() {
                Err(_) => {
                    return Err(EntryError::InvalidEvent(format!(
                        "Invalid Guard number: {}",
                        guard
                    )));
                }
                Ok(v) => v,
            };

            Event::BeginsShift(Guard(guard))
        } else {
            return Err(EntryError::InvalidEvent(String::from(message)));
        };

        Ok(Entry { timestamp, event })
    }
}

#[cfg(test)]
mod test_entry {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_ordering() -> Result<(), EntryError> {
        let mut events: Vec<Entry> = vec![
            "[1518-11-03 00:24] falls asleep".parse()?,
            "[1518-11-02 00:50] wakes up".parse()?,
            "[1518-11-02 00:40] falls asleep".parse()?,
            "[1518-11-04 00:02] Guard #99 begins shift".parse()?,
            "[1518-11-03 00:05] Guard #10 begins shift".parse()?,
        ];

        events.sort_unstable();

        let expected: Vec<Entry> = vec![
            "[1518-11-02 00:40] falls asleep".parse()?,
            "[1518-11-02 00:50] wakes up".parse()?,
            "[1518-11-03 00:05] Guard #10 begins shift".parse()?,
            "[1518-11-03 00:24] falls asleep".parse()?,
            "[1518-11-04 00:02] Guard #99 begins shift".parse()?,
        ];

        assert_eq!(events, expected);

        Ok(())
    }

    #[test]
    fn test_invalid_entry() {
        let result = "[1518-11-01 00:05] electric bugaloo".parse::<Entry>();

        assert_eq!(
            result,
            Err(EntryError::InvalidEvent(String::from("electric bugaloo")))
        );
    }

    #[test]
    fn test_begins_shift() -> Result<(), EntryError> {
        let result: Entry = "[1518-11-03 00:05] Guard #10 begins shift".parse()?;

        let expected = Entry {
            timestamp: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 3),
                NaiveTime::from_hms(0, 5, 0),
            ),
            event: Event::BeginsShift(Guard(10)),
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_falls_asleep() -> Result<(), EntryError> {
        let result: Entry = "[1518-11-01 00:05] falls asleep".parse()?;

        let expected = Entry {
            timestamp: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 1),
                NaiveTime::from_hms(0, 5, 0),
            ),
            event: Event::FallsAsleep,
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_wakes_up() -> Result<(), EntryError> {
        let result: Entry = "[1518-11-02 00:50] wakes up".parse()?;

        let expected = Entry {
            timestamp: NaiveDateTime::new(
                NaiveDate::from_ymd(1518, 11, 2),
                NaiveTime::from_hms(0, 50, 0),
            ),
            event: Event::WakesUp,
        };

        assert_eq!(result, expected);

        Ok(())
    }
}
