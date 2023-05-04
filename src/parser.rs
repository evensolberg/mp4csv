//! Reads video files and extracts relevant data

use file_format::FileFormat;
use serde::Serialize;
use std::{error::Error, path::PathBuf};
use time::{Duration, OffsetDateTime};

use mp4::{atom::Atom, mp4time_to_unix_time, Mp4File};

#[derive(Debug, Serialize, Clone)]
pub struct VideoInfo {
    pub filename: String,
    pub size_bytes: u64,
    pub creation_time: OffsetDateTime,
    pub modification_time: OffsetDateTime,
    pub num_frames: u64,
    pub duration: Duration,
    pub fps: f64,
}

impl VideoInfo {
    /// Create a new VideoInfo struct with the given filename
    #[allow(clippy::cast_possible_truncation)]
    pub fn from(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut s = Self {
            filename: filename.to_string(),
            size_bytes: 0,
            creation_time: OffsetDateTime::now_local()?,
            modification_time: OffsetDateTime::now_local()?,
            num_frames: 0,
            duration: Duration::seconds(0),
            fps: 0.0,
        };

        s.size_bytes = mp4_size_bytes(&PathBuf::from(&s.filename))?;

        let mp4 = open_mp4(&PathBuf::from(&s.filename))?;
        log::debug!("mp4 = {:#?}", mp4);

        // Get creation and modification times
        let atoms = mp4.atoms();
        log::debug!("Atoms: {atoms:?}");

        for atom in atoms {
            log::debug!("atom = {atom:?}");
            if let Atom::Moov(data) = atom {
                let header = data.header();
                log::debug!("header = {header:?}");
                let children = data.children();
                log::debug!("Children:");
                for child in children {
                    if let Atom::Mvhd(mvhd) = child {
                        let ct = mp4time_to_unix_time(mvhd.creation_time).unwrap_or(0);
                        s.creation_time = OffsetDateTime::from_unix_timestamp(ct as i64)?;
                        log::debug!("creation_time = {ct}");

                        let mt = mp4time_to_unix_time(mvhd.modification_time).unwrap_or(0);
                        s.modification_time = OffsetDateTime::from_unix_timestamp(mt as i64)?;
                        log::debug!("modification_time = {mt}");

                        let dur = mvhd.duration as f64 / f64::from(mvhd.timescale);
                        log::debug!("duration = {dur:.2} seconds");

                        s.duration = Duration::seconds(dur as i64);
                        s.fps = mvhd.rate;
                    }
                }
            }
        }

        Ok(s)
    }
}

/// Opens the given file and returns a parsed `Mp4File` struct if it is an MP4 file.
fn open_mp4(filename: &PathBuf) -> Result<Mp4File, Box<dyn Error>> {
    let format = FileFormat::from_file(filename)?;
    let mut mp4 = if format.media_type() == "video/mp4" {
        let filename_str = filename.to_str().unwrap_or_default();
        Mp4File::new(filename_str)?
    } else {
        return Err("File does not appear to be an MP4".into());
    };

    mp4.parse();

    Ok(mp4)
}

/// Returns the MP4 file's size in bytes.
fn mp4_size_bytes(filename: &PathBuf) -> Result<u64, Box<dyn Error>> {
    let metadata = std::fs::metadata(filename)?;
    let size_bytes = metadata.len();

    Ok(size_bytes)
}
