#![allow(unused_variables, dead_code)]
use turtle::Turtle;
use turtle::Point;
use std::env;
use std::fs;
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

#[derive(Copy, Clone, PartialEq)]
enum Pattern {
    Line,
    Spirale,
}
impl Pattern {
    fn to_str(pattern: Pattern) -> String {
        match pattern {
            Pattern::Line => "Line".to_string(),
            Pattern::Spirale => "Spirale".to_string(),
        }
    }

    fn to_pattern(pattern: i32) -> Pattern {
        match pattern {
            0 => Pattern::Line,
            1 => Pattern::Spirale,
            _ => Pattern::Line,
        }
    }
}

// Convenient structure to store what we actually care about
#[derive(Copy, Clone, Debug, PartialEq)]
struct Node { coords: Point, outside: bool }

struct Options {
    pattern: Pattern,
    step_length: f64,
    change_coefficient: f64,
    angle: f64,
    direction: f64,
    log_file_path: String,
}
impl Options {
    fn print_options (&self) {
        println!("Options:");
        println!("\tPattern {}\n\tStep_Length {}\n\tCoefficient {}\n\tAngle {}\n\tDirection {}\n\tLogFile {}"
        , Pattern::to_str(self.pattern), self.step_length, self.change_coefficient, self.angle, self.direction, self.log_file_path);
    }
}

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
    // Default Options values
    let mut options = Options {
        pattern: Pattern::Line,
        step_length: 10.,
        change_coefficient: 1.5,
        angle: 45.,
        direction: WEST,
        log_file_path: format!("turtle.log"),
    };

    // Update options if given in args
    let args: Vec<String> = env::args().collect();
//    let error_message = format!("Usage: <i32>Pattern_Number <f64>Step_Length <f64>Change_Coefficient <f64>Angle <String>Log_File_Path");
    if args.len() == 2 {
        println!("Usage: LogFile [0:Line ; 1:Spirale] StepsLength [DirectionAngle ; TurningAngle, Coefficient]");
    } else if args.len() > 2 {
        let mut error_message = format!("Usage: <String>Log_File_Path");
        options.log_file_path = args[1].clone();

        error_message += " <i32>Pattern_Number[0==Line, 1==Spirale]";
        options.pattern = Pattern::to_pattern(args[2].parse::<i32>().expect(&error_message));

        error_message += " <f64>Steps_Length";
        options.step_length = args[3].parse::<f64>().expect(&error_message);
    
        if options.pattern == Pattern::Line {
            error_message += " <f64>Direction_Angle[0.0==WEST, 90.0==North]";
            options.direction = args[4].parse::<f64>().expect(&error_message);

        } else if options.pattern == Pattern::Spirale {
            error_message += " <f64>Turning_Angle";
            options.angle = args[4].parse::<f64>().expect(&error_message);

            error_message += " <f64>Coefficient_Multiplier";
            options.change_coefficient = args[5].parse::<f64>().expect(&error_message);            
        }
    }

    // Remove mutability from the options
    let options = options;

    options.print_options();

    let previous_position_history = read_position_data(&options.log_file_path);
    println!("Current data in log file <=");
    print_position_data(&previous_position_history);
    println!("=> End of data in log file.");

    // Initialise the history of positions the turtle will go through
    let mut position_history = Vec::<Node>::new();
    
    // Initialising the Turtle
    let mut turtle: Turtle = Turtle::new();
    turtle.set_pen_color("black");
    turtle.set_pen_size(1.0);

    // Creating the Turtle's bag
    let turtle_bag = Bag {
        origin: Point {x: 0., y: 0.},
        side_length: 100.0,
        orientation: WEST
    };

    // Drawing the bag and placing the turtle inside of it
    turtle_bag.draw_bag(&mut turtle, ANGLE_RIGHT);
    turtle_bag.teleport_center_bag(&mut turtle);
    turtle.set_heading(options.direction);

    // Add the starting point
    store_position_data(&mut turtle, &turtle_bag, &mut position_history);

    draw_something(&mut turtle, &turtle_bag, &options, &mut position_history);

    println!("===============");
    println!("New Data written in: {} <=", &options.log_file_path);
    // Print the data we got so far
    print_position_data(&position_history);

    // Write the history of positions into a file (turtle.log)
    write_position_data(&position_history, &options.log_file_path);

    println!(">= Data written.");
    println!("== END OF EXECUTION ==");

}

// Draws according to the drawing algorithm decided in the options
fn draw_something(turtle: &mut Turtle, bag: &Bag, options: &Options, position_history: &mut Vec<Node>) -> bool {
    let mut did_i_leave = bag.outside_bag(turtle);
    let mut distance = options.step_length;
    while !did_i_leave {
        match options.pattern {
            Pattern::Line => turtle.forward(distance),
            Pattern::Spirale => {
                turtle.left(options.angle);
                turtle.forward(distance);
                distance = distance * options.change_coefficient;              
            },
        };
        did_i_leave = bag.outside_bag(turtle);
        store_position_data(turtle, bag, position_history);
    }
    did_i_leave
}

// Stores the turtle's position data per step
fn store_position_data(turtle: &mut Turtle, bag: &Bag, position_history: &mut Vec<Node>) {
    let coords = turtle.position();
    let outside = bag.outside_bag(turtle);
    position_history.push(Node { coords, outside });
}

// Prints the data stored in the position_history vector
fn print_position_data(position_history: &Vec<Node>) {
    for _i in position_history {
        println!("{:?}", _i);
    }
}

// If it exists, rename the previous log file by adding a ".bak" at the end of it
fn backup_logfile(log_file_path: &str)
{
    // Check if file already exists, and if it does, toss it aside to avoid apending
    if std::path::Path::new(log_file_path).exists() {
        fs::rename(log_file_path, log_file_path.to_owned() + ".bak").unwrap();
    }
}

// Writes the history of positions into a log file
fn write_position_data(position_history: &Vec<Node>, log_file_path: &str) {

    backup_logfile(log_file_path);

    let file_name = log_file_path;

    // Create a new file
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

// Reads the data written in the given log file. Panics if anything goes wrong
fn read_position_data(log_file_path: &str) -> Vec<Node>{

    let mut loaded_position_history = Vec::<Node>::new();

    let file_name = log_file_path;
    println!("> Reading data from file: {}", &file_name);

    // Open a file and read it, if it doesn't exist, then, it just returns an empty array
    let log_file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => return loaded_position_history,
    };

    // The reader buffer, handles all readings in one go (system wise) to gain speeeeed
    let reader = BufReader::new(&log_file);
    // Iterating over the obtained lines
    for line_result in reader.lines() {
        // Check every reading, and if an error is met: PANICS
        let line = line_result.unwrap();
        
        let mut elements = Vec::<&str>::new();
        // Split along the whitespaces and push everything into a convenient vector
        for element in line.split_whitespace() {
            elements.push(element);
        }
        // Convert the elements into coords and outside boolean. Panics if any conversion goes wrong
        let coords = Point { x: f64::from_str(elements[0]).unwrap(), y: f64::from_str(elements[1]).unwrap() };
        let outside = bool::from_str(elements[2]).unwrap();

        // Push our new loaded Node to our collection of Nodes
        loaded_position_history.push(Node { coords, outside});
    }

    return loaded_position_history;
}
