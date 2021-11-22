use super::consts;

pub struct AddrSpace {
    bios: [u8; 0x0100],
    bank0: [u8; 0x4000],
    bank1: [u8; 0x4000],
    video_ram: [u8; 0x2000],
    external_ram: [u8; 0x2000],
    work_ram1: [u8; 0x1000],
    work_ram2: [u8; 0x1000],
    sprite_table: [u8; 0xA0],
    io_registers: [u8; 0x80],
    hram: [u8; 0x7F],
    interrupt_enable_register: u8,

    memory_model: u8,
    rom_bank: u8,
    cartridge: Option<Vec<u8>>,
    running_bios: bool,
}

impl AddrSpace {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0..=0xFF if self.running_bios => self.bios[addr as usize],
            0x0..=0x3FFF => self.bank0[addr as usize],
            0x4000..=0x7FFF => self.bank1[(addr - 0x4000) as usize],
            0x8000..=0x9FFF => self.video_ram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.external_ram[(addr - 0xA000) as usize],
            0xC000..=0xCFFF => self.work_ram1[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.work_ram2[(addr - 0xD000) as usize],
            0xE000..=0xFDFF => self.read(addr - 0x2000),
            0xFE00..=0xFE9F => self.sprite_table[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => {
                println!("Prohibited memory address (read)");
                0x00
            }
            0xFF00..=0xFF7F => self.io_registers[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable_register,
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xff46 => {
                for i in 0..0x9F {
                    self.write(0xFE00 | i, self.read(((data as u16) << 8) | i));
                }
            }
            //TODO: cover all the cartridge types and memory models :(
            0x6000..=0x7FFF if *self.cartridge_type() == 1 => {
                self.memory_model = 0x1 & data;
            }
            0x2000..=0x3FFF if *self.cartridge_type() == 1 => {
                self.rom_bank = 0x1.max(0x1F & data);
                self.load_bank(self.rom_bank);
            }
            0x0..=0x3FFF => self.bank0[addr as usize] = data,
            0x4000..=0x7FFF => self.bank1[(addr - 0x4000) as usize] = data,
            0x8000..=0x9FFF => self.video_ram[(addr - 0x8000) as usize] = data,
            0xA000..=0xBFFF => self.external_ram[(addr - 0xA000) as usize] = data,
            0xC000..=0xCFFF => self.work_ram1[(addr - 0xC000) as usize] = data,
            0xD000..=0xDFFF => self.work_ram2[(addr - 0xD000) as usize] = data,
            0xE000..=0xFDFF => self.write(addr - 0x2000, data),
            0xFE00..=0xFE9F => self.sprite_table[(addr - 0xFE00) as usize] = data,
            0xFEA0..=0xFEFF => println!("Prohibited memory address {:04x} (write)", addr),
            0xFF00..=0xFF7F => self.io_registers[(addr - 0xFF00) as usize] = data,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,
            0xFFFF => self.interrupt_enable_register = data,
        };
    }

    pub fn deactivate_bios(&mut self) {
        self.running_bios = false;
    }

    pub fn empty() -> AddrSpace {
        AddrSpace {
            bios: [0; 0x0100],
            bank0: [0; 0x4000],
            bank1: [0; 0x4000],
            video_ram: [0x00; 0x2000],
            external_ram: [0; 0x2000],
            work_ram1: [0; 0x1000],
            work_ram2: [0; 0x1000],
            sprite_table: [0; 0xA0],
            io_registers: [0xff; 0x80],
            hram: [0; 0x7F],
            interrupt_enable_register: 0,
            memory_model: 0,
            rom_bank: 0,
            cartridge: None,
            running_bios: false,
        }
    }

    pub fn new(bios: [u8; 0x100], cartridge: Option<Vec<u8>>) -> AddrSpace {
        let mut addr_space = AddrSpace {
            bios,
            bank0: [0; 0x4000],
            bank1: [0; 0x4000],
            video_ram: [0x00; 0x2000],
            external_ram: [0; 0x2000],
            work_ram1: [0; 0x1000],
            work_ram2: [0; 0x1000],
            sprite_table: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            interrupt_enable_register: 0,
            memory_model: 0,
            rom_bank: 0,
            cartridge,
            running_bios: true,
        };

        addr_space.load_cartridge_head();

        println!("Game title: {}", addr_space.game_title());
        println!("Cartridge type: {}", *addr_space.cartridge_type());
        println!("Rom size: {}", *addr_space.rom_size());
        addr_space
    }

    fn load_bank(&mut self, bank: u8) {
        if let Some(cart) = &self.cartridge {
            let bank_start = bank as usize * 0x4000;
            let bank_end = bank_start + 0x4000;
            self.bank1.clone_from_slice(&cart[bank_start..bank_end]);
        }
    }

    fn load_cartridge_head(&mut self) {
        if let Some(cart) = &self.cartridge {
            self.bank0[0x000..=0x3FFF].clone_from_slice(&cart[0x000..=0x3FFF]);
            self.bank1[0..=0x3FFF].clone_from_slice(&cart[0x4000..=0x7FFF]);
        }
    }

    pub fn reset(&mut self) {
        self.bank0 = [0; 0x4000];
        self.bank1 = [0; 0x4000];
        self.video_ram = [0x00; 0x2000];
        self.external_ram = [0; 0x2000];
        self.work_ram1 = [0; 0x1000];
        self.work_ram2 = [0; 0x1000];
        self.sprite_table = [0; 0xA0];
        self.io_registers = [0; 0x80];
        self.hram = [0; 0x7F];
        self.interrupt_enable_register = 0;
        self.memory_model = 0;
        self.rom_bank = 0;
        self.running_bios = true;
        self.load_cartridge_head();
    }

    // Cartridge info

    pub fn game_title(&self) -> String {
        std::str::from_utf8(&self.bank0[0x134..=0x142])
            .unwrap()
            .replace(0 as char, "")
    }

    pub fn is_color_gb(&self) -> bool {
        self.bank0[0x143] == 0x80
    }

    pub fn gb_indicator(&self) -> &u8 {
        &self.bank0[0x146]
    }

    pub fn cartridge_type(&self) -> &u8 {
        &self.bank0[0x147]
    }

    pub fn rom_size(&self) -> &u8 {
        &self.bank0[0x148]
    }

    pub fn ram_size(&self) -> &u8 {
        &self.bank0[0x149]
    }

    pub fn bg_tile_map_area(&self) -> bool {
        self.read(0xFF40) & 0x08 == 0x08
    }

    pub fn bg_window_tile_data_area(&self) -> bool {
        self.read(0xFF40) & 0x10 == 0x10
    }

    pub fn window_enable(&self) -> bool {
        self.read(0xFF40) & 0x20 == 0x20
    }

    pub fn window_tile_map_area(&self) -> bool {
        self.read(0xFF40) & 0x40 == 0x40
    }

    fn set_if_flag(&mut self, value: bool, bit: u32) {
        let if_value = if value {
            self.read(consts::IF_ADDR) | 2_u8.pow(bit)
        } else {
            self.read(consts::IF_ADDR) & !2_u8.pow(bit)
        };
        self.write(consts::IF_ADDR, if_value);
    }

    pub fn ie_vblank(&self) -> bool {
        self.read(consts::IE_ADDR) & 0x01 == 0x01
    }

    pub fn ie_lc_stat(&self) -> bool {
        self.read(consts::IE_ADDR) & 0x02 == 0x02
    }

    pub fn ie_timer(&self) -> bool {
        self.read(consts::IE_ADDR) & 0x04 == 0x04
    }

    pub fn ie_serial(&self) -> bool {
        self.read(consts::IE_ADDR) & 0x08 == 0x08
    }

    pub fn ie_joypad(&self) -> bool {
        self.read(consts::IE_ADDR) & 0x10 == 0x10
    }

    pub fn if_vblank(&self) -> bool {
        self.read(consts::IF_ADDR) & 0x01 == 0x01
    }

    pub fn if_lc_stat(&self) -> bool {
        self.read(consts::IF_ADDR) & 0x02 == 0x02
    }

    pub fn if_timer(&self) -> bool {
        self.read(consts::IF_ADDR) & 0x04 == 0x04
    }

    pub fn if_serial(&self) -> bool {
        self.read(consts::IF_ADDR) & 0x08 == 0x08
    }

    pub fn if_joypad(&self) -> bool {
        self.read(consts::IF_ADDR) & 0x10 == 0x10
    }

    pub fn set_if_vblank(&mut self, value: bool) {
        self.set_if_flag(value, 0);
    }

    pub fn set_if_lc_stat(&mut self, value: bool) {
        self.set_if_flag(value, 1);
    }

    pub fn set_if_timer(&mut self, value: bool) {
        self.set_if_flag(value, 2);
    }

    pub fn set_if_serial(&mut self, value: bool) {
        self.set_if_flag(value, 3);
    }

    pub fn set_if_joypad(&mut self, value: bool) {
        self.set_if_flag(value, 4);
    }
}
