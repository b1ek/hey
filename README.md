# hey!
`hey` is a command line tool to contact DuckDuckGo Chat API from your terminal.
based on [this article](https://blek.codes/blog/duckduckgo-ai-chat/)

like this:

```sh
$ hey, how do you install windows on arch linux\?
Contacting DuckDuckGo chat AI...
 Here are the basic steps to install Windows on a system that already has Arch Linux installed:

1. Shrink the Arch Linux partition to make space for Windows. This can be done using a disk partitioning tool like GParted. You'll need at least 20-30GB of unallocated space for Windows.

2. Download the Windows ISO file from Microsoft's website and write it to a USB drive to create a bootable Windows installer. 

3. Reboot the system and enter the BIOS/UEFI settings to change the boot order so that the USB drive is prioritized. This will allow you to boot into the Windows installer.

4. When the Windows installer loads, select the "Custom install" option and choose the unallocated space you created earlier as the location to install Windows. 

5. Follow the on-screen instructions to complete the Windows installation. The installer will automatically format the partition and install Windows files. 

6. Once installed, you'll need to reconfigure the bootloader like GRUB to add an entry to dual boot between Arch Linux and Windows. This can be done by running update-grub in Arch Linux.

7. Reboot and you should now see an option to choose between Arch Linux and Windows on startup. You can switch between them as needed.

A few things to note - make sure to backup any important data before shrinking partitions. Also, Windows may overwrite the MBR with its own bootloader, so reconfiguring GRUB is important to retain Arch Linux booting ability. With some preparation, it's possible to smoothly install Windows alongside an existing Arch Linux installation.
```

# installation
if you run linux or macos,
```sh
git clone https://git.blek.codes/blek/hey.git
cd hey
cargo b -r
sudo cp target/release/hey /usr/bin/hey,
```

if you are on windows, idk have fun