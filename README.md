# A compiler for brainfuck

It works by first translating the brainfuck to rust, then compiling the rust code.
Thus, it requires a rust compiler.

```
USAGE:
    brainfuck-compiler [OPTIONS] <IN_FILE>

ARGS:
    <IN_FILE>    File to compile

OPTIONS:
    -d, --data-size <DATA_SIZE>    Size of the data array [default: 1024]
    -h, --help                     Print help information
    -o, --out-file <OUT_FILE>      
    -V, --version                  Print version information
```
