use dialog::DialogBox;

mod app;

fn main() {
    let r = app::main();

    if let Err(err) = r {
        dialog::Message::new(err)
            .title("The editor encountered an error")
            .show()
            .expect("Could not display dialog box");
    }
}
