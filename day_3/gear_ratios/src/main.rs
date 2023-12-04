use std::env;
use std::fs;
use std::process;

fn is_symbol(c: char) -> bool {
    return c != '.' && !char::is_numeric(c);
}

fn find_symbol_in_range(string: &str, start: usize, end: usize) -> bool {
    let binding = string.to_string();
    let symbols_in_range = binding.get(start..end+1);

    return symbols_in_range.unwrap().find(|c| is_symbol(c)).is_some();
}

fn find_adjacents(data_matrix: Vec<&str>, targets_in_row: Vec<&str>, _index: usize) -> i32 {
    let mut i = _index;
    let mut result_vector: Vec<(usize, &str)> = vec![];
    let mut result = 0;

    for target in &targets_in_row {
        let mut target_match: Vec<_> = data_matrix[i].match_indices(target).collect();
        for key in target_match {
            let search_index = i as i32;
            let mut start_point = key.0;
            let mut end_point = start_point + target.len();
            if end_point >= data_matrix[i].len()
            {
                end_point = data_matrix[i].len() - 1;
            }
            if start_point != 0 {
                start_point -= 1;
            }

            let column_above = data_matrix.get((search_index - 1) as usize);
            let column_below = data_matrix.get(i + 1);
            
            if column_above.is_some()
                && find_symbol_in_range(column_above.unwrap(), start_point, end_point)
                || find_symbol_in_range(data_matrix[i], start_point, end_point)
                || column_below.is_some()
                    && find_symbol_in_range(column_below.unwrap(), start_point, end_point)
            {
                // if a index already exists or index is inside of another number index range,
                // replace it instead of push back.
                let mut already_exists = false;
                for (j, x) in result_vector.iter().enumerate()
                {
                    if key.0 >= x.0 && key.0 <= x.0 + x.1.len()
                    {
                        if key.1.parse::<i32>().unwrap() > x.1.parse::<i32>().unwrap()
                        { 
                            result_vector[j].1 = target;
                        }
                        already_exists = true;
                        break;
                    }
                }
                if !already_exists { result_vector.push((key.0, target)); }
            }
        }
    }
    result_vector.sort_by(|a, b| a.0.cmp(&b.0));
    result_vector.dedup_by(|a,b| a.0 >= b.0 && a.0 <= b.0 + b.1.len());
    for n in result_vector { result += n.1.parse::<i32>().unwrap(); }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args : usage ./main <file_path>");
        process::exit(1);
    }
    let data = fs::read_to_string(&args[1]).expect("File open");
    let mut data_matrix: Vec<&str> = data.lines().collect();

    let mut result = 0;

    for (_index, content) in data_matrix.iter().enumerate() {
        let mut x: Vec<_> = content.split(|c| c == '.' || is_symbol(c)).collect();
        x.retain(|&x| x != "");
        x.sort_by(|a, b| a.cmp(b));
        x.dedup();
        result += find_adjacents(data_matrix.clone(), x, _index);
    }
    dbg!(result);
}
