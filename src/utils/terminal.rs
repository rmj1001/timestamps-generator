/*!
# Terminal Functions

These functions are provided for manipulating the terminal process.
*/

use std::process::Command;

/**
Clears the terminal screen.

# Usage

```
use albion_terminal_rpg::prelude::clearscr;

clearscr();
```
*/
pub fn clearscr() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
