# mp4csv

Exports file information from MP4 and (maybe eventually) AVI files to a CSV for statistical processing. I needed this to process a large number of video files from my trail cameras (to keep track of wildlife coming and going) and didn't want to do it by hand.

The following information is exported:

| Column Name | Description |
| ----------- | ----------- |
| `filename` | The name of the file |
| `file_size` | The size of the file in bytes |
| `creation_time` | The creation date and time of the file as a `DateTime<Local>`|
| `modification_time` | The modification date and time of the file as a `DateTime<Local>`|
| `duration` | The duration of the video in seconds |
| `bitrate_kbps` | The bitrate of the video in kilobits per second |
| `fps` | The frame rate of the video in frames per second |

While this is hardly exhaustive, it is all I needed for my purposes.
If you need more, please feel free to open an issue or a PR - it should be relatively easy to add more fields.

## Usage

```ignore
Usage: mp4csv [OPTIONS] <FILE(S)>...

Arguments:
  <FILE(S)>...  One or more file(s) to process. Wildcards and multiple_occurrences files (e.g. here/*.mp4 there/*.MP4)
                are supported.

Options:
  -q, --quiet                          Don't produce any output except errors while working.
  -p, --print-summary                  Print summary detail for each session processed.
  -i, --input-csv <input-csv>          A CSV file with a list of files to process in the first column. The first row is
                                       assumed to be a header and is ignored.
  -c, --csv-filename [<csv-filename>]  The name of the resulting CSV file containing the video metadata summaries.
                                       Default is `video-data.csv` is none is specified.
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

## Installation

### From Source

```bash
git clone
cd mp4csv
cargo install --path .
```

You can also inspect the `justfile` and potentially run `just release` to build a release version of the binary and place it into `/usr/local/bin`.

You can also download a binary from the Releases page and put somewhere in your path.

## To Do

- [ ] Add the ability to recurse directories. Right now you need to be able to use the `**` wildcard to recurse directories.
  - This is not a huge issue since you can just recurse directories yourself and pass the list of files to `mp4csv` using the `-i` option.
- [ ] Add the ability to parse AVI files. Right now I'm having to convert them to MP4s to get the information I need, and getting `ffmpeg` to retain the creation date is a pain.
- [ ] Add more fields
- [ ] Add the ability to specify the output format (e.g. JSON, YAML, etc.)
- [ ] Add the ability to select which fields to output
