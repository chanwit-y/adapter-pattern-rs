# Adapter Pattern in Rust

This implementation demonstrates the **Adapter Design Pattern** based on the concepts described in [Refactoring Guru: Adapter](https://refactoring.guru/design-patterns/adapter).

## Overview

The **Adapter** is a structural design pattern that allows objects with incompatible interfaces to collaborate. In this example, we have a `RoundHole` that only accepts objects implementing the `Circular` trait. We use an adapter to allow `SquarePeg` (which is not circular) to be used within the hole.

## Implementation

```rust
/// The Target trait that the client code expects.
trait Circular {
    fn get_radius(&self) -> f64;
}

/// The Service: A round hole that can only fit circular objects.
struct RoundHole {
    radius: f64,
}

impl RoundHole {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    /// The client method that requires the Circular interface.
    fn fits<T: Circular>(&self, peg: T) -> bool {
        self.radius >= peg.get_radius()
    }
}

/// A compatible concrete implementation of the Circular trait.
struct RoundPeg {
    radius: f64
}

impl Circular for RoundPeg {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

/// The Adaptee: An incompatible class that needs to be adapted.
/// Square pegs measure width, not radius.
struct SquarePeg {
    width: f64,
}

impl SquarePeg {
    fn new(width: f64) -> Self {
        Self { width }
    }

    fn get_width(&self) -> f64 {
        self.width
    }
}

/// The Adapter: Makes SquarePeg compatible with the Circular trait.
struct SquarePegAdapter {
    peg: SquarePeg,
}

impl SquarePegAdapter {
    fn new(peg: SquarePeg) -> Self {
        Self { peg }
    }
}

impl Circular for SquarePegAdapter {
    /// The adapter calculates the minimum circle radius that can fit the square.
    /// Radius = (width * √2) / 2
    fn get_radius(&self) -> f64 {
        self.peg.get_width() * f64::sqrt(2.0) / 2.0
    }
}

pub fn run() {
    let hole = RoundHole::new(5.0);

    // 1. Working with compatible types
    let rpeg = RoundPeg { radius: 5.0 };
    println!("Round peg r5 fits round hole r5: {}", hole.fits(rpeg));

    // 2. Using the Adapter for incompatible SquarePegs
    let small_sqpeg = SquarePeg::new(5.0);
    let small_sqpeg_adapter = SquarePegAdapter::new(small_sqpeg);

    let large_sqpeg = SquarePeg::new(10.0);
    let large_sqpeg_adapter = SquarePegAdapter::new(large_sqpeg);

    // The hole accepts the adapter because it implements Circular
    println!("Square peg w5 fits round hole r5: {}", hole.fits(small_sqpeg_adapter));
    println!("Square peg w10 doesn't fit round hole r5: {}", !hole.fits(large_sqpeg_adapter));
}

```

## How it Works

1. **The Target (`Circular` trait)**: Defines the interface used by the `RoundHole`.
2. **The Adaptee (`SquarePeg`)**: Has a useful functionality (width) but an incompatible interface.
3. **The Adapter (`SquarePegAdapter`)**: Wraps the `SquarePeg` and implements the `Circular` trait. It translates the call to `get_radius()` into a calculation based on the square's width.
4. **The Client (`RoundHole`)**: Can now interact with `SquarePeg` indirectly through the adapter without needing to know the square's internal math.