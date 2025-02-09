- ~~Add functionality for configure using config file~~
    
- ~~Manage the ./data and write simple example about how to custom sheets~~
    
- ~~Add functionality to persist automatically when exiting or using a timer (low priority)~~
    
- ~~Allow running Bluebird as backend service and add Liz to using one shortcut to call~~

- Make it available to use pacman to run it (Ongoing), the PKGBUILD file is created by not working for now.
    
- Add more shortcuts, fitting terminal, vim, etc (Ongoing).

- Replace the ydotool part by a tool written by Rust, which should follow the same idea to use one virtual device to simulate keyboard events, a lightweighted ydotool.

- Replace the Rofi part by a GUI, maybe built on `Tauri`.

- Find a way to show the information about each shortcut including its comments, maybe add one argument of `liz run` like `liz run details`

- Optimize the installation script, following an example like `lunarVim` to install by automatically `wget` a script and build.

- Refactor the bluebird and liz code using `clap` to managing arguments

- Run bluebird as one user service instead, for security.