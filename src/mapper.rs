use rom;

pub struct Mapper {
    upper_bank: [u8; 0x4000],
    lower_bank: [u8; 0x4000],
    rom: rom::iNESFile,
}

impl Mapper {
    pub fn new(rom: rom::iNESFile) -> Mapper {
        let mut up = [0u8; 0x4000];
        let mut lo = [0u8; 0x4000];

        if rom.mapper == 0 {
            if rom.prg_rom_cnt == 1 {
                // load single bank to upper
                up = rom.prg_rom[0];
            }
            else {
                lo = rom.prg_rom[0];
                up = rom.prg_rom[1];
            }
        }

        Mapper { upper_bank: up, lower_bank: lo, rom: rom }
    }
    pub fn read(&self, address: u16) -> u8 {
        if address < 0xC000 {
            self.lower_bank[(address - 0x8000) as usize]
        }
        else {
            self.upper_bank[(address - 0xC000) as usize]
        }
    }
}
