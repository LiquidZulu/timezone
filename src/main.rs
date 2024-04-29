use chrono::{DateTime, TimeZone, Timelike};
use chrono_tz::Tz;
use colored::*;
use std::env;

mod parse;
use parse::*;

mod types;
use types::*;

mod convert_timezones;
use convert_timezones::*;

/*
 * 1pm EST                          - assumes EST -> your timezone
 * 1pm EST BST                      - assumes 1pm EST on the current day
 * 1pm EST BST tomorrow/yesterday/n - n being the nth day of the current month
 * 1pm EST BST 20 feb               - assumes feb of the current year
 * 1pm EST BST 20 feb 2020          - fully specified
 */

fn convert(
    time: String,
    origin: String,
    destination: Option<String>,
    day: Option<String>,
    month: Option<String>,
    year: Option<String>,
) {
    let maybe_time = parse_time(time.clone().to_lowercase());

    if maybe_time == None {
        println!(
            "\n[{}] cannot parse the time \"{}\". Please format the time as one of the following:\n\n\tSimpleAmPm    - eg. 1am, 10pm, etc.\n\tFullAmPm      - eg. 12:24am, 6:30pm, etc.\n\tMilitaryColon - eg. 07:00, 13:52, etc.\n\tMilitary      - eg. 0900, 1634, etc.\n\n[{}] this software is pretty good at working out which format you are using, make sure that if you specified am/pm that you are not using 24 hours and that you don't go over 59 minutes.",
            "ERROR".red(),
            time,
            "HINT".cyan()
        );
        return;
    }

    let (hours, minutes) = maybe_time.unwrap();

    let (maybe_origin_timezone, maybe_destination_timezone) =
        parse_timezone(origin.clone(), destination.clone());

    if maybe_origin_timezone == None {
        println!("\n[{}] cannot parse origin {}", "ERROR".red(), origin);
        return;
    }

    if maybe_destination_timezone == None {
        println!(
            "\n[{}] cannot parse destination {:?} and cannot get local timezone.",
            "ERROR".red(),
            destination
        );
        return;
    }

    let (origin_timezone, destination_timezone) = (
        maybe_origin_timezone.unwrap(),
        maybe_destination_timezone.unwrap(),
    );

    let maybe_day = parse_day(day.clone());

    if maybe_day == None {
        println!("[{}] could not parse day {:?}", "ERROR".red(), day);
        return;
    }

    let day_n = maybe_day.unwrap();

    let maybe_month = parse_month(month.clone());

    if maybe_month == None {
        println!("[{}] could not parse month {:?}", "ERROR".red(), month);
        return;
    }

    let month_n = maybe_month.unwrap();

    let maybe_year = parse_year(year.clone());

    if maybe_year == None {
        println!("[{}] could not parse year {:?}", "ERROR".red(), year);
        return;
    }

    let year_n = maybe_year.unwrap();

    let origin_time = origin_timezone
        .with_ymd_and_hms(year_n, month_n, day_n, hours, minutes, 0)
        .unwrap();
    let destination_time = origin_time.with_timezone(&destination_timezone);

    output(
        time,
        origin,
        destination,
        day,
        month,
        year,
        origin_time,
        destination_time,
    )
}

