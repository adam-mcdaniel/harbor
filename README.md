<div align="center">
  <h1>⚓harbor⚓</h1>

  <p>
    <strong>A language that ports: examining the limits of compilation.</strong>
  </p>

  <p>
    <a href="https://www.buymeacoffee.com/adam.mcdaniel" rel="nofollow"><img src="https://camo.githubusercontent.com/6315cdb9b889562e5b4d78fc3ae8c44852b7826c228f0b59b379b53fb57c6eaf/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f242d737570706f72742d6666363962342e7376673f7374796c653d666c6174" alt="Donate" data-canonical-src="https://img.shields.io/badge/$-support-ff69b4.svg?style=flat" style="max-width:100%;"></a>
    <a href="https://opensource.org/" rel="nofollow"><img src="https://badges.frapsoft.com/os/v3/open-source.svg?v=103" alt="Open Source" data-canonical-src="https://badges.frapsoft.com/os/v3/open-source.svg?v=103" style="max-width:100%;"></a>
  </p>

  <h3>
    <a target="_blank" href="https://adam-mcdaniel.github.io/harbor">Demo</a>
    <span> | </span>
    <a href="https://crates.io/crates/harbor/">Crates</a>
    <span> | </span>
    <a target="_blank" href="http://adam-mcdaniel.net">Contact Me</a>
  </h3>

  <sub>Written in Rust🦀💖</sub>
</div>

<div align="center">
  <a target="_blank" href="https://adam-mcdaniel.github.io/harbor">
    <img alt="Compiled Fibonacci sequence program" src="./assets/fib.png" width="66.9%"/>
  </a>
  <a target="_blank" href="https://adam-mcdaniel.github.io/harbor">
    <img alt="Dynamic Brainfuck interpreter" src="./assets/mem.gif" width="31.9%"/>
  </a>
  <!-- <a target="_blank" href="https://adam-mcdaniel.github.io/harbor">
    <img alt="Compiled Fibonacci sequence program" src="./assets/fib.png" width="68%"/>
  </a>
  <a target="_blank" href="https://adam-mcdaniel.github.io/harbor">
    <img alt="Dynamic Brainfuck interpreter" src="./assets/mem.gif" width="29%"/> -->
  </a>
</div>

***NOTE: Click the images above for an interactive demonstration!***

## About the Author

I'm a *bored* sophomore in college working on projects to fill the time. If you enjoy my work, consider supporting me by buying me a coffee!

<a href="https://www.buymeacoffee.com/adam.mcdaniel" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-blue.png" alt="Buy Me A Coffee" height="60px" width="217px"/>
</a>

## What is this project?

