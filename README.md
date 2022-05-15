# pangalactic-creditconverter

[![Rust](https://github.com/gglyptodon/pangalactic-creditconverter/actions/workflows/rust.yml/badge.svg)](https://github.com/gglyptodon/pangalactic-creditconverter/actions/workflows/rust.yml)

A helpful calculator for hitchiking merchants all across the galaxy and beyond.

- Converts alien numerals and units to Credits.
- Input is read from stdin or text file.
- Output is printed to stdout.

---
You can find the crate's documentation generated from docstrings here:
https://gglyptodon.github.io/pangalactic-creditconverter/doc/pangalacticcc/
---

### How to compile ###
- follow instructions from https://www.rust-lang.org/tools/install for your operating system
- clone this repository, change to project directory, and run or build with ``cargo``: 
  - ``git clone https://github.com/gglyptodon/pangalactic-creditconverter.git``
  - ``cd pangalactic-creditconverter/pangalacticcc``
  - ``cargo build --release`` will build the project.
    - the compiled binary can then be found under ``target/release/pangalacticcc`` .
    - ``target/release/pangalacticcc -h `` displays a help message
    - ``target/release/pangalacticcc tests/input/input1.txt `` runs the program on the provided example data.
  - alternatively, run via cargo:  
    - ``cargo run -- -h`` will display a help message.
    - ``cargo run -- tests/input/input1.txt `` will display example output.


### How to run tests ###
- ``git clone https://github.com/gglyptodon/pangalactic-creditconverter.git``
- ``cd pangalactic-creditconverter/pangalacticcc``
- ``cargo test`` runs all provided tests.

----

### Usage ###

```
Pangalactic Credit Converter

USAGE:
    pangalacticcc [FILE]

ARGS:
    <FILE>    Input file with gathered information and queries. If set to '-' or no FILE is
              specified, input is read from stdin. [default: -]

OPTIONS:
    -h, --help    Print help information

```

---
### Assumptions ###
- input is provided via text file or stdin
- input data is not excessively large
- only one set of "notes" is provided per run
- information for single numerals is provided with no further calculations necessary, e.g.
  - bla is X ✅
  - blub is IV ❌
- alien numerals are unique
- alien numerals do not contain dashes `-` or other special characters
- "Credits" is a known unit
- phrases are following the examples provided in ``tests/input1.txt``, i.e.
  - <alien_numeral_x> is I
  - [...]
  - <alien_numeral_y> is L 
  - [...]
  - <required_alien_numeral_i>  <optional_alien_numeral_j [...] optional_alien_numeral_z> <required_alien_unit> is <required_amount_in_arabic_numerals> Credits.
  - how much is <required_alien_numeral> <optional_alien_numeral_j [...] optional_alien_numeral_z> ?
  - how many Credits is <required_alien_numeral> <optional_alien_numeral_j [...] optional_alien_numeral_z> <required_alien_unit> ?
- empty lines will be ignored    
- anything that does not fit the above structure will be responded to with ``I have no idea what you are talking about`` unless a more specific response can be applied.
- any question that cannot be answered will be responded to with the above response, too.
- input lines can be in any order, e.g. it is ok if questions come first.
- output will be printed in the order the questions are asked
 
