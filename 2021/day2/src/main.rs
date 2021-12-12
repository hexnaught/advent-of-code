use aoc_lib::read_file_lines;
mod part_one;
mod part_two;

fn main() {
    let input_one_path = "./input/part1.txt";
    let input_two_path = "./input/part2.txt";

    if input_one_path != "" {
        let input_one_vec = match read_file_lines::<String>(input_one_path) {
            Ok(contents) => contents,
            Err(error) => panic!("Problem reading the file contents: {:?}", error),
        };
        part_one::solution::run(input_one_vec.clone());
    }

    if input_two_path != "" {
        let input_two_vec = match read_file_lines::<String>(input_two_path) {
            Ok(contents) => contents,
            Err(error) => panic!("Problem reading the file contents: {:?}", error),
        };
        part_two::solution::run(input_two_vec.clone());
    }
}
