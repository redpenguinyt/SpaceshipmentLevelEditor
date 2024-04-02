use rfd::{MessageButtons, MessageDialog, MessageLevel};

mod app;

fn main() {
    let r = app::main();

    if let Err(err) = r {
        MessageDialog::new()
            .set_title("The editor encountered an error")
            .set_level(MessageLevel::Error)
            .set_description(err)
            .set_buttons(MessageButtons::Ok)
            .show();
    }
}
