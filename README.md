# NESエミュレータ開発メモ（Rust）

## 現在の進捗
- 🔖 CPUの命令実装（アドレッシングモード）

## based on
- 📘 [Writing NES Emulator in Rust](https://bugzmanov.github.io/nes_ebook/)
- 📚 [obelisk-6502-guide](https://www.nesdev.org/obelisk-6502-guide/)
- 🧪 [Easy 6502(AssemblyLanguageTest)](https://skilldrick.github.io/easy6502/)

---

## CPUレジスタ一覧

| レジスタ名 | 説明 |
|------------|------|
| **Program Counter (PC)** | 次に実行される機械語命令のアドレスを保持します。 |
| **Stack Pointer (SP)** | メモリ領域 `[0x0100 .. 0x01FF]` をスタックとして使用。スタックは**上から下に成長**する。<br>・プッシュ：SP をデクリメント<br>・ポップ：SP をインクリメント |
| **Accumulator (A)** | 算術・論理演算・メモリアクセスの結果を保持。また、一部命令の入力パラメータとしても使用されます。 |
| **Index Register X (X)** | アドレッシングモードでのオフセットや一時値・カウンターとして使用。 |
| **Index Register Y (Y)** | Xレジスタと同様に、補助的な用途で使用。 |
| **Processor Status (P)** | 8ビットのステータスレジスタ。命令の実行結果に応じて、以下のフラグがセット/リセットされます。<br>例：Zフラグは演算結果が `0` のときセットされる。 |

---

## アドレッシングモード

| モード名 | 説明 |
|----------|------|
| **Immediate** | 即値を直接指定して使用（例：`LDA #$10`） |
| **Zero Page** | アドレスの上位バイトが`$00`（1バイトアドレス）。高速なアクセスが可能。 |
| **Absolute** | 通常の2バイトアドレス参照。フルレンジのメモリアクセスが可能。 |
| **Indirect** | 指定されたアドレスに格納された**実際のアドレス**を使ってアクセスする。<br>例：`JMP ($1234)` は、アドレス `$1234` と `$1235` に格納された値を使ってジャンプ先を決定。<br>⚠ 注意：`$xxFF` をまたぐ場合、バグとして下位バイトだけが次のページから読み込まれる（6502 CPU の仕様バグ） |


---
