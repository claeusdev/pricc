# pricc

A minimal C project generator written in Rust.

## Installation

```bash
cargo install --path .
```

## Usage

Create a new C project:
```bash
pricc my_project                              # Basic project
pricc my_project -a "John Doe" -d "My App"    # With metadata
pricc my_project --standard c17 --tests       # C17 with tests
```

Available options:
```
Options:
  -a, --author <AUTHOR>            Author of the project
  -d, --description <DESCRIPTION>  Description of the project
  -s, --standard <STANDARD>        C standard [default: c11] [possible values: c89, c99, c11, c17]
  -t, --tests                      Include test setup
  -V, --proj-version <VERSION>     Project version [default: 0.1.0]
  -h, --help                       Print help
  -v, --version                    Print version
```

## Generated Project Structure

```
project_name/
├── src/           # Source files
│   └── main.c
├── include/       # Header files
│   └── project.h
├── tests/         # Test files (if enabled)
│   └── test_main.c
├── build/         # Build artifacts
├── bin/           # Compiled binaries
├── Makefile
└── README.md
```

## License

MIT
