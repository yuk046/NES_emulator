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
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // フラグ変更
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

    // &mut selfつまりこの関数内でオブジェクトの状態を変更できる。 引数としてu8型の列programを取る
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                // LDA - Load Accumulator
                0xA9 => {
                    // paramはLDAの引数
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }
                // BRK - Loop Break
                0x00 => {
                    return;
                }
                // TAX - Transfer Accumulator to X
                0xAA => self.tax(),
                _ => todo!(),
            }
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    // zeroフラグが立つLADテスト
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    // negativeフラグが立つLADテスト
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x80, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    // register_xからregister_aにcopyのTAXテスト
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }
}
