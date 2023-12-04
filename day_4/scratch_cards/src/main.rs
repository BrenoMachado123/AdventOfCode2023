use std::env;
use std::process;
use std::fs;

use std::collections::HashSet;

fn get_scratch_result(winning_numbers : Vec<&str>, numbers_in_card : Vec<&str>) -> i32
{
    let hash_winning_numbers = winning_numbers.iter().collect::<HashSet<_>>();
    let hash_numbers_in_card = numbers_in_card.iter().collect::<HashSet<_>>();

    let common_result = hash_numbers_in_card.intersection(&hash_winning_numbers).collect::<Vec<_>>();
    if common_result.len() == 0
    {
        return 0;
    }
    let base: i32 = 2;
    return base.pow((common_result.len() - 1) as u32); // ignore the first value and then proceeds
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
    for content in data.lines()
    {
        let mut content_splitted: Vec<_> = content.split_terminator(":").collect();
        content_splitted.remove(0);

        let numbers: Vec<_> = content_splitted[0].split_terminator("|").collect();

        let winning_numbers: Vec<_> = numbers[0].trim().split_whitespace().collect();
        let numbers_in_card: Vec<_> = numbers[1].trim().split_whitespace().collect();

        result += get_scratch_result(winning_numbers, numbers_in_card);
    }
    dbg!(result);
}