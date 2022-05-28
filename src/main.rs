#![allow(unused_mut,unused_imports,non_upper_case_globals,unused_variables,dead_code,non_camel_case_types, non_snake_case)]
extern crate rand;
extern crate libm;
extern crate image;
extern crate imageproc;
extern crate minifb;
mod filter; use filter::*;
mod layer; use layer::*;
mod cells; use cells::*;
mod misc; use misc::*;
mod typewriter; use typewriter::*;
mod setting; use setting::*;
mod fflo_aux; use fflo_aux::*;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::thread_rng;
use libm::{tanh, sin, cos, exp};
use minifb::*;
mod pixels;use pixels::*;
mod fflo;use fflo::*;
use image::{ImageBuffer, RgbImage, GrayImage, Pixel};
use rusttype::{Font, Scale};


fn main() {


	let (display_window_rows,display_window_cols) = (1000usize,1000usize);
	let mut fflo = Fflo::new(1000,1000,11,9);
	let mut paused = false;
	fflo.update_diagnostics();
	let mut display_window = Window::new("fflo", fflo.cols, fflo.rows, WindowOptions::default()).unwrap();
	//let mut control_window = Window::new("fflo controls", fflo.cols, fflo.rows, WindowOptions::default()).unwrap();
	
	
	//control_window.set_position(10,0);
	display_window.set_position((1000) as isize,0);
	display_window.set_key_repeat_delay(0.5);
	//control_window.set_key_repeat_delay(0.5);
	
	
	
	while display_window.is_open() && !display_window.is_key_down(Key::Escape) {
		display_window.update_with_buffer(&fflo.display_buffer, fflo.cols, fflo.rows).ok();
		//control_window.update_with_buffer(&fflo.control_buffer, fflo.cols, fflo.rows).ok();
		display_window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key|
			match key {	
				
				Key::P => fflo.toggle_paused(), 
				Key::R => fflo.toggle_reset_filters_and_cells(),
				Key::F => fflo.apply_filters(),
				Key::D => fflo.toggle_pixel_option(),
				Key::S => fflo.toggle_delay_option(),
				Key::C => fflo.toggle_classic_mode(),
				Key::H => fflo.toggle_help_mode(),
				_ => (),
			}
    	);
    	
    	if ! fflo.help_mode {
			if ! fflo.paused && fflo.delay_counter ==  fflo.delay_options[fflo.current_delay_option] {
				for i in 0..fflo.speed {
					fflo.apply_filters();		
				}
				fflo.delay_counter = 0;
			} else {
				fflo.delay_counter += 1;
			}
		} else {
			fflo.delay_counter = 0;
			fflo.show_welcome_screen();
		}

	}
}


fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

	
	
