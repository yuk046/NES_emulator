use std::{ops::Add, result};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)] //debugの書式指定を利用するために
#[allow(non_camel_case_types)] //キャメルケース以外を利用する
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    // CPUのレジスターを定義
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0x10000], //0xFFFF
}

//  rustでstructの関数を定義する際はimpl内に記述する
impl CPU {
    // コンストラクタを定義 各フィールドを初期化して構造体(self)を返すコンストラクタ
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0x00; 0x10000],
        }
    }

    // mutに変更した。
    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            // 1byteアドレスにregister_xの値を足す
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            // 1byteアドレスにregister_yの値を足す
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            // 2byteアドレスにregister_xの値を足す
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            // 2byteアドレスにregister_yの値を足す
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            // 1byteアドレスにregister_xの値を足し、そのアドレスの値と次のアドレスの値2byteをアドレスとする
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            // 1byteアドレスと次のアドレスの値をderef_baseとし、deref_baseにregister_yを足したのが最終的なアドレス
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    // memory番地を受け取り格納値を返す
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    // memory番地とdateを受け取り、番地に値を格納する
    fn mem_write(&mut self, addr: u16, date: u8) {
        self.memory[addr as usize] = date;
    }
    // 2byteのデータを取る際のmem_read　リトルエンディアンアドレッシング
    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
    // 2byteの値を格納するmem_write
    fn mem_write_u16(&mut self, pos: u16, date: u16) {
        let hi = (date >> 8) as u8;
        let lo = (date & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    // reset関数
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    // programを受け取りメモリの 0x8000 番地からroadして実行
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }
    // プログラムのバイト列を、メモリの 0x8000 番地から書き込んで、そこから実行開始するようにPCをセットする
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    // メモリの 0x8000 番地からopscode読み込んで実行
    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                // LDA - Load Accumulator
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xB5 => {
                    self.lda(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }
                0xAD => {
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0xBD => {
                    self.lda(&AddressingMode::Absolute_X);
                    self.program_counter += 2;
                }
                0xB9 => {
                    self.lda(&AddressingMode::Absolute_Y);
                    self.program_counter += 2;
                }
                0xA1 => {
                    self.lda(&AddressingMode::Indirect_X);
                    self.program_counter += 1;
                }
                0xB1 => {
                    self.lda(&AddressingMode::Indirect_Y);
                    self.program_counter += 1;
                }
                // BRK - Loop Break
                0x00 => {
                    return;
                }
                // TAX - Transfer Accumulator to X
                0xAA => self.tax(),
                0xe8 => self.inx(),
                _ => todo!(),
            }
        }
    }

    // 引数で取った値をアキュムレータに格納
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // アキュムレータの値をregister_xにコピー
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // register_xをインクリメント
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    // ゼロフラグとネガティブフラグ変更
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // もしresultが0ならzeroフラグを立てる
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
        // もしresultの7番目が立っているのならNegativeフラグを立てる
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

// テスト
#[cfg(test)] //条件付きコンパイルを利用する
mod test {
    use super::*;

    #[test]
    // フラグが立たないLDAテスト
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    // zeroフラグが立つLADテスト
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    // negativeフラグが立つLADテスト
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    // register_xからregister_aにcopyのTAXテスト
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xAA, 0x00]);
        assert_eq!(cpu.register_x, 0x0A);
    }

    // 結合テスト
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

    // register_xのオーバーフローテスト
    #[test]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]); // LDA #$FF, TAX, INX, BRK
        assert_eq!(cpu.register_x, 0x00); // wrap around
        assert!(cpu.status & 0b0000_0010 != 0); // Zero flag should be set
    }
}
