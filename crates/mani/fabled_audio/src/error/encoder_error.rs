use thiserror::*;

#[derive(Debug, Error)]
pub enum AudioEncodingError {
    #[error("No Device or Lost Device mid way while acquiring it before.")]
    NoDeviceError,
    #[error("No Input Config for the Device which is caused by either No Device being available or Target Device is actually a Output Device")]
    NoInputConfigError,
    #[error("No Output Config for the Device which is caused by either No Device being available or Target Device is actually a Input Device")]
    NoOutputConfigError,
    #[error("Error occurred when building either a Input Stream or a Output Stream\n Message : {:?}", .0)]
    BuildStreamError(cpal::BuildStreamError),
}
