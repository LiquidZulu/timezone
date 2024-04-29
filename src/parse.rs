use crate::Time;
use crate::TimeFormat;
use crate::TimeFormat::*;
use crate::TZ_MAP;
use chrono::{Datelike, Duration, NaiveDate, Offset, TimeZone};
use chrono_tz::Tz;
use colored::*;
use regex::Regex;
use std::num::ParseIntError;

pub fn is_pm(time: String) -> bool {
    &time[time.len() - 2..time.len()] == "pm"
}

fn pm_offset(time: String) -> u32 {
    if is_pm(time) {
        12
    } else {
        0
    }
}

pub fn get_time_format(time: String) -> Option<TimeFormat> {
    let SimpleAmPmRegex: Regex = Regex::new(r"^(\d){1,2}(am|pm)$").unwrap();
    let FullAmPmRegex: Regex = Regex::new(r"^(\d){1,2}\:(\d){2}(am|pm)$").unwrap();
    let MilitaryColonRegex: Regex = Regex::new(r"^(\d){2}\:(\d){2}$").unwrap();
    let MilitaryRegex: Regex = Regex::new(r"^(\d){4}$").unwrap();

    if SimpleAmPmRegex.is_match(&time) {
        return Some(SimpleAmPm);
    }

    if FullAmPmRegex.is_match(&time) {
        return Some(FullAmPm);
    }

    if MilitaryColonRegex.is_match(&time) {
        return Some(MilitaryColon);
    }

    if MilitaryRegex.is_match(&time) {
        return Some(Military);
    }

    None
}

pub fn parse_time(time: String) -> Option<(u32, u32)> {
    match get_time_format(time.clone()) {
        Some(format) => match format {
            SimpleAmPm => {
                let offset = pm_offset(time.clone());

                let hours_string: String = if time.len() == 3 {
                    time[0..1].to_string()
                } else {
                    time[0..2].to_string()
                };

                match hours_string.parse::<u32>() {
                    Ok(hours_int) => match Time::hours(hours_int) {
                        Some(Time::Hours(h)) => Some((h + offset, 0)),
                        _ => None,
                    },
                    _ => None,
                }
            }
            FullAmPm => {
                let offset = pm_offset(time.clone());

                let truncated = match time.len() {
                    6 => Some(time[0..4].to_string()),
                    7 => Some(time[0..5].to_string()),
                    _ => None,
                };

                match truncated {
                    Some(time_with_colon) => {
                        let hm: Vec<Result<u32, ParseIntError>> = time_with_colon
                            .split(":")
                            .map(|x| x.parse::<u32>())
                            .collect();

                        let maybe_hours = &hm[0];
                        let maybe_minutes = &hm[1];

                        match (maybe_hours, maybe_minutes) {
                            (Ok(h), Ok(m)) => match (Time::hours(h + offset), Time::minutes(*m)) {
                                (Some(Time::Hours(hr)), Some(Time::Minutes(mn))) => Some((hr, mn)),
                                _ => None,
                            },
                            _ => None,
                        }
                    }
                    None => None,
                }
            }

            MilitaryColon => {
                let hm: Vec<Result<u32, ParseIntError>> =
                    time.split(":").map(|x| x.parse::<u32>()).collect();

                let maybe_hours = &hm[0];
                let maybe_minutes = &hm[1];

                match (maybe_hours, maybe_minutes) {
                    (Ok(h), Ok(m)) => match (Time::hours(*h), Time::minutes(*m)) {
                        (Some(Time::Hours(hr)), Some(Time::Minutes(mn))) => Some((hr, mn)),
                        _ => None,
                    },
                    _ => None,
                }
            }
            Military => {
                let maybe_hours = &time[0..2].parse::<u32>();
                let maybe_minutes = &time[2..4].parse::<u32>();

                match (maybe_hours, maybe_minutes) {
                    (Ok(h), Ok(m)) => match (Time::hours(*h), Time::minutes(*m)) {
                        (Some(Time::Hours(hr)), Some(Time::Minutes(mn))) => Some((hr, mn)),
                        _ => None,
                    },
                    _ => None,
                }
            }
        },
        None => None,
    }
}

pub fn parse_timezone(origin: String, destination: Option<String>) -> (Option<Tz>, Option<Tz>) {
    (
        TZ_MAP.get(&origin).copied(),
        match destination {
            Some(ref d) => TZ_MAP.get(&d).copied(),
            None => {
                let offset = chrono::Local
                    .timestamp(0, 0)
                    .offset()
                    .fix()
                    .local_minus_utc()
                    / 60
                    / 60;
                let local = if offset < 0 {
                    format!("utc-{offset}")
                } else {
                    format!("utc+{offset}")
                };
                println!(
                    "\n[{}] cannot parse destination {:?}. Using local timezone = {local}.",
                    "WARNING".yellow(),
                    destination
                );
                TZ_MAP.get(&local).copied()
            }
        },
    )
}

pub fn parse_day(maybe_day: Option<String>) -> Option<u32> {
    if maybe_day == None {
        return parse_day(Some("today".to_string()));
    }

    let day = maybe_day.unwrap().to_lowercase();
    let today = chrono::Utc::today();

    if day == "today" {
        return Some(today.day());
    }

    if day == "yesterday" {
        return Some((today - Duration::days(1)).day());
    }

    if day == "tomorrow" {
        return Some((today + Duration::days(1)).day());
    }

    match day.parse::<u32>() {
        Ok(n) => match NaiveDate::from_ymd_opt(today.year(), today.month(), n) {
            Some(date) => Some(date.day()),
            None => None,
        },
        Err(_) => None,
    }
}

static MONTH_MAP: phf::Map<&'static str, u32> = phf::phf_map! {
    "jan" => 1,
    "january" => 1,
    "feb" => 2,
    "february" => 2,
    "mar" => 3,
    "march" => 3,
    "apr" => 4,
    "april" => 4,
    "may" => 5,
    "jun" => 6,
    "june" => 6,
    "jul" => 7,
    "july" => 7,
    "aug" => 8,
    "august" => 8,
    "sep" => 9,
    "sept" => 9,
    "september" => 9,
    "oct" => 10,
    "october" => 10,
    "nov" => 11,
    "november" => 11,
    "dec" => 12,
    "december" => 12,
    "1" => 1,
    "01" => 1,
    "2" => 2,
    "02" => 2,
    "3" => 3,
    "03" => 3,
    "4" => 4,
    "04" => 4,
    "5" => 5,
    "05" => 5,
    "6" => 6,
    "06" => 6,
    "7" => 7,
    "07" => 7,
    "8" => 8,
    "08" => 8,
    "9" => 9,
    "09" => 9,
    "10" => 10,
    "11" => 11,
    "12" => 12,
};

pub fn parse_month(maybe_month: Option<String>) -> Option<u32> {
    if maybe_month == None {
        return parse_month(Some(chrono::Utc::today().month().to_string()));
    }

    return MONTH_MAP.get(&maybe_month.unwrap()).copied();
}

pub fn parse_year(maybe_year: Option<String>) -> Option<u32> {
    if maybe_year == None {
        return parse_year(Some(chrono::Utc::today().month().to_string()));
    }

    match maybe_year.unwrap().parse::<i32>() {
        Ok(n) => match NaiveDate::from_ymd_opt(n, 1, 1) {
            Some(date) => Some(date.year().try_into().unwrap()),
            None => None,
        },
        Err(_) => None,
    }
}
