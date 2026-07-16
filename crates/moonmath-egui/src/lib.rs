// moonmath-egui: egui visualization apps.

// egui works entirely in f32 screen coordinates, so bare float literals in these
// scenes intentionally resolve to f32. Newer stable toolchains warn on that via
// `float_literal_f32_fallback`, and CI builds with `-D warnings`; allow it
// crate-wide instead of annotating every coordinate literal.
#![allow(float_literal_f32_fallback)]

pub mod formula_viz;
pub mod algo_viz;
pub mod plotting;
pub mod ifs_3d;
pub mod hnsw;
pub mod gcm;

pub use gcm::GcmApp;
pub use hnsw::HnswApp;
pub use ifs_3d::IfsApp;
