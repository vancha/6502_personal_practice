//https://www.masswerk.at/6502/6502_instruction_set.html#INX
use crate::{
    AddressingMode::{Accumulator, *},
    Mnemonic::*,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

const OPCODE_SIZE: u8 = 1;

static OPCODE_MAP: LazyLock<HashMap<(Mnemonic, AddressingMode), u8>> = LazyLock::new(|| {
    HashMap::from([
        //ADC
        ((Mnemonic::ADC, AddressingMode::Immediate), 0x69),
        ((Mnemonic::ADC, AddressingMode::ZeroPage), 0x65),
        ((Mnemonic::ADC, AddressingMode::ZeroPageX), 0x75),
        ((Mnemonic::ADC, AddressingMode::Absolute), 0x6D),
        ((Mnemonic::ADC, AddressingMode::AbsoluteXIndexed), 0x7D),
        ((Mnemonic::ADC, AddressingMode::AbsoluteYIndexed), 0x79),
        ((Mnemonic::ADC, AddressingMode::IndexedIndirect), 0x61), // (oper,X)
        ((Mnemonic::ADC, AddressingMode::IndirectIndexed), 0x71), // (oper),Y
        //AND
        ((Mnemonic::AND, AddressingMode::Immediate), 0x29),
        ((Mnemonic::AND, AddressingMode::ZeroPage), 0x25),
        ((Mnemonic::AND, AddressingMode::ZeroPageX), 0x35),
        ((Mnemonic::AND, AddressingMode::Absolute), 0x2D),
        ((Mnemonic::AND, AddressingMode::AbsoluteXIndexed), 0x3D),
        ((Mnemonic::AND, AddressingMode::AbsoluteYIndexed), 0x39),
        ((Mnemonic::AND, AddressingMode::IndexedIndirect), 0x21), // (oper,X)
        ((Mnemonic::AND, AddressingMode::IndirectIndexed), 0x31), // (oper),Y
        //ASL
        ((Mnemonic::ASL, AddressingMode::Accumulator), 0x0a),
        ((Mnemonic::ASL, AddressingMode::ZeroPage), 0x06),
        ((Mnemonic::ASL, AddressingMode::ZeroPageX), 0x16),
        ((Mnemonic::ASL, AddressingMode::Absolute), 0x0e),
        ((Mnemonic::ASL, AddressingMode::AbsoluteXIndexed), 0x1e),
        //BCC
        ((Mnemonic::BCC, AddressingMode::Relative), 0x90),
        //BCS
        ((Mnemonic::BCS, AddressingMode::Relative), 0xB0),
        //BEQ
        ((Mnemonic::BEQ, AddressingMode::Relative), 0xF0),
        //BIT
        ((Mnemonic::BIT, AddressingMode::ZeroPage), 0x24),
        ((Mnemonic::BIT, AddressingMode::Absolute), 0x2C),
        //BMI
        ((Mnemonic::BMI, AddressingMode::Relative), 0x30),
        //BNE
        ((Mnemonic::BNE, AddressingMode::Relative), 0xD0),
        //BPL
        ((Mnemonic::BPL, AddressingMode::Relative), 0x10),
        //BRK
        ((Mnemonic::BRK, AddressingMode::Implied), 0x00),
        //BVC
        ((Mnemonic::BVC, AddressingMode::Relative), 0x50),
        //BVS
        ((Mnemonic::BVS, AddressingMode::Relative), 0x70),
        //CLC
        ((Mnemonic::CLC, AddressingMode::Implied), 0x18),
        //CLD
        ((Mnemonic::CLD, AddressingMode::Implied), 0xD8),
        //CLI
        ((Mnemonic::CLI, AddressingMode::Implied), 0x58),
        //CLV
        ((Mnemonic::CLV, AddressingMode::Implied), 0xB8),
        //CMP
        ((Mnemonic::CMP, AddressingMode::Immediate), 0xC9),
        ((Mnemonic::CMP, AddressingMode::ZeroPage), 0xC5),
        ((Mnemonic::CMP, AddressingMode::ZeroPageX), 0xD5),
        ((Mnemonic::CMP, AddressingMode::Absolute), 0xCD),
        ((Mnemonic::CMP, AddressingMode::AbsoluteXIndexed), 0xDD),
        ((Mnemonic::CMP, AddressingMode::AbsoluteYIndexed), 0xD9),
        ((Mnemonic::CMP, AddressingMode::IndexedIndirect), 0xC1), // (oper,X)
        ((Mnemonic::CMP, AddressingMode::IndirectIndexed), 0xD1), // (oper),Y
        //CPX
        ((Mnemonic::CPX, AddressingMode::Immediate), 0xE0),
        ((Mnemonic::CPX, AddressingMode::ZeroPage), 0xE4),
        ((Mnemonic::CPX, AddressingMode::Absolute), 0xEC),
        //CPY
        ((Mnemonic::CPY, AddressingMode::Immediate), 0xC0),
        ((Mnemonic::CPY, AddressingMode::ZeroPage), 0xC4),
        ((Mnemonic::CPY, AddressingMode::Absolute), 0xCC),
        //DEC
        ((Mnemonic::DEC, AddressingMode::ZeroPage), 0xC6),
        ((Mnemonic::DEC, AddressingMode::ZeroPageX), 0xD6),
        ((Mnemonic::DEC, AddressingMode::Absolute), 0xCE),
        ((Mnemonic::DEC, AddressingMode::AbsoluteXIndexed), 0xDE),
        //DEX
        ((Mnemonic::DEX, AddressingMode::Implied), 0xCA),
        //DEY
        ((Mnemonic::DEY, AddressingMode::Implied), 0x88),
        //EOR
        ((Mnemonic::EOR, AddressingMode::Immediate), 0x49),
        ((Mnemonic::EOR, AddressingMode::ZeroPage), 0x45),
        ((Mnemonic::EOR, AddressingMode::ZeroPageX), 0x55),
        ((Mnemonic::EOR, AddressingMode::Absolute), 0x4D),
        ((Mnemonic::EOR, AddressingMode::AbsoluteXIndexed), 0x5D),
        ((Mnemonic::EOR, AddressingMode::AbsoluteYIndexed), 0x59),
        ((Mnemonic::EOR, AddressingMode::IndexedIndirect), 0x41), // (oper,X)
        ((Mnemonic::EOR, AddressingMode::IndirectIndexed), 0x51), // (oper),Y
        //INC
        ((Mnemonic::INC, AddressingMode::ZeroPage), 0xE6),
        ((Mnemonic::INC, AddressingMode::ZeroPageX), 0xF6),
        ((Mnemonic::INC, AddressingMode::Absolute), 0xEE),
        ((Mnemonic::INC, AddressingMode::AbsoluteXIndexed), 0xFe),
        //INX
        ((Mnemonic::INX, AddressingMode::Implied), 0xE8),
        //INY
        ((Mnemonic::INY, AddressingMode::Implied), 0xC8),
        //JMP
        ((Mnemonic::JMP, AddressingMode::Absolute), 0x4C),
        ((Mnemonic::JMP, AddressingMode::Indirect), 0x6C),
        //JSR
        ((Mnemonic::JSR, AddressingMode::Absolute), 0x20),
        //LDA
        ((Mnemonic::LDA, AddressingMode::Immediate), 0xA9),
        ((Mnemonic::LDA, AddressingMode::ZeroPage), 0xA5),
        ((Mnemonic::LDA, AddressingMode::ZeroPageX), 0xB5),
        ((Mnemonic::LDA, AddressingMode::Absolute), 0xAD),
        ((Mnemonic::LDA, AddressingMode::AbsoluteXIndexed), 0xBD),
        ((Mnemonic::LDA, AddressingMode::AbsoluteYIndexed), 0xB9),
        ((Mnemonic::LDA, AddressingMode::IndexedIndirect), 0xA1),
        ((Mnemonic::LDA, AddressingMode::IndirectIndexed), 0xB1),
        //LDX
        ((Mnemonic::LDX, AddressingMode::Immediate), 0xA2),
        ((Mnemonic::LDX, AddressingMode::ZeroPage), 0xA6),
        ((Mnemonic::LDX, AddressingMode::ZeroPageY), 0xB6),
        ((Mnemonic::LDX, AddressingMode::Absolute), 0xAE),
        ((Mnemonic::LDX, AddressingMode::AbsoluteYIndexed), 0xBE),
        //LDY
        ((Mnemonic::LDY, AddressingMode::Immediate), 0xA0),
        ((Mnemonic::LDY, AddressingMode::ZeroPage), 0xA4),
        ((Mnemonic::LDY, AddressingMode::ZeroPageX), 0xB4),
        ((Mnemonic::LDY, AddressingMode::Absolute), 0xAC),
        ((Mnemonic::LDY, AddressingMode::AbsoluteXIndexed), 0xBC),
        //LSR
        ((Mnemonic::LSR, AddressingMode::Immediate), 0x4A),
        ((Mnemonic::LSR, AddressingMode::ZeroPage), 0x46),
        ((Mnemonic::LSR, AddressingMode::ZeroPageX), 0x56),
        ((Mnemonic::LSR, AddressingMode::Absolute), 0x4E),
        ((Mnemonic::LSR, AddressingMode::AbsoluteYIndexed), 0x5E),
        //NOP
        ((Mnemonic::NOP, AddressingMode::Implied), 0xEA),
        //ORA
        ((Mnemonic::ORA, AddressingMode::Immediate), 0x09),
        ((Mnemonic::ORA, AddressingMode::ZeroPage), 0x05),
        ((Mnemonic::ORA, AddressingMode::ZeroPageX), 0x15),
        ((Mnemonic::ORA, AddressingMode::Absolute), 0x0D),
        ((Mnemonic::ORA, AddressingMode::AbsoluteXIndexed), 0x1D),
        ((Mnemonic::ORA, AddressingMode::AbsoluteYIndexed), 0x19),
        ((Mnemonic::ORA, AddressingMode::IndexedIndirect), 0x01), // (oper,X)
        ((Mnemonic::ORA, AddressingMode::IndirectIndexed), 0x11), // (oper),Y
        //PHA
        ((Mnemonic::PHA, AddressingMode::Implied), 0x48),
        //PHP
        ((Mnemonic::PHP, AddressingMode::Implied), 0x08),
        //PLA
        ((Mnemonic::PLA, AddressingMode::Implied), 0x68),
        //PLP
        ((Mnemonic::PLP, AddressingMode::Implied), 0x028),
        //ROL
        ((Mnemonic::ROL, AddressingMode::Accumulator), 0x2A),
        ((Mnemonic::ROL, AddressingMode::ZeroPage), 0x26),
        ((Mnemonic::ROL, AddressingMode::ZeroPageX), 0x36),
        ((Mnemonic::ROL, AddressingMode::Absolute), 0x2E),
        ((Mnemonic::ROL, AddressingMode::AbsoluteXIndexed), 0x3E),
        //ROR
        ((Mnemonic::ROR, AddressingMode::Accumulator), 0x6A),
        ((Mnemonic::ROR, AddressingMode::ZeroPage), 0x66),
        ((Mnemonic::ROR, AddressingMode::ZeroPageX), 0x76),
        ((Mnemonic::ROR, AddressingMode::Absolute), 0x6E),
        ((Mnemonic::ROR, AddressingMode::AbsoluteXIndexed), 0x7E),
        //RTI
        ((Mnemonic::RTI, AddressingMode::Implied), 0x40),
        //RTS
        ((Mnemonic::RTS, AddressingMode::Implied), 0x60),
        //SBC
        ((Mnemonic::SBC, AddressingMode::Immediate), 0xE9),
        ((Mnemonic::SBC, AddressingMode::ZeroPage), 0xE5),
        ((Mnemonic::SBC, AddressingMode::ZeroPageX), 0xF5),
        ((Mnemonic::SBC, AddressingMode::Absolute), 0xED),
        ((Mnemonic::SBC, AddressingMode::AbsoluteXIndexed), 0xFD),
        ((Mnemonic::SBC, AddressingMode::AbsoluteYIndexed), 0xF9),
        ((Mnemonic::SBC, AddressingMode::IndexedIndirect), 0xE1), // (oper, X)
        ((Mnemonic::SBC, AddressingMode::IndirectIndexed), 0xF1), // (oper),Y
        //SEC
        ((Mnemonic::SEC, AddressingMode::Implied), 0x38),
        //SED
        ((Mnemonic::SED, AddressingMode::Implied), 0xF8),
        //SEI
        ((Mnemonic::SEI, AddressingMode::Implied), 0x78),
        //STA
        ((Mnemonic::STA, AddressingMode::ZeroPage), 0x85),
        ((Mnemonic::STA, AddressingMode::ZeroPageX), 0x95),
        ((Mnemonic::STA, AddressingMode::Absolute), 0x8D),
        ((Mnemonic::STA, AddressingMode::AbsoluteXIndexed), 0x9D),
        ((Mnemonic::STA, AddressingMode::AbsoluteYIndexed), 0x99),
        ((Mnemonic::STA, AddressingMode::IndexedIndirect), 0x81),
        ((Mnemonic::STA, AddressingMode::IndirectIndexed), 0x91),
        //STX
        ((Mnemonic::STX, AddressingMode::ZeroPage), 0x86),
        ((Mnemonic::STX, AddressingMode::ZeroPageY), 0x96),
        ((Mnemonic::STX, AddressingMode::Absolute), 0x8E),
        //STY
        ((Mnemonic::STY, AddressingMode::ZeroPage), 0x84),
        ((Mnemonic::STY, AddressingMode::ZeroPageX), 0x94),
        ((Mnemonic::STY, AddressingMode::Absolute), 0x8C),
        //TAX
        ((Mnemonic::TAX, AddressingMode::Implied), 0xAA),
        //TAY
        ((Mnemonic::TAY, AddressingMode::Implied), 0xA8),
        //TSX
        ((Mnemonic::TSX, AddressingMode::Implied), 0xBA),
        //TXA
        ((Mnemonic::SEI, AddressingMode::Implied), 0x78),
        //TXS
        ((Mnemonic::TXS, AddressingMode::Implied), 0x9A),
        //TYA
        ((Mnemonic::TYA, AddressingMode::Implied), 0x98),
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
            &_ => panic!("this instruction looks invalid.."),
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
    //Temporary u8 holds an offset, and should be turned into a Word(u16) when we get the absolute address the offset points to
    //Temporary(u8),
    Label(String),
}

impl Operand {
    fn from_ascii_byte(input: &str) -> Self {
        Operand::Byte(u8::from_str_radix(input, 16).unwrap())
    }

    fn from_word(input: u16) -> Self {
        Operand::Word(input)
    }

    /* note that we can turn this into an eventual size when we have operands of the temporary variety */
    fn size(&self) -> u8 {
        match self {
            Operand::Byte(_) => 1,
            Operand::Word(_) => 2,
            Operand::Label(_) => 1,//not quite certain at this point! String could resolve to absolute or relative address
        }
    }
    /* but we can *not* turn them into bytes. */
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Byte(value) => vec![*value],
            Operand::Word(value) => value.to_le_bytes().to_vec(),
            Operand::Label(_) => panic!("We can not turn Relative addresses into bytes before calculating their actual locations."),
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
    IndexedIndirect, // (oper,X)
    IndirectIndexed, // (oper),Y
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
}
#[derive(Debug, Clone, PartialEq)]
struct Operation {
    mnemonic: Mnemonic,
    opcode: u8,
    addressing_mode: AddressingMode,
    size: u8,
    operand: Option<Operand>,
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Label(String),
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
            mnemonic,
            opcode,
            addressing_mode,
            size: 0,
            operand,
        }
    }

    fn size(&self) -> u8 {
        OPCODE_SIZE
            + match &self.operand {
                Some(operand) => operand.size(),
                None => 0,
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
    fn parse_operand(_operand: &str) -> AddressingMode {
        AddressingMode::Absolute
    }

    ///splits into a first and optional second component for further parsing @todo: should panic when invalid
    fn split_line(input_line: String) -> (Mnemonic, Option<String>) {
        match &input_line.trim_ascii().split_once(char::is_whitespace) {
            None => (Mnemonic::from_str(input_line.as_str()), None),
            Some((first, rest)) => {
                let mnemonic = Mnemonic::from_str(first);
                (mnemonic, Some(rest.to_string()))
            }
        }
    }
    fn looks_like_label(input: &str) -> bool {
        input.trim_ascii().ends_with(':')
    }

    //turns the source code into a vec of Instructions that all have a known size in bytes
    //this is the *first* pass, to be followed by a second pass that turns this into a list of
    //bytes with the correct memory locations
    fn first_pass(&mut self) -> Vec<Instruction> {
        for (index, line) in self
            .source
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            if !Assembler::looks_like_label(line) {
                //let split =
                let (mnemonic, source) = match Assembler::split_line(line.to_string()) {
                    (mnemonic, Some(source)) => (mnemonic, Some(source)),
                    (mnemonic, None) => (mnemonic, None),
                };

                /*
                @TODO: So far this entire match thingy completely ignores the fact that while a *line* could define a label,
                instructions can *also* have labels as their argument! pls fix this...
                 */
                let (address_mode, operand) = match &source {
                    None => (AddressingMode::Implied, None::<Operand>),
                    Some(_operand) => {
                        //if operand is some, then so is source
                        let source = source.unwrap();
                        match source.as_bytes() {
                            [b'A'] => (Accumulator, None),
                            [b'#', b'$', _, _] => {
                                (Immediate, Some(Operand::from_ascii_byte(&source[2..4])))
                            }
                            [b'$', _, _] => {
                                (ZeroPage, Some(Operand::from_ascii_byte(&source[1..3])))
                            }
                            //indexed zero-page paterns can look like this: $80,X (maybe technicall
                            //also like $8, X?)
                            [b'$', _, _, b',', b'X'] => (
                                ZeroPageX,
                                todo!("sorry man i aint implemented any of this :( "),
                            ),
                            [b'$', _, _, b',', b'Y'] => {
                                (ZeroPageY, Some(Operand::from_ascii_byte(&source[1..3])))
                            }
                            [b'$', _, _, _, _] => match mnemonic {
                                BCC | BCS | BEQ | BMI | BNE | BPL | BVC | BVS => (
                                    Relative,
                                    Some(Operand::Byte(
                                        u8::from_str_radix(&source[1..3], 16).unwrap(),
                                    )),
                                ),
                                //ADC | AND | ASL | BIT | CMP | CPX | CPY | DEC | EOR | INC | JMP | JSR | LDA | LDX | LDY | LSR | ORA | ROL
                                //| ROR | SBC | STA | STX | STY
                                _ => (
                                    Absolute,
                                    Some(Operand::Word(
                                        u16::from_str_radix(&source[1..5], 16).unwrap(),
                                    )),
                                ),
                            },
                            [b'$', _, _, _, _, b',', b'X'] => (
                                AbsoluteXIndexed,
                                Some(Operand::Word(
                                    u16::from_str_radix(&source[1..5], 16).unwrap(),
                                )),
                            ),
                            [b'$', _, _, _, _, b',', b'Y'] => (
                                AbsoluteYIndexed,
                                Some(Operand::Word(
                                    u16::from_str_radix(&source[1..5], 16).unwrap(),
                                )),
                            ),
                            [b'(', b'$', _, _, b',', b'X', b')'] => (
                                IndexedIndirect,
                                Some(Operand::Byte(
                                    u8::from_str_radix(&source[2..4], 16).unwrap(),
                                )),
                            ),
                            [b'(', b'$', _, _, _, _, b')'] => (
                                Indirect,
                                Some(Operand::Word(
                                    u16::from_str_radix(&source[2..6], 16).unwrap(),
                                )),
                            ),
                            [b'(', b'$', _, _, b')', b',', b'Y'] => (
                                IndirectIndexed,
                                Some(Operand::Byte(
                                    u8::from_str_radix(&source[2..4], 16).unwrap(),
                                )),
                            ),
                            //It is likely that I actually need the instruction here to know if the instruction supports relative or absolute addressing.
                            //A label can be BOTH :S
                            _ => {
                                //lets see what we support:
                                if Mnemonic::supports_addressing_mode(mnemonic, &AddressingMode::Relative) {
                                    (
                                        Relative,
                                        Some(Operand::Label(source.to_string()))
                                    )

                                } else if Mnemonic::supports_addressing_mode(mnemonic, &AddressingMode::Absolute) {

                                     (
                                        Absolute,
                                        Some(Operand::Label(source.to_string()))
                                    )
                                } else {
                                    panic!("{:?} seems to have an operand i cannot quite decode: {:?}",mnemonic, source)
                                }
                            }
                        }
                    }
                };
                println!("The magic boah: {:0x?}", operand);
                //we can get the opcode from the combination of mnemonic + addressing mode
                let opcode: u8 = match OPCODE_MAP.get(&(mnemonic, address_mode)) {
                    Some(value) => *value,
                    None => panic!("Sorry, no opcode exists for {:?}", (mnemonic, address_mode)),
                };

                //This part, i'm not so sure about..
                self.parsed_source
                    .push(Instruction::Operation(Operation::new(
                        mnemonic,
                        address_mode,
                        opcode,
                        operand,
                    )));
            } else {
                //println!("LABEL: {:?}", line);
                self.parsed_source.push(Instruction::Label(line.to_string()));
            }
        }
        self.parsed_source.clone()
    }

    fn second_pass(&self) -> Vec<u8> {
        vec![]
    }
}

fn main() {
    //should assemble into "A9 2A 85 80 E8"
    //let contents = String::from("LDA #$2A\nSTA $80\nINX");

    //Should assemble into (0600: starting point on skilldrick emulator) a2 08 ca 8e 00 02 e0 03 d0 f8 8e 01 02 00
    let contents = String::from(
        "   LDX #$08
decrement:
    DEX
    STX $0200
    CPX #$03
    BNE decrement
    STX $0201
    BRK
",
    );

    println!("The input to this program is the following: {:?}", contents);
    let mut parser = Assembler::from_string(contents);
    parser.first_pass();
}

#[cfg(test)]
mod tests {
    use super::{AddressingMode, Assembler, Mnemonic, Operand, Operation};
    use crate::AddressingMode::{Absolute, AbsoluteXIndexed, IndirectIndexed, ZeroPageX};

    #[test]
    fn decodes_single_line() {
        assert_eq!(
            Assembler::from_string(String::from("")).first_pass(),
            vec![]
        );
    }

    /*#[test]
    fn strips_comments() {

        Assembler::from_string(String::from("")).first_pass(),
        todo!("This will remove all the comments fromt he source text")
    }*/

    #[test]
    fn parses_labels() {
        assert_eq!(
            Assembler::from_string(String::from("")).first_pass(),
            vec![]
        );
    }

    #[test]
    fn does_branching() {
        //Should assemble into (0600: starting point on skilldrick emulator) a2 08 ca 8e 00 02 e0 03 d0 f8 8e 01 02 00
        let input = "   LDX #$08
decrement:
    DEX
    STX $0200
    CPX #$03
    BNE decrement
    STX $0201
    BRK
";
        let assembler = Assembler::from_string(input.to_string()).first_pass();
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
}
