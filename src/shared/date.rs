use std::fmt;

#[derive(Default, Clone, Copy)]
pub struct Date
{
    pub year: u16,
    pub month: u8,
    pub day: u8
}
/*
   TODO: Date comparison https://doc.rust-lang.org/std/cmp/index.html
*/


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
    
    pub fn today() -> Date
    {
        // TODO: Make this return the current date
        Date
        {
            day: 0,
            month: 0,
            year: 0,
        }
    }
    pub fn from(date: &str) -> Result<Date, &str>
    {
        if date.len() != 10
        {
            return Err("Date string is the wrong length")
        }

        // Attempt to parse the input, but DON'T UNWRAP IT
        let year_wrapped  = &date[0.. 4].parse::<u16>();
        let month_wrapped = &date[5.. 7].parse::<u8>();
        let day_wrapped   = &date[8..10].parse::<u8>();

        // If we were unable to properly parse the input, return ERR
        if year_wrapped.as_ref().is_err() || month_wrapped.as_ref().is_err() || day_wrapped.as_ref().is_err()
        {
            return Err("Unable to properly parse the given numbers, please make sure you provide the date in the format YYYY-MM-DD")
        }

        // Now actually make sure the values are valid
        let year: u16 = year_wrapped.as_ref().unwrap().to_owned();
        let month: u8 = month_wrapped.as_ref().unwrap().to_owned();
        let day: u8 = day_wrapped.as_ref().unwrap().to_owned();

        // Honestly, year can be whatever it wants to be
        // And I am NOT doing per month # day validation, I'm  l a z y

        if month > 12
        {
            return Err("Month > 12, that's bad")
        }
        if day > 31
        {
            return Err("Day > 31, that's bad")
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
}

