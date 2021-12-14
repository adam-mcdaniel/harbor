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

|Operator|Description|
|-|-|
|<|Move the pointer one cell to the left.|
|>|Move the pointer one cell to the right.|
|+|Increment the current cell by 1.|
|-|Decrement the current cell by 1.|
|[|Begin a loop while the cell at the pointer is not zero.|
|]|Mark the ending of a loop body.|
|,|Make the current cell equal to the next byte of input.|
|.|Output the current cell as a byte.|

Dynamic Brainf*** provides six additional operators: two for memory management, two for pointer manipulation, and two for better IO. With these new operators, it's significantly simpler to compile common abstractions like pointers, stack operations, and compound datatypes.

|Operator|Description|
|-|-|
|?|Read the value of the current cell, and allocate that many cells at the end of the tape. Then, set the current cell's value equal to the index of first cell in that allocated block.|
|!|Read the value of the current cell, and free the allocated cells starting at that index.|
|*|Push the pointer to a stack, and set the pointer equal to the value of the current cell.|
|&|Pop the old pointer off the dereference stack, and set the pointer equal to it.|
|#|Make the current cell equal to the next integer in the input buffer.|
|$|Output the current cell as an integer.|

## How does it work?

Harbor source code goes through three stages before the output code: HIR, MIR, and LIR.

![Flow](./assets/flow.svg)

HIR provides a typesystem and performs typechecking, MIR provides a small untyped reverse-polish-notation assembly language, and LIR is an internal representation of Dynamic Brainfuck specially structured to optimize generated code.

The most interesting part of the compilation process is the transition from Harbor MIR to Dynamic Brainfuck. Harbor MIR looks like this:

<img alt="MIR" src="./assets/fib_mir.png" style="width: 50%"/>

