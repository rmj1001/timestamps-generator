use cli_timestamps_generator::prelude::*;

fn main() {
    let mut timestamps: Vec<Timestamp> = vec![];

    loop {
        title();

        let option = select(
            &[
                "1. New Timestamp",
                "2. Save Data",
                "3. Retrieve Data from Disk",
                "4. View timestamps",
                "5. Remove Individual Lines",
                "6. Clear memory",
                "7. Exit",
            ],
            None,
        );

        match option + 1 {
            1 => timestamps.push(Timestamp::menu()),
            2 => build_file(&timestamps),
            3 => timestamps = rebuild_timestamps(),
            4 => view_timestamps(&timestamps),
            5 => remove_line(&mut timestamps),
            6 => {
                timestamps.clear();
                clearscr();

                println!("Memory cleared.");
                pause();
            }
            7 => {
                clearscr();
                std::process::exit(0)
            }
            _ => panic!("Out of bounds"),
        }
    }
}
