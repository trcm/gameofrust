extern crate minifb;
extern crate rand;

use minifb::{Window, Key, Scale, WindowOptions};
use rand::{Rng};
use std::{thread, time};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const GAME_SIZE: usize = WIDTH * HEIGHT;

fn find_neighbours(index: usize, state: &[u32]) -> u32 {

    let location = cartesian(index);
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    let valid: Vec<(u32, u32)> = vec!();
    let mut neighbour_count = 0;
    for direction in directions.iter() {
        let (x, y) = *direction;
        let next = (x + location.0 as i32, y + location.1 as i32); 
        if validate_move(next) && state[point_index(next) as usize] == 1 {
            neighbour_count += 1;
        }
    }
    neighbour_count
}

fn validate_move((x, y): (i32, i32)) -> bool {
    x > -1 && x < WIDTH as i32 && y > -1 && y < WIDTH as i32
}

fn cartesian(index: usize) -> (u32, u32) {
    let x = (index / WIDTH) as u32;
    let y = (index % WIDTH) as u32;
    (x, y)
}

fn point_index((x, y): (i32, i32)) -> u32 {
    (x * WIDTH as i32 + y) as u32
}

fn generate_next_state(current: &[u32]) -> [u32; GAME_SIZE] {
    let mut next_state = [0; GAME_SIZE];
    for (i, item) in current.iter().enumerate() {
        let n = find_neighbours(i, current);
        // println!("Neighbours {}", n);
        if current[i] == 0 {
            if n == 3 {
                next_state[i] = 1
            } 
        } else {
            println!("n {}", n);
            if n < 2 {
                next_state[i] = 0
            } else if n == 2 || n == 3 {
                next_state[i] = 1
            } else if n > 3 {
                println!("KILLING SQUARE");
                next_state[i] = 0
            }
        }
    } 
    next_state
}

fn print_state(state: &[u32]) {
    for (i, item) in state.iter().enumerate() {
        if i % 10 == 0  {
            print!("\n{}", item)
        } else {
            print!("{}", item)
        }
    }
    print!("\n");
}

fn main() {

    let mut active: [u32; 100] = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    
    let mut buffer: Vec<u32> = vec![0; GAME_SIZE];

    let mut window = match Window::new("Noise Test - Press ESC to exit", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: true,
                                           scale: Scale::X32,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    let mut rng = rand::thread_rng();
    let mut ticks = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep(time::Duration::from_millis(100));
        // let mut next_state = active;

        // generate_next_state(&active, &mut next_state);
        let next_state = generate_next_state(&active);
        active = next_state;
        for (i, item) in buffer.iter_mut().enumerate() {
            if next_state[i] == 1 {
                *item = 0xffffff;
            } else {
                *item = 0x000000;
            }
        }


        print_state(&active);
        window.update_with_buffer(&buffer);
        ticks = ticks + 1;
        println!("State after {} ticks", ticks);
    }
}

