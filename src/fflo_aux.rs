use crate::fflo::*;
use crate::filter::*;
use crate::layer::*;
use crate::pixels::*;
use crate::cells::*;
use rand::Rng;

impl Fflo<'_> {
	pub fn initialize(&mut self) {
		let option = self.pixel_options[self.current_pixel_option];
		self.cells = Cells::new(option.0,option.1);
		self.pixels = Pixels::custom_pixels(self.rows, self.cols, option.0,option.1);
	}	
	 
	pub fn apply_filters(&mut self) {
		for i in 0..self.filter.len() {
			match self.filter_target[i] {
				0   => self.filter[i].apply(&mut self.cells.r),
				1   => self.filter[i].apply(&mut self.cells.g),
				2   => self.filter[i].apply(&mut self.cells.b),
				_ => (),
			}
	
		}	
		self.cells.graph(&self.pixels, &mut self.display_buffer);
	}
	
	pub fn toggle_pixel_option(&mut self) {
		self.current_pixel_option += 1;
		if self.current_pixel_option >= self.pixel_options.len() {
			self.current_pixel_option = 0;
		}
		let option = self.pixel_options[self.current_pixel_option];
		self.cells = Cells::new(option.0,option.1);
		self.pixels = Pixels::custom_pixels(self.rows, self.cols, option.0,option.1);
		self.apply_filters();
		self.update_diagnostics();
	}
	
	pub fn toggle_new_random_cells(&mut self) {
		let option = self.pixel_options[self.current_pixel_option];
		self.cells = Cells::new(option.0,option.1);
		self.cells.binarized = self.binarized;
		self.cells.monochrome = self.monochrome;
		self.pixels = Pixels::custom_pixels(self.rows, self.cols, option.0,option.1);
		self.apply_filters();
	}
		
	pub fn toggle_delay_option(&mut self) {
		self.current_delay_option += 1;
		if self.current_delay_option >= self.delay_options.len() {
			self.current_delay_option = 0;
		}
		self.delay = self.delay_options[self.current_delay_option];
		self.delay_counter = 0;
		self.update_diagnostics();
	}
		
	pub fn toggle_max_num_filters(&mut self) {
		self.max_num_filters += 1;
		if self.max_num_filters > 21 {
			self.max_num_filters = 3;
		}
		self.reset_filters();
		self.apply_filters();
		self.update_diagnostics();
	}
	pub fn toggle_binarized(&mut self) {
		self.binarized = !self.binarized;
		self.cells.binarized = self.binarized;
		self.apply_filters();
		self.update_diagnostics();
	}
	
	pub fn toggle_monochrome(&mut self) {
		self.monochrome = ! self.monochrome;
		self.cells.monochrome = self.monochrome;
		self.apply_filters();
		self.update_diagnostics();
	}
	pub fn toggle_paused(&mut self) {
		self.delay_counter = 0;
		self.paused = ! self.paused;
		self.update_diagnostics();
	}
	pub fn toggle_help_mode(&mut self) {
		self.help_mode = !self.help_mode;
		self.delay_counter = 0;
		self.paused = false;
		self.update_diagnostics();
	}
	
	pub fn toggle_reset_filters_and_cells(&mut self) {
		let option = self.pixel_options[self.current_pixel_option];
		self.cells = Cells::new(option.0,option.1);
		self.cells.binarized = self.binarized;
		self.cells.monochrome = self.monochrome;
		self.pixels = Pixels::custom_pixels(self.rows, self.cols, option.0,option.1);
		self.reset_filters();
		self.apply_filters();
		let mut rng = rand::thread_rng();	
		self.speed = rng.gen_range(1..10);
		//self.binarized = rand::random::<bool>();
		self.update_diagnostics();
	}
	pub fn toggle_classic_mode(&mut self) {
		let option = self.pixel_options[self.current_pixel_option];
		self.cells = Cells::new(option.0,option.1);
		self.cells.binarized = self.binarized;
		self.cells.monochrome = self.monochrome;
		self.pixels = Pixels::custom_pixels(self.rows, self.cols, option.0,option.1);
		self.num_filters = 3;
		self.filter = Filter::random_filter_vector(3);
		self.filter_target = vec![0,1,2];
		self.speed = 1;
		//self.binarized = false;
		self.apply_filters();
		self.update_diagnostics();
	}
		
		
		
		 

	

		
		


}