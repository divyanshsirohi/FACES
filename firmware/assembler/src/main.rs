/*
 * This is going to be a super simple assembler for my own asm language implemented for my FPGA
 * board.
 * Specs - 
 * 16 bit instructions [15:0]
 * 4 bit opcode [15:12]
 * Since we have 8 registers - [15:12] - only 3 bits are needed to address those
 * Instruction Formats:
 * - R-Type => Opcode [15:12] | Rd [11:9]  | Rs1 [8:6] | Rs2 [5:3] | 000 [2:0]
 *             (NOP, ADD, SUB, AND, OR, XOR, SHL, SHR, LD, ST)
 * - I-Type => Opcode [15:12] | Rd [11:9]  | 0 [8]     | Imm8 [7:0]
 *             (LDI: zero-extended Imm8, ADDI: sign-extended Imm8)
 * - B-Type => Opcode [15:12] | Rs1 [11:9] | Rs2 [8:6] | Off6 [5:0]
 *             (BEQ, BNE: signed 6-bit PC-relative offset [-32..+31])
 * - J-Type => Opcode [15:12] | Addr12 [11:0]
 *             (JMP, CALL: unsigned 12-bit target word address [0x000..0xFFF])
 *
 * Opcode Map:
 * - 0x0: NOP                  | 0x8: SHL Rd, Rs1
 * - 0x1: LDI Rd, imm8         | 0x9: SHR Rd, Rs1
 * - 0x2: ADDI Rd, imm8        | 0xA: LD  Rd, [Rs1]
 * - 0x3: ADD Rd, Rs1, Rs2     | 0xB: ST  [Rs1], Rs2
 * - 0x4: SUB Rd, Rs1, Rs2     | 0xC: BEQ Rs1, Rs2, label
 * - 0x5: AND Rd, Rs1, Rs2     | 0xD: BNE Rs1, Rs2, label
 * - 0x6: OR  Rd, Rs1, Rs2     | 0xE: JMP label
 * - 0x7: XOR Rd, Rs1, Rs2     | 0xF: CALL label
 *
 * Memory Map:
 * - 0x0000 - 0x07FF: 2048 x 16-bit Unified BRAM (32 Kb)
 * - 0x8000: UART_TX   (Write [7:0] to transmit byte at 115200 baud)
 * - 0x8001: UART_STAT (Read [0]: 1 = busy, 0 = ready)
 * - 0x8002: GPIO_LED  (Write [0] to toggle PIN_16)
*/

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

/**
 * all good things start with a single function.
 */ 

fn get_opcode(mnemonic: &str) -> Option<u16> {
    match mnemonic {
        "NOP"  => Some(0x0),
        "LDI"  => Some(0x1),
        "ADDI" => Some(0x2),
        "ADD"  => Some(0x3),
        "SUB"  => Some(0x4),
        "AND"  => Some(0x5),
        "OR"   => Some(0x6),
        "XOR"  => Some(0x7),
        "SHL"  => Some(0x8),
        "SHR"  => Some(0x9),
        "LD"   => Some(0xA),
        "ST"   => Some(0xB),
        "BEQ"  => Some(0xC),
        "BNE"  => Some(0xD),
        "JMP"  => Some(0xE),
        "CALL" => Some(0xF),
        _      => None,
    }
}

fn parse_reg(token: &str) -> Result<u16, String> {
    //remove the [] from LD and ST
    let clean = token
        .trim()
        .trim_matches(|c| c == '[' || c == ']')
        .to_uppercase();
    //ensure that register names start with R, else rejected
    if let Some(reg_num) = clean.strip_prefix('R') {
        if let Ok(reg) = reg_num.parse::<u16>() {
            if reg <= 7{
                return Ok(reg);
            }
        }
    }
    Err(format!("Invalid register '{}': expected R0-R7", token))
}

// Translates an entire assembly source string into $readmemh-ready 16-bit hex lines.
fn assemble (source: &str) -> Result<String, String> {
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut instructions: Vec<(usize, u16, String)> = Vec::new();

    let mut pc: u16 = 0;

    // PASS 1: Strip comments, record label addresses, and collect instructions
    //


}


fn main() {
    println!("Hello, world!");
    // Prints: Err("Invalid register 'RX7': expected R0-R7")
    println!("{:?}", parse_reg("RX7"));

    // Prints: Ok(2)
    println!("{:?}", parse_reg("[r2]"));
}





