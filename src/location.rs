
#[derive(Copy, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Debug)]
pub struct Location {
	pub x: i8,
	pub y: i8
}


impl Location {
    pub fn new(x: i8, y: i8) -> Location { Location { x: x, y: y } }
}