fn output(
    time: String,
    origin: String,
    destination: Option<String>,
    day: Option<String>,
    month: Option<String>,
    year: Option<String>,
    origin_time: DateTime<Tz>,
    destination_time: DateTime<Tz>,
) {
    println!("\n");
    match (time, origin, destination, day, month, year) {
        (time, origin, Some(destination), Some(day), Some(month), Some(year)) => {
            let (pm, hour) = destination_time.hour12();
            println!(
                "{time} {origin} {day} {month} {year} is {}:{:0>2}{} {destination}",
                hour,
                destination_time.minute(),
                if pm { "pm" } else { "am" }
            );
        }
        (time, origin, Some(destination), Some(day), Some(month), None) => {
            let (pm, hour) = destination_time.hour12();
            println!(
                "{time} {origin} {day} {month} is {}:{:0>2}{} {destination}",
                hour,
                destination_time.minute(),
                if pm { "pm" } else { "am" }
            );
        }
        (time, origin, Some(destination), Some(day), None, None) => {
            let (pm, hour) = destination_time.hour12();
            println!(
                "{time} {origin} {day} is {}:{:0>2}{} {destination}",
                hour,
                destination_time.minute(),
                if pm { "pm" } else { "am" }
            );
        }
        (time, origin, Some(destination), None, None, None) => {
            let (pm, hour) = destination_time.hour12();
            println!(
                "{time} {origin} is {}:{:0>2}{} {destination}",
                hour,
                destination_time.minute(),
                if pm { "pm" } else { "am" }
            );
        }
        (time, origin, None, None, None, None) => {
            let (pm, hour) = destination_time.hour12();
            println!(
                "{time} {origin} is {}:{:0>2}{} local time",
                hour,
                destination_time.minute(),
                if pm { "pm" } else { "am" }
            );
        }

        _ => {
            println!("{} is {}", origin_time, destination_time.to_string().cyan())
        }
    };

    println!("\n")
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() == 0 {
        println!(
            "
Timezone conversion on the command line.

\tUsage: time origin_timezone destination_timezone day month year
\tExample: 1pm et bst tomorrow
\t  ↳ display what 1pm eastern time is in British summer time tomorrow.

time should be in one of the following formats:

\tSimpleAmPm    - eg. 1am, 10pm, etc.
\tFullAmPm      - eg. 12:24am, 6:30pm, etc.
\tMilitaryColon - eg. 07:00, 13:52, etc.
\tMilitary      - eg. 0900, 1634, etc.

The origin and destination timezones can be either a city,
such as Europe/London, or a timezone abbreviation, such
as gmt. A full list of accepted timezones is located at:

https://github.com/LiquidZulu/timezone/blob/main/src/convert_timezones.rs

The day, month, and year are all fairly self-explanatory,
but you can also specify 'today', 'tomorrow' or 'yesterday'
for the day.

This software is robust, you do not have to fully-specify
the conversion that you want to perform. At a minimum you
can specify only the time and the origin, with the rest
being assumed to be your local timezone, the current day,
the current month, and the current year.\n"
        );
        return;
    }

    // underspecified
    if args.len() < 2 {
        println!(
            "\n[{}] cannot convert timezones with only {} argument(s) specified.\n",
            "ERROR".red(),
            args.len()
        );
        return;
    }

    // overspecified
    if args.len() > 6 {
        println!(
            "\n[{}] you have overspecified the conversion.\n\n\tExpected: time origin_timezone destination_timezone day month year\n\tGot:      {} {}\n",
            "WARNING".yellow(),
            args[0..5].join(" "),
            args[6..].join(" ").red()
        );
    }

    match args.len() {
        2 => convert(
            args[0].clone(),
            args[1].clone(),
            None,
            None,
            None,
            None,
        ),
        3 => convert(
            args[0].clone(),
            args[1].clone(),
            Some(args[2].clone()),
            None,
            None,
            None,
        ),
        4 => convert(
            args[0].clone(),
            args[1].clone(),
            Some(args[2].clone()),
            Some(args[3].clone()),
            None,
            None,
        ),
        5 => convert(
            args[0].clone(),
            args[1].clone(),
            Some(args[2].clone()),
            Some(args[3].clone()),
            Some(args[4].clone()),
            None,
        ),
        6.. => convert(
            args[0].clone(),
            args[1].clone(),
            Some(args[2].clone()),
            Some(args[3].clone()),
            Some(args[4].clone()),
            Some(args[5].clone()),
        ),
        _ => unreachable!("[{}] args.len() not covered by match statement. This should never happen, if you see this please report this error at https://github.com/LiquidZulu/timezone/issues", "ERROR".red()),
    };
}
