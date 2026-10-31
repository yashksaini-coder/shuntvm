# shuntvm

[![Crates.io](https://img.shields.io/crates/v/shuntvm.svg)](https://crates.io/crates/shuntvm)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Zero Panic](https://img.shields.io/badge/safety-zero--panic-brightgreen.svg)](#safety-guarantees)

**`shuntvm`** is a deterministic, zero-dependency mathematical and bitwise expression engine written in safe Rust. It compiles infix expressions into Reverse Polish Notation (RPN) via Dijkstra's Shunting-Yard algorithm and executes the resulting bytecode stream on a stateful stack virtual machine.

Built with strict compile-time lint enforcement (`#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]`), `shuntvm` guarantees that arbitrary or malformed user input never causes a runtime panic—every lexical, syntactic, and mathematical fault is surfaced as a strongly-typed `ShuntError`.

---

## Architecture Pipeline

```text
Raw Input (&str)
   │
   ▼  [Stage 1: Lexer (src/lexer.rs)]
   │  • Peekable character stream scanner
   │  • Multi-base literal parsing (Decimal, 0x Hex, 0b Binary)
   │  • Context-sensitive unary negation disambiguation
   │
Infix Token Stream (Vec<Token>)
   │
   ▼  [Stage 2: Shunting-Yard Compiler (src/parser.rs)]
   │  • Operator precedence & associativity resolution
   │  • Function call & parenthesis validation via operator stack
   │
Postfix / RPN Bytecode (Vec<Token>)
   │
   ▼  [Stage 3: Stack Virtual Machine (src/vm.rs)]
   │  • Operand evaluation stack (Vec<f64>)
   │  • Strict integer guards for bitwise & factorial operations
   │  • Stateful session register memory (HashMap<String, f64>)
   │
Result<f64, ShuntError>
```

---

## Key Features

- **Zero-Panic Execution**: Total resilience against invalid syntax, stack underflows, domain violations, and division by zero.
- **Zero External Dependencies**: Implemented entirely with the Rust standard library (`std`).
- **Multi-Base Numeric Literals**: Native support for IEEE-754 floats (`3.1415`), hexadecimal (`0xFF`), and binary (`0b1010`) literals.
- **Bitwise Operations with Integer Guards**: Supports bitwise AND (`&`), OR (`|`), XOR (`^`), left shift (`lshift`), and right shift (`rshift`). Rejects floating-point numbers with fractional components via `ShuntError::IntegerRequired`.
- **Context-Aware Unary Operators**: Automatically distinguishes binary subtraction (`5 - 3`) from unary negation (`-5`, `3 * -2`, `--4`, `-5 ^ 2`) at the lexical boundary.
- **Right-Associative Exponentiation**: Properly evaluates chained powers (`2 ^ 3 ^ 2` evaluates to `2 ^ 9 = 512`, not `64`).
- **Stateful Register Memory**: Built-in constants (`pi`, `e`), automatic `ans` register tracking, and user-defined variable assignment (`x = 0x10 * 2`).

---

## Supported Grammar & Precedence Table

Operators are listed from highest to lowest precedence:

| Precedence | Operator / Construct                       | Associativity | Description                                        |
| :--------: | :----------------------------------------- | :-----------: | :------------------------------------------------- |
|   **6**    | `sin`, `cos`, `sqrt`, `abs`, `log`, `fact` |    Prefix     | Built-in scientific & combinatorial functions      |
|   **5**    | `^` (or `**`)                              |     Right     | Exponentiation                                     |
|   **5**    | `-` (Unary)                                |     Right     | Unary negation                                     |
|   **4**    | `*`, `/`, `%`                              |     Left      | Multiplication, division, modulo                   |
|   **3**    | `+`, `-`                                   |     Left      | Addition, subtraction                              |
|   **2**    | `lshift`, `rshift`                         |     Left      | Bitwise arithmetic shifts (integer guard enforced) |
|   **1**    | `&`, `^` (xor), `\|`                       |     Left      | Bitwise AND, XOR, OR (integer guard enforced)      |
|   **0**    | `=`                                        |     Right     | Variable assignment (`ident = expr`)               |

---

## Quick Start

### Interactive REPL CLI

<!--


Clone the repository and launch the interactive virtual machine shell:

```bash
git clone [https://github.com/yashksaini-coder/shuntvm.git](https://github.com/yashksaini-coder/shuntvm.git)
cd shuntvm
cargo run --release
```
-->

Example session:

```text
shuntvm v0.1.0 — Zero-Panic RPN Expression Engine
Type 'exit' or 'quit' to terminate.

>> 3 + 4 * 2 / (1 - 5) ^ 2 ^ 3
3.0001220703125

>> mask = 0b1010 | 0x05
15

>> (mask & 0x0F) + sqrt(16)
19

>> 3.5 & 2
Error: IntegerRequired("Bitwise operations require exact integer operands")

>> 10 / (5 - 5)
Error: DivisionByZero
```

### Library Usage

`shuntvm` can be embedded directly as a library crate to evaluate expressions inside your own Rust applications:

```rust
use shuntvm::{Engine, ShuntError};

fn main() -> Result<(), ShuntError> {
    let mut vm = Engine::new();

    vm.eval("radius = 5")?;
    let area = vm.eval("pi * radius ^ 2")?;

    println!("Computed Area: {:.4}", area);
    Ok(())
}
```

---

## Safety & Error Taxonomy (`ShuntError`)

Every failure state across the lexer, parser, and virtual machine maps to a deterministic variant in `ShuntError`:

- `UnexpectedChar(char, usize)` — Unrecognized symbol encountered at byte/char offset.
- `MalformedNumber(String)` — Invalid numeric literal (e.g., `3.14.15`, `0xZZ`, lone `.`).
- `MismatchedParentheses` — Unclosed `(` or stray `)` detected during Shunting-Yard compilation.
- `EmptyExpression` — Empty input or whitespace-only expression.
- `StackUnderflow(String)` — Operator or function invoked with insufficient operands on the VM stack.
- `TrailingValues(usize)` — Evaluation terminated with multiple unconsumed values on the stack (e.g., `5 5`).
- `DivisionByZero` — Attempted division (`/`) or modulo (`%`) by `0.0`.
- `DomainError(String)` — Mathematical domain violation (e.g., `sqrt(-1)`, `log(0)`).
- `UndefinedVariable(String)` — Referenced identifier does not exist in VM register memory.
- `IntegerRequired(String)` — Bitwise or factorial operation invoked on a number with a non-zero fractional part.

---

## Running the Test Suite

Run the unit and integration test suites (covering precedence, associativity, multi-base literals, register state, and all error variants):

```bash
cargo test
cargo clippy -- -D warnings
```

## License

Licensed [MIT license](LICENSE) at your option.
