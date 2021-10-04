use crate::{RawAmbisonicClip, RawClip};
use rodio::Source;
use std::time::Duration;

pub type Standard<D> = RawClip<AudioClip<D>>;
pub type Ambisonic<D> = RawAmbisonicClip<AudioClip<D>>;

// A Rust object that represents a sound should implement the `Source` trait.

// We don't know if the data is f32, i16, or u16 and want to support current
// data types and future types deriving from rodio::Sample. Not just restrict it
// to one type.
#[derive(Debug)]
pub struct AudioClip<D> {
    pub data: parking_lot::Mutex<std::vec::IntoIter<D>>,
    pub channel: u16,
    pub sample: u32,
    pub duration: Option<std::time::Duration>,
    pub current_frame_len: Option<usize>,
}

impl<D> Default for AudioClip<D> {
    fn default() -> Self {
        Self {
            data: parking_lot::Mutex::new(vec![].into_iter()),
            channel: 0,
            sample: 0,
            duration: None,
            current_frame_len: None,
        }
    }
}

impl<D> AudioClip<D>
where
    D: rodio::Sample,
{
    pub fn from_file(buffer: Vec<u8>, play_on_awake: bool) -> Self {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer));

        match decoder {
            Ok(source) => {
                let source = source
                    .pausable(!play_on_awake)
                    .stoppable()
                    .convert_samples::<D>();

                Self {
                    channel: source.channels(),
                    sample: source.sample_rate(),
                    duration: source.total_duration(),
                    current_frame_len: source.current_frame_len(),
                    data: parking_lot::Mutex::new(source.collect::<Vec<_>>().into_iter()),
                }
            }
            Err(err) => {
                println!("Error from decoding audio file.\nMessage : {:?}", err);
                AudioClip::default()
            }
        }
    }

    pub fn create_clip(
        data: Vec<D>,
        channel: u16,
        sample: u32,
        duration: Option<std::time::Duration>,
        play_on_awake: bool,
    ) -> AudioClip<D> {
        let current = Self {
            data: parking_lot::Mutex::new(data.into_iter()),
            channel,
            sample,
            duration,
            current_frame_len: None,
        }
        .pausable(!play_on_awake)
        .stoppable();


        Self {
            channel: current.channels(),
            sample: current.sample_rate(),
            duration: current.total_duration(),
            current_frame_len: current.current_frame_len(),
            data: parking_lot::Mutex::new(
                current.convert_samples().collect::<Vec<_>>().into_iter(),
            ),
        }
    }
}

// Standard clip
impl<D> From<AudioClip<D>> for Standard<D>
where
    D: rodio::Sample,
{
    fn from(clip: AudioClip<D>) -> Self {
        RawClip::new(clip)
    }
}

// Ambisonic clip
impl<D> From<AudioClip<D>> for Ambisonic<D>
where
    D: ambisonic::rodio::Sample,
{
    fn from(clip: AudioClip<D>) -> Self {
        RawAmbisonicClip::new(clip)
    }
}

impl<D> Iterator for AudioClip<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // if data is getting changed block the next audio till change completed.
        // rather then returning a zero and playing no audio we block and then play the
        // newly changed value/s when the lock is dropped.
        let mut lock = self.data.lock();
        lock.next()
    }
}


impl<D> rodio::Source for AudioClip<D>
where
    D: rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.current_frame_len
    }

    fn channels(&self) -> u16 {
        self.channel
    }

    fn sample_rate(&self) -> u32 {
        self.sample
    }

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}


impl<D> ambisonic::rodio::Source for AudioClip<D>
where
    D: ambisonic::rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.current_frame_len
    }

    fn channels(&self) -> u16 {
        self.channel
    }

    fn sample_rate(&self) -> u32 {
        self.sample
    }

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}


#[cfg(test)]
mod audio_clip_test {
    use crate::{Ambisonic, AudioClip, Standard};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn default_clip_test() {
        let mut empty_clip: AudioClip<u16> = AudioClip::default();

        assert!(empty_clip.sample.eq(&0));
        assert!(rodio::Source::sample_rate(&empty_clip).eq(&0));
        assert!(ambisonic::rodio::Source::sample_rate(&empty_clip).eq(&0));

        assert!(empty_clip.duration.is_none());
        assert!(rodio::Source::total_duration(&empty_clip).is_none());
        assert!(ambisonic::rodio::Source::total_duration(&empty_clip).is_none());

        assert!(empty_clip.channel.eq(&0));
        assert!(rodio::Source::channels(&empty_clip).eq(&0));
        assert!(ambisonic::rodio::Source::channels(&empty_clip).eq(&0));

        assert!(empty_clip.current_frame_len.is_none());
        assert!(rodio::Source::current_frame_len(&empty_clip).is_none());
        assert!(ambisonic::rodio::Source::current_frame_len(&empty_clip).is_none());

        assert!(empty_clip.next().is_none());

        let lock = empty_clip.data.lock();
        assert!(lock.len().eq(&0));
    }

    #[test]
    fn create_clip_test() {
        let buffer = vec![5.0; 10];
        let channel = 2;
        let sample_rate = 4800;
        let duration = None;

        let mut custom_clip =
            AudioClip::create_clip(buffer.clone(), channel, sample_rate, duration, true);

        assert!(custom_clip.channels().eq(&channel));
        assert!(custom_clip.sample_rate().eq(&sample_rate));
        assert!(custom_clip.total_duration().is_none());
        assert!(custom_clip.current_frame_len().is_none());

        for _ in 0..buffer.len() {
            let elem = custom_clip.next();
            assert!(elem.eq(&Some(5.0)));
        }

        assert!(custom_clip.next().is_none());
    }


    #[test]
    fn load_clip_from_file() {
        let audio_buffer = retrieve_audio_buffer();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(audio_buffer, true);

        assert!(audio_clip.channels().gt(&0));
        assert!(audio_clip.sample_rate().gt(&0));
        assert!(audio_clip.total_duration().is_none()); // unknown duration
        assert!(audio_clip.current_frame_len().is_some());
    }

    fn retrieve_audio_buffer() -> Vec<u8> {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let mut file = std::fs::File::open(path).unwrap();

        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        audio_buffer
    }

    #[test]
    fn convert_clip_to_standard_test() {
        let audio_clip: AudioClip<f32> = AudioClip::default();

        let _audio = Standard::new(audio_clip);
    }

    #[test]
    fn convert_clip_to_ambisonic_test() {
        let audio_clip: AudioClip<f32> = AudioClip::default();

        let _ambisonic_audio = Ambisonic::new(audio_clip);
    }
}
