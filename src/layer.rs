#![allow(unused_mut,unused_imports,non_upper_case_globals,unused_variables,dead_code,non_camel_case_types, non_snake_case)]
use image::{ImageBuffer, RgbImage, GrayImage};
use crate::pixels::*;
pub struct Layer {
	pub rows : usize,
	pub cols : usize,
	pub cells : Vec<f64>,
}

impl Layer {

	pub fn new( rows :usize, cols : usize) -> Layer {
		Layer { rows, cols, cells : random_cells( rows*cols, 1.0) }
	}
	
	pub fn wrapped_location(&self, row_index : i64, col_index : i64) -> f64 {
		let rows = self.rows as i64;
		let cols = self.cols as i64;
		let mut row =  row_index;
		let mut col =  col_index; 
		while row < 0 { row += rows};
		while row > rows - 1 { row -= rows };
		while col < 0 { col += cols};
		while col > cols - 1 { col -= cols };
		debug_assert!(0 <= row && row < rows);
		debug_assert!(0 <= col && col < cols);
		let r = row as usize;
		let c = col as usize;
		self.cells[r*self.cols+c]
	}

	pub fn graph(&self, pixels : &Pixels, buffer : &mut [u32]) {
		let inc = 2.0/256.0;
		for r in 0..self.rows {
			for c in 0..self.cols {
				let x = self.cells[r*self.cols + c];
				let hue = ((x + 1.0)/inc).trunc() as u32;
				pixels.draw(buffer, r,c, hue);
			}
		}
	}
		
				
}



pub fn random_cells(n : usize, max : f64) -> Vec<f64> {
	let mut new_cells = vec![0.0;n];
	for x in new_cells.iter_mut() {
		*x = max - 2.0*max*rand::random::<f64>();
	}
	new_cells
}