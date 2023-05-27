# mp4csv

Exports file information from MP4 and (eventually) AVI files to a CSV for statistical processing

## Usage

```ignore
Usage: mp4csv [OPTIONS] <FILE(S)>...

Arguments:
  <FILE(S)>...  One or more file(s) to process. Wildcards and multiple_occurrences files (e.g. here/*.mp4 there/*.MP4)
                are supported.

Options:
  -q, --quiet                          Don't produce any output except errors while working.
  -p, --print-summary                  Print summary detail for each session processed.
  -c, --csv-filename [<csv-filename>]  The name of the resulting CSV file containing the video metadata summaries. Default is `video-data.csv` is none is specified.
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```
