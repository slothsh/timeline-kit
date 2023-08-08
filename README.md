# Timeline Kit

A toolkit for interacting with timeline-orientated data from various sources.
The kit contains a variety of procedures and data structures for parsing, manipulating,
and storing data generated by various applications that deals with temporal data.

The contents of this package is geared for developers who work with large volumes of data
that is extracted from video/audio/subtitling applications.

The project is, observably, still in an early development stage.

## TODO

### Project Structure

- [ ] Rearrange src/chrono module to only include "Timecode"
- [ ] Move format related code to a dedicated module: "Format"

### Testing

- [ ] Unit tests for EDLParser
- [ ] Unit tests for EDLSession
- [ ] Unit tests for Timecode

### Timecode

- [ ] Implement all number traits
- [ ] Handle drop frame implementation
    - [ ] String representation
    - [ ] Drop-frame to_ticks logic
    - [ ] Setup flags member variable & associated functions

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

- [ ] Better representation of certain fields/entries in protools EDL in EDLSession
    - [ ] Parse & store file path strings as FilePath structures

- [ ] Do basic error reporting pass for EDLParser
    - [ ] Identify potential locations for slotting in error messages
    - [ ] Settle on error type for EDLParser

___
