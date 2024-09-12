mod disassembler;
mod io_spainem;

fn main() {
    let op_buffer = io_spainem::read_from_file("invaders.h");
    // let op_buffer_test: Vec<u8> = vec![0x11, 0x23, 0x32];
    let num_bytes = u16::try_from(op_buffer.len()).expect("Binary file is too big.");
    let mut pc: u16 = 0;

    while pc < num_bytes {
        pc += disassembler::disassemble_8080_op(&op_buffer, pc as usize)
    }
}
