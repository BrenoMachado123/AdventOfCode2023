use std::env;
use std::process;
use std::fs;


fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        println!("Wrong number of args : usage ./main <file_path>");
        process::exit(1);
    }
    let data = fs::read_to_string(&args[1]).expect("File open");

    let mut result = 0;
    for content in data.lines()
    {
        let mut content_splitted: Vec<_> = content.split_terminator(&[':', ';'][..]).collect();
        content_splitted.remove(0);

        let mut highest_blue: i32 = 0;
        let mut highest_red: i32 = 0;
        let mut highest_green: i32 = 0;

        for c in content_splitted {
            let parsed_stuff: Vec<_> = c.split_terminator(",").collect();
            for i in parsed_stuff {
                let number: String = i.chars().filter(|c| c.is_digit(10)).collect();
                if i.find("blue").is_some() {
                    let blue = number.parse::<i32>().unwrap();
                    if blue > highest_blue {
                        highest_blue = blue;
                    }
                }
                else if i.find("green").is_some() {
                    let green = number.parse::<i32>().unwrap();
                    if green > highest_green {
                        highest_green = green;
                    }
                }
                else if i.find("red").is_some() {
                    let red = number.parse::<i32>().unwrap();
                    if red > highest_red {
                        highest_red = red;
                    }
                }
            }
        }
        result += highest_blue * highest_green * highest_red;
    }
    dbg!(result);
}
