#![allow(mixed_script_confusables)]
use mouse_rs::{types::keys::Keys, Mouse};
use std::{thread, time};

fn main() {
	
    let dur = time::Duration::from_millis(5);

    let mouse = Mouse::new();

    let (x_c, y_c) = (2881.0, 555.0); // center coordinates
    let radius   = 450.0;

    let compute_circle_coordinates = |θ_degree: f32| {
        let θ = θ_degree*std::f32::consts::PI/180.0;
        let x_p = (x_c + radius*θ.cos()) as i32;
        let y_p = (y_c + radius*θ.sin()) as i32;
        (x_p, y_p)
    };
                              
                              
    // move to starting point
    let (x, y) = compute_circle_coordinates(1.0);
    mouse.move_to(x, y).expect("Unable to move mouse");

    // start drawing
    mouse.press(&Keys::LEFT).expect("Unable to press button");
    
    for θ_degree in 2..=360 {

            let (x_p, y_p) = compute_circle_coordinates(θ_degree as f32);
            mouse.move_to(x_p, y_p).expect("Unable to move mouse");
            thread::sleep(dur);
            println!("Subscribe!");

    }

    // stop drawing
    mouse.release(&Keys::LEFT).expect("Unable to release button");

}
