use std::{fs::File, io::Write};

fn main() {
    write_fuzz_buzz(50).unwrap()
}

fn write_fuzz_buzz(number: u8) -> std::io::Result<()> {
    let mut file = File::create("fizzbuzz.txt")?;

    for n in 1..number {
        if n % 15 == 0 {
            file.write(b"FizzBuzz\n")?;
        } else if n % 5 == 0 {
            file.write(b"Buzz\n")?;
        } else if n % 3 == 0 {
            file.write(b"Fizz\n")?;
        } else {
            file.write_fmt(format_args!("{}\n", n))?;
        }
    }

    file.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file() -> String {
        return std::fs::read_to_string("fizzbuzz.txt").unwrap();
    }

    fn read_lines(content: &String) -> Vec<&str> {
        let lines: Vec<&str> = content.split("\n").collect();
        return lines
    }

    #[test]
    fn it_should_write_fizz() {
        write_fuzz_buzz(10).unwrap();
        let file = read_file();
        let lines = read_lines(&file);
        assert_eq!(lines[2], "Fizz"); // # 3 is multiple of 3
        assert_eq!(lines[8], "Fizz"); // # 9 is multiple of 9
    }

    #[test]
    fn it_should_write_buzz() {
        write_fuzz_buzz(11).unwrap();
        let file = read_file();
        let lines = read_lines(&file);
        assert_eq!(lines[4], "Buzz"); // # 5 is multiple of 5
        assert_eq!(lines[9], "Buzz"); // # 10 is multiple of 5
    }

    #[test]
    fn it_should_write_fizzbuzz() {
        write_fuzz_buzz(31).unwrap();
        let file = read_file();
        let lines = read_lines(&file);
        assert_eq!(lines[14], "FizzBuzz"); // # 15 is multiple of 15
        assert_eq!(lines[29], "FizzBuzz"); // # 30 is multiple of 15
    }
}

