# palm-detection-switch
## Description
I like to not have my cursor jump around when I'm trying to write an essay for class, but I also like to play Minecraft. Manually going into the Sway configuration file to swap around the palm detection feature is a pain. So, this script automates that.

## Setup
You must have an input configuration for your touchpad, and you must have the line `dwt enabled` or `dwt disabled`. 

For example: 
```
   input "2362:628:PIXA3854:00_093A:0274_Touchpad" {
       tap enabled
       natural_scroll enabled
       dwt enabled
   }
```

## Installation
### Dependencies 
I believe the only dependencies are:
- Mako (for notifications)
- Rust

### Arch Linux
You can download the PKGBUILD from [this project](https://github.com/jefrecantuledesma/pkgbuilds/tree/main/palm-detection-switch). Then, like every other package, you can do a good old `makepkg -si`.

### Other Distributions
You can compile the binary by downloading this project and issuing `cargo build --release`. Then, you can go ahead and move the binary from `./target/release` into your `/usr/bin` directory, or wherever is appropriate.

## Use
Simply issue `palm-detection-switch` from the terminal, or bind it to a Sway hotkey, like so: 
```
bindsym $mod+Shift+p exec palm-detection-switch
```
