#[derive(Debug, Clone)]
struct Year(u64);
impl Year {
    fn is_leap(&self) -> bool {
        (self.0 % 4 == 0) && (self.0 % 100 != 0 || self.0 % 400 == 0)
    }
    fn next(&self) -> Self {
        Year(self.0 + 1)
    }
}

#[derive(Debug, Clone)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Weekday {
    pub fn next(&self) -> Self {
        use self::Weekday::*;
        match *self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }
}

#[derive(Debug, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Month {
    pub fn next(&self) -> Month {
        use self::Month::*;
        match *self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => panic!("No more months"),
        }
    }
    pub fn days(&self, year: &Year) -> u64 {
        use self::Month::*;
        match *self {
            September | April | June | November => 30,
            February => if year.is_leap() { 29 } else { 28 },
            January | March | May | July | August | October | December => 31,
        }
    }
}


#[derive(Debug, Clone)]
struct Date {
    day: u64,
    year: Year,
    month: Month,
    weekday: Weekday,
}

impl Iterator for Date {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.clone();
        self.day = if self.month.days(&self.year) == self.day {
            match self.month {
                Month::December => {
                    self.year = self.year.next();
                    self.month = Month::January;
                }
                _ => self.month = self.month.next(),
            }
            1
        } else {
            self.day + 1
        };
        self.weekday = self.weekday.next();
        Some(ret)
    }
}

pub fn main() -> i64 {
    let begin = Date {
        day: 1,
        month: Month::January,
        weekday: Weekday::Monday,
        year: Year(1900),
    };

    let sundays = begin.take_while(|&Date { year: Year(year), .. }| year <= 2000)
        .filter(|date| match date {
            &Date { weekday: Weekday::Sunday, year: Year(ref year), day: 1, .. }
                if *year >= 1901 => true,
            _ => false,
        });

    sundays.count() as i64
}
