//! Various implementations of tree data structures.

/// Interior Mutability Tree. This tree implementation is both mutable and fully navigable.
pub mod intmuttree;

/// Interior Mutability Tuple-Struct with Enum.
/// This tree implementation is an evolution of intmuttree that represents each type of node as variants in an enum, wrapped in a tuple struct.
pub mod smite;
pub(crate) mod nullo;
