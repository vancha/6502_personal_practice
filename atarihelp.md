# Atari 2600 Assembly
## A complete course from zero to a finished game

**Target:** Atari 2600 / 6507 processor  
**Instruction set:** NMOS 6502-compatible instructions  
**Assembler:** DASM  
**Emulator:** Stella  
**Final project:** OUTPOST — an original Atari 2600 game
 
---

# How to use this workbook

Work through the exercises in order. Each exercise introduces concepts that will be needed later.

For every exercise:

1. Read the assignment and its constraints.
2. Identify what you need to learn.
3. Write the assembly yourself.
4. Assemble and test your program.
5. Verify every acceptance criterion.
6. Explain why your implementation works.

Do not simply aim to make the program work. Aim to understand why it works.

## Conventions

- Hexadecimal numbers use `$` prefixes: `$FF`.
- Binary numbers use `%` prefixes: `%10101010`.
- Decimal numbers have no prefix: `42`.
- Addresses and values are different things.
- Unless specified otherwise, assume initial RAM contents are unknown.
- All programs must assemble successfully with DASM.
- Use documented instructions supported by the Atari 2600's processor.
- Exercise-specific restrictions are mandatory.

## Development environment

Install these tools before beginning the Atari-specific exercises:

| Tool | Purpose |
|---|---|
| DASM | Assembly compiler |
| Stella | Atari 2600 emulator |
| Stella debugger | Inspect CPU and hardware state |
| Text editor | Write assembly |
| Git | Track your progress and revisions |

You may write and test standalone CPU exercises before creating complete Atari ROMs.

---

# PART 1 — Assembly foundations (Learn 6502 edition)

**Difficulty:** 1/10  
**Exercises:** 1–10

**Learning objective:** Understand the CPU, registers, instructions, arithmetic, memory and basic control flow.

## Before starting

These assignments target the Learn 6502 application.

- Write your assembly directly in its built-in editor.
- Do not use DASM directives such as `processor` or `org`.
- Your program starts at `$0600`.
- Use `BRK` to stop a program when appropriate.
- Use the debugger to inspect registers, flags and memory.
- Use memory addresses `$0080` onwards for our ordinary data exercises. Avoid `$0200`–`$05FF`, which are reserved for the virtual display.

---

## Exercise 01 — Hello, accumulator!

**Objective:** Load a value into the CPU.

**Background:** The processor has a small number of internal storage locations called registers. The accumulator is an 8-bit register used extensively for arithmetic and data manipulation.

**Assignment:**

Write a program that places decimal `42` into the accumulator.

```6502
LDA #42
```
**Requirements:**
- The accumulator must initially be treated as containing an unknown value.
- The final accumulator value must be exactly 42.
- The value must be loaded directly, not retrieved from memory.
- No other registers need to be modified.

**Constraints:**
- Use one data-loading instruction.
- Do not access RAM.
- Do not perform arithmetic.

**Acceptance criteria:**
- The program assembles and executes successfully.
- The debugger shows the accumulator containing `$2A`.
- You can explain the relationship between decimal 42 and hexadecimal `$2A`.

**Questions:**

1. What is a CPU register?
2. Why is the accumulator only 8 bits wide?
3. What is immediate addressing?
4. What is the difference between a value and an address?

```
Answers:
1. A register is a small but fast reserved spot inside a cpu for holding data.
2. Because the data that is available to it is also 8 bits in size. 
3. Providing arguments to functions as they are, without having to get them from an actual address to use them.
4. a values *is* the data, and an address could be where data is located.

```
---

## Exercise 02 — Your first memory write

**Objective:** Understand the relationship between CPU registers and RAM.

**Background:** The CPU can transfer information between its internal registers and external memory. Memory is addressed using numerical locations.

**Assignment:**

Write a program that places decimal `42` into memory address `$80`.

```6502
LDA #$2A
STA $80
```
**Requirements:**
- The destination must be `$80`.
- The stored value must be exactly `$2A`.
- The program must not depend on the initial contents of `$80`.
- The final accumulator value must remain unchanged from its value immediately before the memory write.

**Constraints:**
- Use no additional memory locations.
- Do not perform arithmetic.
- Do not use loops or branches.

**Acceptance criteria:**
- Address `$80` contains `$2A`.
- The accumulator retains its original value.
- No other memory address is intentionally modified.

**Questions:**

1. How does the CPU distinguish between an instruction's opcode and the operand that follows it?
2. Why does writing to memory not necessarily change a register?
3. What is the difference between RAM and ROM?

```
Answers:
1. Because the instruction is only ever one byte, it is used to infer what comes after said byte. E.g, load into a register can have both direct addressing or index addressing, but you can tell from the first byte which it is.
2. Because the instruction limits itself to a single operation.
3. Ram is also writeable where ROM is only readable.
```
---

## Exercise 03 — Meet the X register

**Objective:** Understand the CPU's index registers.

**Background:** Besides the accumulator, the 6502 has two additional general-purpose registers: X and Y. These are frequently used as counters and indexes when working with memory.

**Assignment:**

Write a program that places decimal `10` into the X register.

```
LDX #$0A
```

**Requirements:**
- X must initially be treated as containing an unknown value.
- The final X value must be `$0A`.
- The value must be loaded directly.

**Constraints:**
- Do not use RAM.
- Do not perform arithmetic.
- Do not modify the accumulator or Y register.

**Acceptance criteria:**
- The debugger shows X containing `$0A`.
- You can explain why X and the accumulator are separate registers.

**Questions:**

1. What is an index register?
2. How does X differ from the accumulator?
3. Why might a programmer need more than one register?


```Answers
1. 8 bit registers (X and Y) that are used for adressing. 
2. The x and a registers differ in which instructions operate on it, most math operations are done on the accumulator for example, and storing and juggling data around is often done with x.
3. You might already need multiple for looping: one to keep track of an index, and the other to compare against to know if you've looped over enough indices or not?
```
---

## Exercise 04 — Moving values between registers

**Objective:** Understand register-transfer instructions.

**Background:** The 6502 has instructions that copy values between its registers. These transfers do not perform arithmetic.

**Assignment:**

Write a program that demonstrates transferring values between the accumulator and both index registers.

```
LDA #42
TAX
TAY
```

**Requirements:**
- Demonstrate transfers in both directions between A and X.
- Demonstrate transfers in both directions between A and Y.
- Verify the resulting register contents using the debugger.

**Constraints:**
- Do not use RAM.
- Do not perform arithmetic.
- Do not use loops or branches.

**Acceptance criteria:**
- All four transfer operations have been demonstrated.
- You can identify which register is the source and which is the destination for each operation.

**Questions:**

1. What does a register-transfer instruction do?
2. Does transferring a value remove it from the source register?
3. Why might copying a value be useful?

```
Answers:
1. A register transfer instruction transfers data between registers.
2. No it does not
3. Copying a value might be useful because you want to remember it's value after applying another instruction that can potentially be destructive on the value? 
```

---

## Exercise 05 — Exchanging values

**Objective:** Understand temporary storage and register manipulation.

**Background:** The 6502 does not have a dedicated instruction that swaps the contents of A and X. Sometimes a programmer must temporarily store a value to accomplish a task.

**Assignment:**

Write a program that exchanges the contents of the accumulator and X register.

```6502
LDA #$2A
LDX #$17

STA $01
TXA 
LDX $01
```


**Initial conditions:**
- A contains `$2A`.
- X contains `$17`.

**Requirements:**
- At the end, A must contain `$17`.
- At the end, X must contain `$2A`.
- You may use one temporary RAM location.

**Constraints:**
- Do not use Y.
- Do not perform arithmetic.
- Do not use loops or branches.
- Do not intentionally modify any other memory locations.

**Acceptance criteria:**
- Both registers contain the opposite of their initial values.
- The debugger confirms the final contents.
- You can explain why temporary storage was useful.

**Questions:**

1. What is temporary storage?
2. Why can't you simply overwrite one register before preserving its original value?
3. What is the difference between copying and exchanging values?

```
1. When you store something at some location only to be used for a specific purpose, after which it can be safely deleted again.
2. Your code might be executed from a certain subroutine, where once it returns from it again could require certain values that were created *before* calling.. Overwriting those values could make the program invalid.
3. Copying keeps the original values, where exchanging does not, the difference can have effects on execution of software, which may or may not expect certain variables in certain locations.

```

---

## Exercise 06 — Addition

**Objective:** Perform arithmetic using the accumulator.

**Background:** The 6502 uses the `ADC` instruction to add a value to the accumulator. The carry flag participates in this operation, even when you are not adding multiple-byte numbers.

**Assignment:**

Write a program that calculates:

`12 + 30`

The result must be left in the accumulator.

```6502
;clear the carry to prevent adc from adding an additional 1
CLC
LDA #30
ADC #12
```

**Requirements:**
- The accumulator must contain the final result.
- The calculation must use the CPU's addition instruction.
- The carry flag must be deliberately prepared before the addition.

**Constraints:**
- Do not use multiplication.
- Do not use memory to store intermediate results.
- Do not use loops or branches.

**Acceptance criteria:**
- The accumulator contains the correct result.
- You can explain the purpose of the carry flag during addition.

**Questions:**

1. What does ADC stand for?
2. What is the carry flag?
3. Why must the carry flag be considered before an addition?
4. What happens when an arithmetic result exceeds the capacity of the accumulator?


```
1. "Add with Carry"
2. A single bit flag that indicates that either a previous operation overflows, or that we want the current operating to take in an additional value regardless of overflow.
3. Because it affects the operation, when clear it does value + accumulator, but when it's set it does value + accumulator + 1
4. Then the carry flag will be set 
```

---

## Exercise 07 — Subtraction

**Objective:** Understand subtraction and the carry flag.

**Background:** The 6502 uses `SBC` to subtract a value from the accumulator. Despite its name, subtraction also uses the carry flag.

**Assignment:**

Write a program that calculates:

`50 - 8`

The result must be left in the accumulator.

```6502
;set the carry to prevent SBC from subtracting an additional 1
SEC
LDA #50
SBC #8
```

