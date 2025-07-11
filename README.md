<div align="center">

<img src="https://user-images.githubusercontent.com/56625259/186507604-2d4a4fd9-6471-4469-a3f4-ce4bf5aebc4a.png" width="457" height="259">

# fetchit
## Known Issue: Package count does not work properly. Will only count the first package format listed.


A system fetch tool for Linux, written in ***Rust***.

</div>

`fetchit` is a simple system info tool, written in *Rust*, for Linux based operating systems. It offers a few customization options, which are demonstrated in the
screenshots below,
- You can change the colors for the ***top*** and ***bottom*** part of the ascii art, as well as the color for the ***bounding box***.
- The path to a custom ascii text file can be passed to `fetchit` using the `-f` option.

For a custom ascii text file, it is recommended that the ascii art should be contained in a box of `10x28`, i.e. `28` ***spaces*** wide, and `10` ***lines***
in height. If this condition is not met (especially, the height, which should be greater than or equal to `9`, i.e. the number of lines) then `fetchit` will fall back to the default ascii asrt. See [`Usage`](https://github.com/Ruturajn/fetchit#usage) for more details.

## Examples

<div align="center">

<img src="https://user-images.githubusercontent.com/56625259/186508160-746a2cc7-af3f-4d91-84d3-de3262480198.png" width="323" height="163"> &nbsp; <img src="https://user-images.githubusercontent.com/56625259/186508179-18fb1940-27ad-42d2-9d4c-0a72d887f2ef.png" width="323" height="163">

<br>

<img src="https://user-images.githubusercontent.com/56625259/186509037-3fc1f415-cf1f-4563-97c0-2c05443ecd79.png" width="376" height="170"> &nbsp; <img src="https://user-images.githubusercontent.com/56625259/186509080-72c261b8-3f17-4825-b547-16af09fe6602.png" width="376" height="170">

<br>

</div>

<br>

## Installation

You can install `fetchit` using any of the following methods,

### Arch Linux
`fetchit` is available in the AUR. If you have an AUR helper (for example [paru](https://github.com/Morganamilo/paru)),

```
$ paru -S fetchit-git
```

Or alternatively,
```
# Clone the AUR Package.
$ git clone https://aur.archlinux.org/fetchit-git.git

# Change directory into the repo.
$ cd fetchit-git

# Install it, using `makepkg`.
$ makepkg -si
```

### Install from Releases

Head over to [Releases](https://github.com/Ruturajn/fetchit/releases) to grab a binary for `fetchit`. Once downloaded,
```
# Navigate to the directory where you have downloaded the tar file.
$ tar -xvf fetchit-0.1.1-x86_64.tar.gz

# Copy the executable to `~/.local/bin/`, and if this directory doesn't exist create it.
$ if [[ ! -d ~/.local/bin ]] ; then mkdir -p ~/.local/bin/ ; fi
$ cp ./fetchit ~/.local/bin/
```
Finally, make sure, you add `~/.local/bin/` to `PATH`, if you haven't already.

### Building from Source
```
# First of all install Rust, see "https://www.rust-lang.org/tools/install".
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the git repo.
$ git clone https://github.com/Ruturajn/fetchit.git

# Change directory into the repo.
$ cd ./fetchit

# Build the package.
$ cargo build --release

# Create ~/.cargo/bin/ if it does not exist.
$ if [[ ! -d ~/.cargo/bin ]] ; then mkdir -p ~/.cargo/bin/ ; fi

# Copy the executable to `~/.cargo/bin/`.
$ cp ./target/release/fetchit ~/.cargo/bin/
```

<br>

> Note: Please feel free to open ***Issues*** and ***Pull requests***, if you feel something is out of order, or if you are facing any problems.

<br>

## Usage
```
fetchit 0.1.1
Ruturajn <nanotiruturaj@gmail.com>
A System fetch tool for Linux written in Rust

USAGE:
    fetchit [OPTIONS]

OPTIONS:
    -b, --bottom-color <BOTTOM_COLOR>
            Color for the bottom part of the ascii art : black, red, yellow, blue, magenta, cyan,
            white, green

    -f, --file-path <FILE_PATH>
            File path for the ascii text file

    -h, --help
            Print help information

    -o, --outer-box-color <OUTER_BOX_COLOR>
            Color for the box : black, red, yellow, blue, magenta, cyan, white, green

    -t, --top-color <TOP_COLOR>
            Color for the top part of the ascii art : black, red, yellow, blue, magenta, cyan,
            white, green

    -V, --version
            Print version information
```

## References
- https://github.com/anhsirk0/fetch-master-6000
