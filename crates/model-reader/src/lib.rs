// Reads JPA entity models from Java source files and compiled bytecode.
// Produces Vec<EntityModel> consumed by the diff engine.

pub mod bytecode;
pub mod merge;
pub mod source;