use crate::patterns::factory_pattern::{gui, html, windows};
pub fn init(platform: gui::Platform) -> Box<dyn gui::Dialog> {
    match platform {
        gui::Platform::Web => Box::new(html::HtmlDialog {}),
        gui::Platform::Windows => Box::new(windows::WindowsDialog {}),
    }
}
#[cfg(test)]
mod test {
    use crate::patterns::factory_pattern::*;

    #[test]
    fn factory_test_windows() {
        let platform = gui::Platform::Windows;
        let dialog = factory::init(platform);
        dialog.render();
        
        
    }
    #[test]
    fn factory_test_html() {
        let platform = gui::Platform::Web;
        let dialog = factory::init(platform);
        dialog.render();
    }
}
