# Grim (Working Title)
This repo is intended to be a re-write of [Mackiloha](https://github.com/PikminGuts92/Mackiloha).

# Phase 1
- [x] Underlying I/O support
  - [x] File
    - [x] Read integers, floats, strings, bytes
    - [x] Write integers, floats, strings, bytes
  - [x] Memory
    - [x] Read integers, floats, strings, bytes
    - [x] Write integers, floats, strings, bytes
- [ ] Milo scene support
  - [x] Decompress zlib block structured archives
  - [ ] Compress zlib block structured archives
- [ ] Texture support
  - [x] Decode PS2 bitmaps to RGBa
  - [ ] PNG <-> RGBa conversion
  - [ ] Encode RGBa to PS2 encoded bitmaps
- [x] Command Line Interface
  - [x] milo2dir - Extract entries from milo scene to directory
  - [x] dir2milo - Create milo scene from directory
