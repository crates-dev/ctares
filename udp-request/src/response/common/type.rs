use super::*;

/// A type alias for a boxed `ResponseTrait` object.
pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = UdpResponseText, OutputBinary = UdpResponseBinary>>;
