// Connects to the target database and introspects the real schema.
// Produces Vec<DatabaseSchema> consumed by the diff engine.

pub mod oracle;
pub mod postgres;
pub mod probe;
