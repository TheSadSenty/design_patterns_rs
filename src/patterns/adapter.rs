mod message;
mod other_message;
mod service;
mod target;
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
    fn get_first(&self) -> String {
        self.first.clone()
    }
    fn get_second(&self) -> String {
        self.second.clone()
    }
}
#[cfg(test)]
mod test {
    use crate::patterns::adapter;
    use crate::patterns::adapter::service::Service;
    use crate::patterns::adapter::{message, other_message};

    #[test]
    fn adapter() {
        let serv = Service {};
        let msg1 = message::Message {
            first_name: "One".to_string(),
            second_name: "Message".to_string(),
        };
        serv.print_smth(Box::new(msg1));
        let msg2 = other_message::OtherMessage {
            full_name: "Other Message".to_string(),
        };
        let adapter = adapter::Adapter::fit(msg2);
        serv.print_smth(Box::new(adapter));
    }
}
