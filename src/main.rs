#![allow(unused_variables, dead_code)]
use turtle::Turtle;
use turtle::Point;
use std::env;

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

// The bag area
struct Bag {
    origin: (f64, f64),
    side_length: f64,
    corner_angle: f64,
    orientation: f64,
}
impl Bag 
{   
    // Draw the bag
    fn draw_bag(&self, turtle: &mut Turtle) {
        turtle.pen_up();
        turtle.go_to(self.origin);
        turtle.pen_down();
        turtle.set_heading(self.orientation);
        turtle.forward(self.side_length);
        turtle.left(self.corner_angle);
        turtle.forward(self.side_length);
        turtle.left(self.corner_angle);
        turtle.forward(self.side_length);
    }

    fn teleport_center_bag(&self, turtle: &mut Turtle) {
        turtle.pen_up();
        turtle.go_to(self.get_center());
        turtle.pen_down();
    }

    // Check if turtle is inside the bag
    fn outside_bag(&self, turtle: &mut Turtle) -> bool {
        let turtle_position: Point = turtle.position();
        let turtle_x: f64 = turtle_position.x;
        let turtle_y: f64 = turtle_position.y;

        if (turtle_x < self.origin.0) | (turtle_x > (self.origin.0 + self.side_length))
         | (turtle_y < self.origin.1) | (turtle_y > (self.origin.1 + self.side_length)) {
            return true;
        } else {
            return false;
        }
    }

    // Returns the (x, y) of the center of the mag
    fn get_center(&self) -> (f64, f64) {
        let center_x: f64 = ((self.side_length - self.origin.0) / 2.0)+ self.origin.0;
        let center_y: f64 = ((self.side_length - self.origin.1) / 2.0) + self.origin.1;
        (center_x, center_y)
    }
}

// Teleport turtle to designated absolute coordinates
fn teleport(turtle: &mut Turtle, target_coord: (f64, f64)) {
    turtle.pen_up();
    turtle.go_to(target_coord);
    turtle.pen_down();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_choice = args[1].parse::<i32>().expect("Error reading path_choice");
    let distance = args[2].parse::<f64>().expect("Error reading distance");
    let coefficient = args[3].parse::<f64>().expect("Error reading coefficient");
    let active_angle: f64 = args[4].parse::<f64>().expect("Error reading active_angle");

    // Initialising
    let mut turtle: Turtle = Turtle::new();
    turtle.set_pen_color("black");
    turtle.set_pen_size(1.0);

    let origin: (f64, f64) = (0.0, 0.0);

    let turtle_bag: Bag = Bag {
        origin,
        side_length: 100.0,
        corner_angle: ANGLE_RIGHT,
        orientation: WEST
    };

    let bag_center: (f64, f64) = turtle_bag.get_center();

    turtle_bag.draw_bag(&mut turtle);
    turtle_bag.teleport_center_bag(&mut turtle);
    turtle.set_heading(WEST);

    let mut position_history: Vec<(f64, f64, bool)> = Vec::new();
    store_position_data(&mut turtle, &turtle_bag, &mut position_history);

    if path_choice == 0 {
        draw_spirale(&mut turtle, &turtle_bag, active_angle, distance, coefficient, true, &mut position_history);
    } else if path_choice == 1 {
        draw_line(&mut turtle, &turtle_bag, EAST, distance, &mut position_history)
    }

    print_position_data(position_history);

}

// Draws a straight line
fn draw_line(turtle: &mut Turtle, bag: &Bag, heading_angle: f64, distance: f64, position_history: &mut Vec<(f64, f64, bool)>) {
    while !bag.outside_bag(turtle) {
        turtle.set_heading(heading_angle);
        turtle.forward(distance);
        store_position_data(turtle, bag, position_history);
    }
}

// Draws a spirale with a the given angle
fn draw_spirale(turtle: &mut Turtle, bag: &Bag, angle: f64, distance: f64, coefficient: f64, multiply: bool, position_history: &mut Vec<(f64, f64, bool)>) {
    let mut step: f64 = distance;
    while !bag.outside_bag(turtle) {
        turtle.left(angle);
        turtle.forward(step);
        if multiply {
            step *= coefficient;
        } else {
            step += coefficient;
        }
        store_position_data(turtle, bag, position_history);
    }
}

// Stores the turtle's position data per step
fn store_position_data(turtle: &mut Turtle, bag: &Bag, position_log: &mut Vec<(f64, f64, bool)>) {
    let t_position = turtle.position();
    let t_escaped: bool = bag.outside_bag(turtle);
    position_log.push((t_position.x, t_position.y, t_escaped));
}

// Prints the data stored in the position_history vector
fn print_position_data(position_history: Vec<(f64, f64, bool)>) {
    for _i in position_history {
        println!("({} {}) {}", _i.0, _i.1, _i.2);
    }
}
