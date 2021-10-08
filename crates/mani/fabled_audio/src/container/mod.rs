pub use audio_listener::*;
pub use fade_filter::*;
pub use sample_format::*;
pub use wav_spec::*;

mod audio_listener;
mod fade_filter;
mod sample_format;
mod wav_spec;


#[cfg(test)]
mod data_test {
    use crate::{AudioListener, FadeFilter, SampleFormat, WavSpecification};

    #[test]
    fn data_size() {
        let audio_listener_size = std::mem::size_of::<AudioListener>();
        assert_eq!(audio_listener_size & (audio_listener_size - 1), 0);

        let fade_filter_size = std::mem::size_of::<FadeFilter>();
        assert_eq!(fade_filter_size & (fade_filter_size - 1), 0);

        let sample_format_size = std::mem::size_of::<SampleFormat>();
        assert_eq!(sample_format_size & (sample_format_size - 1), 0);

        let wav_spec_size = std::mem::size_of::<WavSpecification>();
        assert_eq!(wav_spec_size & (wav_spec_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let audio_listener_alignment = std::mem::align_of::<AudioListener>();
        assert_eq!(audio_listener_alignment & (audio_listener_alignment - 1), 0);

        let fade_filter_alignment = std::mem::align_of::<FadeFilter>();
        assert_eq!(fade_filter_alignment & (fade_filter_alignment - 1), 0);

        let sample_format_alignment = std::mem::align_of::<SampleFormat>();
        assert_eq!(sample_format_alignment & (sample_format_alignment - 1), 0);

        let wav_spec_alignment = std::mem::align_of::<WavSpecification>();
        assert_eq!(wav_spec_alignment & (wav_spec_alignment - 1), 0);
    }
}
