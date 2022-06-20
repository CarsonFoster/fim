use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

pub struct Delta {
    
}

pub struct Differential {
    deltas: Vec<Delta>
}

impl Differential {
}
