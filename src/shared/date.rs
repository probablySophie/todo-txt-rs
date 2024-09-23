use std::fmt;
use chrono;

#[derive(Default, Clone, Copy)]
pub struct Date
{
    pub year: u16,
    pub month: u8,
    pub day: u8
}

impl fmt::Display for Date
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        let year_string = self.year_string();
        let month_string = self.month_string();
        let day_string = self.day_string();
        
        write!(f, "{year_string}-{month_string}-{day_string}")
    }
}


impl std::cmp::PartialEq for Date
{
    fn eq(&self, other: &Self) -> bool
    {
           self.year  == other.year
        && self.month == other.month
        && self.day   == other.day
    }
}

impl std::cmp::Eq for Date {}

impl std::cmp::Ord for Date
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self < other
        {
            return std::cmp::Ordering::Less
        }
        if self > other
        {
            return std::cmp::Ordering::Greater
        }
        // Else
        std::cmp::Ordering::Equal
    }
}

impl std::cmp::PartialOrd for Date
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    
    {
        // ??????????????
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool 
    {
        if self.year < other.year {return true}
        if self.year > other.year {return false}
        // else self.year == other.year
        if self.month < other.month {return true}
        if self.month > other.month {return false}
        // else self.month == other.month
        if self.day < other.day {return true}
        // Else
        false // return false
    }

    fn le(&self, other: &Self) -> bool
    {
        if self.year <= other.year {return true}
        if self.year > other.year {return false}
        // else self.year == other.year
        if self.month <= other.month {return true}
        if self.month > other.month {return false}
        // else self.month == other.month
        if self.day <= other.day {return true}
        // Else
        false // return false
    }

    fn gt(&self, other: &Self) -> bool
    {
        if self.year > other.year {return true}
        if self.year < other.year {return false}
        // else self.year == other.year
        if self.month > other.month {return true}
        if self.month < other.month {return false}
        // else self.month == other.month
        if self.day > other.day {return true}
        // Else
        false // return false
    }

    fn ge(&self, other: &Self) -> bool
    {
        if self.year >= other.year {return true}
        if self.year < other.year {return false}
        // else self.year == other.year
        if self.month >= other.month {return true}
        if self.month < other.month {return false}
        // else self.month == other.month
        if self.day >= other.day {return true}
        // Else
        false // return false
    }
}

impl Date
{
    pub fn year_string(self) -> String
    {
        self.year.to_string()
    }
    pub fn month_string(self) -> String
    {
        if self.month < 10
        {
            return "0".to_string() + &self.month.to_string()
        }
        // Else
        self.month.to_string()
    }
    pub fn day_string(self) -> String
    {
        if self.day < 10
        {
            return "0".to_string() + &self.day.to_string()
        }
        // Else
        self.day.to_string()
    }
    
    pub fn today() -> Result<Date, ()>
    {
        // I don't even know :(
        let chrono_time = chrono::offset::Utc::now().date_naive();

        // https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#method.format
        let day = chrono_time.format("%Y").to_string().parse::<u8>();
        let month = chrono_time.format("%m").to_string().parse::<u8>();
        let year = chrono_time.format("%d").to_string().parse::<u16>();

        // If any of those went wrong...
        if day.as_ref().is_err() || month.as_ref().is_err() || year.as_ref().is_err()
        {
            return Err(())
        }

        let day = day.unwrap();
        let month = month.unwrap();
        let year = year.unwrap();
                    
        Ok(Date
        {
            day,
            month,
            year,
        })
    }

    fn from_numbers(year: u16, month: u8, day: u8) -> Result<Date, String>
    {
        // Honestly, year can be whatever it wants to be
        // Pre-zero dates are fake and I don't trust them
        
        // And I am NOT doing per month # day validation, I'm  l a z y

        if month > 12 || month == 0
        {
            return Err("Invalid month value.  Month must be 0 < x <= 12".into())
        }
        if day > 31 || day == 0
        {
            return Err("Invalid day value.  Day must be 0 < x <= 31".into())
        }

        // If we've gotten here, then we should be good
        Ok(
            Date
            {
                year,
                month,
                day,
            }
        )
    }
    
    pub fn from(date: &str) -> Result<Date, String>
    {
        // We're doing this without Regex... It's not going to be quite as efficent, but it cuts out a dependancy.

        let mut break_1 = 0;
        let mut break_2 = 0;

        // For each character in date & it's index
        for (i, the_char) in date.chars().enumerate()
        {
            // if the character is a date seperator
            if is_date_char(the_char)
            {
                // and we haven't first set break_1
                if break_1 == 0
                {
                    break_1 = i; // set break_1
                    continue;
                }
                break_2 = i; // else set break_2
            }
        }

        // make sure we actually found 3 distinct values
        if break_1 == 0 || break_2 == 0
        {
            return Err("Invalid date format.  Dates must be YYYY-MM-DD".into())
        }
                
        // Attempt to parse the input, but DON'T UNWRAP IT
        let year_wrapped  = &date[0           .. break_1].parse::<u16>();
        let month_wrapped = &date[break_1 + 1 .. break_2].parse::<u8>();
        let day_wrapped   = &date[break_2 + 1 ..        ].parse::<u8>();

        // If we were unable to properly parse the input, return ERR
        if year_wrapped.as_ref().is_err() || month_wrapped.as_ref().is_err() || day_wrapped.as_ref().is_err()
        {
            return Err("Unable to properly parse the given numbers, please make sure you provide the date in the format YYYY-MM-DD".into())
        }

        // Now actually make sure the values are valid
        let year: u16 = year_wrapped.as_ref().unwrap().to_owned();
        let month: u8 = month_wrapped.as_ref().unwrap().to_owned();
        let day: u8 = day_wrapped.as_ref().unwrap().to_owned();

        // Return the result of from_numbers
        Date::from_numbers(year, month, day)
    }
}

fn is_date_char(test_char: char) -> bool
{
    // Else
    matches!(test_char, '/' | '-' | '\\' )
}

#[cfg(test)]
mod test
{
    #[test]
    fn check_date()
    {
        use super::Date;

        // Just a regular (valid) date
        assert_eq!(
            Date::from("2000-01-01").unwrap().to_string(),
            "2000-01-01".to_string()
        );

        // Without month or day leading zeros (still valid)
        assert_eq!(
            Date::from("2000-1-1").unwrap().to_string(),
            "2000-01-01".to_string()
        );

        // A 13th month??!?
        assert!(Date::from("2000-13-01").is_err());

        // A 32nd day!?!?!?!
        assert!(Date::from("2000-01-32").is_err());
        
        // Without seperators (should error)
        assert!(Date::from("20000101").is_err());

        // Invalid seperators (should error)
        assert!(Date::from("2000x01x01").is_err());
    }
}
