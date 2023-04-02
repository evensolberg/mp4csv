//! Reads video files and extracts relevant data

use file_format::FileFormat;
use serde::Serialize;
use std::{error::Error, fs::File, path::PathBuf};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

use mp4::atom::Atom;
use mp4::Mp4File;

#[derive(Debug, Serialize, Clone)]
struct VideoInfo {
    pub filename: String,
    pub size_bytes: u64,
    pub creation_time: PrimitiveDateTime,
    pub num_frames: u64,
    pub duration: Duration,
    // pub created:
}

/// Reads a file and processes the metadata into a hash table
pub fn parse(filename: &PathBuf, print_detail: bool) -> Result<(), Box<dyn Error>> {
    let f = File::open(filename)?;
    log::debug!("{} metadata: {:?}", filename.display(), f.metadata());
    let format = FileFormat::from_file(filename)?;
    log::debug!("{} file format: {}", filename.display(), format);
    log::debug!(
        "{} details: {:?} - {:?} - {:?} - {:?}",
        filename.display(),
        format.kind(),
        format.media_type(),
        format.name(),
        format.short_name()
    );

    if print_detail {
        println!("Printing all the deets here.");
    }

    if format.media_type() == "video/mp4" {
        log::debug!("Opening MP4.");
        let filename_str = filename.to_str().unwrap_or_default();
        let mut mp4 = Mp4File::new(filename_str).unwrap();
        mp4.parse();

        for atom in mp4.atoms() {
            if let Atom::Moov(data) = atom {
                let header = data.header();
                log::debug!("header = {header:?}");
                let children = data.children();
                log::debug!("Children:");
                for child in children {
                    // log::debug!("child = {child:?}");
                    if let Atom::Mvhd(mvhd) = child {
                        log::debug!("child header = {:?}", &mvhd.header);
                        let ctime = mvhd.creation_time;
                        log::debug!("creation time: {ctime}");
                        let utime = mp4::mp4time_to_unix_time(ctime) as i64;
                        log::debug!("creation time (unix): {utime}");
                        let realtime = OffsetDateTime::from_unix_timestamp(utime)?;
                        log::debug!("Creation time: {realtime}");
                    }
                }
            }
        }
    }
    // return safely
    Ok(())
}
