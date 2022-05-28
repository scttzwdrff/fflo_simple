use crate::filter::*;
use crate::layer::*;
use crate::cells::*;
use crate::misc::*;
use crate::typewriter::*;
use crate::pixels::*;
use image::{RgbImage,Pixel};
use rusttype::{Font, Scale};
use imageproc::drawing::{draw_text_mut, text_size, draw_filled_circle_mut};
use rand::Rng;
use imageproc::rect::Rect;
use imageproc::drawing::draw_filled_rect_mut;
use rand::random;



pub struct Fflo<'a> {
	pub rows : usize,
	pub cols : usize,
	pub display_buffer : Vec<u32>,
	pub control_buffer : Vec<u32>,
	pub control_image : RgbImage,
	pub pixels : Pixels,
	pub cells : Cells,
	pub font : Font<'a>,
	pub filter : Vec<Filter>,
	pub filter_target : Vec<usize>,
	pub num_filters : usize,
	pub delay : usize,
	pub delay_counter : usize,
	pub max_filter : usize,
	pub noise : usize,
	pub monochrome : bool,
	pub binarized : bool,
	pub paused : bool, 
	pub help_mode : bool, 
	pub delay_options : Vec<usize>,
	pub current_delay_option : usize,
	pub pixel_options : Vec<(usize,usize)>,
	pub current_pixel_option : usize,
	pub brightness : f64,
	pub max_num_filters : usize,
	pub speed : usize, 
	
}

