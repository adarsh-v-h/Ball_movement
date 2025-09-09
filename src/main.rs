use macroquad::prelude::*;
// The library contains alot of tools we need, * means we are importing all those,
// by doing so we dont have to write prelude::tool_name everytime, we can just write tool_name 
struct Ball{ // crateing a ball, and defining all the properties a ball might need to be drawn
    x: f32, // horizontal postion
    y: f32, // vertical postion
    radius: f32, // size of the ball
    //velocity: f32 // speed, if this is positive it goes down, if negative goes up
    velocity_x: f32,
    velocity_y: f32,// creating sepearte velocites for both axes
    trail: Vec<(f32, f32)> // creating a vector which will have tuples of 2 values of type of f32
}
impl Ball{ // we are adding the properties to the ball
    fn new() -> Self{ // this is a constructure, creates a new ball with starting values
        Self { 
            x: screen_width()/2.0,
            y: 50.0, // 50 pixels from top
            // screen_width/2.0= in the mide of screen
            radius: 15.0,
        // velocity: 0.0, // not moving yet
        velocity_x: 0.0,
        velocity_y: 0.0,
        trail: Vec::new() // initalize and empty vector 
        }
    }
    fn draw(&self){
        // now we are adding trails to the ball
        for (index, &position) in self.trail.iter().enumerate(){
            let (trail_x, trail_y) = position;
            if self.trail.len() >0{
                let alpha = 1.0 - (index as f32 / self.trail.len() as f32) * 0.5;
                let clr = Color::new(   1.0,0.0,0.0, alpha); // the parameters are, R, G, B, A
                // thats just Red blue and green with A = transperency 
                draw_circle(trail_x, trail_y, self.radius*0.8, clr);// drawing a circle with these propersties
            }
        } // so now we wont just the exact position of the ball but also the previous position of it,
        draw_circle(self.x, self.y, self.radius, RED);// drawing a circle with these propersties
        
    }
    fn update(&mut self){ // updating the ball's physics
        // &mut self allows it to modify self
        self.trail.push((self.x, self.y)); // push adds the tuple of current positon (x,y) into the trail vector
        if self.trail.len() > 6{ // we dont want to keep all the old positions, so we set a limit
            self.trail.remove(0);
        } // this will remove the oldest postion, i.e at index: 0
        self.velocity_y += 0.5; // gravity acceleration
        self.velocity_x *= 0.9;
        self.y += self.velocity_y; // update the possiton based on velocity
        self.x += self.velocity_x;
        // the if checks if the ball hits the ground
        if self.y > screen_height() -self.radius{
            self.y = screen_height() - self.radius; // makes sure the ball doesn't goes in the ground
            self.velocity_y = -self.velocity_y * 0.8; //Bonce with energy loss
        }
        if self.y < self.radius{
            self.y = self.radius;
            self.velocity_y = 0.0; 
        }
        // now we have to also make sure it doesnt crosses the wall
        if self.x < self.radius{
            self.x = self.radius;
            self.velocity_x = -self.velocity_x * 0.8;
        }
        if self.x > screen_width() - self.radius{
            self.x = screen_width() - self.radius;
            self.velocity_x = -self.velocity_x * 0.8;
        }
    }
    // add a function which will jump the ball, make the ball move up with certain verlocity
    fn jump(&mut self){
        self.velocity_y = -15.0; // ball starts moving up with -15.0, 
    }
    // horizontal movement
    fn move_left(&mut self){
        self.velocity_x = -5.0;
    }
    fn move_right(&mut self){
        self.velocity_x = 5.0;
    }
}
#[macroquad::main("Gravity Simulation")]
async fn main() {
    // This creates a window with the title "Gravity Simulation"
    let mut ball = Ball::new();
    // mut is imp, cause we need to the constant
    loop{ // this run 60 times per second
        // will run until we close the window.
        clear_background(BLACK); // clear background screen with a color
        // Check if the spacebar is pressed to make the ball jump
        if is_key_pressed(KeyCode::Space){
            ball.jump();
        }// if the key is pressed, we will execute the jump func
        if is_key_down(KeyCode::A){ // is_key_down checks if the key is still pressed
            ball.move_left();
        }
        if is_key_down(KeyCode::D){
            ball.move_right();
        }
        ball.update(); // update physics
        ball.draw(); // Draw the ball
        // lets draw boundries,
        //Syntax: draw_line(initial_x, initial_y, final_x, final_y, thinkness_of_line, Color_of_line)
        draw_line(0.0,0.0, screen_width(),0.0, 2.0, BLUE );// the celing 
        draw_line(0.0, screen_height(), screen_width(), screen_height(), 2.0, BLUE);// floor
        draw_line(0.0, 0.0, 0.0, screen_height(), 2.0, BLUE);// left wall
        draw_line(screen_width(), 0.0, screen_width(), screen_height(), 2.0, BLUE);
        // showing info
        // synatx:
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE); // draws text on screen
        draw_text(&format!("vertical Velcoity {:.2}", ball.velocity_y), 10.0, 45.0, 20.0, WHITE);
        draw_text(&format!("Horizontal Velcoity: {:.2}", ball.velocity_x), 10.0, 70.0, 20.0, WHITE);
        draw_text(&format!("Positon: {:.2}", ball.y), 10.0, 95.0, 20.0 ,WHITE);  
        // draw_text takes, a string value and display it, and the posit
        //draw_text(&format!(" Press SPACE to jump "), 5.0, 120.0, 20.0, LIGHTGRAY);// to show the option to jump
        next_frame().await; // waits for the next fps frame
    }
}
