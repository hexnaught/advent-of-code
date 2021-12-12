#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn move_forward(&mut self, positions: i32) {
        self.x = self.x + positions;
    }
    fn move_up(&mut self, positions: i32) {
        self.y = self.y - positions;
    }
    fn move_down(&mut self, positions: i32) {
        self.y = self.y + positions;
    }
}

pub fn run(input: Vec<String>) {
    let mut current_position = Coordinate { x: 0, y: 0 };

    for movement in input.iter() {
        let current_move: Vec<&str> = movement.split_whitespace().collect();
        let move_amount = current_move[1].parse().unwrap_or(0);

        match current_move[0] {
            "up" => current_position.move_up(move_amount),
            "down" => current_position.move_down(move_amount),
            "forward" => current_position.move_forward(move_amount),
            &_ => println!("Unknown Move: {}:{}", current_move[0], move_amount),
        }
    }

    println!(
        "[PART_1] Final Position: {:?} - Answer: {}",
        current_position,
        current_position.x * current_position.y
    )
}
