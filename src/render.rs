use crate::palete;
use crate::ppu::NesPPU;

use crate::frame::Frame;

pub fn render(ppu: &NesPPU, frame: &mut Frame) {
    let bank = ppu.ctrl.bkgd_pattern_addr();

    for i in 0..0x03c0 {
        let tile = ppu.vram[i] as u16;
        let tile_x = i % 32;
        let tile_y = i / 32;
        let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;

                let rgb = match value {
                    0 => palete::SYSTEM_PALLETE[0x01],
                    1 => palete::SYSTEM_PALLETE[0x23],
                    2 => palete::SYSTEM_PALLETE[0x27],
                    3 => palete::SYSTEM_PALLETE[0x30],
                    _ => panic!("Can't be"),
                };
                frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb);
            }
        }
    }
}