//! Reads video files and extracts relevant data

use file_format::FileFormat;
use serde::Serialize;
use std::{error::Error, path::PathBuf};
use time::{Duration, OffsetDateTime};

use mp4::{Atom, Mp4File};

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

        let mp4 = open_mp4(&PathBuf::from(&s.filename))?;
        log::debug!("mp4 = {:#?}", mp4);

        s.size_bytes = mp4.file_size();

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
                        let ct = mvhd.creation_time_utc();
                        log::debug!("Mvhd::creation_time = {ct}");

                        let mt = mvhd.modification_time_utc();
                        log::debug!("Mvhd::modification_time = {mt}");

                        let timescale = mvhd.timescale();
                        log::debug!("Mvhd::timescale = {timescale}");

                        let dur = mvhd.duration_seconds();
                        log::debug!("Mvhd::duration = {dur:?} seconds");

                        let rate = mvhd.rate();
                        log::debug!("Mvhd::rate = {rate}");

                        let volume = mvhd.volume();
                        log::debug!("Mvhd::volume = {volume}");

                        let matrix = mvhd.matrix();
                        log::debug!("Mvhd::matrix = {matrix:?}");

                        let next_track_id = mvhd.next_track_id();
                        log::debug!("Mvhd::next_track_id = {next_track_id}");
                    }
                }
            }

            if let Atom::Stts(data) = atom {
                let header = &data.header;
                log::debug!("Stts::header = {header:?}");
                let children = data.entries();
                log::debug!("Stts::Children:");
                for child in children {
                    s.num_frames += child.sample_count as u64;
                }
            }

            if let Atom::Cslg(data) = atom {
                let header = &data.header;
                log::debug!("Cslg::header = {header:?}");
                log::debug!(
                    "Cslg::composition_to_dtsshift = {}",
                    data.composition_to_dtsshift()
                );
                log::debug!(
                    "Cslg::least_decode_to_display_delta = {}",
                    data.least_decode_to_display_delta()
                );
                log::debug!(
                    "Cslg::greatest_decode_to_display_delta = {}",
                    data.greatest_decode_to_display_delta()
                );
                log::debug!(
                    "Cslg::composition_start_time = {}",
                    data.composition_start_time()
                );
                log::debug!(
                    "Cslg::composition_end_time = {}",
                    data.composition_end_time()
                );
            }

            if let Atom::Tkhd(data) = atom {
                log::debug!("Tkhd::header = {:?}", data.header());
                log::debug!("Tkhd::creation_time = {}", data.creation_time());
                log::debug!("Tkhd::modification_time = {}", data.modification_time());
                log::debug!("Tkhd::track_id = {}", data.track_id());
                log::debug!("Tkhd::duration = {}", data.duration());
                log::debug!("Tkhd::layer = {}", data.layer());
                log::debug!("Tkhd::alternate_group = {}", data.alternate_group());
                log::debug!("Tkhd::volume = {}", data.volume());
                log::debug!("Tkhd::matrix = {:?}", data.matrix());
                log::debug!("Tkhd::width = {}", data.width());
                log::debug!("Tkhd::height = {}", data.height());
            }
        }

        Ok(s)
    }
}

/// Opens the given file and returns a parsed `Mp4File` struct if it is an MP4 file.
fn open_mp4(filename: &PathBuf) -> Result<Mp4File, Box<dyn Error>> {
    let format = FileFormat::from_file(filename)?;
    let mp4 = if format.media_type() == "video/mp4" {
        let filename_str = filename.to_str().unwrap_or_default();
        mp4::parse_file(filename_str)?
    } else {
        return Err("File does not appear to be an MP4".into());
    };

    Ok(mp4)
}
