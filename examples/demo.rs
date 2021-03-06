extern crate nest;

use nest::*;
use std::f32::consts::PI;
use std::time::Instant;

fn main() {
    let mut app = Window::new("Demo", 640, 480).expect("error: failed to open window");

    let start = Instant::now();

    // Load the petal texture.
    let petal_texture = app.load_image("examples/petal.png").unwrap();
    // Create an image rectangle from the petal texture with a width of 0.4 and proportional height.
    let petal = image_w(petal_texture, 0.4).translate([0.3, 0.0]);

    // Create flower from 6 petals rotated around the center.
    let flower = (0usize..6)
        .flat_map(|i| petal.rotate(i as f32 / 6.0 * 2.0 * PI))
        .collect::<Vec<_>>();

    loop {
        // Handle events.
        for event in app.poll_events() {
            match event {
                // Close if they close the window or hit escape.
                Event::Closed | Event::KeyboardInput(KeyState::Pressed, Some(Key::Escape)) => {
                    return
                }
                // Print "Space!" if they hit space.
                Event::KeyboardInput(KeyState::Pressed, Some(Key::Space)) => println!("Space!"),
                _ => {}
            }
        }

        // Draw the flower rotating at 1 rad/sec.
        app.draw(flower.rotate(start.elapsed().to_secs()));
    }
}
