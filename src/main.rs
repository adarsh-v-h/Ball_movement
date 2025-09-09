use macroquad::prelude::*;
// The library contains alot of tools we need, * means we are importing all those,
// by doing so we dont have to write prelude::tool_name everytime, we can just write tool_name 
struct Ball{ // crateing a ball, and defining all the properties a ball might need to be drawn
    x: f32, // horizontal postion
    y: f32, // vertical postion
    radius: f32, // size of the ball
    //velocity: f32 // speed, if this is positive it goes down, if negative goes up
    velocity_x: f32,
    velocity_y: f32
    // creating sepearte velocites for both axes
}
impl Ball{ // we are adding the properties to the ball
    fn new() -> Self{ // this is a constructure, creates a new ball with starting values
        Self { 
            x: screen_width()/2.0,
            y: 50.0, // 50 pixels from top
            // screen_width/2.0= in the mide of screen
            radius: 20.0,
        // velocity: 0.0, // not moving yet
        velocity_x: 0.0,
        velocity_y: 0.0
        }
    }
    fn draw(&self){
        draw_circle(self.x, self.y, self.radius, RED);
    }// drawing a circle with these propersties
    fn update(&mut self){ // updating the ball's physics
        // &mut self allows it to modify self
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
        // showing info
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE); // draws text on screen
        draw_text(&format!("vertical Velcoity {:.2}", ball.velocity_y), 10.0, 45.0, 20.0, WHITE);
        draw_text(&format!("Horizontal Velcoity: {:.2}", ball.velocity_x), 10.0, 70.0, 20.0, WHITE);
        draw_text(&format!("Positon: {:.2}", ball.y), 10.0, 95.0, 20.0 ,WHITE);  
        // draw_text takes, a string value and display it, and the posit
        draw_text(&format!(" Press SPACE to jump "), 5.0, 120.0, 20.0, LIGHTGRAY);// to show the option to jump
        next_frame().await; // waits for the next fps frame
    }
}
