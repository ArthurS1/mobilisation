use chrono::*;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub picture_url: Option<Url>,
    pub begins_on: DateTime,
    pub ends_on: DateTime,
}

#[derive(Debug, PartialEq)]
pub enum HumanReadableDateTime {
    Now,
    Later(String),
}

impl Event {
    // TODO: memoize ?
    pub fn is_long(&self) -> bool {
        let delta = self.ends_on.underlying - self.begins_on.underlying;
        delta.num_days() >= 1
    }

    // TODO: memoize ?
    pub fn compute_duration_in_hours(&self) -> i64 {
        let delta = self.ends_on.underlying - self.begins_on.underlying;
        delta.num_hours()
    }

    pub fn compute_human_readable_begining(
        &self,
        get_now: Option<&impl Fn() -> chrono::DateTime<chrono::Local>>,
    ) -> HumanReadableDateTime {
        let now = match get_now {
            None => Self::get_now(),
            Some(f) => f(),
        };
        let underlying_local = self.begins_on.underlying.with_timezone(&chrono::Local);
        let breakpoints = Breakpoints::new(now);
        match breakpoints {
            _ if self.begins_on.underlying < now && now < self.ends_on.underlying => {
                HumanReadableDateTime::Now
            }
            Some(breakpoints) if underlying_local < breakpoints.midnight => {
                HumanReadableDateTime::Later(underlying_local.format("%H:%M").to_string())
            }
            Some(breakpoints)
                if underlying_local >= breakpoints.midnight
                    && underlying_local < breakpoints.next_week =>
            {
                HumanReadableDateTime::Later(underlying_local.format("%A").to_string())
            }
            Some(breakpoints)
                if underlying_local >= breakpoints.next_week
                    && underlying_local < breakpoints.next_month =>
            {
                HumanReadableDateTime::Later(underlying_local.format("%A %e").to_string())
            }
            Some(breakpoints)
                if underlying_local >= breakpoints.next_month
                    && underlying_local < breakpoints.next_year =>
            {
                HumanReadableDateTime::Later(underlying_local.format("%B %e").to_string())
            }
            Some(breakpoints) if underlying_local >= breakpoints.next_year => {
                HumanReadableDateTime::Later(underlying_local.format("%B %e %G").to_string())
            }
            _ => HumanReadableDateTime::Later(underlying_local.to_string()),
        }
    }

    pub fn get_now() -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }
}

#[derive(Default, Debug)]
pub struct DateTime {
    underlying: chrono::DateTime<chrono::Utc>,
}

impl DateTime {
    pub fn new(time: chrono::DateTime<chrono::Utc>) -> DateTime {
        DateTime { underlying: time }
    }

    pub fn to_rfc3339(&self) -> String {
        self.underlying.to_rfc3339()
    }
}

#[derive(Debug)]
struct Breakpoints {
    midnight: chrono::DateTime<chrono::Local>,
    next_week: chrono::DateTime<chrono::Local>,
    next_month: chrono::DateTime<chrono::Local>,
    next_year: chrono::DateTime<chrono::Local>,
}

