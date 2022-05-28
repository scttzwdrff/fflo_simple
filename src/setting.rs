use minifb::*;
use crate::typewriter::*;
use image::{ImageBuffer, RgbImage, GrayImage, Pixel};

pub struct Setting {
	pub desc : String,
	pub val : usize,
	pub code :  char,
	pub max_filter : usize,
	pub max_filters : usize,
	pub max_delay : usize,
}

impl Setting {
	pub fn new() -> Setting {
		Setting { desc : "unassigned".to_string() , val : 0, code : 'u', max_filter : 6, max_filters : 6, max_delay : 10 }
	}
	pub fn blank_settings(n : usize) -> Vec<Setting> {
		let mut output : Vec<Setting> = vec![];
		for _ in 0..n {
			output.push(Setting::new());
		}
		output
	}
	
	

	
}

	

	
		

		