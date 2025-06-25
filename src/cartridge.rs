const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOUR_SCREEN,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

pub fn mock_rom(code: Vec<u8>) -> Rom {
    Rom {
        prg_rom: code,
        chr_rom: [].to_vec(),
        mapper: 0,
        screen_mirroring: Mirroring::VERTICAL,
    }
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
        if raw.len() < 16 {
            return Err("This is not an iNES file!".to_string());
        }
        if raw[..4] != NES_TAG {
            return Err("Wrong Nes Tag!".to_string());
        }

        let prg_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let ctrl_byte1 = raw[6];
        let ctrl_byte2 = raw[7];

        let vertical_mirroring = (ctrl_byte1 & 0b0000_0001) == 1;
        let _batery_ram = ctrl_byte1 & 0b0000_0010;
        let has_trainer = (ctrl_byte1 & 0b0000_0100) != 0;
        let four_screen = (ctrl_byte1 & 0b0000_1000) != 0;
        let mapper_lo = ctrl_byte1 >> 4;

        let _ines_20 = (ctrl_byte2 & 0b0000_1100) == 0b0000_1000;
        let ines_10 = (ctrl_byte2 & 0b0000_1100) == 0b0000_0000;

        if ines_10 && (ctrl_byte2 & 0b0000_0011 != 0) {
            return Err("Rom is iNES 1.0 but control bytes are not zero!".to_string());
        }

        let mapper_hi = ctrl_byte2 & 0b1111_0000;
        let mapper = mapper_hi | mapper_lo;

        let _prg_ram_size = raw[8] as usize * 8 * 1024;

        let screen_mirroring = match (vertical_mirroring, four_screen) {
            (_, true) => Mirroring::FOUR_SCREEN,
            (true, false) => Mirroring::VERTICAL,
            (false, false) => Mirroring::HORIZONTAL,
        };
        let prg_rom_begin = 16 + if has_trainer { 512 } else { 0 };
        let chr_rom_begin = prg_rom_begin + prg_size;

        return Ok(Rom {
            prg_rom: raw[prg_rom_begin .. prg_rom_begin + prg_size].to_vec(),
            chr_rom: raw[chr_rom_begin .. chr_rom_begin + chr_size].to_vec(),
            mapper: mapper,
            screen_mirroring: screen_mirroring,
        });
    }
}

mod test {
    use std::{fs::File, io::Read, path::{self, Path}};

    use super::*;

    struct TestRom {
        header: Vec<u8>,
        trainer: Option<Vec<u8>>,
        pgr_rom: Vec<u8>,
        chr_rom: Vec<u8>
    }

    fn create_rom(rom: TestRom) -> Vec<u8> {
        let mut result = Vec::with_capacity(
            rom.header.len()
            + rom.trainer.as_ref().map_or(0, |t| t.len())
            + rom.pgr_rom.len()
            + rom.chr_rom.len()
        );
        result.extend(&rom.header);
        if let Some(t) = rom.trainer {
            result.extend(t)
        }
        result.extend(&rom.pgr_rom);
        result.extend(&rom.chr_rom);
        result
    }

    #[test]
    fn wrong_nes_tag() {
        let res = Rom::new(&vec![
            0x01, 0x20, 0x00, 0x32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        match res {
            Result::Ok(_) => assert!(false, "should not load room"),
            Result::Err(s) => assert_eq!(s, "Wrong Nes Tag!"),
        }
    }

    #[test]
    fn correct_nes_tag() {
        let res = Rom::new(&vec![
            0x4E, 0x45, 0x53, 0x1A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        match res {
            Result::Ok(_) => assert!(true),
            Result::Err(s) => assert_eq!(s, "Wrong Nes Tag!"),
        }
    }

    #[test]
    fn test_pgp_chr() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 0x10, 00, 00, 00, 00, 00, 00, 00, 00,
            ],
            // 13 = 0b0000_0001_0000_0011
            // 31 = 0b0000_0011_0000_0001
            trainer: None,
            pgr_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        let rom = Rom::new(&test_rom).unwrap();

        assert_eq!(rom.chr_rom, vec![2; 1 * CHR_ROM_PAGE_SIZE]);
        assert_eq!(rom.prg_rom, vec![1; 2 * PRG_ROM_PAGE_SIZE]);
        assert_eq!(rom.mapper, 0x13);
        assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
    }

    #[test]
    fn test_trainer() {
        let test_rom = create_rom(TestRom {
            header: vec![
                0x4E, 0x45, 0x53, 0x1A, 
                0x02, 0x01, 0x31 | 0b100, 0x10, 
                00, 00, 00, 00, 00, 00, 00, 00,
            ],
            // 31 = 0b0000_0011_0000_0001
            trainer: Some(vec![5; 512]),
            pgr_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
            chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
        });

        let rom = Rom::new(&test_rom).unwrap();

        assert_eq!(rom.chr_rom, vec![2; 1 * CHR_ROM_PAGE_SIZE]);
        assert_eq!(rom.prg_rom, vec![1; 2 * PRG_ROM_PAGE_SIZE]);
        assert_eq!(rom.mapper, 0x13);
        // assert_eq!(rom.trainer, vec![5;512]);
        assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
    }

    #[test]
    fn real_file(){
        let path = Path::new("roms/snake.nes");
        let mut file = File::open(&path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let res = Rom::new(&buffer);

        match res {
            Result::Ok(_) => assert!(true),
            Result::Err(s) => assert!(false)
        }


    }
}
