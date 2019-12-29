use bitflags::bitflags;

bitflags! {
    pub struct NumberSet: u16 {
        const N1 = 0b00000001;
        const N2 = 0b00000010;
        const N4 = 0b00000100;
        const N5 = 0b00001000;
        const N6 = 0b00010000;
        const N7 = 0b00100000;
        const N8 = 0b01000000;
        const N9 = 0b10000000;
    }
}

impl NumberSet {
	/// Returns true if exactly one flag is set
	pub fn is_singleton(&self) -> bool {
		self.bits().count_ones() == 1 
	}
}

pub struct Sudoku<T> {
    // row-major
    arr: [[T; 9]; 9],
}

/// TODO: Replace u8 with type that can only represent 1-9
// impl<T> Sudoku<T> {
// 	pub fn row<'a>(i: u8) -> impl Iterator<Item=&'a T> + 'a {
// 		unimplemented!()
// 	}

// 	pub fn col<'a>(i: u8) -> impl Iterator<Item=&'a T> + 'a {
// 		unimplemented!()
// 	}

// 	pub fn block<'a>(i: u8, j: u8) -> impl Iterator<Item=&'a T> + 'a {
// 		unimplemented!()
// 	}

// 	pub fn row_mut<'a>(i: u8) -> impl Iterator<Item=&'a mut T> + 'a {
// 		unimplemented!()
// 	}

// 	pub fn col_mut<'a>(i: u8) -> impl Iterator<Item=&'a mut T> + 'a {
// 		unimplemented!()
// 	}

// 	pub fn block_mut<'a>(i: u8, j: u8) -> impl Iterator<Item=&'a mut T> + 'a {
// 		unimplemented!()
// 	}
// }
