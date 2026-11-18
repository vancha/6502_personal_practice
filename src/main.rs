use std::ops::Add;
//https://www.masswerk.at/6502/6502_instruction_set.html#INX
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::AddressingMode::Accumulator;

const OPCODE_SIZE: u8 = 1;

static OPCODE_MAP: LazyLock<HashMap<(Mnemonic, AddressingMode), u8>> = LazyLock::new(|| {
    HashMap::from([
        //LDA
        ((Mnemonic::LDA, AddressingMode::Immediate), 0xA9),
        ((Mnemonic::LDA, AddressingMode::ZeroPage), 0xA5),
        ((Mnemonic::LDA, AddressingMode::ZeroPageX), 0xB5),
        ((Mnemonic::LDA, AddressingMode::Absolute), 0xAD),
        ((Mnemonic::LDA, AddressingMode::AbsoluteXIndexed), 0xBD),
        ((Mnemonic::LDA, AddressingMode::AbsoluteYIndexed), 0xB9),
        ((Mnemonic::LDA, AddressingMode::IndexedIndirect), 0xA1),
        ((Mnemonic::LDA, AddressingMode::IndirectIndexed), 0xB1),
        //INX
        ((Mnemonic::INX, AddressingMode::Implied), 0xE8),
        //STA
        ((Mnemonic::STA, AddressingMode::ZeroPage), 0x85),
        ((Mnemonic::STA, AddressingMode::ZeroPageX), 0x95),
        ((Mnemonic::STA, AddressingMode::Absolute), 0x8D),
        ((Mnemonic::STA, AddressingMode::AbsoluteXIndexed), 0x9D),
        ((Mnemonic::STA, AddressingMode::AbsoluteYIndexed), 0x99),
        ((Mnemonic::STA, AddressingMode::IndexedIndirect), 0x81),
        ((Mnemonic::STA, AddressingMode::IndirectIndexed), 0x91),
        //((Mnemonic::, AddressingMode::), 0x),
    ])
});

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Mnemonic {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    //This is going to be our fallback, should never be encountered and we should probably panic
    //when it does
    INVALID,
}

