use crate::patterns::factory_pattern::gui;
struct WindowsButton {
    text: &'static str,
}

pub struct WindowsDialog {}
impl gui::Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn gui::Button> {
        Box::new(WindowsButton { text: "Windows" })
    }
}

impl gui::Button for WindowsButton {
    fn render(&self) {
        println!("CreateWindow({}....)", self.text);
        self.on_click();
    }
    fn on_click(&self) {
        println!("You've clicked on {} button", self.text);
    }
}