impl Fflo<'_> {
	pub fn new( rows : usize, cols : usize, num_filters : usize, max_filter : usize) -> Fflo <'static> {
		let mut instance = Fflo {
			rows,
			cols,
			display_buffer : random_u32_vector(rows*cols),
			control_buffer : vec![0u32;rows*cols],
			control_image  : RgbImage::new(cols as u32,rows as u32),
			pixels : Pixels::default_pixels(rows, cols),
			cells : Cells::new(100,100),
			font : Fflo::loaded_font(0),
			filter : Filter::random_filter_vector(num_filters),
			filter_target : vec![0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2],
			num_filters,
			delay : 1,
			delay_counter : 0,
			max_filter ,
			noise : 0,
			monochrome : false,
			binarized  : false,
			paused : false,
			help_mode : true,
			delay_options : vec![1,11,21,31,41,51,],
			current_delay_option :  0,
			pixel_options : vec![(50,100),(25,50),(50,50),(25,100),(100,25), (100,100),(100,200),],
			current_pixel_option  : 0,
			brightness : 0.1,
			max_num_filters : 14,
			speed : 1,
			
		};
		instance.initialize();
		instance
	}
	pub fn loaded_font(code : usize) -> Font <'static> {
		let font_code =  match code {
			0 => Vec::from(include_bytes!( "cc.ttf") as &[u8]),
			1 => Vec::from(include_bytes!( "scp.ttf") as &[u8]),
			2 => Vec::from(include_bytes!( "german.ttf") as &[u8]),
			3 => Vec::from(include_bytes!( "sg.ttf") as &[u8]),
			4 => Vec::from(include_bytes!( "lr.ttf") as &[u8]),
			
			_ => Vec::from(include_bytes!( "mt.ttf") as &[u8]),
		};
		Font::try_from_vec(font_code).unwrap()
	}
	
	pub fn restore_filter_targets(&mut self) {
		self.filter_target = vec![0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2,0,1,2];
	}
		
	pub fn show_welcome_screen(& mut self) { 
		let scale = Scale { x: 40.0, y: 40.0};
		let rect = Rect::at(0, 0).of_size(1000, 1000);
		let mut rgb = [0u8;3];
		draw_filled_rect_mut(&mut self.control_image, rect, image::Rgb([0,0,0]));
		let mut rgb = [255u8;3];
		let title = "fflo".to_string();
		let scale = Scale { x: 200.0, y: 200.0};
		draw_text_mut(& mut self.control_image, image::Rgb(rgb), 20, 100, scale,&self.font,   &title);
		let mut instruction = vec!["not implemented yet".to_string();7];
		instruction[0] = format!("(h)elp");
		instruction[1] = format!("(r)andom");
		instruction[2] = format!("(c)lassic");
		instruction[3] = format!("(s)peed");
		instruction[4] = format!("(p)ause");
		instruction[5] = format!("(f)orward");
		instruction[6] = format!("(d)isplay");
		let scale = Scale { x: 60.0, y: 60.0};
		for i in 0..instruction.len() {
			draw_text_mut(& mut self.control_image, image::Rgb(rgb), 20, (400 + 60*i) as i32, scale,&self.font, &instruction[i]);
		}	
		self.display_buffer = Fflo::as_u32buffer(&self.control_image);
	}
	
		
	pub fn update_diagnostics(& mut self) {
		let size_float = 40.0; 
		let size_int = 40;
		let scale = Scale { x: size_float, y: size_float};
		let rect = Rect::at(0, 0).of_size(1000, 1000);
		let mut rgb = [0u8;3];
		draw_filled_rect_mut(&mut self.control_image, rect, image::Rgb([0,0,0]));
		let mut diagnostic = vec!["not implemented yet".to_string();self.filter.len() + 8];
		for i in 0..self.filter.len() {
			diagnostic[i] = format!("f[{}] = {},{} -> {}",
				i, self.filter[i].rows,self.filter[i].cols, self.filter_target[i]);
		}
		let option = self.pixel_options[self.current_pixel_option];
		diagnostic[self.filter.len()] = format!("pixels = {},{}", option.0,option.1);
		let option = self.delay_options[self.current_delay_option];
		diagnostic[self.filter.len()+1] = format!("frames between updates = {}", option);
		diagnostic[self.filter.len()+2] = format!("binarized = {}", self.binarized);
		diagnostic[self.filter.len()+3] = format!("monochrome = {}", self.monochrome);
		diagnostic[self.filter.len()+4] = format!("paused = {}", self.paused);
		diagnostic[self.filter.len()+5] = format!("max filters = {}", self.max_num_filters);
		diagnostic[self.filter.len()+6] = format!("help mode = {}", self.help_mode);
		diagnostic[self.filter.len()+7] = format!("speed = {}", self.speed);
		let mut rgb = [255u8;3];
		for i in 0..diagnostic.len() {
			let x = 10;
			let y = (i*size_int) as i32;
			draw_text_mut(& mut self.control_image, image::Rgb(rgb), x, y, scale, &self.font,  &diagnostic[i]);
		}
		
		self.control_buffer = Fflo::as_u32buffer(&self.control_image)	
	}
	
	pub fn update_control_buffer(&mut self) {
		self.update_diagnostics();
	}
	


	pub fn reset_cells(&mut self) {
		let option = self.pixel_options[self.current_pixel_option]; 
		self.cells = Cells::new(option.0,option.1);
	}
	
	pub fn randomize_filter_masks(&mut self) {
		let brightness = self.brightness*rand::random::<f64>();
		for filter in self.filter.iter_mut() {
			filter.mask = Filter::random_filter_mask(filter.rows*filter.cols, brightness);
		}
		self.update_diagnostics();
	}
			
	pub fn reset_filters(&mut self) {
		let mut rng = rand::thread_rng();
		self.num_filters = rng.gen_range(1..self.max_num_filters+1);
		self.filter = Filter::random_filter_vector(self.num_filters);
		self.filter_target = vec![0;self.num_filters];	
		for i in 0..self.filter.len() {
			self.reset_filter(i);
		}
		self.update_diagnostics();
	}
	
	pub fn reset_filter(&mut self, i : usize) { 
		let mut rng = rand::thread_rng();	
		let rows : usize = rng.gen_range(1..self.max_filter/2+1);
		let cols : usize = rng.gen_range(1..self.max_filter/2+1);
		self.filter[i] = Filter::new(2*rows + 1,2*cols + 1);
		self.filter_target[i] = rng.gen_range(0..3);

	}
		
	
	pub fn as_u32buffer(im : &RgbImage) -> Vec<u32> {
		let rows = im.height() as usize; 
		let cols = im.width() as usize; 
		let mut buffer = vec![155u32;rows*cols];
		for row in 0..rows {
			for col in 0..cols{
				let rgb = im.get_pixel(col as u32,row as u32).channels();
				let r = rgb[0] as u32;
				let g = rgb[1] as u32;
				let b = rgb[2] as u32;
				let code = r << 16 | g << 8 | b;
				buffer[row*cols+col] = code;
			}
		}	
		buffer
	}
			
						
}
			
	
	