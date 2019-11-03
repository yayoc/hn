extern crate time;

use chrono::{TimeZone, Utc};
use time::Duration;

/// Compare unixtime and get relative time label.
/// t1 should be past time compared to t2.
pub fn get_relative_time(t1: i64, t2: i64) -> String {
    let dt1 = Utc.timestamp(t1, 0).naive_local();
    let dt2 = Utc.timestamp(t2, 0).naive_local();
    let du: Duration = dt2.signed_duration_since(dt1);
    if du.num_weeks() > 0 {
        if du.num_weeks() == 1 {
            return String::from("a week ago");
        } else {
            return format!("{} weeks ago", du.num_weeks());
        }
    } else if du.num_days() > 0 {
        if du.num_days() == 1 {
            return String::from("a day ago");
        } else {
            return format!("{} days ago", du.num_days());
        }
    } else if du.num_hours() > 0 {
        if du.num_hours() == 1 {
            return String::from("an hour ago");
        } else {
            return format!("{} hours ago", du.num_hours());
        }
    } else if du.num_minutes() > 0 {
        if du.num_minutes() == 1 {
            return String::from("a minute ago");
        } else {
            return format!("{} minutes ago", du.num_minutes());
        }
    }
    if du.num_seconds() == 1 {
        return String::from("a second ago");
    } else {
        return format!("{} seconds ago", du.num_seconds());
    }
}

#[cfg(test)]
mod tests {
    use crate::time::get_relative_time;

    #[test]
    fn test_get_relative_time() {
        assert_eq!("a second ago", get_relative_time(1572737269, 1572737270));
        assert_eq!("59 seconds ago", get_relative_time(1572737211, 1572737270));
        assert_eq!("a minute ago", get_relative_time(1572737210, 1572737270));
        assert_eq!("59 minutes ago", get_relative_time(1572733730, 1572737270));
        assert_eq!("an hour ago", get_relative_time(1572733670, 1572737270));
        assert_eq!("2 hours ago", get_relative_time(1572730070, 1572737270));
        assert_eq!("23 hours ago", get_relative_time(1572654470, 1572737270));
        assert_eq!("a day ago", get_relative_time(1572650870, 1572737270));
        assert_eq!("2 days ago", get_relative_time(1572564470, 1572737270));
        assert_eq!("6 days ago", get_relative_time(1572218870, 1572737270));
        assert_eq!("a week ago", get_relative_time(1572132470, 1572737270));
        assert_eq!("2 weeks ago", get_relative_time(1571527670, 1572737270));
    }
}
