//https://www.masswerk.at/6502/6502_instruction_set.html#INX
use std::path::Path;
use std::collections::HashMap;
use std::sync::{LazyLock};


static OPCODE_MAP: LazyLock<HashMap<(Mnemonic, AddressingMode),u8>> =
    LazyLock::new(|| HashMap::from([ 
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
    ]));


#[derive(Hash, PartialEq, Eq, Debug, Copy,Clone)]
pub enum Mnemonic{ 
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
    INVALID
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
             &_ => Mnemonic::INVALID
        }
    }
        fn supports_addressing_mode(self, addressing_mode: &AddressingMode) -> bool {
            OPCODE_MAP.contains_key(&(self, *addressing_mode))
        }
}

#[derive(Hash,Clone, Copy, PartialEq, Eq,Debug)]
pub enum AddressingMode{
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
    RELATIVE,
}
#[derive(Debug)]
struct Instruction {
    mnemonic: Mnemonic,
    opcode: u8,
    adressing_mode: AddressingMode,
    size: u8, //in bytes
operand: Option<u16>,
}

impl Instruction {
    fn new(mnemonic: Mnemonic, addressing_mode: AddressingMode, opcode: u8, operand: Option<u16>) -> Self {
        Instruction {
            mnemonic: mnemonic, 
            opcode: opcode,
            adressing_mode:addressing_mode,
            size: 0,
            operand: operand
        }
    }
}
#[derive(Debug)]
struct Assembler {
    source: String,
    application: Vec<Instruction>,
}

impl Assembler {
    fn new(file_name: &Path) -> Self {
        Assembler {
            source: std::fs::read_to_string(file_name).unwrap(),
            application: vec![],
        }
    }
    //turns the source code into a vec of actual instructions
    fn parse(&mut self) {
        for line in self.source.lines().filter(|line|!line.is_empty()) {
            let components = line.split_whitespace().collect::<Vec<&str>>();
            //i think components at this point can not be empty, because  filtered out empty lines
            //first will always return a value, so safe to unwrap
            let mnemonic = Mnemonic::from_str(components.first().unwrap());
            let address_mode = match components.len() {
                //No arguments so implied addressing, does mnemonic suggest operation tht supports
                //implied adressing?
                1 => {
                    if  mnemonic.supports_addressing_mode(&AddressingMode::Implied) {
                        AddressingMode::Implied
                    } else {
                        panic!("This mnemonic does *not* support implied addressing modes: {:?}",components.first().unwrap());
                    }
                },
                _ => {
                    match components[1].chars().next().unwrap() {
                        '#' => {
                            AddressingMode::Immediate
                        },
                        '$'=> {
                            //technically we could also have a length smaller than two for values
                            //like $1, or $2..
                            if components[1][1..].len() == 1 || components[1][1..].len() == 2{
                                AddressingMode::ZeroPage
                            } else {
                                AddressingMode::Absolute
                            }
                        },
                        _ => panic!("I'm sorry bruh maybe i don't have this adderassing mode implemented yet:  {:?}",components[1])
                    }
                },
            };

            //we can get the opcode from the combination of mnemonic + addressing mode
            let opcode:u8 = match OPCODE_MAP.get(&(mnemonic, address_mode)) {
                Some(value) => *value,
                None => panic!("Sorry, no opcode exists for {:?}", &(mnemonic, address_mode)  )
            };

            //we can derive the total size based on the addressing mode
            let size:u8 = match address_mode {
                AddressingMode::Implied | AddressingMode::Accumulator => 1,
                AddressingMode::Immediate | AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY | AddressingMode::Indirect => 2,
                _ => 3

            };

            self.application.push(Instruction::new(mnemonic, address_mode, opcode, Some(1)));
        }

    }
}

fn main() {
    //contains "LDA #$2A\nSTA $80\nINX" should compile to "A9 2A 85 80 E8"
    let mut parser = Assembler::new(Path::new("practice_asm.asm"));
    parser.parse();
    println!("stayt: {:?}",parser);

}
