use std::env;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 as usize || args.len() > 3 as usize {
        eprintln!("invalid number of arguments");
        return;
    }

    let input_one = &args[1];
    let input_two = &args[2];
    println!("Reading File {:?}", input_one);

    let input_one_vec = match read_all::<i32>(input_one) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    let input_two_vec = match read_all::<i32>(input_two) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    part_one(input_one_vec.clone());
    part_two(input_two_vec.clone());

    // bootstrap::<i32>(input_one, &part_one);
}

fn part_one(input_as_num: Vec<i32>) {
    let mut increases = 0;
    let mut previous_number = 2147483647;
    for n in input_as_num.into_iter() {
        if n > previous_number {
            increases += 1
        }
        previous_number = n
    }

    println!("Increases: {}", increases)
}

fn part_two(input_as_num: Vec<i32>) {
    let mut increases = 0;
    let mut previous_sum = 2147483647;

    for n in 2..input_as_num.len() {
        let sliding_window_sum = input_as_num[n - 2] + input_as_num[n - 1] + input_as_num[n];
        if sliding_window_sum > previous_sum {
            increases += 1;
        }
        previous_sum = sliding_window_sum;
    }

    println!("Increases: {}", increases)
}

fn bootstrap<T: FromStr>(input_file_path: &str, f: &dyn Fn(&Vec<T>))
where
    <T as FromStr>::Err: Debug,
{
    let input = match read_all::<T>(input_file_path) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    f(&input)
}

fn read_all<T: FromStr>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
