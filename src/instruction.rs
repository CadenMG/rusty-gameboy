enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

enum StackTarget { BC, DE }

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

enum Instruction {
    ADD(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT,
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }
    
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
    
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
