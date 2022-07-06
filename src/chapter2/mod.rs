// exercises of chapter2
#[cfg(test)]
mod exercises;
mod f8;
mod inplace_swap;
mod rbg;
mod reverse_array;
mod show_bytes;

// re-export
pub use inplace_swap::*;
pub use rbg::*;
pub use reverse_array::*;
pub use show_bytes::*;
