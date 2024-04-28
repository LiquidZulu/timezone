pub enum Time {
    Hours(u32),
    Minutes(u32),
}

impl Time {
    pub fn hours(h: u32) -> Option<Time> {
        if h < 24 {
            Some(Time::Hours(h))
        } else {
            None
        }
    }

    pub fn minutes(m: u32) -> Option<Time> {
        if m < 60 {
            Some(Time::Minutes(m))
        } else {
            None
        }
    }
}

pub enum TimeFormat {
    SimpleAmPm,    // 1pm
    FullAmPm,      // 12:34pm
    MilitaryColon, // 13:00
    Military,      // 1400
}
