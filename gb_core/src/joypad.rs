use crate::addr::AddrSpace;
use crate::consts::JOYPAD_ADDR;

pub struct JoypadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl JoypadState {
    pub fn new() -> Self {
        JoypadState {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    fn get_bitset_1(&self) -> u8 {
        let mut bitset = 0x10;
        if self.right {
            bitset = bitset | 1;
        }
        if self.left {
            bitset |= 1 << 1;
        }
        if self.up {
            bitset |= 1 << 2;
        }
        if self.down {
            bitset |= 1 << 3;
        }
        !bitset
    }

    fn get_bitset_2(&self) -> u8 {
        let mut bitset = 0x20;
        if self.a {
            bitset |= 1;
        }
        if self.b {
            bitset |= 1 << 1;
        }
        if self.select {
            bitset |= 1 << 2;
        }
        if self.start {
            bitset |= 1 << 3;
        }
        !bitset
    }

    pub fn update_joypad(&self, addr_space: &mut AddrSpace) {
        let mut joypad = addr_space.read(JOYPAD_ADDR);
        if joypad & 0x10 == 0 {
            joypad = self.get_bitset_1();
        } else if joypad & 0x20 == 0 {
            joypad = self.get_bitset_2();
        }
        addr_space.write(JOYPAD_ADDR, joypad);
    }

    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
        self.a = false;
        self.b = false;
        self.start = false;
        self.select = false;
    }
}
