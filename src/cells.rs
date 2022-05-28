#![allow(unused_mut,unused_imports,non_upper_case_globals,unused_variables,dead_code,non_camel_case_types, non_snake_case)]
use crate::layer::*;
use crate::pixels::*;

pub struct Cells {
	pub rows : usize,
	pub cols : usize,
	pub r: Layer,
	pub g: Layer,
	pub b: Layer,
	pub monochrome : bool,
	pub binarized : bool,
}

impl Cells {
	pub fn new(rows : usize, cols : usize) -> Cells { 
		Cells { rows, cols, r : Layer::new(rows,cols), g : Layer::new(rows,cols), 
		b :  Layer::new(rows,cols), monochrome : false, binarized : false, }
	}

	pub fn graph(&self, pixels : &Pixels, buffer : &mut [u32]) {
		let inc = 2.0/256.0;
		for r in 0..self.rows {
			for c in 0..self.cols {
				let x = self.r.cells[r*self.cols + c];
				let y = self.g.cells[r*self.cols + c];
				let z = self.b.cells[r*self.cols + c];
				let mut red = ((x + 1.0)/inc).trunc() as u32;
				let mut green = ((y + 1.0)/inc).trunc() as u32;
				let mut blue = ((z + 1.0)/inc).trunc() as u32;
				let mut hue : u32 = 0;
				if self.binarized && self.monochrome {
					let ave = (red + green + blue)/3;
					let ave = if ave > 127 {255} else {0};
					hue = ave << 16 | ave << 8 | ave;
				}
				if self.binarized && ! self.monochrome {
					red = if red > 127 { 255} else {0};
					green = if green > 127 { 255} else {0};
					blue = if blue > 127 { 255} else {0};
					hue = red << 16 | green << 8 | blue;
				}
				if !self.binarized && self.monochrome {
					let ave = (red + green + blue)/3;
					hue = ave << 16 | ave << 8 | ave;
				}
				if !self.binarized && !self.monochrome {
					hue = red << 16 | green << 8 | blue;
				}
				pixels.paint(buffer, r,c, hue);
			}
		}
	}		
}  

