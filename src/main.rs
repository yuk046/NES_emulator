fn main() {
    println!("Hello, world!");
}

pub struct CPU {
    // CPUのレジスターを定義
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
 }

//  rustでstructの関数を定義する際はimpl内に記述する
 impl CPU {
    // コンストラクタを定義 各フィールドを初期化して構造体(self)を返すコンストラクタ
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    // &mut selfつまりこの関数内でオブジェクトの状態を変更できる。 引数としてu8型の列programを取る
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let  opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                // LDA - Load Accumulator
                0xA9 => {
                    // paramはLDAの引数
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    // もしアキュムレータが0ならzeroフラグを立てる
                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }
                    // もしアキュムレータの7番目が立っているのならNegativeフラグを立てる
                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1101;
                    }
                }
                _ => todo!(),
            }
        }
    } 
 }