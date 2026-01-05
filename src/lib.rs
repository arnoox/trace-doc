//! # trace-doc
//!
//! A library for documenting, modeling, and linking requirements, architecture, test specifications,
//! and realization elements directly in Rust's type system.
//!
//! By expressing traceability using structs, traits, and attributes, we allow the Rust compiler itself
//! to act as a unified tool for compiling source code and verifying formal traceability in safety-critical
//! software lifecycles.
//!
//! ## Core Concepts
//!
//! - **Artifact**: Any item in the development lifecycle (requirement, realization, test, etc.).
//! - **Requirement**: A specification or constraint for the system.
//! - **Realization**: An artifact that fulfills a requirement.
//! - **Test**: An artifact that validates a requirement.
//! - **DependsOn**: A relationship between two artifacts.
//! - **Realizes**: A realization fulfilling a requirement.
//! - **Tests**: A test validating a requirement.
//!
//! ## Example
//!
//! ```rust
//! use trace_doc::{Artifact, Requirement, Realization, Realizes, Test, Tests};
//!
//! struct Req;
//! impl Artifact for Req {}
//! impl Requirement for Req { const TITLE: &str = "A requirement"; }
//!
//! struct Real;
//! impl Artifact for Real {}
//! impl Realization for Real {}
//! impl Realizes<Req> for Real {}
//!
//! struct T;
//! impl Artifact for T {}
//! impl Test for T {}
//! impl Tests<Req> for T {}
//! ```

#![deny(missing_docs)]

/// The status of an artifact in the development lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The artifact is in the initial stages of development.
    Draft,
    /// The artifact has been reviewed and is ready for use.
    Accepted,
}

/// Represents any artifact in the development lifecycle (requirement, realization, test, etc.).
pub trait Artifact {
    /// The status of the artifact in the development lifecycle.
    const STATUS: Status = Status::Draft;
}

/// Represents a dependency relationship between any two artifacts.
pub trait DependsOn<A>
where
    A: Artifact,
    Self: Artifact,
{
}

/// Represents a requirement artifact of a system.
/// The description of the requirement can be provided as a doc comment on the implementing type.
pub trait Requirement: Artifact {
    /// A short title for the requirement.
    const TITLE: &str;
}

/// Represents a realization artifact of a system.
/// The description of the Realization can be provided as a doc comment on the realizing type.
pub trait Realization: Artifact {}

/// Represents a test specification artifact of a system.
/// The description of the Test can be provided as a doc comment on the implementing type.
pub trait Test: Artifact {}

/// Represents the traceability of a realization to a requirement.
/// This trait is implemented by the realizations that fulfill specific requirements.
pub trait Realizes<R>
where
    R: Requirement,
    Self: Realization,
{
}

/// Represents the traceability of a test specification to a requirement.
/// This trait is implemented by the test specifications that validate specific requirements.
pub trait Tests<R>
where
    R: Requirement,
    Self: Test,
{
}
