# Timeline Kit

A toolkit for interacting with timeline-orientated data from various sources.
The kit contains a variety of procedures and data structures for parsing, manipulating,
and storing data generated by various applications that deals with temporal data.

The contents of this package is geared for developers who work with large volumes of data
that is extracted from video/audio/subtitling applications.

The project is, observably, still in an early development stage.

## TODO

### Project Structure

- [x] Rearrange src/chrono module to only include "Timecode"
- [x] Move format related code to a dedicated module: "Format"

### Testing

- [ ] Unit tests for EDLParser
- [ ] Unit tests for EDLSession
- [ ] Unit tests for Timecode

### Timecode

- [ ] Implement all number traits
- [ ] Implement PartialEq trait
- [ ] Implement Ord trait
- [ ] Handle display of ticks/sub-frames
- [ ] Handle parsing of timecodes with sub-frames
- [ ] Handle drop frame implementation
    - [x] Set drop-frame flags in ctor associated functions
    - [x] String representation
    - [ ] Drop-frame to_ticks logic
    - [x] Setup flags member variable & associated functions
- [ ] Determine what num_traits are necessary, if any at all
- [ ] Complete documentation comments for implemented code
- [ ] Bounds checking for direct data initialization in from_parts()

### Avid Protools Data

- [x] Basic Avid Protools EDLParser for parsing EDL text files exported from Protools with default options
    - [x] Basic architecture
    - [x] Parse session headers section
    - [x] Parse online files section
    - [x] Parse offline files section
    - [x] Parse plugins section
    - [x] Parse online clips section
    - [x] Parse tracks listing section
    - [x] Parse markers listing section

- [x] Avid Protools EDLSession data structure 
    - [x] Map EDL export data to a data structure
    - [x] Child structures for EDLSession parent

- [ ] Handle multi-mono clip events when parsing tracks in EDLParser
- [ ] Determine best way to parse track IO topology

- [ ] Better representation of certain fields/entries in protools EDL in EDLSession
    - [ ] Parse & store file path strings as FilePath structures

- [ ] Do basic error reporting pass for EDLParser
    - [ ] Identify potential locations for slotting in error messages
    - [ ] Settle on error type for EDLParser

___
