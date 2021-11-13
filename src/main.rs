use std::io;
use std::io::prelude::*;
extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(Debug, PartialEq, Eq, FromPrimitive, Copy, Clone)]
enum Day {
    MONDAY = 0,
    TUESDAY = 1,
    WEDNESDAY = 2,
    THURSDAY = 3,
    FRIDAY = 4,
    SATURDAY = 5,
    SUNDAY = 6,
    UNKNOWN
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const YEAR2000: Year = Year{doomsday: Day::TUESDAY, year: 2000};

impl Day {
    fn next_day(self, offset: i128) -> Day {
        let x = num::FromPrimitive::from_u8((self as u8 + (offset%7+7) as u8)%7);
        return match x {
            Some(a) => a,
            None => Day::UNKNOWN
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Year {
    doomsday: Day,
    year: i128
}

impl Year {
    fn alternating_next_year(year: &mut i128, last_was_negative: &mut bool) {
        if *last_was_negative {
            *year += 1;
        } else {
            *year -= 1;
        }
        *last_was_negative = !*last_was_negative;
    }
    fn next_doomsday(self, reference_year: Option<Year>) -> Day {
        let ref_year = reference_year.unwrap_or(YEAR2000);
        let mut year = ref_year.year;
        let mut day = ref_year.doomsday;
        
        if year == self.year {
            return day;
        }

        let offset = if year < self.year {1} else {-1};
        
        loop {
                year += offset;
                if Year::is_leap_year(year) {
                    day = day.next_day(offset*2);
                } else {
                    day = day.next_day(offset);
                }
                if year == self.year {
                    return day;
                }
        }
    }
    fn is_leap_year(year: i128) -> bool {
        if year % 4 != 0 { 
            return false;
        }
        if year % 100 != 0 {
            return true;
        } 
        return year % 400 == 0;
    }
}
struct Inputs {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    input: String
}
fn get_first_year(inputs: &mut Inputs) -> i128 {
    loop {
        println!("Enter the start year.");
        print!("> ");
        io::stdout().flush().unwrap();
        inputs.input.clear();
        inputs.stdin.read_line(&mut inputs.input).unwrap();
        let result = inputs.input.trim().parse::<i128>();
        if result.is_ok() {return result.unwrap()};
        println!("Start year must be an integer. Try again.");
        println!();
    }
}

fn get_last_year(inputs: &mut Inputs) -> i128 {
    loop {
        println!("Enter the last year.");
        print!("> ");
        io::stdout().flush().unwrap();
        inputs.input.clear();
        inputs.stdin.read_line(&mut inputs.input).unwrap();
        let result = inputs.input.trim().parse::<i128>();
        if result.is_ok() {return result.unwrap()};
        println!("Last year must be an integer. Try again.");
        println!();
    }
}

fn get_years(inputs: &mut Inputs) -> (i128, i128) {
    loop {
        let first_year = get_first_year(inputs);
        let last_year = get_last_year(inputs);
        if first_year >= last_year {
            println!("First year must be strictly bigger than last year. Try again.");
            println!();
            continue;
        }

        let first_year_doomsday = YEAR2000.doomsday.next_day(1);
        return (first_year, last_year);
    }
}

struct State {
    years: Vec<Day>,
    first_year: i128,
    last_year: i128,
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    input: String
}



impl State {
    fn new(years: Vec<Day>, first_year: i128, last_year: i128, inputs: Inputs) -> Self {
        return State{years, first_year, last_year, stdin: inputs.stdin, stdout: inputs.stdout, input: inputs.input};
    }
    fn main(mut self) {
        println!("{:?}", self.years);
        println!("{}", self.years.len());
        println!("{}", self.years.capacity());
        println!("{} - {}", self.first_year, self.last_year);
        
        self.set_year(self.last_year, Day::WEDNESDAY);
        println!("{}", self.get_year(self.last_year));
        println!("{:?}", self.years);
        
    }

    fn get_year(&self, year: i128) -> Day {
        return self.years[(year-self.first_year) as usize];
    }

    fn set_year(&mut self, year: i128, day: Day) {
        self.years[(year-self.first_year) as usize] = day;
    }
}
fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let input = String::new();
    let mut inputs = Inputs{stdin, stdout, input};
    let first_year;
    let last_year;
    let mut years: Vec<Day>;
    {
        let inputs_ref = &mut inputs;

        let (first_year_, last_year_) = get_years(inputs_ref);
        last_year = last_year_;
        first_year = first_year_;
        years = Vec::with_capacity((last_year-first_year+1) as usize);
        for _ in first_year..last_year+1 {
            years.push(Day::UNKNOWN);
        }
    }
    return State::new(years, first_year, last_year, inputs).main();
}
