#![allow(unused_variables, dead_code)]
use turtle::Turtle;
use turtle::Point;

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
    fn inside_bag(&self, turtle: &mut Turtle) -> bool {
        let turtle_position: Point = turtle.position();
        let turtle_x: f64 = turtle_position.x;
        let turtle_y: f64 = turtle_position.y;

        if (turtle_x < self.origin.0) | (turtle_x > (self.origin.0 + self.side_length))
         | (turtle_y < self.origin.1) | (turtle_y > (self.origin.1 + self.side_length)) {
            return false;
        } else {
            return true;
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
    turtle_bag.draw_bag(&mut turtle);
    turtle_bag.teleport_center_bag(&mut turtle);

    turtle.set_heading(WEST);


    let bag_center: (f64, f64) = turtle_bag.get_center();

    let path_choice: i32 = 0;

    let distance: f64 = 1.0;
    let coefficient: f64 = 1.02;
    let active_angle: f64 = 120.0;

    if path_choice == 0 {
        draw_spirale(&mut turtle, &turtle_bag, active_angle, distance, coefficient, true);
    } else if path_choice == 1 {
        draw_line(&mut turtle, &turtle_bag, EAST, distance)
    }

}

fn draw_line(turtle: &mut Turtle, bag: &Bag, angle: f64, distance: f64) {
    while bag.inside_bag(turtle) {
        turtle.set_heading(angle);
        turtle.forward(distance);
    }
}

fn draw_spirale(turtle: &mut Turtle, bag: &Bag, angle: f64, distance: f64, coefficient: f64, multiply: bool) {
    let mut step: f64 = distance;
    while bag.inside_bag(turtle) {
        turtle.left(angle);
        turtle.forward(step);
        if multiply {
            step *= coefficient;
        } else {
            step += coefficient;
        }
    }
}
