use crate::patterns::factory_pattern::gui;

struct HtmlButton {
    text: &'static str,
}
pub struct HtmlDialog {}
impl gui::Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn gui::Button> {
        Box::new(HtmlButton { text: "HTML" })
    }
}
impl gui::Button for HtmlButton {
    fn render(&self) {
        println!("<button type=\"button\">{}</button>", self.text);
        self.on_click();
    }
    fn on_click(&self) {
        println!("You've clicked on {} button", self.text);
    }
}
