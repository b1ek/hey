# hey!
`hey` is a command line tool to contact DuckDuckGo Chat API from your terminal.
based on [this article](https://blek.codes/blog/duckduckgo-ai-chat/)

demo:

<p align=center><img src='hey-demo.gif' alt='a gif demostrating a prompt about a bedtime story' width=1000></p>

# disclaimer
to clarify, as of may 17 2024, using a third party client [does not violate the ToS](https://duckduckgo.com/aichat/privacy-terms).

by using this client, you acknowledge that you will be liable for any ToS violations as per GPLv3

this project is not intended for API scraping purposes, and actually [has a soft protection against it](https://git.blek.codes/blek/hey/src/branch/main/src/main.rs#L34).

# Download and Run the Executable

This project provides pre-built executable files for various platforms. You can download the appropriate file for your operating system from the GitHub Actions artifacts.

## Download Instructions

1. Go to the [GitHub Actions page](https://github.com/iwannet/hey/actions) for this repository.
2. Find the latest successful workflow run.
3. Click on the "Artifacts" section to see the available files.
4. Download the file that matches your operating system (e.g., `hey-linux`, `hey-macos`, `hey-windows`). (you may need to login to be able to download it)

## Usage Instructions

1. Extract the downloaded file to a directory of your choice.
2. Open a terminal or command prompt and navigate to the directory where you extracted the file.
3. Run the executable using the following command:
   - Linux/macOS: `./hey-linux`
   - Windows: `.\hey`

The executable should now run on your system.

## Integrate with the System

To make the `hey` executable easily accessible, you can place it in a directory that is already in your system's `PATH` environment variable, such as `/usr/local/bin/`.

### Linux/macOS

1. Open a terminal and navigate to the directory where you extracted the `hey` folder.

2. Run the following command to copy the folder to the `/usr/local/bin/` directory (you may need to use `sudo` depending on your user permissions):

`sudo cp hey-linux /usr/local/bin/hey`
3. After running the command, you should be able to run the `hey` command from any directory in your terminal.


### Windows

#### 1. **Add to PATH using PowerShell**:

   To make the `hey` command available globally, you can add the directory containing the `hey-windows.exe` executable to your system's `PATH` environment variable using PowerShell.

   1. Open PowerShell as an administrator.
   2. Run the following command, replacing `C:\path\to\hey` with the actual path to the directory containing the `hey.exe` executable:

      ```powershell
      [Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";C:\path\to\hey\", [EnvironmentVariableTarget]::Machine)
      ```

   3. Close and reopen your PowerShell or command prompt window for the changes to take effect.

   After running these steps, you should be able to run the `hey` command from any directory in your PowerShell or command prompt.

#### 2. **Add to PATH** manually: 
To make the `hey` command available globally, you can add the directory containing the `hey.exe` executable to your system's `PATH` environment variable.
- Open the Start menu, search for "Environment Variables", and click on "Edit the system environment variables".
- Click on the "Environment Variables" button.
- In the "System Variables" section, find the "Path" variable, select it, and click "Edit".
- Click "New" and add the full path to the directory containing the `hey.exe` executable.
- Click "OK" to save the changes.



# Download via a package manager

arch (AUR) - `paru -S hey-duck`

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
