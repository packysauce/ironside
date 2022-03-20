# Ironside
## I strapped Rust onto Klipper and the name chose itself

# Requirements
- [Rust](https://www.rust-lang.org/learn/get-started)
- https://xpack.github.io/qemu-arm/ at some point
# Dev Setup
TL; DR - 
```sh
git clone https://github.com/packysauce/ironside
cd ironside
git submodule update --init
cd klipper
```
[go build klipper](klipper/docs/Installation.md#building-and-flashing-the-micro-controller)
```
cargo test 
```

If that goes well, congrats you're set up

# Overall, like, vibe, man
I pretty much only like stuff if it's exciting.
Things are exciting when there's lots to learn and discover.
Learning and discovering also requires trial and error.
Trial and error speak for themselves.

Trials can be made to take less time through benchmarks, more/better code gen, wider library support, simulator/emulator usage, etc.
Errors are not to be avoided, they are to be _handled_.
Mistakes are merely "unexpected bug reports", and should be made impossible via compiler enforcement.

I'm a big fan of [pits of success](https://blog.codinghorror.com/falling-into-the-pit-of-success/).
The right thing should be obvious, mundane stuff should be automatic, and stupid/dangerous/promise-its-for-debugging stuff should be hard - [*not necessarily* impossible](https://doc.rust-lang.org/nomicon/#the-dark-arts-of-unsafe-rust)

"If it compiles, it works"

# What is this project?
Use every swanky feature Rust has to demonstrate how I envision
"reinforcing" C with Rust.
- load up with all kinds of [targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) for testing and even {si,e}mulating klipper
- pull in [crates](https://crates.io/) to avoid reinventing wheels
- think of all the [Zero Cost Abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html)

It's a love letter, of sorts. A ship in a bottle, if you will.
A zen garden for me to write some rust, and get to know a neat
piece of software while I'm at it.

# Who's this project for?
Me. Perhaps any other 3d printing rustaceans?

# Objectives
Clarity above all. [Code runs on people](https://rachelbythebay.com/w/2021/09/05/clever/)
- I like to turn off my inlay hints every once in a while, yaknow? see how the other side lives
Encourage experimentation in Klipper through safe APIs and testing
- cargo test, cargo bench, examples, and binaries oh my!

