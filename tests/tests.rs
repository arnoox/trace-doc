//! This example demonstrates how to use the `trace_doc` library to define a generic specification
//! with traceability between Requirements, Realizations, and Tests artifacts.

use trace_doc::*;

/// Collection of requirement artifacts.
///
/// The doc string is the requirement formulation.
pub mod requirements {
    use super::*;

    /// Req1 formulation.
    pub struct Req1;
    impl Artifact for Req1 {
        const STATUS: Status = Status::Accepted;
    }
    impl Requirement for Req1 {
        const TITLE: &str = "Req1 Title";
    }

    /// Req2 formulation.
    pub struct Req2;
    impl Artifact for Req2 {
        const STATUS: Status = Status::Accepted;
    }
    impl Requirement for Req2 {
        const TITLE: &str = "Req2 Title";
    }

    /// Req3 formulation.
    pub struct Req3;
    impl Artifact for Req3 {
        const STATUS: Status = Status::Accepted;
    }
    impl Requirement for Req3 {
        const TITLE: &str = "Req3 Title";
    }

    /// Req4 formulation.
    pub struct Req4;
    impl Artifact for Req4 {
        const STATUS: Status = Status::Accepted;
    }
    impl Requirement for Req4 {
        const TITLE: &str = "Req4 Title";
    }
}

/// Collection of test artifacts.
///
/// In the future, we can figure out how to link the actual test realization and result to the Test artifacts.
pub mod tests {
    use super::*;

    /// Test1 description.
    pub struct Test1;
    impl Artifact for Test1 {
        const STATUS: Status = Status::Accepted;
    }
    impl Test for Test1 {}
    impl Tests<requirements::Req1> for Test1 {}

    /// Test2 description.
    pub struct Test2;
    impl Artifact for Test2 {
        const STATUS: Status = Status::Accepted;
    }
    impl Test for Test2 {}
    impl Tests<requirements::Req2> for Test2 {}

    /// Test3 description.
    pub struct Test3;
    impl Artifact for Test3 {
        const STATUS: Status = Status::Accepted;
    }
    impl Test for Test3 {}
    impl Tests<requirements::Req3> for Test3 {}

    /// Test4 description.
    pub struct Test4;
    impl Artifact for Test4 {
        const STATUS: Status = Status::Accepted;
    }
    impl Test for Test4 {}
    impl Tests<requirements::Req4> for Test4 {}
}

/// Collection of realization artifacts.
///
/// Each realization can be placed in its own module or file to organize the code base better.
/// They can also directly implement real functionality in Rust code, if needed. Here, they are just empty structs
/// to demonstrate the traceability features of the `trace_doc` library.
pub mod realizations {
    use super::*;

    /// Realization1 description.
    pub struct Realization1;
    impl Artifact for Realization1 {
        const STATUS: Status = Status::Accepted;
    }
    impl Realization for Realization1 {}
    impl Realizes<requirements::Req1> for Realization1 {}

    /// Realization2 description.
    pub struct Realization2;
    impl Artifact for Realization2 {
        const STATUS: Status = Status::Accepted;
    }
    impl Realization for Realization2 {}
    impl Realizes<requirements::Req2> for Realization2 {}
    impl DependsOn<Realization1> for Realization2 {}

    /// Realization3 description.
    pub struct Realization3;
    impl Artifact for Realization3 {
        const STATUS: Status = Status::Accepted;
    }
    impl Realization for Realization3 {}
    impl Realizes<requirements::Req3> for Realization3 {}
    impl DependsOn<Realization1> for Realization3 {}

    /// Realization4 description.
    pub struct Realization4;
    impl Artifact for Realization4 {
        const STATUS: Status = Status::Accepted;
    }
    impl Realization for Realization4 {}
    impl Realizes<requirements::Req4> for Realization4 {}
    impl DependsOn<Realization2> for Realization4 {}
    impl DependsOn<Realization3> for Realization4 {}
}
