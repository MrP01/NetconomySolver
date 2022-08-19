# Snake Cube Solver

Solver for a cube puzzle by Netconomy.

I received this cube as a promotional gift a few years ago, the problem sparked my interest because noone I knew could solve it by hand (in feasible time) so I decided to code it up in Rust (first time for me).
There are 4294967296 (4^16) technically possible combinations of the cube and potentially multiple solutions.

To run the solver (with the associated GUI):

```bash
cargo build
cargo run
```

Press 1,2,3...,9 to mutate corners 1 to 9 respectively, press [Space] to perform one mutation and press [S] to start the solver.

![Screenshot](https://user-images.githubusercontent.com/12398709/185673734-a1d2fe60-d542-4d5c-be7d-c75186f54805.png)

(This project is in no way affiliated with *NETCONOMY GmbH*)