**Requirements:**
- The accumulator must contain the final result.
- The calculation must use the CPU's subtraction instruction.
- The carry flag must be deliberately prepared before subtraction.

**Constraints:**
- Do not use multiplication or division.
- Do not use memory to store intermediate results.
- Do not use loops or branches.

**Acceptance criteria:**
- The accumulator contains the correct result.
- You can explain the relationship between the carry flag and subtraction.

**Questions:**

1. What does SBC stand for?
2. Why does subtraction use the carry flag?
3. What is the difference between carry and borrow?
4. What happens when the result of a subtraction is negative?

```
1. "Subtract with carry"
2. because the carry flag allows one subtraction to communicate it's borrow state to the *next* potential subtraction.
3. carry means the previous addition operation "overflowed" into requireing another bit, "borrow" means the previous  subtraction operation was lower than 0.
4. The N (negative) flag would be set
```

---

## Exercise 08 — Comparing values

**Objective:** Understand comparisons and processor flags.

**Background:** The 6502 has instructions that compare values without replacing the accumulator with the result of the comparison.

**Assignment:**

Write a program that compares values in the accumulator against other values.

Perform at least these comparisons:

- 42 against 42.
- 42 against 43.
- 42 against 41.

```6502
LDA #42
CMP #42
CMP #43
CMP #41
```

**Requirements:**
- Use the CPU's comparison instruction.
- The accumulator must retain its original value after each comparison.
- Inspect the processor flags after every comparison.

**Constraints:**
- Do not use arithmetic to calculate comparison results.
- Do not use conditional branches.

**Acceptance criteria:**
- All three comparisons execute successfully.
- You can explain the relevant flag changes.
- The accumulator remains unchanged.

**Questions:**

1. What does a comparison instruction actually compare?
2. What is the zero flag?
3. What is the carry flag used for during comparisons?
4. How can a CPU communicate the result of a comparison without storing a numerical result?


```
1. It performs a comparison between the accumulator and its operand by internally subtracting the operand from the accumulator. It then sets the CPU flags based on the result, without storing the result.
2. No, it only modifies the CPU flags.
3. The Zero flag indicates whether the compared values are equal. The Carry flag indicates whether the accumulator is greater than or equal to the operand (using unsigned comparison).
4. So they can still be used for other operations after the comparison has completed.
```
---

## Exercise 09 — Making decisions

**Objective:** Understand conditional branching.

**Background:** Programs often need to make decisions. The 6502 provides conditional branch instructions that change the program's execution path depending on processor flags.

**Assignment:**

Write a program that compares two values and takes different execution paths depending on whether they are equal.

The program must record its decision in memory address `$90`.
```6502
    ;loads the literal number 42 in a
    LDA #42
    ;compares a with the same (equal) number
    CMP #42
    ;branches to SET_START if equal (sets $90 to 1 (true))
    BEQ SET_START
    ;but if not equal, it stores 0 (false) in $90
    LDA #0
    STA $90
    BRK
SET_START:
    ;when two numbers were equal
    LDA #1
    STA $90
    BRK
```

**Requirements:**
- Use a comparison instruction.
- Use at least one conditional branch.
- Record a distinguishable result for the equal and unequal cases.
- Test both cases independently.

**Constraints:**
- Do not use subroutines.
- Do not use indirect jumps.
- Do not use any additional memory locations.

**Acceptance criteria:**
- The equal case produces the correct memory result.
- The unequal case produces a different memory result.
- You can explain why the CPU took one branch instead of another.

**Questions:**

1. What is a branch?
2. What is a conditional branch?
3. What is the program counter?
4. How does a conditional branch use processor flags?

```
1. A branch is like a code "path", like a route to take depending on a specific value
2. A conditional branch uses certain CPU flags (which could have been set by things like comparison operations) to make decisions on what to do next. They depend on a "condition".
3. The program counter is a register that the cpu uses to know which instruction to fetch next.
4. It uses the values of the processor flags to decide on where to move next. E.g, it uses the fact that zero is set to true, to know that a previous equals instruction could have been between two identical values. it sets the program counter to a value in one path, but leaves it as is in the other.
```
---

## Exercise 10 — Counting from zero to ten

**Objective:** Combine registers, arithmetic and conditional branching.

**Background:** Loops allow a program to repeat instructions. A counter can keep track of how many times a loop has executed.

**Assignment:**

Write a program that counts from decimal `0` through decimal `10`, inclusive, and stores each number in consecutive memory locations.

```6502
    ;this loads 0 in both the accumulator and x register
    LDA #0
    LDX #0
    ;this will be the loop
INCREMENT:
    ;it stores 0 the first time, at adres $80 + 0
    STA $80,X
    ;it then clears the carry ands increments A by 1 too
    CLC
    ADC #1
    ;it then increments the x register, and checks if it equals 11
    INX
    CPX #11
    ;if it does *not* equal 11, we start the loop again, but with an increased A and X
    BNE INCREMENT
    BRK
```

**Requirements:**
- The first value must be stored at `$80`.
- The final value must be stored at `$8A`.
- All eleven values must be stored in ascending order.
- Use an index register to keep track of the memory position.
- Use a conditional branch to control repetition.

**Constraints:**
- Do not use subroutines.
- Do not use multiplication or division.
- Do not intentionally modify memory outside `$80`–`$8A`.

**Acceptance criteria:**
- All eleven memory locations contain the expected sequence.
- The program terminates after storing the final value.
- You can explain how the counter, index register and branch work together.

**Questions:**

1. What is a loop?
2. What is a loop counter?
3. What is an infinite loop?
4. Why does a loop need a termination condition?
5. What is indexed addressing?

```
1. A loop is a repeated sequence of events, that keeps repeated untill or unless some stop condition has been reached
2. THe loop counter decides on the number of iterations for the loop
3. an infinite loop is a loop where the stop condition is never met
4. without a terminal condition, it can only run forever
5. Indexed adressing is where the value of a register is used to calculate the location in memory to write to or read from
```

---

# PART 1 COMPLETION TEST

Before moving on, you should be able---d to:

- Explain the purpose of A, X and Y.
- Explain how the CPU accesses memory.
- Distinguish RAM from ROM.
- Explain immediate and memory addressing.
- Transfer values between registers.
- Perform addition and subtraction.
- Explain the carry, zero and negative flags.
- Compare values.
- Use conditional branches.
- Construct a simple counting loop.

**Final requirement:** Be able to step through your own programs one instruction at a time and explain what happens to the registers, flags and memory after each instruction.

No memorised solutions. No guessing. Understand what the processor is actually doing.

```
A is used for calculations and other manipulation of data, where both X and Y are more used for indexing, which is why those latter two are called the "index registers".

The cpu can access memory a variety of different ways, reading values either directly, read values through a referenced location where the value tells the cpu where the actual data is located rather than it being the data itself, and it can use the index registers to read memory with an offset, of lets say $0a + current_value_of_x_register. 

RAM and ROM are both places that hold data, the primary difference is that rom exists only to read from, where the ram can be both read from and written to.

Immediate addressing is where the data itself is immediately provided with the instruction given, for example `LDA #6`. This does not require additional cpu cycles to fetch the required data from elsewhere, as opposed to fetching the data from elsewhere in memory. For example, only 2 cycles are required for immediate addressing, where something like absolute adressing takes 4 at the minimum, and absolute addressing together with a register offset can take 5 at most.

```

## Exercise 10½ — Your First Little Program

**Difficulty:** 2/10

**Objective:** Combine the concepts from Part 1 into one small, self-contained 6502 program.

Create a program that:

1. Starts with a counter at zero.
2. Counts upward, one step at a time, until it reaches 10.
3. Stores every counter value in consecutive memory locations, starting at `$80`.
4. Keeps track of how many values have been stored.
5. Once finished, compares th![](image-url)e final counter value against 10.
6. Stores a success marker at `$90` if the comparison succeeds.

```6502
    LDA #0
    LDX #0

INCREMENT:
    STA $80,X
    CLC
    ADC #1
    INX
    CPX #11
    BNE INCREMENT

    LDA #10
    CMP $8A
    BNE BAD
    BEQ GOOD

BAD:
    LDA #0
    STA $90
    BRK

GOOD:
    LDA #1
    STA $90
    BRK
