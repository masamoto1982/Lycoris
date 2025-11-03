# Lycoris

**A Stack Rewriting Language - Where the Stack IS the Program**

Lycoris is a concatenative programming language where every stack state represents both data and executable code. Programs are constructed by building stack states, and execution is the reduction of those states. Vectors act as protective membranes, controlling when and how code becomes data, and data becomes code.

## Core Concepts

### 1. Stack as Program

The stack itself is the program. Every stack state is simultaneously:
- A data structure (collection of values)
- An executable program (sequence of operations)

```
# Building a program
1 2 add 3 mul
# Stack: [1] [2] [add] [3] [mul]
# This IS the program "compute (1+2)*3"

# Executing
[1 2 add 3 mul] run
# Result: [9]
```

### 2. Vector as Protective Membrane

Vectors protect their contents from execution:

```
[1 2 add]    # Protected (data)
unpack       # Unprotected → executes
# Result: [3]
```

### 3. Scope Specification

Control the granularity of operations:

| Scope | Symbol | Meaning | Example |
|-------|--------|---------|---------|
| Local | (none) | Stack top N elements | `add` |
| Map | `@` | Apply to each vector element | `@mul` |
| Reduce | `*` | Fold entire vector | `*add` |
| Global | `#` | Treat whole stack as vector | `#add` |

## Language Features

### Static Typing (No Declaration Required)

All values carry type tags at runtime:
- **Rational**: Exact fraction arithmetic (no rounding errors)
- **String**: Text enclosed in single quotes `'text'`
- **Bool**: `true` or `false`
- **Nil**: `nil`
- **Vector**: Collection `[...]`

### Exact Rational Arithmetic

```
1 3 div          # 1/3 (exact fraction)
1 3 div 3 mul    # 1 (no precision loss)
```

### Postfix Notation

Consistent postfix (reverse Polish) notation throughout:

```
5 3 add          # 5 + 3 = 8
5 dup mul        # 5 * 5 = 25
```

## Quick Start

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install dependencies
npm install

# Build
npm run build

# Run development server
npm run dev
```

Open http://localhost:8080

### Basic Examples

```lycoris
# Arithmetic
5 3 add              # 8
10 3 div             # 10/3 (exact fraction)
5 dup mul            # 25 (5²)

# Vectors
[1 2 3]              # Create vector
[1 2 3] 2 @mul       # [2 4 6] (map multiply by 2)
[1 2 3 4 5] *add     # 15 (sum all elements)

# Stack operations
5 dup                # [5 5] (duplicate)
1 2 swap             # [2 1] (swap)
1 2 3 rot            # [2 3 1] (rotate)

# Custom words
[dup mul] 'square def    # Define 'square'
5 [square] run           # 25

# Program execution
[1 2 add 3 mul]      # Protected program
run                  # Execute → 9
```

## Built-in Words

### Arithmetic
- `add`, `sub`, `mul`, `div`, `pow`, `mod`
- `@add`, `@sub`, `@mul`, `@div` (map operations)
- `*add`, `*sub`, `*mul`, `*div` (reduce operations)

### Stack Operations
- `dup` - Duplicate top
- `drop` - Remove top
- `swap` - Exchange top two
- `over` - Copy second to top
- `rot` - Rotate top three

### Vector Operations
- `vec` - Create vector from N stack items
- `unpack` - Expand vector to stack
- `nth` - Get element at index
- `length` - Vector length
- `concat` - Join two vectors

### Execution Control
- `run` - Execute vector as program
- `quote` - Wrap value in vector (protect)

### Dictionary
- `def` - Define custom word
- `undef` - Remove definition

### I/O
- `print` - Output value
- `clear` - Clear output

## Advanced Examples

### Factorial

```lycoris
# Create range [1 2 3 4 5]
[1 2 3 4 5]
*mul        # 120
```

### Map and Reduce

```lycoris
# Square each element and sum
[1 2 3 4 5]
[dup mul] @apply    # Map square
*add                # Reduce sum
# Result: 55
```

### Custom Function Definition

```lycoris
# Define cube function
[dup dup mul mul] 'cube def

# Use it
3 [cube] run        # 27
```

## Technical Details

### Token Recognition

Lycoris uses longest-match dictionary lookup rather than whitespace:

```
Input:  1 2add3mul
Tokens: [1] [2] [add] [3] [mul]
```

Priority order:
1. String literals `'...'`
2. Vector literals `[...]`
3. Numbers (integer/fraction/scientific)
4. Reserved words (true/false/nil)
5. Dictionary longest match (with scope prefix)

### Trie-based Dictionary

Functions are stored in a trie (prefix tree) for efficient longest-match lookup.

### Technology Stack

- **Rust**: Core language implementation, compiled to WebAssembly
- **WebAssembly**: Fast execution in browser
- **TypeScript**: UI and browser integration
- **num-rational/num-bigint**: Exact arithmetic

## Project Structure

```
lycoris/
├── src/
│   ├── lib.rs          # Core Rust implementation
│   └── main.ts         # TypeScript UI
├── www/
│   ├── index.html      # Entry point
│   ├── styles.css      # Styling
│   ├── pkg/           # (generated) WASM module
│   └── js/            # (generated) TypeScript output
├── Cargo.toml         # Rust configuration
├── package.json       # Node.js configuration
└── tsconfig.json      # TypeScript configuration
```

## Design Philosophy

### Homoiconicity

Code and data are unified through vectors. Everything is a vector, and every vector can be executed.

### Explicit Control

Unlike lazy evaluation languages, Lycoris makes execution explicit through `run` and scope modifiers.

### Reversibility

The design supports potential future features like `unstep` to reverse execution.

### Clarity

Stack state is always visible, making the program's execution transparent.

## Comparison with Other Languages

| Language | Paradigm | Key Difference |
|----------|----------|----------------|
| FORTH | Stack-based | Stack is workspace; Lycoris treats stack as program itself |
| Lisp | List-based | Code/data separated; Lycoris unifies through vectors |
| Joy | Concatenative | Quotations are special; Lycoris uses vectors uniformly |
| Haskell | Functional | Implicit laziness; Lycoris has explicit protection (vectors) |

## Future Roadmap

- [ ] IndexedDB persistence
- [ ] Web Worker for heavy computations
- [ ] `step`/`unstep` for stepwise execution
- [ ] Pattern matching
- [ ] More standard library functions
- [ ] REPL improvements

## License

MIT License

## Author

masamoto yamashiro

---

**Lycoris** - Like the red spider lily that blooms on the boundary between worlds, Lycoris exists at the boundary between code and data, between stack and program.
