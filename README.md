# 🌺 Lycoris

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=flat&logo=webassembly&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue)

Lycoris (Red Spider Lily) - A meta-circular programming language where code and data are one.

## Features

- **Meta-circular evaluator**: Code is data, data is code
- **Self-modifying programs**: Programs that rewrite themselves
- **Minimal core**: ~500 lines implementing a complete language
- **Stack-based**: FORTH-inspired with LISP's meta-programming power
- **WebAssembly powered**: Runs at native speed in browsers

## Quick Start
```forth
# Define factorial using meta-programming
[ [ n ] [ n 1 = : 1 : n n 1 - FACT * ] ] 'FACT' DEF
5 FACT PRINT  # => 120

# Self-referential code
[ 'SELF' ? @ ] 'SELF' DEF

# Higher-order functions
[ 1 2 3 4 5 ] [ 2 * ] MAP  # => [ 2 4 6 8 10 ]
