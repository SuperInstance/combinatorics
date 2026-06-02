# combinatorics

Combinatorics in Rust. Counting things, elegantly.

## Features

- **Basic counting**: factorial, permutations, combinations, multinomial coefficients, stars & bars
- **Catalan numbers**: Dyck paths, binary trees, balanced parentheses
- **Integer partitions**: partition numbers, Young diagrams, Euler's pentagonal theorem
- **Inclusion-Exclusion**: principle and applications
- **Pigeonhole principle**: minimum load bounds
- **Generating functions**: ordinary and exponential, coefficient extraction
- **Graph coloring**: chromatic polynomial, greedy coloring
- **Tree enumeration**: Cayley's formula, Prüfer sequences
- **Burnside's lemma**: counting under symmetry
- **Topology enumeration**: network configurations, hierarchies, pipelines

## Usage

```toml
[dependencies]
combinatorics = "0.1.0"
```

```rust
use combinatorics::*;

// Basic counting
assert_eq!(factorial(5), 120);
assert_eq!(binomial(10, 3), 120);

// Catalan numbers
assert_eq!(catalan(4), 14);

// Stars and bars
assert_eq!(stars_and_bars(5, 3), 21);

// Burnside's lemma
let fixes = vec![16, 2, 2, 4, 2, 2, 4, 2];
assert_eq!(burnside_orbits(8, &fixes), 4);

// Topology summary
let s = topology_summary(3);
assert_eq!(s.communication_topologies, 3);
assert_eq!(s.hierarchies, 9);
```

## License

MIT OR Apache-2.0
