// bytes to string
macro_rules! b2s {
    ($( $x:expr ), *) => {{
        let mut temp_string = "$".to_owned();
        $(
            temp_string.push_str(&format!("{:02x}", $x));
        )*
        temp_string
    }};
}

pub fn disassemble_8080_op(buffer: &Vec<u8>, pc: usize) -> u16 {
    let code = buffer[pc];
    let mut op_bytes: u16 = 1;

    match code {
        0x00 => {
            // no-op
            println!("NOP");
        }
        0x01 => {
            // B <- byte3, C <- byte2
            println!("LXI   B, #${}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x02 => {
            // (BC) <- A
            println!("STAX  B");
        }
        0x03 => {
            // BC <- BC + 1
            println!("INX   B");
        }
        0x04 => {
            // B <- B + 1
            println!("INR   B");
        }
        0x05 => {
            // B <- B - 1
            println!("DCR   B");
        }
        0x06 => {
            // B <- byte2
            println!("MVI   B, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x07 => {
            // A << 1; bit0 = prev bit7, CY=prev bit7
            println!("RLC");
        }
        0x09 => {
            // HL <- HL + BC
            println!("DAD   B");
        }
        0x0a => {
            // A <- (BC)
            println!("LDAX  B");
        }
        0x0b => {
            // BC <- BC - 1
            println!("DCX   B");
        }
        0x0c => {
            // C <- C + 1
            println!("INR   C");
        }
        0x0d => {
            // C <- C - 1
            println!("DCR   C");
        }
        0x0e => {
            // C <- byte2
            println!("MVI   C, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x0f => {
            // A >> 1, bit7 = prev bit0, CY=prev bit0
            println!("RRC");
        }
        0x11 => {
            // D <- byte3, E <- byte2
            println!("LXI   D, {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x12 => {
            // (DE) <- A
            println!("STAX  D");
        }
        0x13 => {
            // DE <- DE + 1
            println!("INX   D");
        }
        0x14 => {
            // D <- D + 1
            println!("INR   D");
        }
        0x15 => {
            // D <- D - 1
            println!("DCR   D");
        }
        0x16 => {
            // D <- byte2
            println!("MVI   D, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x17 => {
            // A = A << 1; bit0 = prev CY; CY = prev bit7
            println!("RAL");
        }
        0x19 => {
            //HL = HL + DE
            println!("DAD   D");
        }
        0x1a => {
            // A <- (DE)
            println!("LDAX  D");
        }
        0x1b => {
            // DE = DE - 1
            println!("DCX   D");
        }
        0x1c => {
            // E <- E + 1
            println!("INR   E");
        }
        0x1d => {
            // E <- E - 1
            println!("DCR   E");
        }
        0x1e => {
            // E <- byte2
            println!("MVI   E, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x1f => {
            // A = A >> 1; bit7 = prev bit7; CY = prev bit0
            println!("RAR");
        }
        0x21 => {
            // H <- byte3, L <- byte2
            println!("LXI   H, #{}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x22 => {
            // (adr) <- L; (adr+1) <- H
            println!("SHLD  {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x23 => {
            // HL <- HL + 1
            println!("INX   H");
        }
        0x24 => {
            // H <- H + 1
            println!("INR   H");
        }
        0x25 => {
            // H <- H - 1
            println!("DCR   H");
        }
        0x26 => {
            // H <- byte2
            println!("MVI   H, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x27 => {
            // special
            println!("DAA");
        }
        0x29 => {
            // HL = HL + HI
            println!("DAD   H");
        }
        0x2a => {
            // L <- (adr); H <- (adr + 1)
            println!("LHLD  {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x2b => {
            // HL = HL - 1
            println!("DCX   H");
        }
        0x2c => {
            // L <- L + 1
            println!("INR   L");
        }
        0x2d => {
            // L <- L - 1
            println!("DCR   L");
        }
        0x2e => {
            // L <- byte2
            println!("MVI   L, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x2f => {
            // A <- !A
            println!("CMA");
        }
        0x31 => {
            // SP.hi <- byte3, SP.lo <- byte2
            println!("LXI   SP, #{}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x32 => {
            // (adr) <- A
            println!("STA   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x33 => {
            // SP <- SP + 1
            println!("INX   SP");
        }
        0x34 => {
            // (HL) <- (HL) + 1
            println!("INR   M");
        }
        0x35 => {
            // (HL) <- (HL) - 1
            println!("DCR   M");
        }
        0x36 => {
            // (HL) <- byte2
            println!("MVI   M, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x37 => {
            // CY = 1
            println!("STC");
        }
        0x39 => {
            // HL = HL + SP
            println!("DAD   SP");
        }
        0x3a => {
            // A <- (adr)
            println!("LDA   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0x3b => {
            // SP <- SP - 1
            println!("DCX   SP");
        }
        0x3c => {
            // A <- A + 1
            println!("INR   A");
        }
        0x3d => {
            // A <- A - 1
            println!("DCR   A");
        }
        0x3e => {
            // A <- byte2
            println!("MVI   A, #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0x3f => {
            // CY = !CY
            println!("CMC");
        }
        0x40 => {
            // B <- B
            println!("MOV   B, B");
        }
        0x41 => {
            // B <- C
            println!("MOV   B, C");
        }
        0x42 => {
            // B <- D
            println!("MOV   B, D");
        }
        0x43 => {
            // B <- E
            println!("MOV   B, E");
        }
        0x44 => {
            // B <- H
            println!("MOV   B, H");
        }
        0x45 => {
            // B <- L
            println!("MOV   B, L");
        }
        0x46 => {
            // B <- (HL)
            println!("MOV   B, M");
        }
        0x47 => {
            // B <- A
            println!("MOV   B, A");
        }
        0x48 => {
            // C <- B
            println!("MOV   C, B");
        }
        0x49 => {
            // C <- C
            println!("MOV   C, C");
        }
        0x4a => {
            // C <- D
            println!("MOV   C, D");
        }
        0x4b => {
            // C <- E
            println!("MOV   C, E");
        }
        0x4c => {
            // C <- H
            println!("MOV   C, H");
        }
        0x4d => {
            // C <- L
            println!("MOV   C, L");
        }
        0x4e => {
            // C <- (HL)
            println!("MOV   C, M");
        }
        0x4f => {
            // C <- A
            println!("MOV   C, A");
        }
        0x50 => {
            // D <- B
            println!("MOV   D, B");
        }
        0x51 => {
            // D <- C
            println!("MOV   D, C");
        }
        0x52 => {
            // D <- D
            println!("MOV   D, D");
        }
        0x53 => {
            // D <- E
            println!("MOV   D, E");
        }
        0x54 => {
            // D <- H
            println!("MOV   D, H");
        }
        0x55 => {
            // D <- L
            println!("MOV   D, L");
        }
        0x56 => {
            // D <- (HL)
            println!("MOV   D, M");
        }
        0x57 => {
            // D <- A
            println!("MOV   D, A");
        }
        0x58 => {
            // E <- B
            println!("MOV   E, B");
        }
        0x59 => {
            // E <- C
            println!("MOV   E, C");
        }
        0x5a => {
            // E <- D
            println!("MOV   E, D");
        }
        0x5b => {
            // E <- E
            println!("MOV   E, E");
        }
        0x5c => {
            // E <- H
            println!("MOV   E, H");
        }
        0x5d => {
            // E <- L
            println!("MOV   E, L");
        }
        0x5e => {
            // E <- (HL)
            println!("MOV   E, M");
        }
        0x5f => {
            // E <- A
            println!("MOV   E, A");
        }
        0x60 => {
            // H <- B
            println!("MOV   H, B");
        }
        0x61 => {
            // H <- C
            println!("MOV   H, C");
        }
        0x62 => {
            // H <- D
            println!("MOV   H, D");
        }
        0x63 => {
            // H <- E
            println!("MOV   H, E");
        }
        0x64 => {
            // H <- H
            println!("MOV   H, H");
        }
        0x65 => {
            // H <- L
            println!("MOV   H, L");
        }
        0x66 => {
            // H <- (HL)
            println!("MOV   H, M");
        }
        0x67 => {
            // H <- A
            println!("MOV   H, A");
        }
        0x68 => {
            // L <- B
            println!("MOV   L, B");
        }
        0x69 => {
            // L <- C
            println!("MOV   L, C");
        }
        0x6a => {
            // L <- D
            println!("MOV   L, D");
        }
        0x6b => {
            // L <- E
            println!("MOV   L, E");
        }
        0x6c => {
            // L <- H
            println!("MOV   L, H");
        }
        0x6d => {
            // L <- L
            println!("MOV   L, L");
        }
        0x6e => {
            // L <- (HL)
            println!("MOV   L, M");
        }
        0x6f => {
            // L <- A
            println!("MOV   L, A");
        }
        0x70 => {
            // M <- B
            println!("MOV   M, B");
        }
        0x71 => {
            // M <- C
            println!("MOV   M, C");
        }
        0x72 => {
            // M <- D
            println!("MOV   M, D");
        }
        0x73 => {
            // M <- E
            println!("MOV   M, E");
        }
        0x74 => {
            // M <- H
            println!("MOV   M, H");
        }
        0x75 => {
            // M <- L
            println!("MOV   M, L");
        }
        0x76 => {
            // special
            println!("HLT");
        }
        0x77 => {
            // M <- A
            println!("MOV   M, A");
        }
        0x78 => {
            // A <- B
            println!("MOV   A, B");
        }
        0x79 => {
            // A <- C
            println!("MOV   A, C");
        }
        0x7a => {
            // A <- D
            println!("MOV   A, D");
        }
        0x7b => {
            // A <- E
            println!("MOV   A, E");
        }
        0x7c => {
            // A <- H
            println!("MOV   A, H");
        }
        0x7d => {
            // A <- L
            println!("MOV   A, L");
        }
        0x7e => {
            // A <- (HL)
            println!("MOV   A, M");
        }
        0x7f => {
            // A <- A
            println!("MOV   A, A");
        }
        0x80 => {
            // A <- A + B
            println!("ADD   B");
        }
        0x81 => {
            // A <- A + C
            println!("ADD   C");
        }
        0x82 => {
            // A <- A + D
            println!("ADD   D");
        }
        0x83 => {
            // A <- A + E
            println!("ADD   E");
        }
        0x84 => {
            // A <- A + H
            println!("ADD   H");
        }
        0x85 => {
            // A <- A + L
            println!("ADD   L");
        }
        0x86 => {
            // A <- A + (HL)
            println!("ADD   M");
        }
        0x87 => {
            // A <- A + A
            println!("ADD   A");
        }
        0x88 => {
            // A <- A + B + CY
            println!("ADC   B");
        }
        0x89 => {
            // A <- A + C + CY
            println!("ADC   C");
        }
        0x8a => {
            // A <- A + D + CY
            println!("ADC   D");
        }
        0x8b => {
            // A <- A + E + CY
            println!("ADC   E");
        }
        0x8c => {
            // A <- A + H + CY
            println!("ADC   H");
        }
        0x8d => {
            // A <- A + L + CY
            println!("ADC   L");
        }
        0x8e => {
            // A <- A + (HL) + CY
            println!("ADC   M");
        }
        0x8f => {
            // A <- A + A + CY
            println!("ADC   A");
        }
        0x90 => {
            // A <- A - B
            println!("SUB   B");
        }
        0x91 => {
            // A <- A - C
            println!("SUB   C");
        }
        0x92 => {
            // A <- A - D
            println!("SUB   D");
        }
        0x93 => {
            // A <- A - E
            println!("SUB   E");
        }
        0x94 => {
            // A <- A - H
            println!("SUB   H");
        }
        0x95 => {
            // A <- A - L
            println!("SUB   L");
        }
        0x96 => {
            // A <- A - (HL)
            println!("SUB   M");
        }
        0x97 => {
            // A <- A - A
            println!("SUB   A");
        }
        0x98 => {
            // A <- A - B - CY
            println!("SBB   B");
        }
        0x99 => {
            // A <- A - C - CY
            println!("SBB   C");
        }
        0x9a => {
            // A <- A - D - CY
            println!("SBB   D");
        }
        0x9b => {
            // A <- A - E - CY
            println!("SBB   E");
        }
        0x9c => {
            // A <- A - H - CY
            println!("SBB   H");
        }
        0x9d => {
            // A <- A - L - CY
            println!("SBB   L");
        }
        0x9e => {
            // A <- A - (HL) - CY
            println!("SBB   M");
        }
        0x9f => {
            // A <- A - A - CY
            println!("SBB   A");
        }
        0xa0 => {
            // A <- A & B
            println!("ANA   B");
        }
        0xa1 => {
            // A <- A & C
            println!("ANA   C");
        }
        0xa2 => {
            // A <- A & D
            println!("ANA   D");
        }
        0xa3 => {
            // A <- A & E
            println!("ANA   E");
        }
        0xa4 => {
            // A <- A & H
            println!("ANA   H");
        }
        0xa5 => {
            // A <- A & L
            println!("ANA   L");
        }
        0xa6 => {
            // A <- A & (HL)
            println!("ANA   M");
        }
        0xa7 => {
            // A <- A & A
            println!("ANA   A");
        }
        0xa8 => {
            // A <- A ^ B
            println!("XRA   B");
        }
        0xa9 => {
            // A <- A ^ C
            println!("XRA   C");
        }
        0xaa => {
            // A <- A ^ D
            println!("XRA   D");
        }
        0xab => {
            // A <- A ^ E
            println!("XRA   E");
        }
        0xac => {
            // A <- A ^ H
            println!("XRA   H");
        }
        0xad => {
            // A <- A ^ L
            println!("XRA   L");
        }
        0xae => {
            // A <- A ^ (HL)
            println!("XRA   M");
        }
        0xaf => {
            // A <- A ^ A
            println!("XRA   A");
        }
        0xb0 => {
            // A <- A | B
            println!("ORA   B");
        }
        0xb1 => {
            // A <- A | C
            println!("ORA   C");
        }
        0xb2 => {
            // A <- A | D
            println!("ORA   D");
        }
        0xb3 => {
            // A <- A | E
            println!("ORA   E");
        }
        0xb4 => {
            // A <- A | H
            println!("ORA   H");
        }
        0xb5 => {
            // A <- A | L
            println!("ORA   L");
        }
        0xb6 => {
            // A <- A | (HL)
            println!("ORA   M");
        }
        0xb7 => {
            // A <- A | A
            println!("ORA   A");
        }
        0xb8 => {
            // A - B
            println!("CMP   B");
        }
        0xb9 => {
            // A - C
            println!("CMP   C");
        }
        0xba => {
            // A - D
            println!("CMP   D");
        }
        0xbb => {
            // A - E
            println!("CMP   E");
        }
        0xbc => {
            // A - H
            println!("CMP   H");
        }
        0xbd => {
            // A - L
            println!("CMP   L");
        }
        0xbe => {
            // A - (HL)
            println!("CMP   M");
        }
        0xbf => {
            // A - A
            println!("CMP   A");
        }
        0xc0 => {
            // if NZ, RET
            println!("RNZ");
        }
        0xc1 => {
            // C <- (sp); B <- (sp+1); sp <- sp+2
            println!("POP   B");
        }
        0xc2 => {
            // if NZ, PC <- adr
            println!("JNZ   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xc3 => {
            // PC <- adr
            println!("JMP   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xc4 => {
            // if NZ, CALL adr
            println!("CNZ   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xc5 => {
            // (sp-2) <- C; (sp-1) <- B; sp <- sp-2
            println!("PUSH  B");
        }
        0xc6 => {
            // A <- A + byte
            println!("ADI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xc7 => {
            // CALL $0
            println!("RST   0");
        }
        0xc8 => {
            // if Z, RET
            println!("RZ");
        }
        0xc9 => {
            // PC.lo <- (sp); PC.hi <- (sp+1); sp <- sp+2
            println!("RET");
        }
        0xca => {
            // iF Z, PC <- adr
            println!("JZ    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xcc => {
            // if Z, CALL adr
            println!("CZ    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xcd => {
            // (sp-1) <- PC.hi; (sp-2) <- PC.lo; sp <- sp-2; PC <- adr
            println!("CALL  {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xce => {
            // A <- A + byte2 + CY
            println!("ACI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xcf => {
            // CALL $8
            println!("RST   1");
        }
        0xd0 => {
            // if NCY, RET
            println!("RNC");
        }
        0xd1 => {
            // E <- (sp); D <- (sp+1); sp <- sp+2
            println!("POP   D");
        }
        0xd2 => {
            // if NCY, PC <- adr
            println!("JNC   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xd3 => {
            // special
            println!("OUT   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xd4 => {
            // if NCY, CALL adr
            println!("CNC   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xd5 => {
            // (sp-2) <- E; (sp-1) <- D; sp <- sp-2
            println!("PUSH  D");
        }
        0xd6 => {
            // A < A - byte2
            println!("SUI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xd7 => {
            // CALL $10
            println!("RST   2")
        }
        0xd8 => {
            // if CY, RET
            println!("RC");
        }
        0xda => {
            // if CY, PC <- adr
            println!("JC    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xdb => {
            // special
            println!("IN    #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xdc => {
            // if CY, CALL adr
            println!("CC    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xde => {
            // A <- A - data - CY
            println!("SBI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xdf => {
            // CALL $18
            println!("RST   3");
        }
        0xe0 => {
            // if PO, RET
            println!("RPO");
        }
        0xe1 => {
            // L <- (sp); H <- (sp+1); sp <- sp+2
            println!("POP   H");
        }
        0xe2 => {
            // if PO, PC <- adr
            println!("JPO   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xe3 => {
            // L <-> (sp); H <-> (sp+1)
            println!("XTHL");
        }
        0xe4 => {
            // if PO, CALL adr
            println!("CPO   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xe5 => {
            // (sp-2) <- L; (sp-1) <- H; sp <- sp-2
            println!("PUSH  H");
        }
        0xe6 => {
            // A <- A & byte2
            println!("ANI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xe7 => {
            // CALL $20
            println!("RST   4");
        }
        0xe8 => {
            // if PE, RET
            println!("RPE");
        }
        0xe9 => {
            // PC.hi <- H; PC.lo <- L
            println!("PCHL");
        }
        0xea => {
            // if PE, PC <- adr
            println!("JPE   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xeb => {
            // H <-> D; L <-> E
            println!("XCHG");
        }
        0xec => {
            // if PE, CALL adr
            println!("CPE   {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xee => {
            // A <- A ^ byte2
            println!("XRI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xef => {
            // CALL $28
            println!("RST   5");
        }
        0xf0 => {
            // if P, RET
            println!("RP");
        }
        0xf1 => {
            // flags <- (sp); A <- (sp+1); sp <- sp+2
            println!("POP   PSW");
        }
        0xf2 => {
            // if P=1, PC <- adr
            println!("JP    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xf3 => {
            // special
            println!("DI");
        }
        0xf4 => {
            // if P, CALL adr
            println!("CP    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xf5 => {
            // (ps-2) <- flags; (sp-1) <- A; sp <- sp-2
            println!("PUSH  PSW");
        }
        0xf6 => {
            // A <- A | byte2
            println!("ORI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xf7 => {
            // CALL $30
            println!("RST   6");
        }
        0xf8 => {
            // if M, RET
            println!("RM");
        }
        0xf9 => {
            // SP <- HL
            println!("SPHL");
        }
        0xfa => {
            // if M, PC <- adr
            println!("JM    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3;
        }
        0xfb => {
            // special
            println!("EI");
        }
        0xfc => {
            // if M, CALL adr
            println!("CM    {}", b2s!(buffer[pc + 2], buffer[pc + 1]));
            op_bytes = 3
        }
        0xfe => {
            // A - data
            println!("CPI   #{}", b2s!(buffer[pc + 1]));
            op_bytes = 2;
        }
        0xff => {
            // CALL $38
            println!("RST   7");
        }
        _ => {
            println!("NOP");
        }
    }

    op_bytes
}
