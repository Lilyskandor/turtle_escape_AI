#![allow(unused_variables, dead_code)]
use turtle::Turtle;
use turtle::Point;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::str::FromStr;

const ANGLE_ZERO: f64 = 0.0;
const ANGLE_RIGHT: f64 = 90.0;
const ANGLE_LEFT: f64 = -90.0;
const ANGLE_TURN: f64 = 180.0;
const ANGLE_CIRCLE: f64 = 360.0;
const ANGLE_HALF_RIGHT: f64 = 45.0;
const ANGLE_HALF_LEFT: f64 = -45.0;

const WEST: f64 = 0.0;
const NORTH: f64 = 90.0;
const EAST: f64 = 180.0;
const SOUTH: f64 = 270.0;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Node { coords: Point, outside: bool }

// The bag area
struct Bag {
    origin: Point,
    side_length: f64,
    orientation: f64,
}
impl Bag 
{   
    // Draw the bag
    fn draw_bag(&self, turtle: &mut Turtle, corner_angle: f64) {
        turtle.pen_up();
        turtle.go_to(self.origin);
        turtle.pen_down();
        turtle.set_heading(self.orientation);
        turtle.forward(self.side_length);
        turtle.left(corner_angle);
        turtle.forward(self.side_length);
        turtle.left(corner_angle);
        turtle.forward(self.side_length);
    }

    fn teleport_center_bag(&self, turtle: &mut Turtle) {
        turtle.pen_up();
        turtle.go_to(self.get_center());
        turtle.pen_down();
    }

    // Check if turtle is inside the bag
    fn outside_bag(&self, turtle: &Turtle) -> bool {
        let turtle_position = turtle.position();

        if (turtle_position.x < self.origin.x) | (turtle_position.x > (self.origin.x + self.side_length))
         | (turtle_position.y < self.origin.y) | (turtle_position.y > (self.origin.y + self.side_length)) {
            return true;
        } else {
            return false;
        }
    }

    // Returns the (x, y) of the center of the mag
    fn get_center(&self) -> Point {
        let x = ((self.side_length - self.origin.x) / 2.0)+ self.origin.x;
        let y = ((self.side_length - self.origin.y) / 2.0) + self.origin.y;
        Point {x, y}
    }
}

// Teleport turtle to designated absolute coordinates
fn teleport(turtle: &mut Turtle, target_coord: (f64, f64)) {
    turtle.pen_up();
    turtle.go_to(target_coord);
    turtle.pen_down();
}

fn main() {
    // Default values
    let mut pattern = 0;
    let mut step_length = 10.;
    let mut change_coefficient = 1.5;
    let mut turning_angle = 45.;
    let mut log_file_path = "turtle.log";

    // Checking if data has been given in args
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 {
        pattern = args[1].parse::<i32>().expect("Error reading pattern");
        step_length = args[2].parse::<f64>().expect("Error reading step_length");
        change_coefficient = args[3].parse::<f64>().expect("Error reading change_coefficient");
        turning_angle = args[4].parse::<f64>().expect("Error reading turning_angle");
        log_file_path = &args[5];
    }
    
    // Removing Mutability
    let pattern = pattern;
    let step_length = step_length;
    let change_coefficient = change_coefficient;
    let turning_angle = turning_angle;
    let log_file_path = log_file_path;

    println!("Running with pattern N°{} with a step {}u long, a coefficient of {} and a turning angle of {}°.", &pattern, &step_length, &change_coefficient, &turning_angle);
    println!("Log file path: {}", &log_file_path);

    // Initialise the history of positions the turtle will go through
    let mut position_history = read_position_data(&log_file_path);

    println!("Current data in log file <=");
    print_position_data(&position_history);
    println!("=> End of data in log file.");
    
    // Initialising the Turtle
    let mut turtle: Turtle = Turtle::new();
    turtle.set_pen_color("black");
    turtle.set_pen_size(1.0);

    // Creating the Turtle's bag
    let turtle_bag: Bag = Bag {
        origin: Point {x: 0., y: 0.},
        side_length: 100.0,
        orientation: WEST
    };

    // Drawing the bag and placing the turtle inside of it
    turtle_bag.draw_bag(&mut turtle, ANGLE_RIGHT);
    turtle_bag.teleport_center_bag(&mut turtle);
    turtle.set_heading(WEST);

    // Add the starting point
    store_position_data(&mut turtle, &turtle_bag, &mut position_history);

    // Choosing which premade pattern algorithm to follow
    if pattern == 0 {
        draw_spirale(&mut turtle, &turtle_bag, turning_angle, step_length, change_coefficient, true, &mut position_history);
    } else if pattern == 1 {
        draw_line(&mut turtle, &turtle_bag, EAST, step_length, &mut position_history)
    }

    // Print the data we got so far
    print_position_data(&position_history);

    // Write the history of positions into a file (turtle.log)
    write_position_data(&position_history, &log_file_path);

    println!("== END ==");

}

