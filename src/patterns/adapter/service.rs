use crate::patterns::adapter::*;
pub(crate) struct Service {}
impl Service {
    pub fn print_smth(&self, msg: Box<dyn target::Target>) {
        println!("{} {}", msg.get_first(), msg.get_second())
    }
}
