#[derive(Debug)]
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
    let time = Time::from_3(5,23,56);
    println!("{}",time.to_string());
    let time1 = Time::from_string("23:45:00",':');
    println!("{:?}",time1);
    let time3 = Time::from_3(0,0,0);
    println!("{}",time3.to_string());
    let time4 = Time::from_string("0:0:0",':');
    println!("{}, {:?}",time4.to_string(), time4);
}
