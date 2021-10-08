mod audio_clip;
mod raw_clip;


pub use audio_clip::*;
pub use raw_clip::*;


#[cfg(test)]
mod data_test {
    use crate::{AudioClip, RawAmbisonicClip, RawClip};

    #[test]
    fn data_size() {
        let audio_clip_u16_size = std::mem::size_of::<AudioClip<u16>>();
        println!("audio clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<AudioClip<i16>>();
        println!("audio clip i16 {}", audio_clip_i16_size);

        let audio_clip_f32_size = std::mem::size_of::<AudioClip<f32>>();
        println!("audio clip f32 {}", audio_clip_f32_size);

        //------------------------------------------------------------------------

        let audio_clip_u16_size = std::mem::size_of::<RawClip<AudioClip<u16>>>();
        println!("raw clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<RawClip<AudioClip<i16>>>();
        println!("raw clip i16 {}", audio_clip_i16_size);

        let raw_clip_f32_size = std::mem::size_of::<RawClip<AudioClip<f32>>>();
        println!("raw clip f32 {}", raw_clip_f32_size);

        //------------------------------------------------------------------------

        let raw_ambisonic_clip_f32_size = std::mem::size_of::<RawAmbisonicClip<AudioClip<f32>>>();
        println!("raw ambisonic clip f32 {}", raw_ambisonic_clip_f32_size);

        //------------------------------------------------------------------------
    }


    #[test]
    fn data_alignment() {
        let audio_clip_u16_alignment = std::mem::align_of::<AudioClip<u16>>();
        assert_eq!(audio_clip_u16_alignment & (audio_clip_u16_alignment - 1), 0);

        let audio_clip_i16_alignment = std::mem::align_of::<AudioClip<i16>>();
        assert_eq!(audio_clip_i16_alignment & (audio_clip_i16_alignment - 1), 0);

        let audio_clip_f32_alignment = std::mem::align_of::<AudioClip<f32>>();
        assert_eq!(audio_clip_f32_alignment & (audio_clip_f32_alignment - 1), 0);

        //------------------------------------------------------------------------
        let raw_clip_u16_alignment = std::mem::align_of::<RawClip<AudioClip<u16>>>();
        assert_eq!(raw_clip_u16_alignment & (raw_clip_u16_alignment - 1), 0);

        let raw_clip_i16_alignment = std::mem::align_of::<RawClip<AudioClip<i16>>>();
        assert_eq!(raw_clip_i16_alignment & (raw_clip_i16_alignment - 1), 0);

        let raw_clip_f32_alignment = std::mem::align_of::<RawClip<AudioClip<f32>>>();
        assert_eq!(raw_clip_f32_alignment & (raw_clip_f32_alignment - 1), 0);

        //------------------------------------------------------------------------

        let raw_ambisonic_clip_f32_alignment =
            std::mem::align_of::<RawAmbisonicClip<AudioClip<f32>>>();
        assert_eq!(
            raw_ambisonic_clip_f32_alignment & (raw_ambisonic_clip_f32_alignment - 1),
            0
        );

        //------------------------------------------------------------------------
    }
}
