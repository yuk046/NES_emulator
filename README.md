
Writing NES Emulator in Rust

CPU Registers

・Program Counter (PC) - holds the address for the next machine language instruction to be executed.

・Stack Pointer - Memory space [0x0100 .. 0x1FF] is used for stack. The stack pointer holds the address of the top of that space. NES Stack (as all stacks) grows from top to bottom: when a byte gets pushed to the stack, SP register decrements. When a byte is retrieved from the stack, SP register increments.

・Accumulator (A) - stores the results of arithmetic, logic, and memory access operations. It used as an input parameter for some operations.

・Index Register X (X) - used as an offset in specific memory addressing modes (more on this later). Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)

・Index Register Y (Y) - similar use cases as register X.

・Processor status (P) - 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
