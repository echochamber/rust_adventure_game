use location::Location;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Area {
	pub locations: HashMap<Location, GameCell>
}

impl Area {
    pub fn new() -> Area {
    	Area {
    		locations: HashMap::new()
    	}
    }
}


#[derive(Copy, Eq, PartialEq, Clone, Hash, PartialOrd, Ord, Debug)]
pub struct GameCell {
	pub data: u8
}