Harbor is a high level programming language with type checking (supports unsigned integers, booleans, characters, pointers, tuples) and manual memory management. What does that mean? Harbor is basically a stripped down version of C. What makes Harbor special then? It compiles to a dialect of [Brainf***](https://www.youtube.com/watch?v=hdHjjBS4cs8) called [Dynamic Brainf***](https://adam-mcdaniel.github.io/harbor).


Brainfuck programs are composed entirely of the following operators *only*:
<div>
  <img alt="MIR" align="right" width="300" src="./assets/interpreter.gif"/>
  <table>
    <tr>
      <th>Operator</th>
      <th>Description</th>
    </tr>
    <tr>
      <td>&lt;</td>
      <td>Move the pointer one cell to the left.</td>
    </tr>
    <tr>
      <td>&gt;</td>
      <td>Move the pointer one cell to the right.</td>
    </tr>
    <tr>
      <td>+</td>
      <td>Increment the current cell by 1.</td>
    </tr>
    <tr>
      <td>-</td>
      <td>Decrement the current cell by 1.</td>
    </tr>
    <tr>
      <td>[</td>
      <td>Begin a loop while the cell at the pointer is not zero.</td>
    </tr>
    <tr>
      <td>]</td>
      <td>Mark the ending of a loop body.</td>
    </tr>
    <tr>
      <td>,</td>
      <td>Make the current cell equal to the next byte of input.</td>
    </tr>
    <tr>
      <td>.</td>
      <td>Output the current cell as a byte.</td>
    </tr>
  </table>
</div>
<!-- 
|Operator|Description|
|-|-|
|<|Move the pointer one cell to the left.|
|>|Move the pointer one cell to the right.|
|+|Increment the current cell by 1.|
|-|Decrement the current cell by 1.|
|[|Begin a loop while the cell at the pointer is not zero.|
|]|Mark the ending of a loop body.|
|,|Make the current cell equal to the next byte of input.|
|.|Output the current cell as a byte.| -->

Dynamic Brainf*** provides six additional operators: two for memory management, two for pointer manipulation, and two for better IO. With these new operators, it's possible to compile common abstractions like references, stack operations, and compound datatypes.

|Operator|Description|
|-|-|
|?|Read the value of the current cell, and allocate that many cells at the end of the tape. Then, set the current cell's value equal to the index of first cell in that allocated block.|
|!|Read the value of the current cell, and free the allocated cells starting at that index.|
|*|Push the pointer to a stack, and set the pointer equal to the value of the current cell.|
|&|Pop the old pointer off the dereference stack, and set the pointer equal to it.|
|#|Make the current cell equal to the next integer in the input buffer (like `scanf("%d", &tape[pointer])`).|
|$|Output the current cell as an integer (like `printf("%d", tape[pointer])`).|

## How does it work?

Harbor source code goes through three stages before the output code: HIR, MIR, and LIR.

![Flow](./assets/flow.svg)

HIR provides a typesystem and performs typechecking, MIR provides a small untyped reverse-polish-notation assembly language, and LIR is an internal representation of Dynamic Brainf\*\*\* specially structured to optimize generated code.

The most interesting part of the compilation process is the transition from Harbor MIR to Dynamic Brainfuck. Harbor MIR looks like this:

<img alt="MIR" src="./assets/fib_mir.png" style="float: left; width: 48%"/><img alt="Fib DBF" src="./assets/fib_dbf.png" style="float: right; width: 44.20%"/>

### Memory Layout

MIR provides 14 registers:
- `SP`: the stack pointer.
- `FP`: the frame pointer.
- `TMP0` through `TMP5`: 6 temporary registers for helping with arithmetic operations. These are for the compiler only.
- `R0` through `R5`: 6 general purpose registers for the user.

The registers are statically allocated by the compiler at the first 14 cells, with the stack beginning immediately after.

![Registers](./assets/registers.svg)

You might notice that `FP` strangely comes after `TMP0` and `TMP1`, but before `TMP2`. There's a good reason for this: copying memory cells in Brainf*** dialects is a *very expensive* (and very frequent) operation. When memory is copied, it uses `TMP0` as a buffer for the assignment code:

![Copy Cell](./assets/copy_cell.png)

So, `TMP0` is placed before `FP` to increase locality, but I'm sure the effect is negligible. `TMP1` is also placed before `FP` for similar reasons: it's used frequently in almost all alrithmetic operations. `TMP2` through `TMP5` are more specialized registers, mainly used for integer division, multiplication, and setting up stack frames and activation records for functions.

### MIR Opcodes

|Opcode|Description|
|-|-|
|`set 123`|Pops an address and stores a value at that address|
|`=`|Pops an address and pops a value into that address. Also takes an optional size parameter for the number of cells store at the address like: `%int`, `%(%int, %bool)`, or `%char`.|
|`@`|Pops an address and loads a value from that address. Also takes an optional size parameter to load from the address like: `%int`, `%(%int, %int)`, or `%char`.|
|`get %int`|Pushes a block of memory on the stack with the given size. `%int` allocates one cell, `%(%int, %int)` allocates 2, etc.|
|`dump %int`|Deallocates a block of memory on the stack with the given size.|
|`123`|Integer literals are pushed to the stack.|
|`+`|Pop two numbers off the stack and push their result.|
|`-`|Pop two numbers off the stack and push their difference.|
|`*`|Pop two numbers off the stack and push their product.|
|`/`|Pop two numbers off the stack and push their quotient.|
|`==`|Pop two numbers off the stack and push their equality.|
|`!=`|Pop two numbers off the stack and push their inequality.|
|`\|`|Pop two numbers off the stack and push their logical or (anything not zero is true).|
|`&`|Pop two numbers off the stack and push their logical and.|
|`!`|Pop a number off the stack and push its logical complement.|
|`alloc`|Pop a number off the stack, allocate that many cells at the end of the tape, and push the address of the allocated block.|
|`free`|Pop an address off the stack and free the cells at that block.|
|`dup`|Duplicate the top cell on the stack.|
|`frame %int -> %(%int, %int) do ... end`|Create a stack frame for a code block that takes an argument and returns a value. The FP points at the first argument, and the return value is left on the stack when the code block ends after the frame is destructed.|
|`if (2 4 *) do ... end`|Perform an if statement. Else clauses are not supported.|
|`$R0`, `$R1`, ..., `$R5`|Push a register's value onto the stack.|
|`&R0`, `&R1`, ..., `&R5`|Push a register's address onto the stack.|

There are also 6 predefined macros for MIR. `putnum` and `putchar` both pop a cell off the stack and print it. `getchar` retrieves a byte of user input and pushes it onto the stack. `getnum` retrieves an integer from user input and pushes it as well. Finally, `inc` and `dec` increment or decrement the value pointed to by the top value on the stack.

MIR opcodes are composed of a sort of "microcode" that's really interesting and fun to write/optimize. The code generator for the addition opcode illustrates this pretty well:

![Addition](./assets/addition.png)

Originally, I implemented addition by popping the two values into temporary registers (`TMP1` and `TMP2`), performing the addition, and then pushing the result onto the stack. This solution is much more efficient, as everything is done in place instead of moving values around in memory!

It's also extremely satisfying to see the result of the optimizations on the output code as well: because everything implemented in brainfuck seems to be on the order of O(n^2), any reduction in memory usage seems to have a dramatic effect.