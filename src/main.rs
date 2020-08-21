//def.rs
const KB_4:usize = 4096;
const B_16:usize = 16;

pub struct Motherboard{
	pub memory: [u8;KB_4],
	pub registers: [u8;B_16],
	pub program_counter: u16
}

impl Motherboard {
    pub fn new() -> Motherboard {
    	Motherboard {
    		memory: [0;KB_4],
    		registers: [0;B_16],
    		program_counter: 0
    	}
    }
}

fn main() {
    let mut motherboard = Motherboard::new();
    println!("{}", motherboard.memory[0]);
}
