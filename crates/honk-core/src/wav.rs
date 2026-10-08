//! WAV output for `honk --wav`.

use std::io::Cursor;
use std::path::Path;

use crate::error::{Error, Result};
use crate::synth::SAMPLE_RATE;

fn spec() -> hound::WavSpec {
    hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}

fn to_i16(s: f32) -> i16 {
    (s.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16
}

fn wav_err(e: hound::Error) -> Error {
    Error::Wav(e.to_string())
}

/// Encode samples as a 16-bit mono WAV in memory.
pub fn encode_wav(samples: &[f32]) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut w = hound::WavWriter::new(&mut cursor, spec()).map_err(wav_err)?;
        for &s in samples {
            w.write_sample(to_i16(s)).map_err(wav_err)?;
        }
        w.finalize().map_err(wav_err)?;
    }
    Ok(cursor.into_inner())
}

/// Write samples to `path` as a 16-bit mono WAV.
pub fn write_wav(path: &Path, samples: &[f32]) -> Result<()> {
    let bytes = encode_wav(samples)?;
    std::fs::write(path, bytes).map_err(|e| Error::Wav(format!("{}: {e}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn round_trips_through_hound() {
        let samples = [0.0, 0.5, -0.5, 1.0, -1.0];
        let bytes = encode_wav(&samples).unwrap();
        let mut reader = hound::WavReader::new(Cursor::new(bytes)).unwrap();
        let spec = reader.spec();
        assert_eq!(
            (spec.channels, spec.sample_rate, spec.bits_per_sample),
            (1, 44_100, 16)
        );
        let back: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
        assert_eq!(back, vec![0, 16384, -16384, 32767, -32767]);
    }

    #[test]
    fn write_wav_creates_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("x.wav");
        write_wav(&path, &[0.1, -0.1]).unwrap();
        assert!(std::fs::metadata(&path).unwrap().len() > 44);
    }

    #[test]
    fn write_wav_to_a_missing_directory_is_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let err = write_wav(&dir.path().join("nope/x.wav"), &[0.0]).unwrap_err();
        assert!(err.to_string().contains("x.wav"), "{err}");
    }
}