impl Mnemonic {
    fn from_str(mnemonic: &str) -> Self {
        match mnemonic.trim_ascii().to_ascii_uppercase().as_str() {
            "ADC" => Mnemonic::ADC,
            "AND" => Mnemonic::AND,
            "ASL" => Mnemonic::ASL,
            "BCC" => Mnemonic::BCC,
            "BCS" => Mnemonic::BCS,
            "BEQ" => Mnemonic::BEQ,
            "BIT" => Mnemonic::BIT,
            "BMI" => Mnemonic::BMI,
            "BNE" => Mnemonic::BNE,
            "BPL" => Mnemonic::BPL,
            "BRK" => Mnemonic::BRK,
            "BVC" => Mnemonic::BVC,
            "BVS" => Mnemonic::BVS,
            "CLC" => Mnemonic::CLC,
            "CLD" => Mnemonic::CLD,
            "CLI" => Mnemonic::CLI,
            "CLV" => Mnemonic::CLV,
            "CMP" => Mnemonic::CMP,
            "CPX" => Mnemonic::CPX,
            "CPY" => Mnemonic::CPY,
            "DEC" => Mnemonic::DEC,
            "DEX" => Mnemonic::DEX,
            "DEY" => Mnemonic::DEY,
            "EOR" => Mnemonic::EOR,
            "INC" => Mnemonic::INC,
            "INX" => Mnemonic::INX,
            "INY" => Mnemonic::INY,
            "JMP" => Mnemonic::JMP,
            "JSR" => Mnemonic::JSR,
            "LDA" => Mnemonic::LDA,
            "LDX" => Mnemonic::LDX,
            "LDY" => Mnemonic::LDY,
            "LSR" => Mnemonic::LSR,
            "NOP" => Mnemonic::NOP,
            "ORA" => Mnemonic::ORA,
            "PHA" => Mnemonic::PHA,
            "PHP" => Mnemonic::PHP,
            "PLA" => Mnemonic::PLA,
            "PLP" => Mnemonic::PLP,
            "ROL" => Mnemonic::ROL,
            "ROR" => Mnemonic::ROR,
            "RTI" => Mnemonic::RTI,
            "RTS" => Mnemonic::RTS,
            "SBC" => Mnemonic::SBC,
            "SEC" => Mnemonic::SEC,
            "SED" => Mnemonic::SED,
            "SEI" => Mnemonic::SEI,
            "STA" => Mnemonic::STA,
            "STX" => Mnemonic::STX,
            "STY" => Mnemonic::STY,
            "TAX" => Mnemonic::TAX,
            "TAY" => Mnemonic::TAY,
            "TSX" => Mnemonic::TSX,
            "TXA" => Mnemonic::TXA,
            "TXS" => Mnemonic::TXS,
            "TYA" => Mnemonic::TYA,
            &_ => Mnemonic::INVALID,
        }
    }
    fn supports_addressing_mode(self, addressing_mode: &AddressingMode) -> bool {
        OPCODE_MAP.contains_key(&(self, *addressing_mode))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operand {
    Byte(u8),
    Word(u16),
}

impl Operand {
    fn from_byte(input: u8) -> Self {
        Operand::Byte(input)
    }
    fn from_word(input: u16) -> Self {
        Operand::Word(input)
    }
    fn size(&self) -> u8 {
        match self {
            Operand::Byte(_) => 1,
            Operand::Word(_) => 2,
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Byte(value) => vec![*value],
            Operand::Word(value) => value.to_le_bytes().to_vec(),
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AddressingMode {
    //implied single byte instruction
    Accumulator,
    //address $HHLL
    Absolute,
    //X-indexed	OPC $LLHH,X	operand is address; effective address is address incremented by X with carry
    AbsoluteXIndexed,
    //Y-indexed	OPC $LLHH,Y	operand is address; effective address is address incremented by Y with carry **
    AbsoluteYIndexed,
    //operand is a byte
    Immediate,
    //operand implied
    Implied,
    //$LLHH operand is an address, effective address is contents of word at address
    Indirect,
    //note the difference between these two, likely to cause headache later
    IndirectIndexed,
    IndexedIndirect,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
}
#[derive(Debug, Clone, PartialEq)]
struct Operation {
    mnemonic: Mnemonic,
    opcode: u8,
    adressing_mode: AddressingMode,
    size: u8, //in bytes
    operand: Option<Operand>,
}

#[derive(Clone, Debug)]
enum Instruction {
    //I think labels
    Label(u16),
    //seems to be valid rust..
    Operation(Operation),
}

impl Operation {
    fn new(
        mnemonic: Mnemonic,
        addressing_mode: AddressingMode,
        opcode: u8,
        operand: Option<Operand>,
    ) -> Self {
        Operation {
            mnemonic: mnemonic,
            opcode: opcode,
            adressing_mode: addressing_mode,
            size: 0,
            operand: operand,
        }
    }

    fn size(&self) -> u8 {
        match &self.operand {
            Some(operand) => match self.adressing_mode {
                Accumulator | AddressingMode::Implied => 1u8,
                _ => 2u8,
            },
            None => 1u8,
        }
    }
}
#[derive(Debug)]
struct Assembler {
    source: String,
    parsed_source: Vec<Instruction>,
}

impl Assembler {
    fn new(file_name: &Path) -> Self {
        Assembler {
            source: std::fs::read_to_string(file_name).unwrap(),
            parsed_source: vec![],
        }
    }

    fn from_string(file_contents: String) -> Self {
        Assembler {
            source: file_contents,
            parsed_source: vec![],
        }
    }
    fn parse_operand(operand: &str) -> AddressingMode {
        AddressingMode::Absolute
    }

    ///splits into a first and optional second component for further parsing, panics when invalid
    fn split_line(input_line: String) -> (Mnemonic, Option<(String)>) {
        match &input_line.split_once(char::is_whitespace) {
            None => (Mnemonic::from_str(input_line.as_str()), None),
            Some((first, rest)) => {
                let mnemonic = Mnemonic::from_str(first);
                //let operand = Operand::Word(5);
                (mnemonic, Some((rest.to_string())))
            } //(Mnemonic::from_str(first), Operand::new(rest.to_string())),
        }
    }
    fn looks_like_label(input: &str) -> bool {
        input.ends_with(':')
    }
    //turns the source code into a vec of actual instructions
    //fn parse(&mut self) -> Vec<Operation> {
    fn parse(&mut self) -> Vec<Instruction> {
        //I think i might want to collect this entire thing into a Vec of instructions, as defined above. I need to remember to also
        //detect labels
        for line in self.source.lines().filter(|line| !line.is_empty()) {
            if !Assembler::looks_like_label(line) {
                //let split =
                let (mnemonic, source) = match Assembler::split_line(line.to_string()) {
                    (mnemonic, Some((source))) => (mnemonic, Some(source)),
                    (mnemonic, None) => (mnemonic, None),
                };
                use crate::{AddressingMode::*, Mnemonic::*};

                //let (address_mode, operand) = match &source {
                let address_mode = match &source {
                    None => AddressingMode::Implied,
                    Some(operand) => {
                        //if operand is some, then so is source
                        let source = source.unwrap();
                        match source.as_bytes() {
                            [b'#', b'$', _, _] => Immediate,
                            [b'$', high_byte, low_byte] => ZeroPage,
                            //indexed zero-page paterns can look like this: $80,X
                            [b'$', _, _, b',', b'X'] => ZeroPageX,
                            //but they can also look like $8, X
                            [b'$', _, b',', b'X'] => ZeroPageX,
                            [b'$', _, _, b',', b'Y'] => ZeroPageY,
                            [b'$', _, _, _, _] => match mnemonic {
                                BCC | BCS | BEQ | BMI | BNE | BPL | BVC | BVS => Relative,
                                //ADC | AND | ASL | BIT | CMP | CPX | CPY | DEC | EOR | INC | JMP | JSR | LDA | LDX | LDY | LSR | ORA | ROL
                                //| ROR | SBC | STA | STX | STY
                                _ => Absolute,
                            },
                            [b'$', _, _, _, _, b',', b'X'] => AbsoluteXIndexed,
                            [b'$', _, _, _, _, b',', b'Y'] => AbsoluteYIndexed,
                            [b'(', b'$', _, _, _, _, b')'] => Indirect,
                            [b'(', b'$', _, _, b',', b'X', b')'] => IndexedIndirect,
                            [b'(', b'$', _, _, b')', b',', b'Y'] => IndirectIndexed,
                            [b'A'] => Accumulator,
                            _ => panic!("Invalid addressing mode"),
                        }
                    }
                };

                //we can get the opcode from the combination of mnemonic + addressing mode
                let opcode: u8 = match OPCODE_MAP.get(&(mnemonic, address_mode)) {
                    Some(value) => *value,
                    None => panic!(
                        "Sorry, no opcode exists for {:?}",
                        &(mnemonic, address_mode)
                    ),
                };

                //This part, i'm not so sure about..
                self.parsed_source.push(Instruction::Operation(Operation::new(
                    mnemonic,
                    address_mode,
                    opcode,
                    Some(Operand::Byte(4)),
                )));
            } else {
                self.parsed_source.push(Instruction::Label(1u16));
                //self.parsed_source.push();
            }

            //println!("TOOPLE: {:?}",TOOPLE);
            //let components = line.split_whitespace().collect::<Vec<&str>>();
            //i think components at this point can not be empty, because  filtered out empty lines
            //first will always return a value, so safe to unwrap
            //let mnemonic = Mnemonic::from_str(components.first().unwrap());
        }
        self.parsed_source.clone()
    }
}

fn main() {
    //should assemble into "A9 2A 85 80 E8"
    let contents = String::from("LDA #$2A\nSTA $80\nINX");
    let mut parser = Assembler::from_string(contents); //Assembler::new(Path::new("practice_asm.asm"));
    parser.parse();
    println!("stayt: {:?}", parser);
}

#[cfg(test)]
mod tests {

    use crate::AddressingMode::{Absolute, AbsoluteXIndexed, IndirectIndexed, ZeroPageX};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::{AddressingMode, Assembler, Mnemonic, Operand, Operation};

    #[test]
    fn decodes_single_line() {
        assert_eq!(Assembler::from_string(String::from("")).parse(), vec![]);
    }

    fn strips_comments() {
        todo!("This will remove all the comments fromt he source text")
    }

    fn parses_labels() {
        todo!("associates jumps to the correct labels/memory locations")
    }

    fn detects_addressing_modes() {
        let immediate = String::from("LDA #$2A");
        let zeropage = String::from("LDA $80");
        let absolute = String::from("LDA $1234");
        let zeropagex = String::from("LDA $80");
        let absolutexindexed = String::from("LDA $1234,X");
        let absoluteyindexed = String::from("LDA $1234,Y");
        let indexedindirect = String::from("LDA ($80,X)");
        let indirectindexed = String::from("LDA ($80),Y");
        //todo!("correctly interprets where labels in the source point to for jumps")
    }

    fn derives_opcode_from_mnemonic() {
        todo!("Correctly knows which opcode corresponds with which mnemonic")
    }

    fn parses_load_correctly() {
        //Immediate addressing

        /*
        LDA $1234,X ; Load accumulator from absolute,X
        LDA $1234,Y ; Load accumulator from absolute,Y
        LDA ($80,X) ; Load accumulator from indexed indirect
        LDA ($80),Y ; Load accumulator from indirect indexed

        LDX #$2A    ; Load X register with immediate value
        LDX $80     ; Zero page
        LDX $1234   ; Absolute
        LDX $80,Y   ; Zero page,Y
        LDX $1234,Y ; Absolute,Y

        LDY #$2A    ; Load Y register with immediate value
        LDY $80     ; Zero page
        LDY $1234   ; Absolute
        LDY $80,X   ; Zero page,X
        LDY $1234,X ; Absolute,X */
    }

    #[test]
    fn parses_immediate_load_instruction() {
        let assembly_input = String::from("LDA #$2A");
        let load_instruction: &Operation = &Operation::new(
            Mnemonic::LDA,
            AddressingMode::Immediate,
            0xA9,
            Some(Operand::Byte(0x2A)),
        );

        let binding = Assembler::from_string(assembly_input.clone()).parse();
        let assembler_result = binding.first().unwrap();
        //let assembler_result = Assembler::from_string(assembly_input.clone()).parse().first().unwrap();
        assert_eq!(load_instruction, assembler_result);
    }
}
