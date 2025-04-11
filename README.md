NOW
CPUの命令実装（アドレッシングモード）

Based on　"Writing NES Emulator in Rust"
https://bugzmanov.github.io/nes_ebook/

CPU Registers

・Program Counter (PC) - 次に実行される機械語命令のアドレスを保持します。

・Stack Pointer - メモリ領域 [0x0100 .. 0x01FF] はスタックとして使用されます。スタックポインタはそのスタックの一番上のアドレスを保持します。NESのスタック（他の多くのスタックと同様に）は上から下へ成長します。つまり、バイトがスタックにプッシュされるとSPレジスタはデクリメント（減少）し、バイトがスタックから取得されるとSPレジスタはインクリメント（増加）します。

・Accumulator (A) - 算術、論理、メモリアクセスの結果を保持します。また、いくつかの命令において入力パラメータとしても使用されます。

・Index Register X (X) - 特定のメモリアドレッシングモードにおいてオフセットとして使用されます。また、一時的な値を保持したり、カウンターとして使用されるなど、補助的な目的にも使用されます。

・Index Register Y (Y) - Xレジスタと同様に、上記のような用途で使用されます。

・Processor status (P) - 8ビットのレジスタで、命令の実行結果に応じて設定・解除される7つのステータスフラグを表します（たとえば、Zフラグは演算結果が0であればセット（1）され、それ以外の場合はリセット（0）されます）。
