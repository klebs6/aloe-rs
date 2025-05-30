# Aloe Container

Aloe Container is a comprehensive, high-performance Rust crate designed to form the backbone for a variety of data structures, focusing primarily on list-based and set-based collections, as well as efficient memory and thread handling. From managing atomic locks to handling sortable containers, this crate amalgamates modern Rust practices with classic data structure necessities.

## Features

- **SortedSet**: Manages unique primitive elements in an order which supports fast look-up through binary-chop methods.
- **ListenerList**: Simplifies the process of invoking callback functions across a collection of objects.
- **OwnedArray and ReferenceCountedArray**: Provide automatic memory management, deleting elements when they are removed or going out of scope.
- **HashMap**: Offers a robust key-value store using customizable hash functions.
- **AbstractFifo**: Implements a lock-free FIFO manager that supports single-reader, single-writer configurations.

## Usage

### SortedSet
Designed to only allow unique values and maintain them in a sorted order, use SortedSet for operations where fast lookup is paramount.

```rust
let mut set = SortedSet::<i32>::default();
set.add(&42);
assert!(set.contains(&42));
```

### ListenerList
Ideal for managing callback functions across multiple objects whenever a specific event triggers.

```rust
struct MyListener;
impl MyListener {
    fn on_event(&self) {}
}

let mut listeners = ListenerList::<MyListener>::default();
listeners.call(|listener| listener.on_event());
```

### License

This project is licensed under the GPL-3.0 license.

---

*This README.md file was generated by an AI model and may not be 100% accurate, but it should be pretty good.*

This crate is a translation of the JUCE module.

JUCE is a c++ software framework for developing high performance audio applications.

Usage falls under the GPLv3 as well as the JUCE commercial license.

See github.com/juce-framework/JUCE and the JUCE license page for details.

This crate is in the process of being translated from c++ to rust. For progress updates, please see the workspacer rust project. designed specifically for rust projects.
