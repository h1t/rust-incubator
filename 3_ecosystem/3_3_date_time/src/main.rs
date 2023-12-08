use time::macros::format_description;
use time::{Date, Month};

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User {
    birthdate: Date,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        let month = Month::try_from(month as u8).expect("bad month value");
        let birthdate = Date::from_calendar_date(year, month, day as u8).expect("bad date format");
        Self { birthdate }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let format = format_description!("[year]-[month]-[day]");
        let now = Date::parse(NOW, &format).expect("can't parse NOW date");
        let now_month = u8::from(now.month());
        let now_day = now.day();
        let user_month = u8::from(self.birthdate.month());
        let user_day = self.birthdate.day();
        let mut age = now.year() - self.birthdate.year();

        //NOTE: before birthday in this year
        if user_month <= now_month && user_day <= now_day {
            age -= 1;
        }
        age.try_into().unwrap_or(0)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in [
            ((1990, 6, 4), 28),
            ((1990, 7, 4), 29),
            ((0, 1, 1), 2018),
            ((1970, 1, 1), 48),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in [
            ((2032, 6, 25), 0),
            ((2016, 6, 27), 3),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }
}
