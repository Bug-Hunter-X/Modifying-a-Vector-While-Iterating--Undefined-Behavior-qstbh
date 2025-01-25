# Modifying a Vector While Iterating in Rust

This repository demonstrates a common error in Rust: modifying a vector while iterating over it using a mutable iterator. This can lead to undefined behavior and unexpected results, as the iterator's internal state may become invalid.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file shows how to avoid this issue.

**Key takeaway:** Always ensure data structures are not modified during iteration unless specifically designed to handle such modification (like using `drain_filter` or similar methods).