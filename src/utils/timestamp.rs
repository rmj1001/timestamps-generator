use crate::prelude::*;
use regex::Regex;
use std::{
    fmt::{Display, Error},
    fs,
};

pub const FILEPATH: &str = "timestamps.txt";

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub description: String,
}

impl Timestamp {
    pub fn menu() -> Self {
        clearscr();
        let mut timestamp = TimestampBuilder::default();

        println!("AMA Timestamp Generator");
        println!("-----------------------");
        println!();

        loop {
            let hour = input_generic::<isize>("Hours").unwrap();

            if hour < 0 {
                println!("Negative hours don't exist. Try again.");
                pause();
            } else {
                timestamp.hours(hour as usize);
                break;
            }
        }

        loop {
            let minute = input_generic::<isize>("Minutes").unwrap();

            if !(0..=59).contains(&minute) {
                println!("There are only 0-59 seconds. Try again.");
                pause();
            } else {
                timestamp.minutes(minute as usize);
                break;
            }
        }

        loop {
            let seconds = input_generic::<isize>("Seconds").unwrap();

            if !(0..=59).contains(&seconds) {
                println!("There are only 0-59 seconds. Try again.");
                pause();
            } else {
                timestamp.seconds(seconds as usize);
                break;
            }
        }

        timestamp.description(prompt("Description"));

        let timestamp = timestamp.build();

        println!();
        println!("Timestamp Created:");
        println!("- {}", timestamp);
        println!();
        pause();

        timestamp
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minute: String = if self.minute < 10 {
            format!("0{}", self.minute)
        } else {
            format!("{}", self.minute)
        };

        let second: String = if self.second < 10 {
            format!("0{}", self.second)
        } else {
            format!("{}", self.second)
        };

        write!(
            f,
            "{}:{}:{} {}",
            self.hour, minute, second, self.description
        )
    }
}

impl TryFrom<String> for Timestamp {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let timestamp_pattern = Regex::new(
            r"(?x)(?P<hour>\d{1}):(?P<minute>\d+):(?P<second>\d+)\s+(?P<description>.+)",
        )
        .unwrap();

        let stamps = match timestamp_pattern.captures(&value) {
            Some(s) => s,
            None => return Err(std::fmt::Error),
        };

        let hour = &stamps["hour"]
            .to_string()
            .parse::<usize>()
            .expect("Should be usize");

        let minute = &stamps["minute"]
            .to_string()
            .parse::<usize>()
            .expect("Should be usize");

        let second = &stamps["second"]
            .to_string()
            .parse::<usize>()
            .expect("Should be usize");

        let description = &stamps["description"].to_string();

        let mut ts = TimestampBuilder::default();

        ts.hours(*hour);
        ts.minutes(*minute);
        ts.seconds(*second);
        ts.description(description.clone());

        Ok(ts.build())
    }
}

#[derive(Default)]
pub struct TimestampBuilder {
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub description: String,
}

impl TimestampBuilder {
    pub fn hours(&mut self, number: usize) -> &Self {
        self.hour = number;
        self
    }

    pub fn minutes(&mut self, number: usize) -> &Self {
        self.minute = number;
        self
    }

    pub fn seconds(&mut self, number: usize) -> &Self {
        self.second = number;
        self
    }

    pub fn description(&mut self, text: String) -> &Self {
        self.description = text;
        self
    }

    pub fn build(self) -> Timestamp {
        Timestamp {
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            description: self.description,
        }
    }
}

pub fn rebuild_timestamps() -> Vec<Timestamp> {
    let mut memory: Vec<Timestamp> = vec![];

    let file_contents = match fs::read_to_string(FILEPATH) {
        Ok(contents) => contents,
        Err(_) => return memory,
    };

    let lines = file_contents.lines();

    lines.for_each(|line| {
        if let Ok(timestamp) = Timestamp::try_from(line.to_string()) {
            memory.push(timestamp);
        }
    });

    memory
}

pub fn build_timestamps(timestamps: &[Timestamp]) -> String {
    let mut string: String = String::from("Timestamps:\n");

    timestamps.iter().for_each(|timestamp| {
        string.push_str(&format!("{}\n", timestamp));
    });

    string
}

pub fn build_file(timestamps: &[Timestamp]) {
    let string = build_timestamps(timestamps);
    const FILEPATH: &str = "timestamps.txt";

    fs::write(FILEPATH, string).unwrap();

    title();
    println!("Wrote timestamps to '{}'.", FILEPATH);
    pause();
    clearscr();
}

pub fn view_timestamps(timestamps: &[Timestamp]) {
    let string = build_timestamps(timestamps);
    let lines = string.lines().collect::<Vec<&str>>();
    let pages = lines.chunks(40);

    pages.for_each(|page| {
        title();

        page.iter().for_each(|f| {
            println!("{}", f);
        });

        println!();
        pause();
    })
}

pub fn remove_line(timestamps: &mut Vec<Timestamp>) {
    title();

    let line = select(timestamps, None);

    let text = timestamps
        .get(line)
        .expect("This should always choose a line")
        .clone();

    timestamps.remove(line);

    println!("Removed line '{text}'.");
    pause();
}
