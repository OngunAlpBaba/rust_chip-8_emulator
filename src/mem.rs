pub struct Memory{
	pub memory: [u8;4096]
}

impl Memory {
    pub fn new() -> Memory {
    	Memory {memory: [1;4096]}
    }
}
