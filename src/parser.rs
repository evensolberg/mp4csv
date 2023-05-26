//! Reads video files and extracts relevant data
use crate::utils::{duration_seconds, mp4_time_to_datetime_local};

use chrono::{DateTime, Duration, Local};
use file_format::FileFormat;
use mp4::TrackType;
use serde::Serialize;
use serde_with::serde_as;
use std::{error::Error, fs::File, io::BufReader};

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub struct VideoInfo {
    pub filename: String,
    pub size_bytes: u64,
    pub creation_time: DateTime<Local>,
    pub modification_time: DateTime<Local>,
    #[serde_as(as = "serde_with::DurationSeconds<f64>")]
    pub duration: Duration,
    pub bitrate_kbps: f64,
    pub fps: f64,
}

impl Default for VideoInfo {
    fn default() -> Self {
        Self {
            filename: String::new(),
            size_bytes: 0,
            creation_time: Local::now(),
            modification_time: Local::now(),
            duration: Duration::zero(),
            bitrate_kbps: 0.0,
            fps: 0.0,
        }
    }
}

impl VideoInfo {
    /// Create a new VideoInfo struct with the given filename
    #[allow(clippy::cast_possible_truncation)]
    pub fn from(filename: &str) -> std::result::Result<Self, Box<dyn Error>> {
        let mut s = Self::default();

        s.filename = filename.to_string();

        let format = FileFormat::from_file(filename)?;
        if format.media_type() != "video/mp4" {
            return Err(format!("Unsupported media type: {}", format.media_type()).into());
        }

        let f = File::open(filename)?;
        let size = f.metadata()?.len();
        let reader = BufReader::new(f);

        let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

        s.size_bytes = mp4.size();
        s.creation_time = mp4_time_to_datetime_local(mp4.moov.mvhd.creation_time);
        s.modification_time = mp4_time_to_datetime_local(mp4.moov.mvhd.modification_time);

        let dur = mp4.moov.mvhd.duration;
        let ts = mp4.moov.mvhd.timescale;
        s.duration = duration_seconds(dur as f64, ts as f64);

        for track in mp4.tracks().values() {
            if track.track_type().unwrap() == TrackType::Video {
                if track.trak.mdia.minf.stbl.stsd.avc1.is_some() {
                    s.bitrate_kbps = (track.bitrate() / 1000) as f64;
                    s.fps = track.frame_rate();
                }
            }
        }

        Ok(s)
    }
}