```
**Requirements:**
- Use registers and RAM.
- Use arithmetic and conditional branching.
- Use a loop.
- Do not hardcode the sequence of values into memory.
- Do not use more memory locations than necessary.
- The program must terminate on its own.

**Acceptance criteria:**
- `$80` through `$89` contain the values 0 through 9.
- `$90` contains your chosen success marker.
- The program terminates without getting stuck in an infinite loop.
- You can explain what each part of the program does.

**Questions to answer:**
1. Which register did you use as your counter, and why?
2. How did you keep track of where the next value should be stored?
3. What causes the loop to stop?
4. How does the CPU know whether to take a conditional branch?
5. What would happen if your stopping condition were incorrect?

```
1. I used the X register as a counter, mostly because I'd reach for X before Y since in my head it makes more sense in alphabetical order. No particular reason for me to have chosen x over y.
2. I used the actual value of X to also know where the next index is, by doing `STA $80,X`.
3. The loop stopped, because the conditional branch that depends on the flags set by a comparison operator (CPX #11) cause it to stop. Until `CPX #11` sets the zero flag, the loop keeps going, after that it falls through to the next instruction (LDA #10).
4. as mentioned, it decides which branch based off of the value of the status flags.
5. Then the loop would behave unpredictably, it might end up looping infinitely, or not run at all.
```

---

This is your first little integration project. No new instructions are required beyond what you've encountered in Exercises 01–10. Don't worry about making it elegant; make it work, and understand why it works.



## Exercise 11 — Absolute memory addressing

**Background:** Instructions can access memory directly using an address. The address is distinct from the contents stored there.

**Assignment:**

Create a program that reads a value from address `$90` and stores an identical copy at `$91`.

```6502
;assuming something has happened before
LDA $90
STA $91

```

**Requirements:**
- `$90` is the source.
- `$91` is the destination.
- The original source value must remain unchanged.
- The program must work for all possible byte values.

**Constraints:**
- Do not use arithmetic.
- Do not assume the source value is known before execution.

**Acceptance criteria:**
Test with `$00`, `$7F`, `$80`, and `$FF`.

**Questions:**
1. What is absolute addressing?
2. How many bits are required to represent an address?
3. Why is the value at an address not necessarily the address itself?

```
1. absolute addressing is where you provide the *entire* 16 bit address of the value you want, as opposed to for example the 8 you'd use for a zero page address or immediate value.
2. 16
3. The distinction can be made through thinking of them in terms of mailboxes.. *which* mailbox a letter is in does not necesserily tell you which letters are inside. And for a mailman to know where to deliver an individual letter, he also has to know which mailbox to deliver it to. The "address"" of a memory cell, is like the home address of a mailbox, and the value is the letter.
```
---

## Exercise 12 — Zero-page addressing

**Background:** The 6502 has a special addressing region in the first 256 bytes of its address space. This region has useful instruction and timing characteristics.

**Assignment:**

Create a program that maintains two independent variables in this special memory region.

Copy the first variable to the second without changing the first.

```6502
;loads a variable
LDA #6
;the location it is being stored to is in the zero page
STA $00

;this part copies the value, by loading it first
LDA $00
;and then writing it to a location that's *also* in the zero page
STA $01
```

**Requirements:**
- Both variables must have distinct addresses.
- Their addresses must be within the permitted region.
- All possible byte values must be supported.

**Constraints:**
- Do not use absolute addressing for these variables.
- Do not introduce arrays yet.

**Acceptance criteria:**
- The destination matches the source.
- The source remains unchanged.
- You can identify why this addressing region is useful.

---

## Exercise 13 — Creating an array

**Background:** An array is a collection of values stored in consecutive memory locations.

**Assignment:**

Create an array containing five bytes.

Use the following initial data:

`12, 34, 56, 78, 90`

```6502
;store the first array element, the number 12
LDA #12
;The "start" of this array is $00
STA $00

LDA #34
STA $01

LDA #56
STA $02

LDA #78
STA $03

LDA #90
;The "end" of this array is $04
STA $04



```

**Requirements:**
- The values must occupy five consecutive addresses.
- Their order must be preserved.
- No values may overlap other program variables.
- Document the starting and ending addresses.

**Constraints:**
- Do not use a separate memory address for every value in your processing logic.
- Do not perform arithmetic on the array contents.

**Acceptance criteria:**
- All five values can be independently inspected.
- The order matches the specification.
- The array occupies exactly five bytes.

---

## Exercise 14 — Indexed memory access

**Objective:** Understand how the processor accesses elements in an array.

**Assignment:**

Using the array from Exercise 13, write a program that retrieves each element in order and copies it into a separate output area.

```6502
    LDA #12
    STA $00

    LDA #34
    STA $01

    LDA #56
    STA $02

    LDA #78
    STA $03

    LDA #90
    STA $04

    ;Lets define the output area to start at $10
    ; we will now loop over the array, and copy it over.

    ;starting from idx 0
    LDX #0
    
LOOP:
    LDA $00, X
    STA $10, X
    INX
    CPX #05
    BNE LOOP
```

**Requirements:**
- Process exactly five elements.
- Preserve the original array.
- The output must contain the same values in the same order.
- The program must use an index that changes during execution.

**Constraints:**
- Do not write five independent copy operations.
- Do not hardcode individual source addresses in the processing logic.

**Acceptance criteria:**
- Every element is copied.
- No element is skipped or processed twice.
- The program terminates after five elements.

---


# Exercise 14½ — Accumulating an array

**Difficulty:** 3/10

## Objective

Combine indexed memory access, loops, and addition to calculate a running total.

## Background

When adding several values, you need to keep track of the accumulated result between iterations.

For this exercise, we'll deliberately keep the result small enough to fit in a single byte. No multi-byte arithmetic yet!

## Assignment

Using the five-element array from Exercise 13, calculate the sum of its first three elements.

```6502
    ;since we only need the first three elements, we'll start with storing those:
      LDA #12
    STA $00

    LDA #34
    STA $01

    LDA #56
    STA $02
    
    ;initialize the location of the sum (we store it at $10)
    LDA #0
    STA $10
    
    ;store the first idx in x
    LDX #0
    ;start at $00, end at $02
SUM:
    ;load the value at $00 + current index into accumulator
    LDA $00, X
    ;clear the possible carry from previous instructions and add the accumulators number to whaterver is the current sum
    CLC
    ADC $10
    ;store result back at $10, and increment the index
    STA $10
    INX
    ;go again until the index reaches 3
    CPX #3
    BNE SUM
BRK
```

## Requirements

- Read the values from the original array.
- Add each value to a running total.
- Store the running total in memory.
- Process exactly three elements.
- Preserve the original array.

## Constraints

- Use indexed addressing.
- Use a loop.
- Do not hardcode the individual source addresses.
- Do not hardcode the final answer.
- The result must fit in one byte.

## Acceptance criteria

- The result is `102` (decimal).
- The first three elements contribute exactly once each.
- The original array remains unchanged.
- The program terminates after three elements.

## Questions

1. Where did you store your running total?
2. Why does the accumulator need to be loaded with the previous total during each iteration?
3. What would happen if you accidentally reset the accumulator to zero on every iteration?

```
1. The total was stored at $10
2. Because the accumulator is the register that is used in the 6502's arithmetic operations, like ADC
3. then we would keep disregarding the previous value of our summation loop, and probably end up with the final value in the array as the eventual "sum" in $10.
```
---


# Exercise 14¾ — The overflowing addition

**Difficulty:** 3/10

## Objective

Understand how the 6502 represents and stores a result that exceeds one byte.

## Background

The accumulator is 8 bits wide and can represent values from 0 to 255.

When an addition exceeds that range, the result wraps around and the Carry flag is set.

We can use two memory locations together to represent a larger number:

| Memory location | Purpose |
|---|---|
| `$10` | Lower byte of the result |
| `$11` | Higher byte of the result |

Together, these two bytes can represent values from 0 to 65,535.

## Assignment

Calculate the following addition:

`200 + 100`

```
    ;initialize to zero both $10 and $11
    LDA #0
    STA $10
    STA $11
    LDA #200
    CLC
    ADC #100
    ;| Memory location | Purpose |
    ;|---|---|
    ;| `$10` | Lower byte of the result |
    ;| `$11` | Higher byte of the result|
    STA $10
    BCS INC_HIGH
    BRK
INC_HIGH:
    CLC
    LDA #1
    ADC $11
    STA $11
    BRK
```

Store the resulting two-byte value in memory.

## Requirements

- Load 200 into the accumulator.
- Add 100 using `ADC`.
- Store the lower byte at `$10`.
- Store the higher byte at `$11`.
- Use the Carry flag to determine whether the higher byte needs to be incremented.

## Constraints

- Do not hardcode the final result.
- Do not use additional memory locations.
- Do not use loops.
- Do not use any instructions we haven't covered.

## Acceptance criteria

- `$10` contains `$2C` (44 decimal).
- `$11` contains `$01` (1 decimal).
- Together, the two bytes represent 300.
- The program terminates.

## Questions

1. Why does the accumulator contain 44 rather than 300?
2. What causes the Carry flag to become set?
3. How does the program know whether the higher byte needs to be incremented?
4. Why can two bytes represent a larger number than one byte?

```
1. Because the number can not be expressed as an 8 bit value. The binary for 300 would be 100101100, and that takes 9 bits to store. Since it "overflows", the addition instruciton sets a carry bit and keeps the rest into A. 100101100 without the first bit is 44 in decimal.
2. Because the addition operations result overflows 8 bits.
3. By checking the appropriate status flag.
4. Because you've got twice as many bits! If you sum a big number per byte, you can remember the overflowing bit for the next operation and still end up with the currect answer 
```

---

## Exercise 15 — Summing an array

**Objective:** Combine indexed access and arithmetic.

**Assignment:**

Calculate the sum of the five values in Exercise 13.

```6502
INIT_ARRAY:
    ;store the first array element, the number 12
    LDA #12
    ;The "start" of this array is $00
    STA $00

    LDA #34
    STA $01

    LDA #56
    STA $02

    LDA #78
    STA $03

    ;The "end" of this array is $04
    LDA #90
    STA $04

    LDA #0
    STA $10
    STA $11
    TAX


LOOP:
    LDA $00, X
    CLC
    ADC $10
    STA $10
    BCC CHECK_FINISHED
    ;if carry *is* set, then increment the high order byte
    LDA #1
    CLC
    ADC $11
    STA $11
    
CHECK_FINISHED:
    INX
    CPX #5
    BNE LOOP

```

**Requirements:**
- Read the values from the array.
- Calculate the result at runtime.
- Preserve the original array.
- Store the result in a documented memory location.

**Constraints:**
- Use a loop.
- Do not hardcode the sum.
- The result must fit in the selected result representation.

**Acceptance criteria:**
- The result is 270.
- All five elements contribute exactly once.
- Changing an input element changes the result accordingly.

**Questions:**
1. Can the sum of several bytes fit in one byte?
2. What happens when an accumulator exceeds its range?
3. How could a program represent a result larger than 255?

```

```
---

## Exercise 16 — Finding the largest value

**Objective:** Perform data-dependent selection.

**Assignment:**

Search an array of eight unsigned bytes and identify its largest value.

**Requirements:**
- The array must contain eight elements.
- The program must inspect every element.
- Duplicate maximum values must not cause failure.
- The original array must remain unchanged.

**Constraints:**
- Do not sort the array.
- Do not use a lookup table.
- Do not hardcode the expected maximum.

**Acceptance criteria:**
Test arrays containing:
- Increasing values.
- Decreasing values.
- Duplicate maximum values.
- Zero and `$FF`.
- A single element that is larger than all others.

**Questions:**
1. What is an unsigned comparison?
2. Why must every element be inspected?
3. How can you retain a candidate result while continuing to search?

---

## Exercise 17 — Counting matching values

**Objective:** Build a counting algorithm.

**Assignment:**

Inspect an array of ten bytes and count how many elements equal zero.

**Requirements:**
- The array contains exactly ten elements.
- The count begins at zero.
- Every element is examined once.
- The final count is stored in RAM.

**Constraints:**
- Do not modify the array.
- Do not manually count the values before execution.
- The implementation must work when all elements are zero.

**Acceptance criteria:**
Test:
- No zero values.
- One zero value.
- Five zero values.
- Ten zero values.

**Questions:**
1. What is the difference between a counter and an index?
2. Why should counters be initialised?
3. What is the maximum possible result?

---

## Exercise 18 — Extracting individual bits

**Objective:** Understand binary masks and bitwise logic.

**Background:** An 8-bit value contains eight individual bits. These bits can represent flags, hardware states, or packed information.

**Assignment:**

Given a byte in memory, produce a result indicating whether its third bit, counting from bit zero, is set.

**Requirements:**
- Preserve the original byte.
- Produce `$01` if the selected bit is set.
- Produce `$00` if it is clear.
- Support every possible byte value.

**Constraints:**
- Do not use a table of 256 possible results.
- Do not convert the byte to decimal text.

**Acceptance criteria:**
Test values where the selected bit is both set and clear, including bytes with several other bits set.

**Questions:**
1. What is a bit mask?
2. Why is binary representation useful when dealing with hardware?
3. How can a single bit represent a Boolean value?

---

## Exercise 19 — Modifying individual bits

**Objective:** Change selected bits without destroying unrelated information.

**Assignment:**

Given a byte in memory, implement three independent operations:

1. Set a specified bit.
2. Clear a specified bit.
3. Toggle a specified bit.

**Requirements:**
- Unselected bits must retain their original values.
- The original value must be recoverable for testing.
- Each operation must be tested independently.

**Constraints:**
- Do not replace the entire byte with a hardcoded value.
- Do not use a lookup table.
- Do not assume unrelated bits are zero.

**Acceptance criteria:**
Test all three operations with several starting values, including `$00`, `$FF`, and mixed bit patterns.

**Questions:**
1. What does toggling a bit mean?
2. Why is preserving unrelated bits important?
3. How are bitwise operations used when communicating with hardware registers?

---

## Exercise 20 — Reversing a byte

**Objective:** Combine bit manipulation and repeated processing.

**Assignment:**

Write a program that reverses the order of the eight bits in a byte.

For example, the input pattern:

`00010110`

must become:

`01101000`

**Requirements:**
- Support all possible 8-bit inputs.
- Store the result separately from the original.
- Process all eight bits.
- Do not use a table of every possible input.

**Constraints:**
- Use an iterative or otherwise general algorithm.
- Do not manually hardcode the result for the example.

**Acceptance criteria:**
- Eight-bit reversal works for all test cases.
- Reversing the result again restores the original value.
- You can explain how information moves between bit positions.

---

# PART 3 — Program structure and control flow

**Difficulty:** 3/10  
**Exercises:** 21–30

**Learning objective:** Build reusable programs instead of isolated instruction sequences.

## Exercise 21 — Your first subroutine

**Background:** A subroutine is a reusable section of code that can be called from another location.

**Assignment:**

Create a subroutine that sets a designated memory variable to zero.

Call it from two different locations in your main program.

**Requirements:**
- Both calls must operate on the same documented variable.
- The calling program must continue after each call.
- The subroutine must be reusable.
- No unrelated memory locations may be modified.

**Constraints:**
- Do not duplicate the subroutine's body.
- Do not use interrupts.

**Acceptance criteria:**
Both calls complete successfully, and execution continues at the correct location.

**Questions:**
1. What information does a subroutine call need to preserve?
2. What is a return address?
3. What happens if a subroutine never returns?

---

## Exercise 22 — A reusable arithmetic routine

**Assignment:**

Create a reusable subroutine that adds two unsigned byte values and makes the result available to its caller.

**Requirements:**
- The caller supplies two input values.
- The subroutine performs the calculation.
- The result must be accessible after returning.
- Document which registers and memory locations may change.

**Constraints:**
- The caller must not depend on undocumented side effects.
- The subroutine must support all input combinations whose results fit in the specified result representation.

**Acceptance criteria:**
Test several input pairs, including zero, maximum values, and results that cross the 8-bit boundary.

---

## Exercise 23 — Multiple subroutines

**Assignment:**

Build a program containing at least three reusable subroutines:

- One for initialising a variable.
- One for processing a value.
- One for storing a result.

The program must call them from a main routine.

**Requirements:**
- Each routine must have one clearly documented responsibility.
- Calls must occur in a defined order.
- The program must finish with a verifiable result.

**Constraints:**
- Do not duplicate routine bodies.
- Document register and memory side effects.

**Acceptance criteria:**
All routines can be called independently from the main program without corrupting each other's data.

---

## Exercise 24 — Nested loops

**Background:** Nested loops are loops inside other loops. They are useful for processing two-dimensional data and repeating operations.

**Assignment:**

Create a program that executes an inner operation five times for each of three outer iterations.

**Requirements:**
- The inner operation must execute exactly 15 times in total.
- Maintain independent counts for both levels.
- Store the final operation count in RAM.

**Constraints:**
- Do not write the inner operation fifteen times.
- Do not use an interrupt or external timer.

**Acceptance criteria:**
- The outer loop completes three iterations.
- The inner loop completes five iterations per outer iteration.
- The final count is 15.

---

## Exercise 25 — Understanding the stack

**Background:** The 6502 has a hardware stack used for return addresses and temporary storage.

**Assignment:**

Write a program that temporarily preserves a register's contents while another operation uses that register.

Restore the original contents before continuing.

**Requirements:**
- The preserved value must survive the intervening operation.
- The stack must be balanced.
- Execution must continue normally.

**Constraints:**
- Do not use a separate RAM variable as the temporary storage.
- Do not leave unconsumed values on the stack.

**Acceptance criteria:**
- The original register value is restored.
- The stack pointer returns to its original value.
- No return address is corrupted.

---

## Exercise 26 — Preserving processor status

**Background:** The processor status register contains flags describing the CPU's current state.

**Assignment:**

Create a routine that performs an operation which changes processor flags, then restores the caller's previous processor status.

**Requirements:**
- The caller's original status must be preserved.
- The routine must complete normally.
- The caller must observe the original flags after the routine finishes.

**Constraints:**
- Do not save the flags in an ordinary RAM variable.
- Keep stack operations balanced.

**Acceptance criteria:**
Test with different initial combinations of relevant status flags.

---

## Exercise 27 — Lookup tables

**Background:** A lookup table stores precomputed values that a program can retrieve using an index.

**Assignment:**

Create a table containing ten predefined byte values.

Write a routine that retrieves the value at a requested index.

**Requirements:**
- Indices range from 0 through 9.
- The original table must remain unchanged.
- Invalid indices must be handled safely.
- Document the table's memory location and length.

**Constraints:**
- Do not write ten separate retrieval routines.
- Do not modify the program's executable code.

**Acceptance criteria:**
Every valid index returns the corresponding value. Invalid indices must not cause unintended memory access.

---

## Exercise 28 — Generating a sequence

**Assignment:**

Generate the first ten values of the following sequence:

`3, 6, 9, 12, 15, 18, 21, 24, 27, 30`

Store them in consecutive memory locations.

**Requirements:**
- Generate the values during execution.
- Store exactly ten results.
- Preserve the correct order.
- Do not use a prewritten array of results.

**Constraints:**
- Use a loop.
- The generated sequence must not depend on uninitialised memory.

**Acceptance criteria:**
All ten memory locations contain the expected values.

---

## Exercise 29 — A three-state machine

**Background:** A state machine represents a system whose behaviour depends on its current state.

**Assignment:**

Create a program with three states:

- IDLE
- ACTIVE
- COMPLETE

The program must transition from IDLE to ACTIVE and then to COMPLETE.

**Requirements:**
- Represent each state using a documented byte value.
- Only valid transitions are permitted.
- The current state must be stored in memory.
- Invalid state values must be detected.

**Constraints:**
- Do not implement the three states as three unrelated programs.
- The state must influence program behaviour.

**Acceptance criteria:**
All valid transitions work, and invalid values do not silently produce an unrelated state.

---

## Exercise 30 — Mini-project: Array processor

**Objective:** Integrate memory access, loops, arithmetic, comparisons, and subroutines.

**Assignment:**

Create a program that processes an array of ten unsigned bytes.

It must calculate:

1. The sum.
2. The largest value.
3. The number of zero elements.

**Requirements:**
- The input array must remain unchanged.
- Each result must occupy a documented memory location.
- The program must process the array at runtime.
- The same processing code must work with different input arrays.

**Constraints:**
- No hardcoded results.
- No manually duplicated processing operations.
- Document any result representation requiring more than one byte.

**Acceptance criteria:**
Test an array containing:
- Ten zeros.
- Ten identical nonzero values.
- Increasing values.
- Randomly ordered values.
- Values near the maximum unsigned byte.

**Completion checkpoint:**

You should now be able to write small, structured 6502 programs without following an instruction-by-instruction template.

---

# PART 4 — Meet the Atari 2600

**Difficulty:** 4/10  
**Exercises:** 31–40

**Learning objective:** Understand the actual console architecture and communicate with its hardware.

## Hardware orientation

The Atari 2600 contains three major components relevant to this course:

- **6507:** The processor executing your program.
- **TIA:** The chip responsible for video, audio, and collision detection.
- **RIOT (6532):** Provides 128 bytes of RAM, I/O ports, and a programmable timer.

The cartridge supplies program ROM.

There is no conventional graphics framebuffer. The TIA generates video using hardware registers controlled by the CPU.

For this course, the initial target is the standard NTSC Atari 2600. PAL differences will be introduced separately.

## Exercise 31 — Understanding the memory map

**Assignment:**

Create a documented map of the Atari 2600's accessible memory regions.

Identify:
- Cartridge ROM.
- TIA registers.
- RIOT RAM.
- RIOT I/O and timer registers.
- The processor's address-space limitations.

**Requirements:**
- Include read/write permissions.
- Identify mirrored address regions.
- Distinguish hardware registers from ordinary memory.
- Explain why the 6507 does not expose the complete 6502 address space.

**Deliverable:**
A memory map in Markdown or a diagram.

**Acceptance criteria:**
You can correctly classify a supplied list of addresses by their hardware purpose.

---

## Exercise 32 — Communicating with the TIA

**Background:** The TIA is controlled through memory-mapped registers.

**Assignment:**

Write a minimal Atari ROM that changes the background colour.

**Requirements:**
- The ROM must execute on the Atari 2600.
- The background must visibly differ from the emulator's initial display.
- The display must remain stable.
- No player or playfield graphics are required.

**Constraints:**
- Do not use a prebuilt graphics kernel.
- Do not use an operating system or runtime.
- Use actual TIA hardware registers.

**Acceptance criteria:**
The emulator displays a stable, recognisably coloured screen.

---

## Exercise 33 — Colour control

**Assignment:**

Create a program that uses different colours for:
- The background.
- Player 0.
- Player 1.
- The playfield.

**Requirements:**
- All four colour assignments must be independently configurable.
- The background must remain visible.
- Each visible object must use the intended colour.
- Use valid NTSC colour values.

**Constraints:**
- Do not use a generated framebuffer.
- Do not rely on emulator-specific graphical extensions.

**Acceptance criteria:**
You can change each object's colour independently and observe the result.

---

## Exercise 34 — Understanding player graphics

**Background:** Each player object is controlled by an 8-bit graphics register. Its bits determine which portions of the player are visible.

**Assignment:**

Create a ROM that displays a static Player 0 graphic.

**Requirements:**
- The player must be visible.
- The graphic must be at least eight scanlines tall.
- Its appearance must be distinguishable from the background.
- The player must have a separate colour from the background.

**Constraints:**
- Do not use an external image or bitmap renderer.
- Do not draw the player as a playfield pattern.

**Acceptance criteria:**
The player appears as an identifiable, stable graphic.

---

## Exercise 35 — The playfield

**Background:** The playfield is a low-resolution graphical object controlled by three hardware registers.

It occupies the full width of the visible screen and can be configured to repeat or reflect its pattern.

**Assignment:**

Draw a static playfield pattern consisting of walls and open spaces.

**Requirements:**
- Use the actual playfield hardware.
- Include at least one enclosed region.
- Include at least one opening.
- Use a colour distinct from the background.

**Constraints:**
- Do not use player sprites to construct the walls.
- Do not use an external framebuffer.

**Acceptance criteria:**
The pattern is stable and the intended geometry is visible.

---

## Exercise 36 — Playfield configuration

**Assignment:**

Create three different playfield configurations:

1. A duplicated pattern.
2. A reflected pattern.
3. A pattern that uses the two halves differently.

**Requirements:**
- Each configuration must be independently testable.
- The same basic pattern data must be reusable where appropriate.
- The differences must be visually identifiable.

**Constraints:**
- Do not replace the playfield with player graphics.
- Do not use three separate ROMs.

**Acceptance criteria:**
All three configurations display correctly.

---

## Exercise 37 — Vertical synchronisation

**Background:** A television picture consists of repeated horizontal scanlines grouped into frames.

The TIA provides a synchronisation mechanism that tells the television when a new frame begins.

**Assignment:**

Create a program that produces a correctly synchronised video frame.

**Requirements:**
- The frame must begin with an appropriate vertical synchronisation interval.
- Synchronisation must occur consistently.
- The program must not produce a rolling or unstable image.
- Use the standard NTSC timing model.

**Constraints:**
- Do not rely on the emulator automatically repairing an invalid frame.
- Do not use external timing hardware.

**Acceptance criteria:**
The picture remains vertically stable during extended execution.

---

## Exercise 38 — Vertical blanking

**Background:** Not every scanline is visible. Some portions of a frame are intentionally blank.

The vertical blanking interval is useful for performing work that does not need to happen while visible pixels are being generated.

**Assignment:**

Modify your previous ROM so that it explicitly controls the vertical blanking interval.

**Requirements:**
- The visible image must begin at a consistent vertical position.
- The blanking interval must be repeatable.
- The program must preserve correct frame synchronisation.

**Constraints:**
- Do not assume that blanking and synchronisation are the same operation.
- Do not omit the visible portion of the frame.

**Acceptance criteria:**
The visible image appears at a stable location and the frame does not roll.

---

## Exercise 39 — Horizontal synchronisation

**Background:** The TIA also provides a mechanism for synchronising CPU execution with horizontal scanlines.

**Assignment:**

Create a program that generates a visible horizontal region whose position is consistent across frames.

**Requirements:**
- Each intended scanline must begin at a predictable time.
- The visible region must have a consistent height.
- No uncontrolled horizontal drift is permitted.

**Constraints:**
- Do not assume that CPU execution automatically aligns with scanline boundaries.
- Do not rely on timing that changes between iterations.

**Acceptance criteria:**
The image is stable across repeated frames.

---

## Exercise 40 — Mini-project: A complete frame

**Assignment:**

Build a ROM that generates a complete, correctly synchronised NTSC frame.

The frame must contain:
- A vertical synchronisation interval.
- A vertical blanking interval.
- A visible picture.
- An overscan interval.

**Requirements:**
- Use a documented frame structure.
- Maintain consistent timing.
- Include a visible background and playfield.
- The frame must repeat continuously.

**Constraints:**
- Do not use a prebuilt display kernel.
- Do not leave timing unspecified.
- Document the number of scanlines assigned to each frame region.

**Acceptance criteria:**
The picture is stable and the frame has the intended structure.

---

# PART 5 — Drawing on the television

**Difficulty:** 5/10  
**Exercises:** 41–50

**Learning objective:** Understand scanline-based rendering and build a reliable display kernel.

## Exercise 41 — Counting scanlines

**Assignment:**

Create a visible region with a precisely defined height of 32 scanlines.

**Requirements:**
- The region must begin at a consistent vertical position.
- It must contain exactly 32 scanlines.
- The surrounding visible region must have a different appearance.

**Constraints:**
- Count scanlines rather than relying on approximate frame timing.
- The result must repeat consistently.

**Acceptance criteria:**
Stella's debugger and visual output confirm the intended height.

---

## Exercise 42 — Horizontal colour bands

**Assignment:**

Divide the visible screen into four horizontal bands of equal height.

Each band must have a different background colour.

**Requirements:**
- All four bands must be visible.
- Their heights must be equal.
- The boundaries must be horizontal.
- The colours must remain consistent between frames.

**Constraints:**
- Use TIA colour registers.
- Do not use a pre-rendered image.

**Acceptance criteria:**
All four regions occupy the intended scanlines.

---

## Exercise 43 — Drawing horizontal playfield patterns

**Assignment:**

Create a screen containing at least three different horizontal playfield patterns.

**Requirements:**
- The patterns must differ visibly.
- Each pattern must occupy a specified vertical region.
- Transitions between patterns must be stable.

**Constraints:**
- Use the playfield hardware.
- Do not use additional player objects to simulate the patterns.

**Acceptance criteria:**
All patterns appear in the correct vertical positions.

---

## Exercise 44 — Symmetrical graphics

**Assignment:**

Create a symmetrical arena using the playfield.

**Requirements:**
- The left and right halves must be mirror images.
- Include an opening in the upper half.
- Include a second opening in the lower half.
- The central region must remain symmetrical.

**Constraints:**
- Do not manually duplicate every visible bit in a separate bitmap.
- The pattern must be generated from documented graphics data.

**Acceptance criteria:**
The result is symmetrical across the screen's vertical centre line.

---

## Exercise 45 — Data-driven graphics

**Assignment:**

Create a collection of at least eight playfield patterns stored as data.

Display each pattern in a different horizontal region of the screen.

**Requirements:**
- The patterns must be stored separately from the display logic.
- Changing pattern data must not require rewriting the entire display kernel.
- All patterns must be identifiable.

**Constraints:**
- Do not manually create eight separate drawing routines.
- Do not generate patterns using external tools at runtime.

**Acceptance criteria:**
You can replace a pattern by changing its data without changing the general drawing routine.

---

## Exercise 46 — Mid-frame register changes

**Assignment:**

Change a visible TIA property partway down the screen.

The resulting image must contain two clearly distinguishable vertical regions.

**Requirements:**
- The transition must occur at a defined scanline.
- The upper and lower regions must have different appearances.
- The transition must repeat at the same position.

**Constraints:**
- Do not modify the display outside the intended transition.
- Respect the CPU's available cycles.

**Acceptance criteria:**
The transition is stable and does not shift vertically between frames.

---

## Exercise 47 — A rectangular room

**Assignment:**

Build a top-down rectangular room.

**Requirements:**
- Four visible boundaries.
- A floor distinct from the walls.
- At least one entrance.
- A clearly defined interior region.

**Constraints:**
- Use playfield graphics.
- Do not use an external bitmap.
- Keep the room within the visible area.

**Acceptance criteria:**
The room is recognisable and the opening is positioned correctly.

---

## Exercise 48 — Corridors and obstacles

**Assignment:**

Expand the room into a small maze containing at least three connected corridors.

**Requirements:**
- Every corridor must connect to at least one other region.
- Include at least two dead ends.
- Include at least one route connecting opposite sides of the room.

**Constraints:**
- Store the maze as reusable graphical data.
- Do not create a separate display kernel for each corridor.

**Acceptance criteria:**
The displayed maze matches your documented layout.

---

## Exercise 49 — Animated background

**Assignment:**

Create a background that changes appearance over time.

**Requirements:**
- At least four distinct animation states.
- Animation must continue while the game runs.
- The transition must not disturb frame synchronisation.
- The animation must repeat indefinitely.

**Constraints:**
- Do not modify the program's ROM at runtime.
- Do not use external animation libraries.

**Acceptance criteria:**
The animation remains stable over at least 1,000 consecutive frames.

---

## Exercise 50 — Mini-project: Complete screen composition

**Assignment:**

Create a complete game screen containing:

- A background.
- A playfield.
- Two independently coloured player objects.
- Multiple horizontal colour regions.
- At least one mid-frame graphical transition.

**Requirements:**
- The screen must be synchronised.
- All objects must be visible.
- The display kernel must be reusable.
- All graphical data must be documented.

**Constraints:**
- No framebuffer.
- No prebuilt graphics engine.
- No emulator-specific rendering shortcuts.

**Acceptance criteria:**
The complete screen displays consistently for at least 1,000 frames.

---

# PART 6 — Sprites and positioning

**Difficulty:** 6/10  
**Exercises:** 51–60

**Learning objective:** Understand the Atari's movable graphical objects.

## Hardware orientation

The TIA provides:
- Player 0.
- Player 1.
- Missile 0.
- Missile 1.
- Ball.

These objects have different graphical capabilities and positioning mechanisms.

## Exercise 51 — First player character

**Assignment:**

Create an eight-scanline character graphic using Player 0.

**Requirements:**
- The graphic must be distinguishable from the background.
- Each scanline must be independently represented.
- The character must have a consistent appearance across frames.

**Constraints:**
- Use the TIA player graphics system.
- Do not construct the character from the playfield.

**Acceptance criteria:**
The character is visible and its eight graphical rows are correct.

---

## Exercise 52 — Character design

**Assignment:**

Design three different character graphics.

Each must use the same player object but have a different appearance.

**Requirements:**
- Each character must be independently identifiable.
- Graphics must be stored as data.
- Switching characters must not require replacing the entire program.

**Constraints:**
- Do not use an external graphics renderer.
- Do not store all three graphics in the same memory locations.

**Acceptance criteria:**
All three characters can be displayed independently.

---

## Exercise 53 — Two independent players

**Assignment:**

Display Player 0 and Player 1 simultaneously.

**Requirements:**
- Each player has different graphics.
- Each player has an independent colour.
- Both remain visible during the same frame.
- Their vertical positions may differ.

**Constraints:**
- Do not simulate the second player using playfield graphics.
- Do not allow one player's data to overwrite the other's.

**Acceptance criteria:**
Both players appear correctly and remain independently configurable.

---

## Exercise 54 — Sprite animation

**Assignment:**

Create a character with at least four animation frames.

**Requirements:**
- All frames must represent the same character.
- The animation must loop.
- The frame rate must be controllable.
- The animation must not interrupt display synchronisation.

**Constraints:**
- Do not store animation data in writable executable memory.
- Do not rely on external animation assets at runtime.

**Acceptance criteria:**
The character animates smoothly and repeats correctly.

---

## Exercise 55 — Horizontal positioning

**Background:** Player objects have hardware positioning mechanisms. Their position is not simply a normal RAM coordinate.

**Assignment:**

Place Player 0 at a specified horizontal location.

**Requirements:**
- The location must be repeatable.
- The player must remain within the visible region.
- The position must be independently configurable.

**Constraints:**
- Use the TIA positioning hardware.
- Do not approximate the result by changing the playfield.
- Account for horizontal blanking and the hardware's timing.

**Acceptance criteria:**
The player appears at the intended horizontal coordinate.

---

## Exercise 56 — Vertical positioning

**Assignment:**

Position Player 0 at three different vertical coordinates.

**Requirements:**
- Each coordinate must be independently configurable.
- The player must appear at the intended scanlines.
- The character must disappear outside its intended vertical region.

**Constraints:**
- Do not assume a dedicated hardware Y-coordinate register exists for the player.
- Use the scanline-based nature of the display.

**Acceptance criteria:**
All three positions are correct and repeatable.

---

## Exercise 57 — Sprite size

**Assignment:**

Display the same character using multiple horizontal size configurations.

**Requirements:**
- Include normal width.
- Include at least two enlarged configurations.
- Preserve the character's intended graphic pattern.
- Compare the resulting visible widths.

**Constraints:**
- Do not create separate manually enlarged bitmaps for every configuration.
- Use the player's hardware size capabilities.

**Acceptance criteria:**
Each configuration is visibly distinct.

---

## Exercise 58 — Multiple player copies

**Assignment:**

Display multiple copies of one player object on the same scanline.

**Requirements:**
- Include at least two copies.
- Experiment with different spacing configurations.
- Maintain consistent positioning.
- Document the resulting positions.

**Constraints:**
- Do not create separate player objects for every copy.
- Do not use manually duplicated playfield patterns.

**Acceptance criteria:**
The copies appear at the intended positions.

---

## Exercise 59 — Projectile graphics

**Assignment:**

Create a projectile using one of the TIA missile objects.

**Requirements:**
- The projectile must be visible.
- Its colour must be distinguishable.
- Its position must be configurable.
- It must be possible to disable its visibility.

**Constraints:**
- Do not use a player object for the projectile.
- Do not draw the projectile using playfield data.

**Acceptance criteria:**
The projectile can be independently displayed, positioned, and hidden.

---

## Exercise 60 — Mini-project: Character scene

**Assignment:**

Create a complete scene containing:

- Player 0.
- Player 1.
- Missile 0.
- Missile 1.
- The ball.
- A playfield.

**Requirements:**
- Every object must be independently controllable where supported by the hardware.
- All objects must be visible simultaneously.
- Use distinct graphical configurations.
- Document object limitations and shared hardware settings.

**Constraints:**
- Do not fake missing objects with framebuffer graphics.
- Keep the display kernel within its available timing budget.

**Acceptance criteria:**
All five movable objects and the playfield appear correctly.

---

# PART 7 — Input and movement

**Difficulty:** 7/10  
**Exercises:** 61–70

**Learning objective:** Turn the graphical scene into an interactive environment.

## Exercise 61 — Reading the fire button

**Background:** A standard Atari joystick has a digital fire button. Its state is exposed through the console's input hardware.

**Assignment:**

Create a program that detects whether the fire button is pressed.

**Requirements:**
- Detect both pressed and released states.
- Display a visible indication of the current state.
- Update the indication while the program runs.

**Constraints:**
- Use the actual controller input.
- Do not hardcode the button state.
- Test with a real controller or Stella's joystick input.

**Acceptance criteria:**
The display changes correctly when the button is pressed and released.

---

## Exercise 62 — Reading horizontal movement

**Assignment:**

Read the joystick's horizontal directional inputs.

**Requirements:**
- Detect left.
- Detect right.
- Detect neither.
- Detect both simultaneously if the input hardware permits it.
- Display a different visual indication for each supported state.

**Constraints:**
- Do not assume directional inputs are active-high.
- Do not confuse controller input with console switches.

**Acceptance criteria:**
All tested input combinations produce the intended indications.

---

## Exercise 63 — Reading vertical movement

**Assignment:**

Extend the previous input program to detect vertical directions.

**Requirements:**
- Detect up and down.
- Combine horizontal and vertical input.
- Support diagonal input.
- Distinguish neutral input.

**Constraints:**
- Preserve the horizontal input functionality.
- Handle active-low signals correctly.

**Acceptance criteria:**
All eight directional combinations and neutral state are tested.

---

## Exercise 64 — Moving horizontally

**Assignment:**

Make Player 0 move horizontally in response to joystick input.

**Requirements:**
- Movement must follow left and right input.
- Neutral input must stop horizontal movement.
- Movement must continue while a direction remains held.
- The player's position must persist between frames.

**Constraints:**
- Do not use a pre-recorded animation of movement.
- Do not move the player by changing the background.

**Acceptance criteria:**
The player can travel in both directions and remain stationary when the joystick is neutral.

---

## Exercise 65 — Moving vertically

**Assignment:**

Extend Player 0 movement to include vertical directions.

**Requirements:**
- The player can move in all four cardinal directions.
- Diagonal movement must be supported.
- Horizontal movement must continue working.
- The player must retain its position between frames.

**Constraints:**
- Use a consistent coordinate system.
- Do not allow vertical movement to change horizontal coordinates.

**Acceptance criteria:**
The player can move freely within the visible play area.

---

## Exercise 66 — Screen boundaries

**Assignment:**

Prevent Player 0 from moving outside the visible play area.

**Requirements:**
- Define left, right, upper, and lower boundaries.
- The player must remain entirely within the permitted region.
- Boundary checks must work in every direction.
- The player must not wrap around the screen.

**Constraints:**
- Do not rely on the TIA automatically clipping the player as your only boundary mechanism.
- Account for the character's visible dimensions.

**Acceptance criteria:**
Repeated directional input at every boundary does not move the player outside the permitted area.

---

## Exercise 67 — Frame-based movement

**Assignment:**

Create movement that remains consistent across frames.

**Requirements:**
- Movement speed must be configurable.
- Holding a direction for a fixed number of frames must produce predictable displacement.
- Input must remain responsive.
- Movement must not depend on emulator execution speed.

**Constraints:**
- Use a defined update frequency.
- Do not use uncontrolled busy-wait timing as the game's movement clock.

**Acceptance criteria:**
Movement is repeatable over multiple test runs.

---

## Exercise 68 — Acceleration

**Assignment:**

Implement acceleration-based movement.

**Requirements:**
- The player begins at rest.
- Holding a direction increases movement speed.
- Speed must have a defined maximum.
- Reversing direction must behave predictably.

**Constraints:**
- Position and velocity must be represented separately.
- Movement must remain within the screen boundaries.

**Acceptance criteria:**
The player visibly accelerates and cannot exceed the defined speed limit.

---

## Exercise 69 — Deceleration

**Assignment:**

Add deceleration to the movement system.

**Requirements:**
- Releasing the joystick must not immediately stop the player.
- Velocity must gradually decrease.
- The player must eventually stop.
- Reverse-direction input must remain supported.

**Constraints:**
- Do not introduce random movement.
- Do not allow deceleration to create unintended movement beyond the screen boundaries.

**Acceptance criteria:**
Movement demonstrates acceleration, coasting, and eventual stopping.

---

## Exercise 70 — Mini-project: Controllable character

**Assignment:**

Build a controllable character in a single-screen arena.

**Requirements:**
- Full directional movement.
- Fire-button detection.
- Screen boundaries.
- Persistent position.
- Configurable movement speed.
- Stable display.
- At least one static obstacle.

**Constraints:**
- Do not use a prebuilt game engine.
- Keep game logic separate from display generation.

**Acceptance criteria:**
The character can be controlled reliably for at least two minutes without display corruption.

---

# PART 8 — Game mechanics

**Difficulty:** 8/10  
**Exercises:** 71–80

**Learning objective:** Implement actual gameplay systems.

## Exercise 71 — Autonomous enemy

**Assignment:**

Create an enemy that moves independently of player input.

**Requirements:**
- The enemy must have a persistent position.
- Its movement must repeat.
- Its speed must be configurable.
- Its graphics must remain visible.

**Constraints:**
- Do not use joystick input to control the enemy.
- Do not hardcode its position separately for every frame.

**Acceptance criteria:**
The enemy moves autonomously for at least 1,000 frames.

---

## Exercise 72 — Enemy boundary behaviour

**Assignment:**

Modify the enemy so that it changes direction when reaching a horizontal boundary.

**Requirements:**
- The enemy must travel in both directions.
- It must not leave the visible area.
- Direction changes must occur at the defined boundaries.
- The movement must repeat indefinitely.

**Constraints:**
- Do not manually script individual positions.
- Do not teleport the enemy to the opposite side.

**Acceptance criteria:**
The enemy moves back and forth consistently.

---

## Exercise 73 — Projectile movement

**Assignment:**

Create a projectile that moves independently after being activated.

**Requirements:**
- It must have a position.
- It must have a direction.
- It must have a configurable speed.
- It must become inactive when it leaves the permitted region.

**Constraints:**
- The projectile must not depend on player movement after launch.
- It must not remain visible indefinitely after leaving the screen.

**Acceptance criteria:**
The projectile travels predictably and deactivates correctly.

---

## Exercise 74 — Player shooting

**Assignment:**

Allow the player to launch projectiles using the fire button.

**Requirements:**
- Pressing fire activates a projectile.
- The projectile begins at the player's position.
- Its direction must be determined by a documented rule.
- Releasing the button must not destroy an already launched projectile.

**Constraints:**
- Do not allow the projectile to appear at an unrelated screen position.
- Prevent unintended repeated launches from a single press if your chosen firing model requires press-edge detection.

**Acceptance criteria:**
The player can launch multiple projectiles over time.

---

## Exercise 75 — Projectile limits

**Assignment:**

Limit the player to one active projectile.

**Requirements:**
- A second projectile cannot be launched while the first is active.
- A new projectile can be launched after the existing one becomes inactive.
- The projectile state must remain consistent.

**Constraints:**
- Do not create extra projectile objects to bypass the limit.
- Do not confuse a projectile's visibility with its logical activity.

**Acceptance criteria:**
Repeated fire-button presses obey the one-projectile rule.

---

## Exercise 76 — Hardware collision detection

**Background:** The TIA contains collision-detection hardware. Collision results are exposed through dedicated readable registers.

**Assignment:**

Create a program that detects a collision between Player 0 and the playfield.

**Requirements:**
- Detect the relevant collision.
- Display a visible indication when the collision occurs.
- Clear or reset collision state appropriately between tests.
- Distinguish a new collision from an old latched collision.

**Constraints:**
- Use the TIA's collision hardware.
- Do not calculate the overlap using software coordinates for this exercise.

**Acceptance criteria:**
The collision is detected when the player overlaps the playfield and is not falsely reported when separated.

---

## Exercise 77 — Collision response

**Assignment:**

Make an enemy disappear when struck by a player projectile.

**Requirements:**
- The enemy must be visible before impact.
- A valid collision must deactivate it.
- The enemy must not disappear when no collision occurs.
- The projectile must also follow a defined post-impact behaviour.

**Constraints:**
- Use actual collision detection.
- Do not simply deactivate the enemy after a fixed number of frames.

**Acceptance criteria:**
Repeated tests distinguish successful hits from missed shots.

---

## Exercise 78 — Score counter

**Background:** The Atari has limited display hardware. A numerical score must be represented and rendered using available graphical resources.

**Assignment:**

Implement a score counter that increases when an enemy is successfully destroyed.

**Requirements:**
- The score starts at zero.
- Each valid enemy destruction increases it by a defined amount.
- Missed shots do not increase it.
- The score must be displayed on screen.

**Constraints:**
- Do not use an external font renderer.
- The displayed value must reflect the actual score variable.
- Handle the chosen maximum score explicitly.

**Acceptance criteria:**
The displayed score matches the internal score after every successful hit.

---

## Exercise 79 — Lives and player death

**Assignment:**

Implement a player lives system.

**Requirements:**
- The player starts with a defined number of lives.
- A valid player death removes exactly one life.
- Multiple collision reads from the same event must not remove multiple lives.
- Reaching zero lives must enter a defined terminal game state.

**Constraints:**
- Do not decrement lives continuously while the player remains in contact with an enemy.
- Do not permit the lives count to underflow silently.

**Acceptance criteria:**
Test single collisions, repeated collisions, and the final life.

---

## Exercise 80 — Mini-project: Playable shooter

**Assignment:**

Build a complete small shooting game.

**Required features:**
- Controllable player.
- At least two autonomous enemies.
- Player projectiles.
- Collision detection.
- Enemy destruction.
- Score counter.
- Lives counter.
- Game-over condition.

**Requirements:**
- All systems must operate together.
- Display timing must remain stable.
- The game must have a clear win or loss condition.
- State changes must be deterministic.

**Constraints:**
- No external game engine.
- No prebuilt gameplay framework.
- No hardcoded sequence of gameplay events.

**Acceptance criteria:**
A player can start the game, shoot enemies, earn points, lose lives, and reach game over.

---

# PART 9 — Advanced Atari programming

**Difficulty:** 9/10  
**Exercises:** 81–90

**Learning objective:** Master timing, sound, animation, memory management, and optimisation.

## Exercise 81 — RIOT timer

**Background:** The RIOT provides a programmable hardware timer with selectable clock intervals.

**Assignment:**

Create a program that measures a defined interval using the RIOT timer.

**Requirements:**
- The timer must be configured with a documented interval.
- Its state must be observable.
- The program must detect when the interval expires.
- The result must be repeatable.

**Constraints:**
- Do not use an arbitrary software delay as the primary timing mechanism.
- Do not assume the timer is already initialised.

**Acceptance criteria:**
The measured interval agrees with the selected timer configuration.

---

## Exercise 82 — Game clock

**Assignment:**

Implement a game clock that counts elapsed time while the game runs.

**Requirements:**
- Time must advance at a defined rate.
- The clock must not depend on emulator speed.
- Pausing the game must stop the game clock.
- Resuming must continue from the previous value.

**Constraints:**
- Separate the clock from the visible score.
- Document the precision and maximum range.

**Acceptance criteria:**
The clock behaves consistently during normal play, pause, and resume.

---

## Exercise 83 — Animated characters

**Assignment:**

Create an animated enemy with at least eight distinct animation frames.

**Requirements:**
- Animation must be independent of horizontal movement.
- Animation speed must be configurable.
- The animation must loop.
- The display kernel must remain stable.

**Constraints:**
- Store animation frames as data.
- Do not use an external graphics engine.

**Acceptance criteria:**
All eight frames appear in the intended order.

---

## Exercise 84 — Enemy behaviour

**Assignment:**

Implement an enemy with at least three behavioural states:

- Patrol.
- Attack.
- Retreat.

**Requirements:**
- Each state must have distinguishable behaviour.
- State transitions must be based on game conditions.
- The enemy must maintain a valid state at all times.
- Invalid states must be handled safely.

**Constraints:**
- Do not script the entire behaviour as a fixed sequence of frame numbers.
- Keep behaviour independent of rendering.

**Acceptance criteria:**
The enemy transitions between states in response to changing game conditions.

---

## Exercise 85 — First sound effect

**Background:** The TIA provides two independent audio channels. Each has programmable control, frequency, and volume settings.

**Assignment:**

Create a short sound effect that plays when the player fires.

**Requirements:**
- The sound must be distinguishable from silence.
- It must have a defined duration.
- It must stop when its duration expires.
- Repeated firing must trigger the effect reliably.

**Constraints:**
- Use the TIA audio hardware.
- Do not use external audio playback.
- Do not block display generation while sound is playing.

**Acceptance criteria:**
The sound plays consistently when firing and stops at the intended time.

---

## Exercise 86 — Musical sequence

**Assignment:**

Create a short musical sequence containing at least eight notes.

**Requirements:**
- Notes must have distinct pitches.
- Each note must have a defined duration.
- The sequence must play in order.
- Playback must not interfere with game timing.

**Constraints:**
- Store the sequence as data.
- Do not use external audio files.
- Document the selected sound characteristics.

**Acceptance criteria:**
The sequence plays from beginning to end and can be repeated.

---

## Exercise 87 — Title and game-over screens

**Assignment:**

Add a title screen and a game-over screen to your existing shooter.

**Requirements:**
- The title screen appears when the ROM starts.
- The player can begin the game using a documented control.
- The game-over screen appears after the terminal game condition.
- Restarting returns the game to a clean initial state.

**Constraints:**
- Reuse the display system.
- Do not create separate ROMs.
- Do not leave gameplay variables in an undefined state.

**Acceptance criteria:**
The complete sequence of title, gameplay, game over, and restart works repeatedly.

---

## Exercise 88 — RAM optimisation

**Background:** The Atari 2600 has only 128 bytes of internal RIOT RAM, shared by game variables, temporary data, and stack usage.

**Assignment:**

Audit an existing game and reduce its RAM usage.

**Requirements:**
- Document every variable.
- Document every reserved stack region.
- Identify temporary storage.
- Establish a maximum permitted stack depth.
- Ensure all memory regions are non-overlapping.

**Constraints:**
- Do not remove required gameplay features.
- Do not rely on accidental memory aliasing.
- Do not silently overwrite hardware registers.

**Acceptance criteria:**
The complete game operates within the documented RAM budget.

---

## Exercise 89 — Cycle counting

**Background:** The NTSC Atari 2600 CPU has 76 CPU cycles available per scanline. The visible display occupies only part of that time.

**Assignment:**

Audit a display kernel and calculate its cycle consumption.

**Requirements:**
- Account for every instruction's execution time.
- Account for conditional branch timing.
- Account for indexed addressing penalties where applicable.
- Account for hardware synchronisation.
- Document the cycle budget for each scanline.

**Constraints:**
- Do not estimate cycle counts by visual inspection.
- Do not assume all instructions take the same number of cycles.
- Do not exceed the available timing budget.

**Acceptance criteria:**
Every scanline has a documented, valid timing budget and the kernel displays consistently.

---

## Exercise 90 — Mini-project: Optimised 4 KB game

**Assignment:**

Build a complete small game that fits within a standard 4 KB cartridge ROM.

**Requirements:**
- Complete title screen.
- Player movement.
- Enemies.
- Projectiles.
- Collision detection.
- Score and lives.
- Sound effects.
- Game-over screen.
- Stable NTSC display.

**Constraints:**
- Standard 4 KB cartridge ROM.
- No cartridge bankswitching.
- No additional cartridge RAM.
- Respect the internal RAM budget.
- Meet scanline timing requirements.

**Acceptance criteria:**
The assembled ROM fits within the specified cartridge size and operates correctly in Stella.

---

# PART 10 — Final project: OUTPOST

**Difficulty:** 10/10  
**Exercises:** 91–100

**Learning objective:** Demonstrate that you can independently design, implement, debug, and complete an Atari 2600 game.

# Game specification: OUTPOST

You are defending a remote research outpost against waves of hostile alien creatures.

The game takes place in a single-screen arena.

The player must survive, destroy enemies, and achieve the highest possible score.

The visual style is yours to design, within the capabilities of the Atari 2600.

## Technical requirements

- NTSC Atari 2600 compatibility.
- Standard 4 KB cartridge ROM.
- No cartridge bankswitching.
- No additional cartridge RAM.
- Internal RAM budget of 128 bytes.
- Standard joystick input.
- TIA graphics and sound.
- Correct frame synchronisation.
- Stable display kernel.

## Gameplay requirements

- Player movement.
- Enemy movement.
- Player projectiles.
- Enemy attacks.
- Collision detection.
- Score.
- Lives.
- Multiple enemy waves.
- Increasing difficulty.
- Game-over state.
- Restart functionality.

---

## Exercise 91 — Architecture and memory design

**Objective:** Design the game before implementing it.

**Assignment:**

Create a technical design document for OUTPOST.

**Requirements:**
- Define every game state.
- Define every entity.
- Define the player data.
- Define enemy data.
- Define projectile data.
- Define score and lives representation.
- Define the frame structure.
- Define the RAM allocation.
- Define the maximum stack usage.

**Constraints:**
- Every RAM allocation must have a documented purpose.
- No overlapping allocations.
- All variables must have known initialisation requirements.
- The design must fit within the hardware limits.

**Deliverable:**
A memory map, state diagram, and high-level program architecture.

**Acceptance criteria:**
You can explain where every important piece of game information lives and how it changes.

---

## Exercise 92 — Boot sequence and title screen

**Objective:** Build the initial game experience.

**Assignment:**

Create the ROM's startup sequence and title screen.

**Requirements:**
- Initialise the hardware into a known state.
- Initialise all game variables.
- Display the game title.
- Provide a visible indication that the game is waiting for input.
- Begin gameplay after the player activates the designated control.

**Constraints:**
- Startup must be deterministic.
- No uninitialised RAM may influence the game.
- The display must remain stable while waiting.

**Acceptance criteria:**
The title screen appears after every cold start and gameplay begins only after the required input.

---

## Exercise 93 — Player implementation

**Objective:** Implement the complete player character.

**Assignment:**

Create the player entity and integrate it into the arena.

**Requirements:**
- Player graphics.
- Horizontal and vertical movement.
- Screen boundaries.
- Input handling.
- Persistent coordinates.
- Configurable movement speed.
- A clearly defined initial position.

**Constraints:**
- The player must not leave the arena.
- Movement must be independent of rendering.
- The display kernel must remain within its cycle budget.

**Acceptance criteria:**
The player can move around the entire permitted arena without graphical corruption.

---

## Exercise 94 — Enemy implementation

**Objective:** Implement multiple independent enemies.

**Assignment:**

Create an enemy system capable of managing at least three enemies simultaneously.

**Requirements:**
- Each enemy has an independent position.
- Each enemy has an activity state.
- Each enemy has a movement behaviour.
- Enemies must be able to change direction.
- Destroyed enemies must become inactive.
- New waves must be possible.

**Constraints:**
- Enemy data must fit within the RAM budget.
- Do not allocate unlimited memory.
- Rendering must account for all active enemies.

**Acceptance criteria:**
Three enemies can move independently without corrupting one another's state.

---

## Exercise 95 — Projectile system

**Objective:** Implement player and enemy attacks.

**Assignment:**

Create the complete projectile system.

**Requirements:**
- Player projectiles.
- Enemy projectiles.
- Independent projectile positions.
- Defined projectile directions.
- Configurable projectile speed.
- Activation and deactivation.
- Screen-boundary handling.

**Constraints:**
- Projectile storage must fit within the documented RAM allocation.
- Inactive projectiles must not participate in collisions.
- Projectile rendering must not interfere with the other objects.

**Acceptance criteria:**
Both sides can fire projectiles and their movement remains independent of the firing entity.

---

## Exercise 96 — Collision and damage system

**Objective:** Connect the graphical and gameplay systems.

**Assignment:**

Implement all required collision interactions.

**Required interactions:**
- Player projectile versus enemy.
- Enemy projectile versus player.
- Player versus enemy.
- Projectile versus arena boundaries.

**Requirements:**
- Each collision must have a defined response.
- One collision event must not be processed repeatedly by accident.
- Destroyed entities must become inactive.
- Damage and invulnerability rules must be documented.

**Constraints:**
- Use the available TIA collision information.
- Supplement hardware collision detection with software logic where necessary.
- Do not treat inactive objects as valid collision participants.

**Acceptance criteria:**
All four interactions work consistently.

---

## Exercise 97 — Scoring, lives, and waves

**Objective:** Implement the game's progression system.

**Assignment:**

Create a complete scoring and difficulty progression system.

**Requirements:**
- Player begins with three lives.
- Destroying an enemy awards a documented score.
- Losing a life occurs only after a valid player death event.
- A wave completes when all its enemies are destroyed.
- A new wave begins after a completed wave.
- Enemy difficulty increases between waves.
- The game ends when the player has no remaining lives.

**Constraints:**
- Score and lives must be represented consistently.
- Prevent arithmetic underflow.
- Ensure wave transitions occur exactly once.
- Document the maximum supported score and wave count.

**Acceptance criteria:**
A complete sequence of waves can be played, and the game-over condition works correctly.

---

## Exercise 98 — Sound and visual polish

**Objective:** Give the game a finished presentation.

**Assignment:**

Implement sound and graphical feedback.

**Required effects:**
- Player firing.
- Enemy destruction.
- Player damage.
- Wave transition.
- Game over.

**Visual requirements:**
- Distinguishable player and enemy graphics.
- At least two enemy animation frames.
- Visible projectile graphics.
- Clear collision feedback.
- A readable score and lives display.

**Constraints:**
- Use the TIA's audio and graphics capabilities.
- Do not use external audio or graphics playback.
- Maintain display timing.
- Keep all animation data within the ROM budget.

**Acceptance criteria:**
All events produce the appropriate visual or audio feedback without disrupting gameplay.

---

## Exercise 99 — Integration, debugging, and optimisation

**Objective:** Prepare the game for release.

**Assignment:**

Perform a complete technical audit of OUTPOST.

**Requirements:**
- Inspect all game states.
- Inspect all memory allocations.
- Inspect stack usage.
- Inspect scanline timing.
- Inspect controller input.
- Inspect collision handling.
- Inspect score and lives.
- Inspect sound behaviour.
- Test game restart.
- Test prolonged execution.

**Required test scenarios:**
1. Start a new game.
2. Move against every arena boundary.
3. Fire repeatedly.
4. Destroy multiple enemies.
5. Take damage repeatedly.
6. Complete several waves.
7. Lose all lives.
8. Restart.
9. Repeat the entire process.

**Constraints:**
- Do not introduce new features during the audit.
- Every discovered defect must be documented.
- Optimisations must preserve existing behaviour.

**Acceptance criteria:**
No known game-breaking bugs, memory corruption, unstable display, or broken state transitions remain.

---

## Exercise 100 — The finished game

**Objective:** Demonstrate independent Atari 2600 game development.

**Assignment:**

Produce the final OUTPOST ROM.

You are responsible for the complete implementation, including:

- Program architecture.
- Memory allocation.
- Hardware initialisation.
- Frame synchronisation.
- Display kernel.
- Graphics.
- Controller input.
- Movement.
- Enemies.
- Projectiles.
- Collision detection.
- Sound.
- Scoring.
- Lives.
- Game progression.
- Title screen.
- Game-over screen.
- Restart functionality.

### Final constraints

- The ROM must assemble successfully using DASM.
- It must run in Stella.
- It must fit within 4 KB.
- It must use only the Atari 2600's standard internal resources.
- It must remain stable during extended play.
- It must not depend on a prebuilt game engine.

### Final deliverables

1. Complete assembly source code.
2. Compiled ROM.
3. Memory map.
4. Hardware register documentation.
5. Display kernel documentation.
6. Game-state diagram.
7. Testing notes.
8. A short explanation of the most difficult technical problems you encountered.

### Final assessment

You should be able to explain:

- How the 6507 executes your program.
- How your memory is organised.
- How the TIA generates the picture.
- How scanline timing works.
- How controller input is read.
- How the game updates its state.
- How collisions are detected.
- How sound is generated.
- How you stayed within the hardware limitations.

**The ultimate test:** Someone should be able to open your source code, assemble it, run the ROM, and play a complete game that you designed and programmed yourself.

---

# End of curriculum

Congratulations. If you complete all 100 exercises, you will have progressed from knowing nothing about assembly to independently developing a game for one of the most constrained and interesting game consoles ever made.

Remember:

- Understanding matters more than speed.
- A working program is not necessarily a well-understood program.
- Debugging is part of programming.
- You are allowed to get stuck.
- You are allowed to take breaks.
- You do not need to complete this course on anyone else's schedule.

**One instruction at a time. One scanline at a time. One game at a time.**
