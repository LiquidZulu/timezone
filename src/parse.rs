use crate::Time;
use crate::TimeFormat;
use crate::TimeFormat::*;
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
        None => {
            println!(
                "\n[{}] cannot parse time \"{}\". Please format the time as one of the following:\n\n\tSimpleAmPm    - eg. 1am, 10pm, etc.\n\tFullAmPm      - eg. 12:24am, 6:30pm, etc.\n\tMilitaryColon - eg. 07:00, 13:52, etc.\n\tMilitary      - eg. 0900, 1776, etc.\n",
                "ERROR".red(),
                time
            );
            None
        }
    }
}
