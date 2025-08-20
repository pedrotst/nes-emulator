pub mod bus;
pub mod byte_utils;
pub mod cartridge;
pub mod cpu;
pub mod opcodes;
pub mod ppu;
pub mod render;
pub mod trace;

use bus::Bus;
use bus::BusOP;
use cartridge::Rom;
use cpu::CPU;
use cpu::Mem;
use ppu::NesPPU;
use render::frame::Frame;
use render::palette;
use trace::trace;

// use rand::Rng;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

use std::path::Path;

// const ROM: &str = "snake.nes";
// const ROM: &str = "nestest.nes";
const ROM: &str = "pacman.nes";

#[allow(dead_code)]
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Nes Emulator", (256 * 3) as u32, (240 * 3) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let path = Path::new("roms/").join(ROM);

    let bytes = std::fs::read(path).unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();

    let bus = Bus::new(rom, move |ppu: &NesPPU| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => { /* Do Nothing */ }
            }
        }
    });
    let mut cpu = CPU::new(bus);
    println!("resetting");
    cpu.reset();
    println!("running");
    cpu.run_with_callback(|cpu| {
        println!("{}", trace(cpu));
    });
}

#[allow(dead_code)]
fn handle_user_input<T: BusOP>(cpu: &mut CPU<T>, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                std::process::exit(0);
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                cpu.mem_write(0xff, 0x77);
            }

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                cpu.mem_write(0xff, 0x73);
            }

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                cpu.mem_write(0xff, 0x61);
            }

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                cpu.mem_write(0xff, 0x64);
            }

            _ => { /* do nothing */ }
        }
    }
}

#[allow(dead_code)]
fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

#[allow(dead_code)]
fn read_screen_state<T: BusOP>(mut cpu: CPU<T>, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true
        }
        frame_idx += 3;
    }
    update
}

#[allow(dead_code)]
fn show_tile(chr_rom: &Vec<u8>, bank: usize, tile_n: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let bank = (bank * 0x1000) as usize;

    let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];
    for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        for x in (0..=7).rev() {
            let value = (1 & upper) << 1 | (1 & lower);
            upper = upper >> 1;
            lower = lower >> 1;
            let rgb = match value {
                0 => palette::SYSTEM_PALETTE[0x01],
                1 => palette::SYSTEM_PALETTE[0x23],
                2 => palette::SYSTEM_PALETTE[0x27],
                3 => palette::SYSTEM_PALETTE[0x30],
                _ => panic!("can't be"),
            };
            frame.set_pixel(x, y, rgb);
        }
    }
    frame
}
