# hey!
`hey` is a command line tool to contact DuckDuckGo Chat API from your terminal.
based on [this article](https://blek.codes/blog/duckduckgo-ai-chat/)

demo:

<p align=center><img src='hey-demo.gif' alt='a gif demostrating a prompt about a bedtime story' width=1000></p>

# disclaimer
to clarify, as of may 17 2024, using a third party client [does not violate the ToS](https://duckduckgo.com/aichat/privacy-terms).

by using this client, you acknowledge that you will be liable for any ToS violations as per GPLv3

this project is not intended for API scraping purposes, and actually [has a soft protection against it](https://git.blek.codes/blek/hey/src/branch/main/src/main.rs#L34).

# installation
if you run linux or macos,
```sh
git clone https://git.blek.codes/blek/hey.git
cd hey
cargo b -r
sudo cp target/release/hey /usr/bin/hey,
```

if you are on windows, [download the binary file](#download-the-binary-file) or compile it yourself if you have the knowledge

## via a package manager

arch (AUR) - `paru -S hey-duck`

## download the binary file
prebuilt binaries are available on [the releases page](https://git.blek.codes/blek/hey/releases) for macOS, Linux and Windows

### note for packagers
to avoid name conflicts, packages should be named `hey-duck` or its form in a different naming convention.  
please submit an issue or a PR if you have packaged this to a distro, or email one of the maintainers.

# configuration & cache
there is a config file in `~/.config/hey/conf.toml` and a cache file in `~/.cache/hey/cache.json`

you can set their paths and filenames via `HEY_CONFIG_PATH`, `HEY_CONFIG_FILENAME` and `HEY_CACHE_PATH`, `HEY_CACHE_FILENAME`.

## config file reference
```toml
model = "Claude" # or "GPT3"
tos = false # whether if you agree to ddg chat tos
```

## cache file reference
cache file stores the last VQD used. it is (probably) there so that the ai model would remember your history. [read more about duckduckgo chat api](https://blek.codes/blog/duckduckgo-ai-chat/)

if you want to reset the VQD, just delete the file.
