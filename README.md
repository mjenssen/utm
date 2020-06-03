# Universal Turing Machine

This is an implementation of a universal Turing machine in Rust. The machine itself is implemented in `src/lib.rs` and there are 4 examples that implement the tasks presented by [Rosetta Code](https://rosettacode.org/wiki/Universal_Turing_machine). For completeness, these tasks are presented here as well.

## Running the examples

To run the examples, use `cargo`:

    $ cargo run --example <name> --release

Where `<name>` is one of `increment`, `sorting`, `busy_beaver3`, or `busy_beaver5`.

## Terminology

The universal Turing machine is expected to use rules that are written as a 5-element tuple:

    (current state, current value, new value, direction, new state)

So given the current state and symbol, the rule will return a new value, the direction to move the position in the tape, and the new state.

Direction can be one of left, right, or stay.

## Simple incrementer

- States: Q0, QF
- End state: QF
- Tape: [1, 1, 1]

Rules:

| Current state | Current value | New value | Direction | New state |
|---------------|--------------:|----------:|-----------|-----------|
| Q0            |             1 |         1 | Right     | Q0        |
| Q0            |             0 |         1 | Stay      | QF        |

## Sorting

- States: A, B, C, D, E, Halt
- End state: Halt
- Tape: [2, 2, 2, 1, 2, 2, 1, 2, 1, 2, 1, 2, 1, 2]

| Current state | Current value | New value | Direction | New state |
|---------------|--------------:|----------:|-----------|-----------|
| A             |             1 |         1 | Right     | A         |
| A             |             2 |         3 | Right     | B         |
| A             |             0 |         0 | Left      | E         |
| B             |             1 |         1 | Right     | B         |
| B             |             2 |         2 | Right     | B         |
| B             |             0 |         0 | Left      | C         |
| C             |             1 |         2 | Left      | D         |
| C             |             2 |         2 | Left      | C         |
| C             |             3 |         2 | Left      | E         |
| D             |             1 |         1 | Left      | D         |
| D             |             2 |         2 | Left      | D         |
| D             |             3 |         1 | Right     | A         |
| E             |             1 |         1 | Left      | E         |
| E             |             0 |         0 | Right     | Halt      |

## Busy beaver (3-state, 2-symbol)

- States: A, B, C, Halt
- End state: Halt
- Tape: []

Rules:

| Current state | Current value | New value | Direction | New state |
|---------------|--------------:|----------:|-----------|-----------|
| A             |             0 |         1 | Right     | B         |
| A             |             1 |         1 | Left      | C         |
| B             |             0 |         1 | Left      | A         |
| B             |             1 |         1 | Right     | B         |
| C             |             0 |         1 | Left      | B         |
| C             |             1 |         1 | Stay      | Halt      |

## Busy beaver (5-state, 2-symbol)

- States: A, B, C, D, E, Halt
- End state: Halt
- Tape: []

Rules:

| Current state | Current value | New value | Direction | New state |
|---------------|--------------:|----------:|-----------|-----------|
| A             |             0 |         1 | Right     | B         |
| A             |             1 |         1 | Left      | C         |
| B             |             0 |         1 | Right     | C         |
| B             |             1 |         1 | Right     | B         |
| C             |             0 |         1 | Right     | D         |
| C             |             1 |         0 | Left      | E         |
| D             |             0 |         1 | Left      | A         |
| D             |             1 |         1 | Left      | D         |
| E             |             0 |         1 | Right     | Halt      |
| E             |             1 |         0 | Left      | A         |

## References

For more details about universal Turing machines as well as the busy beaver, here are some links:

- [Rosetta Code: Universal Turing machine](https://rosettacode.org/wiki/Universal_Turing_machine)
- [Wikipedia: Universal Turing machine](https://en.wikipedia.org/wiki/Universal_Turing_machine)
- [Wikipedia: Turing machine examples](https://en.wikipedia.org/wiki/Turing_machine_examples)
- [Wikipedia: Busy beaver](https://en.wikipedia.org/wiki/Busy_beaver)
- [Pascal Michel: Historical Survey of Busy Beavers](http://www.logique.jussieu.fr/~michel/ha.html)

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
