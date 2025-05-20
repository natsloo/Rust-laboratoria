#[derive(Debug)]
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
    December
}

impl Month {
    fn from_u8(value: u8) -> Option<Month> { // liczba u8 na wariant enuma
        match value {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
    fn to_u8(&self) -> u8 {         // wariant enuma na liczbę
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}
#[derive(Debug)]
struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>
}

impl Date {
    fn from_3(day: u8, month: Month, year: u16) -> Date {
        Date {
            day,
            month,
            year,
            time: None,
        }
    }
    fn from_3_with_time(day: u8, month: Month, year: u16, time: Time) -> Date {
        Date {
            day,
            month,
            year,
            time: Some(time),
        }
    }
    fn from_string(string: &str, delim: char) -> Date {
        let data: Vec<&str> = string.split(delim).collect();        // split po znaku do vectora
        let day = data[0].parse::<u8>().unwrap_or(0);    // przerabia str na podany typ i rozpakowywuje
        let month = data[1].parse::<u8>().unwrap_or(0);  // numer miesiąca
        let year = data[2].parse::<u16>().unwrap_or(0); // str na u16
        Date {
            day,
            month : Month::from_u8(month).unwrap_or(Month::January), // numer miesiąca na wariant enuma
            year,
            time: None
        }
    }
    fn from_string_with_time(date_str: &str, time_str: &str, date_delim: char, time_delim: char) -> Date {
        let data: Vec<&str> = date_str.split(date_delim).collect();
        let day = data[0].parse::<u8>().unwrap_or(0);
        let month = data[1].parse::<u8>().unwrap_or(0);
        let year = data[2].parse::<u16>().unwrap_or(0);
        let godzina: Vec<&str> = time_str.split(time_delim).collect();
        let hour = godzina[0].parse::<u8>().unwrap_or(0);
        let min = godzina[1].parse::<u8>().unwrap_or(0);
        let sec = godzina[2].parse::<u8>().unwrap_or(0);
        let t = Time {
            hour,
            min,
            sec
        };
        Date {
            day,
            month : Month::from_u8(month).unwrap_or(Month::January),
            year,
            time: Some(t)
        }
    }
    fn to_string(&self) -> String {
        let data = format!("{:02}.{:02}.{}",self.day,self.month.to_u8(),self.year);
        match &self.time {
            Some(time) => format!("{} {}", data, time.to_string()),
            None => data,
        }
    }
    fn has_time(&self) -> bool {
        self.time.is_some()
    }
    fn set_time(&mut self, time: Time){
        self.time = Some(time);
    }
    fn clear_time(&mut self){
        self.time = None;
    }
}

#[derive(Debug, Clone)]
struct Time {
    hour: u8,
    min: u8,
    sec: u8,
}

impl Time {
    fn from_3(hour: u8, min: u8, sec: u8) -> Time {
        Time {
            hour,
            min,
            sec,
        }
    }
    fn from_string(string: &str, delim: char) -> Time {
        let godzina: Vec<&str> = string.split(delim).collect();
        let hour = godzina[0].parse::<u8>().unwrap_or(0);
        let min = godzina[1].parse::<u8>().unwrap_or(0);
        let sec = godzina[2].parse::<u8>().unwrap_or(0);
        Time {
            hour,
            min,
            sec
        }
    }
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}",self.hour,self.min,self.sec)
    }
}


fn main() {
    let mut d = Date::from_string("14.05.2025", '.');
    println!("{}", d.to_string()); // bez czasu

    let t = Time::from_3(12, 30, 45);
    d.set_time(t.clone());
    println!("{}", d.to_string()); // z czasem

    d.clear_time();
    println!("{}", d.to_string()); // znowu bez czasu

    let data = Date::from_3_with_time(14,Month::from_u8(5).unwrap(),2025,t);
    println!("{}", data.to_string());
}