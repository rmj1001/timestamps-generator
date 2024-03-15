/*!
# Terminal User Interface

Design terminal menus for users to navigate.
*/

use crate::prelude::clearscr;

pub fn title() {
    clearscr();

    println!("AMA Timestamp Generator");
    println!("-----------------------");
    println!();
}

/**
Equivalent of DOS pause command. This waits for the user to press the
ENTER/RETURN key on their keyboard before moving on to other code.

# Usage

```
use albion_terminal_rpg::prelude::pause;

pause();
```
*/
pub fn pause() {
    println!("[PRESS (RETURN/ENTER) TO CONTINUE]");
    let mut garbage = String::new();
    let _ = std::io::stdin().read_line(&mut garbage);
}
