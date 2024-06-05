use crate::patterns::adapter::other_message;
use crate::patterns::adapter::target;
pub(crate) struct Adapter {
    first: String,
    second: String,
}
impl Adapter {
    pub(crate) fn fit(msg: other_message::OtherMessage) -> Self {
        let data: Vec<&str> = msg.full_name.split(' ').collect();
        Adapter {
            first: data[0].to_string(),
            second: data[1].to_string(),
        }
    }
}
impl target::Target for Adapter {
    fn get_second(&self) -> String {
        self.second.clone()
    }
    fn get_first(&self) -> String {
        self.first.clone()
    }
}
