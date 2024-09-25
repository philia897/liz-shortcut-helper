# Liz Shortcut Helper

A Rust-based helper for those who can not remember all the shortcuts (especially those rarely used ones), which supports:

 - Show shortcuts with great filtering function based on [Rofi](https://github.com/davatorium/rofi/)

 - Automatically execute the selected shortcut using [ydotool](https://github.com/ReimuNotMoe/ydotool)

 - Customize the shortcuts and add whatever shortcut or command you want.

The project name is inspired by a great animation named "Liz and the Blue bird".

# Quick Start

## Installation



## How to use

```bash
# Show all shortcuts by rofi, then autorun the selected shortcut
liz
# or
liz run

# Reload all the sheets inside the given dir, use this when the sheets are modified
liz reload path/to/sheets
liz reload # use default sheets path: /home/{whoami}/.config/liz/sheets

```

# Project Structure

 - **Liz**: The girl sending command to the daemon for execution, and show the results from Bluebird.

 - **Bluebird**: The cute blue bird always waiting in the background and respond to the command from Liz.

 - **andthe**: Define the communication methods between Liz and Bluebird.

# Credits & License

 - Thanks to the wonderful projects [Rofi](https://github.com/davatorium/rofi/) and [ydotool](https://github.com/ReimuNotMoe/ydotool), which makes this project possible.

 - License: [GPL-3.0](./LICENSE)