# sdvconvertergui

A gui for popular Stardew valley converters

## Supported converters

- TMXL2CP by [AnotherPillow](https://github.com/AnotherPillow/TMXL2CP)
- CP2AT by [holy-the-sea](https://github.com/holy-the-sea/CP2AT)
- FurnitureConverter (CF2DGA) by [elizabethcd](https://github.com/elizabethcd/FurnitureConverter)

## How to use

1. Download the latest release from the [releases page](https://github.com/anotherpillow/sdvconvertergui/releases).
2. Install the dependencies (see below).
3. Run the program.

## Dependencies

The GUI will ask you to install Python 3.8-3.10, while this is not technically required, it is needed for all the current converters. If you already have Python installed, you can skip this step.

## Known Issues/Constraints

- All JSON files must not be json5 (no trailing commas, no comments)

## Are the converters the latest version?

Yes, the GUI downloads the latest version of the converter from github.

## Compiling

1. Clone the repo
2. Make your changes
3. Compile the program with `cargo build --release`

## Credits

Most of these converters were not made by me (except for [TMXL2CP](https://github.com/anotherpillow/TMXL2CP), which I made), I do not distribute their source code, and I do not claim to own them. I am simply providing a gui for them. I have of course gotten permission to made this.