// Draws a straight line
fn draw_line(turtle: &mut Turtle, bag: &Bag, heading_angle: f64, step_length: f64, position_history: &mut Vec<Node>) {
    while !bag.outside_bag(turtle) {
        turtle.set_heading(heading_angle);
        turtle.forward(step_length);
        store_position_data(turtle, bag, position_history);
    }
}

// Draws a spirale with a the given angle
fn draw_spirale(turtle: &mut Turtle, bag: &Bag, angle: f64, step_length: f64, change_coefficient: f64, multiply: bool, position_history: &mut Vec<Node>) {
    let mut step: f64 = step_length;
    while !bag.outside_bag(turtle) {
        turtle.left(angle);
        turtle.forward(step);
        if multiply {
            step *= change_coefficient;
        } else {
            step += change_coefficient;
        }
        store_position_data(turtle, bag, position_history);
    }
}

// Stores the turtle's position data per step
fn store_position_data(turtle: &mut Turtle, bag: &Bag, position_log: &mut Vec<Node>) {
    let coords = turtle.position();
    let outside: bool = bag.outside_bag(turtle);
    position_log.push(Node { coords, outside });
}

// Prints the data stored in the position_history vector
fn print_position_data(position_history: &Vec<Node>) {
    for _i in position_history {
        println!("{:?}", _i);
    }
}

// Writes the history of positions into a file
fn write_position_data(position_history: &Vec<Node>, log_file_path: &str) {

    let file_name = log_file_path;

    // Create a new file, smash it if it already exists
    let mut log_file = File::create(&file_name).unwrap();
    
    // Build up the data into a buffer, which will then write everything at once when we hit a .flush()
    let mut writer = BufWriter::new(&mut log_file);
    for _i in position_history {
        writer.write(format!("{} {} {}\n", _i.coords.x, _i.coords.y, _i.outside).as_bytes()).unwrap();
    }

    // The .flush() but we horrifingly forces the eventual error to be ignored by using unwrap(), bad
    writer.flush().unwrap();

    println!("> Position history has been written in file: {}", file_name);
}

// Supposedly reads the data written in the file turtle.log. Panics if anything goes wrong
fn read_position_data(log_file_path: &str) -> Vec<Node>{

    let mut loaded_position_history = Vec::<Node>::new();

    let file_name = log_file_path;
    println!("> Reading data from file: {}", &file_name);

    // Open a file and read it, if it doesn't exist, then, it just returns an empty array
    let log_file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => return loaded_position_history,
    };

    let reader = BufReader::new(&log_file);
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        
        let mut elements = Vec::<&str>::new();
        for element in line.split_whitespace() {
            elements.push(element);
        }
        let coords = Point { x: f64::from_str(elements[0]).unwrap(), y: f64::from_str(elements[1]).unwrap() };
        let outside = bool::from_str(elements[2]).unwrap();

        loaded_position_history.push(Node { coords, outside});
    }

    return loaded_position_history;
}
