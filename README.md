# Interpreter
This is an tiny interpreter project based on pratt parsing algorithm. This interpreter is fully written in rust.

## Pratt Parsing
Prat parsing is a technique for parsing expressions with operators and precedence. It also called Top-Down Operator Precedence parser.

Instead of writing many grammar rules for every operator precedence level, Pratt parsing uses functions associated with tokens to decide how expressions should be parsed.
Each token can define how it behaves in two contexts:
1. Prefix position (beginning of an expression)
2. Infix position (between expressions)

## Example
Execute the interpreter
```bash
cargo run
```
```bash
>> 5 - 2 + 3
6
>> a = 20
>> b = 30 
>> a * b
600
```
