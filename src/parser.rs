//! Reads video files and extracts relevant data
use crate::utils::{duration_seconds, mp4_time_to_datetime_local};

use chrono::{DateTime, Duration, Local};
use file_format::FileFormat;
use mp4::TrackType;
use serde::Serialize;
use serde_with::serde_as;
use std::{error::Error, fs::File, io::BufReader};

/// A struct that holds information about a video file that has been read
#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub struct VideoInfo {
    /// The name of the file
    pub filename: String,

    /// The size of the file in bytes
    pub size_bytes: u64,

    /// The creation time of the file as a `DateTime<Local>`
    pub creation_time: DateTime<Local>,

    /// The modification time of the file as a `DateTime<Local>`
    pub modification_time: DateTime<Local>,

    /// The duration of the video as a `Duration` (typically in seconds)
    #[serde_as(as = "serde_with::DurationSeconds<f64>")]
    pub duration: Duration,

    /// The bitrate of the video in kilobits per second
    pub bitrate_kbps: f64,

    /// The frame rate of the video in frames per second
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
    /// Create a new `VideoInfo` struct with the given filename
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice that holds the name of the file to read
    ///
    /// # Returns
    ///
    /// * `Result<VideoInfo, Box<dyn Error>>` - A `VideoInfo` struct, or an error
    ///
    /// # errors
    ///
    /// * `Box<dyn Error>` - An error if the file cannot be read
    /// * `Box<dyn Error>` - An error if the file is not a video file
    /// * `Box<dyn Error>` - An error if the file is not a supported video file
    ///
    /// # Example
    ///
    /// ```ignore
    /// let video_info = VideoInfo::from("video.mp4");
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub fn from(filename: &str) -> std::result::Result<Self, Box<dyn Error>> {
        let mut vi = Self::default();

        vi.filename = filename.to_string();

        let format = FileFormat::from_file(filename)?;
        if format.media_type() != "video/mp4" {
            return Err(format!("Unsupported media type: {}", format.media_type()).into());
        }

        let f = File::open(filename)?;
        let size = f.metadata()?.len();
        let reader = BufReader::new(f);

        let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

        vi.size_bytes = mp4.size();
        vi.creation_time = mp4_time_to_datetime_local(mp4.moov.mvhd.creation_time);
        vi.modification_time = mp4_time_to_datetime_local(mp4.moov.mvhd.modification_time);

        let dur = mp4.moov.mvhd.duration;
        let ts = mp4.moov.mvhd.timescale;
        vi.duration = duration_seconds(dur as f64, f64::from(ts));

        for track in mp4.tracks().values() {
            if track.track_type()? == TrackType::Video
                && track.trak.mdia.minf.stbl.stsd.avc1.is_some()
            {
                vi.bitrate_kbps = f64::from(track.bitrate() / 1000);
                vi.fps = track.frame_rate();
            }
        }

        Ok(vi)
    }
}
