use crate::patterns::adapter::target::Target;

pub struct Message {
    pub(crate) first_name: String,
    pub(crate) second_name: String,
}

impl Target for Message {
    fn get_first(&self) -> String {
        self.first_name.clone()
    }
    fn get_second(&self) -> String {
        self.second_name.clone()
    }
}
