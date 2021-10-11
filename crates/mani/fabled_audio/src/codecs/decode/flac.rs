use crate::{AudioDecodingError, AudioSpecification, FlacReaderOptions, SampleFormat};

#[derive(Default)]
pub struct FlacReader;

impl FlacReader {
    pub fn read_flac<P: AsRef<std::path::Path>>(
        &self,
        flac_path: P,
        option: FlacReaderOptions,
    ) -> Result<AudioSpecification, AudioDecodingError> {
        let file = std::fs::File::open(flac_path)?;

        let buf_reader = std::io::BufReader::new(file);

        let reader = claxon::FlacReader::new_ext(buf_reader, option.into())
            .map_err(AudioDecodingError::FlacError)?;

        let stream_info = reader.streaminfo();

        Ok(AudioSpecification {
            channel_count: stream_info.channels as _,
            sample_rate: stream_info.sample_rate,
            bit_per_sample: stream_info.bits_per_sample as _,
            // FLAC format supports only integer samples, not floating-point
            sample_format: SampleFormat::I16,
            duration: stream_info.samples.unwrap() as u32 / stream_info.sample_rate,
        })
    }
}


#[cfg(test)]
mod flac_decoder_test {
    use crate::{FlacReader, FlacReaderOptions};
    use rodio::Source;

    #[test]
    fn decode_file() {
        let flac_path = [
            env!("CARGO_MANIFEST_DIR"),
            "/src/audio/WolfgangAmadeusMozart-SymphonyNo.40InGMinorK.550-04-AllegroAssai.flac",
        ]
        .join("");

        let flac_reader = FlacReader::default();
        let audio_spec = flac_reader
            .read_flac(flac_path, FlacReaderOptions::READ_VORBIS_COMMENT)
            .unwrap();

        println!("{:?}", audio_spec);
    }

    #[test]
    fn compare_with_rodio() {
        let flac_path: String = [
            env!("CARGO_MANIFEST_DIR"),
            "/src/audio/WolfgangAmadeusMozart-SymphonyNo.40InGMinorK.550-04-AllegroAssai.flac",
        ]
        .join("");


        let file = std::fs::File::open(flac_path.as_str()).unwrap();

        let rodio_decoder = rodio::Decoder::new_flac(file).unwrap();
        let flac_reader = FlacReader::default();
        let audio_spec = flac_reader
            .read_flac(flac_path.as_str(), FlacReaderOptions::READ_VORBIS_COMMENT)
            .unwrap();

        assert_eq!(rodio_decoder.channels(), audio_spec.channel_count);
        assert_eq!(rodio_decoder.sample_rate(), audio_spec.sample_rate);
        assert_eq!(
            rodio_decoder.total_duration().unwrap().as_secs() as u32,
            audio_spec.duration
        );
    }
}
