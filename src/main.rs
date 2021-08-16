pub mod cpu;
use std::{fs::File, io::Read};
use cpu::Cpu;
fn main() {

    let mut file = File::open("data/INVADERS").unwrap();

    let mut data = Vec::new();

    file.read_to_end(&mut data).unwrap();


    let mut cpu = Cpu::new();

    cpu.load_ROM(&data);


}
