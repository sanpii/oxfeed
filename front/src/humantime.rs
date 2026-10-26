use std::borrow::Cow;
use std::cmp::max;
use std::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
enum Tense {
    Past,
    Present,
    Future,
}

const S_MINUTE: i64 = 60;
const S_HOUR: i64 = S_MINUTE * 60;
const S_DAY: i64 = S_HOUR * 24;
const S_WEEK: i64 = S_DAY * 7;
const S_MONTH: i64 = S_DAY * 30;
const S_YEAR: i64 = S_DAY * 365;

#[derive(Clone, Copy, Debug)]
enum TimePeriod {
    Now,
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Weeks(i64),
    Months(i64),
    Years(i64),
    Eternity,
}

impl TimePeriod {
    fn to_text(self) -> Cow<'static, str> {
        match self {
            Self::Now => "now".into(),
            Self::Seconds(n) => format!("{n} seconds").into(),
            Self::Minutes(1) => "a minute".into(),
            Self::Minutes(n) => format!("{n} minutes").into(),
            Self::Hours(1) => "an hour".into(),
            Self::Hours(n) => format!("{n} hours").into(),
            Self::Days(1) => "a day".into(),
            Self::Days(n) => format!("{n} days").into(),
            Self::Weeks(1) => "a week".into(),
            Self::Weeks(n) => format!("{n} weeks").into(),
            Self::Months(1) => "a month".into(),
            Self::Months(n) => format!("{n} months").into(),
            Self::Years(1) => "a year".into(),
            Self::Years(n) => format!("{n} years").into(),
            Self::Eternity => "eternity".into(),
        }
    }
}

pub struct Humantime(jiff::Span);

impl Humantime {
    fn tense(&self) -> Tense {
        if self.0.total(jiff::Unit::Second).unwrap_or_default() < 11. {
            Tense::Present
        } else if self.0.is_negative() {
            Tense::Past
        } else if self.0.is_positive() {
            Tense::Future
        } else {
            Tense::Present
        }
    }

    fn to_text_en(&self, tense: Tense) -> String {
        let mut periods = self.period();

        let first = periods.remove(0).to_text();
        let last = periods.pop().map(|last| last.to_text());

        let mut text = periods
            .into_iter()
            .fold(first, |acc, p| format!("{}, {}", acc, p.to_text()).into());

        if let Some(last) = last {
            text = format!("{} and {}", text, last).into();
        }

        match tense {
            Tense::Past => format!("{} ago", text),
            Tense::Future => format!("in {}", text),
            Tense::Present => text.into_owned(),
        }
    }

    fn period(&self) -> Vec<TimePeriod> {
        let period = match self.0.total(jiff::Unit::Second).unwrap_or_default().abs() as i64 {
            n if n > 547 * S_DAY => TimePeriod::Years(max(n / S_YEAR, 2)),
            n if n > 345 * S_DAY => TimePeriod::Years(1),
            n if n > 45 * S_DAY => TimePeriod::Months(max(n / S_MONTH, 2)),
            n if n > 29 * S_DAY => TimePeriod::Months(1),
            n if n > 10 * S_DAY + 12 * S_HOUR => TimePeriod::Weeks(max(n / S_WEEK, 2)),
            n if n > 6 * S_DAY + 12 * S_HOUR => TimePeriod::Weeks(1),
            n if n > 36 * S_HOUR => TimePeriod::Days(max(n / S_DAY, 2)),
            n if n > 22 * S_HOUR => TimePeriod::Days(1),
            n if n > 90 * S_MINUTE => TimePeriod::Hours(max(n / S_HOUR, 2)),
            n if n > 45 * S_MINUTE => TimePeriod::Hours(1),
            n if n > 90 => TimePeriod::Minutes(max(n / S_MINUTE, 2)),
            n if n > 45 => TimePeriod::Minutes(1),
            n if n > 10 => TimePeriod::Seconds(n),
            0..=10 => TimePeriod::Now,
            _ => TimePeriod::Eternity,
        };

        vec![period]
    }
}

impl TryFrom<&jiff::Zoned> for Humantime {
    type Error = jiff::Error;

    fn try_from(value: &jiff::Zoned) -> Result<Self, Self::Error> {
        let now = jiff::Zoned::now();
        let span = value.since(&now)?;
        let humantime = Self(span);

        Ok(humantime)
    }
}

impl fmt::Display for Humantime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tense = self.tense();
        f.write_str(&self.to_text_en(tense))
    }
}
