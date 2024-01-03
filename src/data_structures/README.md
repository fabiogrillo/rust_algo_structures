# Custom Data Structures in Rust

This directory contains a collection of basic data structures implemented in Rust. Each data structure is contained in its own `.rs` file. The data structures included are:

- B-Tree Map (`b_tree_map.rs`)
- Linked List (`linked_list.rs`)
- Queue (`queue.rs`)
- Vector (`vec.rs`)

Each data structure comes with a set of unit tests to ensure correct functionality. These tests are contained within each `.rs` file under a `#[cfg(test)]` module.

## Running Tests

To run the tests for a specific data structure, use the following command:

```shell
cargo test --test [name of the data structure]
```

Replace [name of the data structure] with the name of the data structure you want to test (without the .rs extension). For example, to run the tests for the B-Tree Map, you would use:

```shell
cargo test --test b_tree_map
```
Thank you for exploring these custom data structures.
