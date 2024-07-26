//! Contains functionality to read tsplib95 problem and write its solution.

mod reader;
pub use self::reader::TsplibProblem;

mod writer;
pub use self::writer::TsplibSolution;
