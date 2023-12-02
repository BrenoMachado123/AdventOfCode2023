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
        let game_id: String = content_splitted.get(0).unwrap().chars().filter(|c| c.is_digit(10)).collect();
        content_splitted.remove(0);
        let mut blue: i32 = 0;
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut is_game_possible : bool = true;
        for c in content_splitted {
            let parsed_stuff: Vec<_> = c.split_terminator(",").collect();
            for i in parsed_stuff {
                let number: String = i.chars().filter(|c| c.is_digit(10)).collect();
                if i.find("blue").is_some() {
                    blue = number.parse::<i32>().unwrap();
                }
                else if i.find("green").is_some() {
                    green = number.parse::<i32>().unwrap();
                }
                else if i.find("red").is_some() {
                    red = number.parse::<i32>().unwrap();
                }
            }
            is_game_possible = blue <= 14 && green <= 13 && red <= 12;
            if !is_game_possible {
                break;
            }
        }
        if is_game_possible {
            result += game_id.parse::<i32>().unwrap();
        }
    }
    dbg!(result);
    //let mut result = 0;
}
