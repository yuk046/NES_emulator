use std::result;

fn main() {
    println!("Hello, world!");
}

pub struct CPU {
    // CPUのレジスターを定義
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

//  rustでstructの関数を定義する際はimpl内に記述する
impl CPU {
    // コンストラクタを定義 各フィールドを初期化して構造体(self)を返すコンストラクタ
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0x00;0xFFFF],
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
    // 引数に2byte取る際のmem_read　リトルエンディアンアドレッシング
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
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
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
                    // paramはLDAの引数
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;

                    self.lda(param);
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
    fn lda(&mut self, value: u8) {
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
        if self.register_x == 0b1111_1111 {
            self.register_x = 0;
        } else {
            self.register_x += 1;
        }
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
#[cfg(test)]
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
        cpu.register_a = 10;
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10);
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
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1)
    }

}
