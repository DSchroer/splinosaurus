#![deny(missing_docs)]

//! A highly flexible spline and surface library.
//!
//! Features include:
//! - BSpline
//! - NURBS (spline)
//! - BSurface
//! - NURBS (surface)
//!
//! Planned:
//! - TSpline
//!
//! The goals of this library are to:
//! - Hide unnecessary complexity
//! - Expose both high and low level manipulation APIs
//! - Allow for type flexibility (fixed or floating point)
//! - Minimal dependencies

/// Defining and manipulating control points.
pub mod control_points;
/// Exporting to other formats such as meshes.
pub mod export;
/// Defining and manipulating knots.
pub mod knots;
/// N dimensional splines.
pub mod splines;
/// N dimensional surfaces.
pub mod surfaces;

mod algorithms;
mod grid;
mod step_iter;
mod types;
