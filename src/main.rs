struct CPU {
    opcode: Opcode,
    memory: [u8; 4096],
    stack: [u16, 16],
    V: [u8; 16],
    I: u16,
    PC: u16,
    SP: u8
}

type RegisterIndex = u8;
type Address = u16;

enum SE_val {
    Direct(u8),
    Register(RegisterIndex)
}

struct LD_lval {
    Register(RegisterIndex),
    Index,
    DelayTimer,
}

struct LD_val {
    Direct(u8),
    Register(RegisterIndex),
    Address(Address),
    DelayTimer,
}

struct ADD_lval {
    Register(RegisterIndex),
    Index
}

struct ADD_val {
    Direct(u8),
    Register(RegisterIndex)
}

enum Opcode {
    // Draw opcodes
    CLS,
    DRW(RegisterIndex, RegisterIndex, u8), // TODO: create u4

    // Control Flow
    CALL(Address),
    RET,
    JP(Address),
    SE(RegisterIndex, SE_val),
    SNE(RegisterIndex, SE_val),
    SKP(RegisterIndex),
    SKNP(RegisterIndex),

    // Operators
    LD(LD_lval, LD_val),
    ADD(ADD_lval, ADD_val),
    SUB(RegisterIndex, RegisterIndex),
    SUBN(RegisterIndex, RegisterIndex),
    AND(RegisterIndex, RegisterIndex),
    OR(RegisterIndex, RegisterIndex),
    XOR(RegisterIndex, RegisterIndex),
    SHL(RegisterIndex),
    SHR(RegisterIndex),
    RND(RegisterIndex, u8),
}

impl Opcode {
    fn execute(&self, cpu: &mut CPU) {
        match self {
            CLS => (),
            DRW => (),
            CALL(addr) => {
                cpu.stack[cpu.SP] = cpu.PC;
                cpu.SP += 1;
                cpu.PC = addr
            },
            RET => {
                cpu.SP -= 1;
                cpu.PC = cpu.stack[cpu.SP];
            },
            JP(addr) => {
                cpu.PC = addr;
            },
            SE(lval, rval) => {
                if lval == rval {
                    cpu.PC += 2
                }
            },
            SNE(lval, rval) => {
                if lval != rval {
                    cpu.PC += 2
                }
            },
            SKP(rval) => {
                
            },
            SKNP(rval) => {
                
            },
            LD(lval, rval) => {
                *lval = *rval;
            },
            ADD(lval, rval) => {
                *lval + *rval;
            },
            SUB,
            SUBN,
            AND,
            OR,
            XOR,
            SHL,
            SHR,
            RND
        }
    }
}

fn reg_idx(opcode: u16, idx: u8) -> u8 {
    opcode & std::math::pow(idx, 8 * 2) >> (8 * idx - 1)
}

impl CPU {
    fn new() -> CPU {
        // init
    }

    fn load_game(&mut self, game: &str) {
        // Load file into memory, from 0x200 ? I think
    }

    fn emulate_cycle(&mut self) {
        let opcode = self.memory[self.PC] << 8 | self.memory[self.PC + 1];
        self.PC += 2;

        match opcode {
            0x00E0 => Opcode::CLS,
            0x00EE => Opcode::RET,
            0x1000 .. 0x1FFF => Opcode::JP(opcode & 0x0FFF),
            0x2000 .. 0x2FFF => Opcode::CALL(opcode & 0x0FFF),
            0x3000 .. 0x3FFF => Opcode::SE(reg_idx(opcode, 3), SE_val::Direct(opcode & 0x00FF)),
            0x4000 .. 0x4FFF => Opcode::SNE(reg_idx(opcode, 3), SE_val::Direct(opcode & 0x00FF)),
            num if num & 0xF00F == 0x5000 => Opcode::SE(reg_idx(opcode, 3), SE_val::Register(reg_idx(opcode, 2))),
            0x6000 .. 0x6FFF => Opcode::LD(LD_lval::Register(reg_idx(opcode, 3)), LD_val::Direct(opcode & 0x00FF)),
            0x7000 .. 0x7FFF => Opcode::ADD(reg_idx(opcode, 3), ADD_val::Direct(opcode & 0x0FF)),
            num if num & 0xF00F == 0x8000 => Opcode::LD(LD_lval::Register(reg_idx(opcode, 3)), LD_val::Register(reg_idx(opcode_2))),
            num if num & 0xF00F == 0x8001 => Opcode::OR(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x8002 => Opcode::AND(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x8003 => Opcode::XOR(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x8004 => Opcode::ADD(reg_idx(opcode, 3), ADD_val::Register(reg_idx(opcode, 2))),
            num if num & 0xF00F == 0x8005 => Opcode::SUB(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x8006 => Opcode::SHR(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x8007 => Opcode::SUBN(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x800E => Opcode::SHL(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            num if num & 0xF00F == 0x9000 => Opcode::SNE(reg_idx(opcode, 3), reg_idx(opcode, 2)),
            0xA000 .. 0xAFFF => Opcode::LD(LD_lval::Index, LD_val::Address(opcode & 0x0FFF)),
            0xB000 .. 0xBFFF => (), // TODO
            0xC000 .. 0xCFFF => Opcode::RND(reg_idx(opcode, 3), opcode & 0x00FF),
            0xD000 .. 0xDFFF => Opcode::DRW(reg_idx(opcode, 3), reg_idx(opcode, 2), reg_idx(opcode, 1)), // Not really an index but it still works
            num if num & 0xF0FF == 0xE09E => Opcode::SKP(reg_idx(opcode, 3)),
            num if num & 0xF0FF == 0xE0A1 => Opcode::SKNP(reg_idx(opcode, 3)),
            num if num & 0xF0FF == 0xF007 => Opcode::LD(LD_lval::Register(reg_idx(opcode, 3)), LD_val::DelayTimer),
            num if num & 0xF0FF == 0xF00A => Opcode::LD(LD_lval::Register(reg_idx(opcode, 3)), LD_val::Keyboard), // TODO: This LD is really different. I should give it its own opcode I think
            num if num & 0xF0FF == 0xF015 => Opcode::LD(LD_lval::DelayTimer, LD_val::Register(reg_idx(opcode, 3))),
            num if num & 0xF0FF == 0xF018 => Opcode::LD(LD_lval::SoundTimer, LD_val::Register(),
            num if num & 0xF0FF == 0xF01E => (),
            num if num & 0xF0FF == 0xF029 => (),
            num if num & 0xF0FF == 0xF033 => (),
            num if num & 0xF0FF == 0xF055 => (),
            num if num & 0xF0FF == 0xF065 => (),
        }

        // decode opcode
        // execute opcode

        // update timers
    }
}

fn main() {
    setup_graphics();
    setup_inputs();

    let cpu = CPU::new();
    cpu.load_game("pong");

    loop {
        cpu.emulate_cycle();
        if cpu.should_draw() {
            draw_graphics(cpu);
        }

        cpu.set_keys();
    }
}
