use bitflags::bitflags;

bitflags! {
    pub struct NumberSet: u16 {
        const N1 = 0b000000001;
        const N2 = 0b000000010;
        const N3 = 0b000000100;
        const N4 = 0b000001000;
        const N5 = 0b000010000;
        const N6 = 0b000100000;
        const N7 = 0b001000000;
        const N8 = 0b010000000;
        const N9 = 0b100000000;
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

#[repr(usize)]
pub enum Ix {
	Ix1 = 0,
	Ix2,
	Ix3,
	Ix4,
	Ix5,
	Ix6,
	Ix7,
	Ix8,
	Ix9,
}


/// TODO: Replace usize with type that can only represent 1-9
impl<T> Sudoku<T> {
	pub fn row<'a>(&'a self, r: Ix) -> impl Iterator<Item=&'a T> {
		self.arr[r as usize].iter()
	}

	pub fn row_mut<'a>(&'a mut self, r: Ix) -> impl Iterator<Item=&'a mut T> {
		self.arr[r as usize].iter_mut()
	}

//  pub fn col<'a>(&'a self, c: usize) -> impl Iterator<Item=&'a T> {
//  	unimplemented!()
//  }

// 	pub fn block<'a>(&'a self, i: usize, j: usize) -> impl Iterator<Item=&'a T> {
// 		unimplemented!()
// 	}

// 	pub fn col_mut<'a>(&'a mut self, i: usize) -> impl Iterator<Item=&'a mut T> {
// 		unimplemented!()
// 	}

// 	pub fn block_mut<'a>(&'a mut self, i: usize, j: usize) -> impl Iterator<Item=&'a mut T> {
// 		unimplemented!()
// 	}
}
