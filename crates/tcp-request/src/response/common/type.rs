use super::*;

/// Boxed trait object for response operations with text and binary output types.
pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = TcpResponseText, OutputBinary = TcpResponseBinary>>;