impl Breakpoints {
    pub fn new(now: chrono::DateTime<Local>) -> Option<Self> {
        let base = Self::mapped_local_time_to_opt(chrono::Local.with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            0,
            0,
            0,
        ));
        let midnight_breakpoint = base.and_then(|t| t.checked_add_days(Days::new(1)))?;
        let next_week_breakpoint = base.and_then(|t| t.checked_add_days(Days::new(7)))?;
        let next_month_breakpoint = base.and_then(|t| t.checked_add_months(Months::new(1)))?;
        let next_year_breakpoint = base.and_then(|t| t.with_year(t.year() + 1))?;
        Some(Self {
            midnight: midnight_breakpoint,
            next_week: next_week_breakpoint,
            next_month: next_month_breakpoint,
            next_year: next_year_breakpoint,
        })
    }

    fn mapped_local_time_to_opt<T>(a: chrono::MappedLocalTime<T>) -> Option<T> {
        match a {
            chrono::offset::MappedLocalTime::Single(t) => Some(t),
            chrono::offset::MappedLocalTime::Ambiguous(a, _) => Some(a),
            chrono::offset::MappedLocalTime::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    fn create_event(begining: &str, end: &str) -> super::Event {
        super::Event {
            title: "test".to_string(),
            picture_url: None,
            begins_on: create_date(begining),
            ends_on: create_date(end),
        }
    }

    fn create_date(rfc3339: &str) -> super::DateTime {
        let a = chrono::DateTime::parse_from_rfc3339(rfc3339).unwrap();
        let b = a.with_timezone(&chrono::Utc);
        super::DateTime::new(b)
    }

    fn create_fake_now(rfc3339: &str) -> impl Fn() -> chrono::DateTime<chrono::Local> {
        move || {
            chrono::DateTime::parse_from_rfc3339(rfc3339)
                .unwrap()
                .with_timezone(&chrono::Local)
        }
    }

    #[test]
    fn human_readable_produces_time_below_midnight_breakpoint() {
        let fake_now = Some(&create_fake_now("2001-01-30T00:00:00+01:00"));
        let event = create_event("2001-01-30T00:00:00+01:00", "2001-01-30T02:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("00:00".to_string())
        );

        let event = create_event("2001-01-30T03:00:00+01:00", "2001-01-30T05:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("03:00".to_string())
        );

        let event = create_event("2001-01-30T23:59:00+01:00", "2001-01-31T00:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("23:59".to_string())
        );
    }

    #[test]
    fn human_readable_produces_time_below_week_breakpoint() {
        let fake_now = Some(&create_fake_now("2001-01-30T00:00:00+01:00"));
        let event = create_event("2001-01-31T00:00:00+01:00", "2001-01-31T02:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("Wednesday".to_string())
        );

        let event = create_event("2001-02-05T23:59:00+01:00", "2001-02-06T23:59:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("Monday".to_string())
        );
    }

    #[test]
    fn human_readable_produces_time_below_month() {
        let fake_now = Some(&create_fake_now("2001-01-01T00:00:00+01:00"));
        let event = create_event("2001-01-20T00:00:00+01:00", "2001-01-31T02:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("Saturday 20".to_string())
        );
    }

    #[test]
    fn human_readable_produces_time_below_year() {
        let fake_now = Some(&create_fake_now("2001-01-01T00:00:00+01:00"));
        let event = create_event("2001-02-20T00:00:00+01:00", "2001-01-31T02:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Later("February 20".to_string())
        );
    }

    #[test]
    fn human_readable_now() {
        let fake_now = Some(&create_fake_now("2001-01-01T12:00:00+01:00"));
        let event = create_event("2001-01-01T00:00:00+01:00", "2001-01-01T15:00:00+01:00");
        assert_eq!(
            event.compute_human_readable_begining(fake_now),
            super::HumanReadableDateTime::Now
        );
    }

    #[test]
    fn event_is_short() {
        let a = super::Event {
            title: "test".to_string(),
            picture_url: None,
            begins_on: create_date("2001-02-20T00:00:00+01:00"),
            ends_on: create_date("2001-02-20T03:00:00+01:00"),
        };
        assert_eq!(a.is_long(), false);
    }

    #[test]
    fn event_is_long() {
        let b = super::Event {
            title: "test".to_string(),
            picture_url: None,
            begins_on: create_date("2001-02-20T00:00:00+01:00"),
            ends_on: create_date("2001-02-21T03:00:00+01:00"),
        };
        assert_eq!(b.is_long(), true);
    }

    #[test]
    fn duration_in_hours() {
        let a = super::Event {
            title: "test".to_string(),
            picture_url: None,
            begins_on: create_date("2001-02-20T00:00:00+01:00"),
            ends_on: create_date("2001-02-20T03:00:00+01:00"),
        };
        assert_eq!(a.compute_duration_in_hours(), 3);
    }
}
