use rand::random;
use rand::Rng;
use crate::cells::*;
use crate::layer::*;
use libm::{sin,cos, tanh, exp};
use float_extras::f64::fmod;
pub fn roll(x : f64) -> f64 {
	fmod(x + 1.0, 2.0) - 1.0
}
pub fn fancy(x : f64) -> f64 {
	exp(sin(x))
}

pub fn weird(x : f64) -> f64 {
	sin(exp(x))
}
pub fn invert(x : f64) -> f64 {
	-x
}

pub struct Filter {
	pub mask : Vec<f64>,
	pub rows : usize,
	pub cols : usize,
}

impl Filter {
	pub fn new(rows : usize, cols : usize) -> Filter {
		debug_assert!(rows % 2 == 1 && cols % 2 == 1);
		Filter { mask : Filter::random_filter_mask(rows*cols, 4.0), rows, cols }
	}
	
	pub fn apply_to_cell( &self, current_layer : &Layer, row : i64, col : i64) -> f64 {
		let current_cell_state = current_layer.wrapped_location(row,col); 
		if self.mask.len() == 1 {
			return self.mask[0]*current_cell_state;
		}
		let mut sum = 0.0;
		let mut counter : usize = 0;
		let row_span = (self.rows as i64 - 1 ) / 2 ;
		let col_span = (self.cols as i64 - 1 ) / 2 ;
		let first_row = row - row_span;
		let last_row = row + row_span;		
		let first_col = col - col_span;
		let last_col = col + col_span;
		for r in  first_row..=last_row {
			for c in first_col..= last_col {
				sum += current_layer.wrapped_location(r,c)*self.mask[counter];
				counter += 1;
			}
		}
// 		let strength = 5.0;
// 		let noise = strength - 2.0*strength*rand::random::<f64>();
		tanh(sum)
	}
	
	pub fn apply(&self, layer : &mut Layer) {
		let mut workspace = vec![0.0; layer.rows*layer.cols];	
		for r in 0..layer.rows {
			for c in 0..layer.cols {
				workspace[r*layer.cols +c] = self.apply_to_cell(layer, r as i64, c as i64);
			}
		}
		layer.cells = workspace;
	}
	
	
	pub fn random_filter_vector(n : usize) -> Vec<Filter> {
		let mut f : Vec<Filter> = vec![];
		for i in 0..n {
			f.push(Filter::random_filter());
		}
		f
	}
	
	pub fn random_filter() -> Filter {
		let mut rng = rand::thread_rng();	
		let random_row_half : usize = rng.gen_range(0..5);
		let random_col_half : usize = rng.gen_range(0..5);
		Filter::new(2*random_row_half+1,2*random_col_half+1)
	}
	
	pub fn random_filter_mask(n : usize, max : f64) -> Vec<f64> {
		let mut new_mask = vec![0.0;n];
		for x in new_mask.iter_mut() {
			*x = max - 2.0*max*rand::random::<f64>();
		}
		new_mask
	}

	
}


	


