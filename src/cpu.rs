pub struct Cpu {
	mem: [u8; 4096],
}

impl Cpu {
	pub fn new() -> Self {
		Cpu {
			
			mem: [0; 4096]
		
		
		}
	
	}


	//load our game in 0x200 offset of memeory
	
	pub fn load_ROM(&mut self, data: &Vec<u8>) {
		
		let offset= 0x200;
		for i in 0..data.len(){

			self.write_byte((offset+1), data[i]);


		}

	}

	pub fn write_byte(&mut self, addr: u16, value: u8) {


	}

	pub fn read_byte(&mut self, addr: u16, value: u8) {


	}
}
