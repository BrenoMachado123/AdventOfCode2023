use std::env;
use std::process;
use std::fs;

use std::collections::HashSet;

fn get_scratch_result(winning_numbers : Vec<&str>, numbers_in_card : Vec<&str>) -> usize
{
    let hash_winning_numbers = winning_numbers.iter().collect::<HashSet<_>>();
    let hash_numbers_in_card = numbers_in_card.iter().collect::<HashSet<_>>();

    let common_result = hash_numbers_in_card.intersection(&hash_winning_numbers).collect::<Vec<_>>();

    return common_result.len();
}

fn distribute_results(amount_of_cards: Vec<usize>, start: usize, end: usize, distribution: usize) -> Vec<usize>
{
    // iterate to next element and distribute the scratch results
    let mut copy_amount = amount_of_cards;
    for i in start+1..end+1 {
        if copy_amount.get(i).is_some() {
            copy_amount[i] += distribution;
        }
    }
    copy_amount
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
    // every card has one instance by default
    let mut amount_of_cards: Vec<usize> = vec![1; data.lines().collect::<Vec<_>>().len()];
    for (_index, content) in data.lines().collect::<Vec<_>>().iter().enumerate()
    {
        let mut content_splitted: Vec<_> = content.split_terminator(":").collect();
        content_splitted.remove(0);

        let numbers: Vec<_> = content_splitted[0].split_terminator("|").collect();

        let winning_numbers: Vec<_> = numbers[0].trim().split_whitespace().collect();
        let numbers_in_card: Vec<_> = numbers[1].trim().split_whitespace().collect();

        let distribution = get_scratch_result(winning_numbers, numbers_in_card);

        if distribution != 0 {
            let repeats = if _index == 0 { 1 } else { amount_of_cards[_index] };
            let mut distribution_value = 0;
            for _x in 0..repeats { distribution_value += 1; }
            amount_of_cards = distribute_results(amount_of_cards, _index, _index+distribution, distribution_value);
        }
    }
    let result: u32 = amount_of_cards.iter().map(|&i| i as u32).sum();
    dbg!(result);
}
