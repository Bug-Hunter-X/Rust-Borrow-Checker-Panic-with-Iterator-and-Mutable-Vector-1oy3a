# Rust Borrow Checker Panic with Iterator and Mutable Vector

This example demonstrates a common error in Rust: panicking due to violating the borrow checker rules when using iterators and mutable vectors.

The code attempts to iterate over a vector and add a new element to it simultaneously, causing a runtime panic.

The solution shows how to resolve this by cloning the vector or using different methods that don't conflict with borrow checker rules.