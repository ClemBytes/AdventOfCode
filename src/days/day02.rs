use std::fs;

pub fn run() {
    println!("------- DAY02 -------");
    let input = fs::read_to_string("inputs/input_day02")
        .expect("Unable to read input!");
    day02_part1(&input);
    day02_part2(&input);
}

fn day02_part1(_input: &String) {

}

fn day02_part2(_input: &String) {
    
}

struct Box {
    length: u32,
    width: u32,
    height: u32,
}

impl Box {
    fn get_side_areas(&self) -> Vec<u32> {
        vec![
            &self.length * &self.width,
            &self.length * &self.height,
            &self.width  * &self.height,
        ]
    }
    
    fn smallest_side_area(&self) -> u32 {
        &self.get_side_areas().iter().min().unwrap()
    }
}