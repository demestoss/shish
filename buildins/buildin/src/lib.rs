/// This module contains the trait for the buildin command.
/// The trait is used to define the command execution behavior as a cli command.
pub trait Invoke {
    fn invoke(&self) -> anyhow::Result<i32>;
}

/// The trait is used to return the output of the command inside the code.
pub trait Retrieve {
    /// The output type of the command.
    type Output;

    fn retrieve(&self) -> anyhow::Result<Self::Output>;
}
