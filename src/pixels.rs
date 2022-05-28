pub struct Pixels {
	pub buffer_rows : usize,
	pub buffer_cols : usize,
	pub pixel_height : usize,
	pub pixel_width : usize,
	pub pixel_rows : usize,
	pub pixel_columns : usize,
}

impl Pixels {

	pub fn draw(&self, buffer : &mut [u32], r : usize, c : usize, hue : u32) {
		debug_assert!(hue < 256);
		debug_assert!( (c + 1)*self.pixel_width -1 < self.buffer_cols);
		debug_assert!( (r + 1)*self.pixel_height -1 < self.buffer_rows);
		let start_col = c*self.pixel_width;
		let start_row = r*self.pixel_height;

		for sub_row in 0..self.pixel_height {
			for sub_col in 0..self.pixel_width {
				buffer[(start_row + sub_row)*self.buffer_cols + start_col + sub_col] = hue << 16 | hue << 8 | hue;
			}
		}
				
	}
	
	pub fn paint(&self, buffer : &mut [u32], r : usize, c : usize, hue : u32) {
		debug_assert!(hue < 256);
		debug_assert!( (c + 1)*self.pixel_width -1 < self.buffer_cols);
		debug_assert!( (r + 1)*self.pixel_height -1 < self.buffer_rows);
		let start_col = c*self.pixel_width;
		let start_row = r*self.pixel_height;

		for sub_row in 0..self.pixel_height {
			for sub_col in 0..self.pixel_width {
				buffer[(start_row + sub_row)*self.buffer_cols + start_col + sub_col] = hue;
			}
		}
				
	}
	
	pub fn default_pixels(rows: usize, cols: usize) -> Pixels {
		Pixels { 
			buffer_rows : rows, buffer_cols : cols,
			pixel_height : 10, pixel_width : 10, 
			pixel_rows : 100, pixel_columns : 100,
		}
	}
	pub fn custom_pixels(rows: usize, cols: usize, p_rows: usize, p_cols: usize) -> Pixels {
		debug_assert!(rows % p_rows == 0 && p_rows < rows);
		debug_assert!(cols % p_cols == 0 && p_cols < cols);
		let height = rows / p_rows;
		let width = cols /  p_cols;
		
		Pixels { 
			buffer_rows : rows, buffer_cols : cols,
			pixel_height : height, pixel_width : width, 
			pixel_rows : p_rows, pixel_columns : p_cols,
		}
	}
	
}