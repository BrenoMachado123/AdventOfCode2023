use std::env;
use std::process;
use std::fs;

use phf::phf_ordered_map;


type NumberMapping = phf::OrderedMap<&'static str, &'static str>;

static NUMBERS : NumberMapping = phf_ordered_map! {
    "one" => "1",
    "two" => "2",
    "three" => "3",
    "four" => "4",
    "five" => "5",
    "six" => "6",
    "seven" => "7",
    "eight" => "8",
    "nine" => "9",
};


fn get_extensive_numbers(line : &str) -> Vec<(usize, &str)>
{
    let result: String = line.to_string();
    let mut extensive_digits: Vec<(usize, &str)>= vec![];
    for (key, value) in &NUMBERS
    {
        let numbers_match : Vec<_> = result.match_indices(key).collect();
        for i in numbers_match
        {
            extensive_digits.push((i.0, *value));
        }
    }
    return extensive_digits;
}

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

    for content in data.lines() {
        let mut extensive_numbers: Vec<(usize, &str)> = get_extensive_numbers(content);
        let mut elements: Vec<_> = content.match_indices(char::is_numeric).collect();
        elements.append(&mut extensive_numbers);
        // Sort to organize numbers in the correct order
        elements.sort_by(|a, b| a.cmp(b));

        let full_digit: String = format!("{}{}", elements.first().unwrap().1, elements.last().unwrap().1);
        let num : i32 = full_digit.parse().unwrap();
        result += num;
        println!("{} -> {}", result, num);
    }
    //println!("{}", result);
}
