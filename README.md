# Rust Raw Pointer and Vector Modification Bug
This repository demonstrates a potential issue related to using raw pointers in Rust when modifying a vector. The core problem is that after obtaining a raw pointer to a vector element, modifying the vector itself (e.g., adding, removing, or resizing) can invalidate the pointer, leading to undefined behavior if the pointer is still dereferenced.

## Bug Description:
The `bug.rs` file contains a program that retrieves a raw pointer to an element within a vector and then attempts to modify the vector in a way that invalidates the pointer, resulting in unexpected output. Modifying the vector's contents through its original reference, while holding a raw pointer to one of its elements, can lead to memory safety violations.

## Solution:
The `bugSolution.rs` file offers a corrected version that avoids raw pointers and focuses on safe, idiomatic Rust methods to modify vector elements. This illustrates better practices for handling data structures to prevent common memory safety problems.
