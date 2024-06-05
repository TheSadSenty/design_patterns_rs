use crate::patterns::adapter::*;
struct Service {}
impl Service {
    pub fn print_smth(&self, msg: Box<dyn target::Target>) {
        println!("{} {}", msg.get_first(), msg.get_second())
    }
}
#[cfg(test)]
mod test {
    use crate::patterns::adapter::service::Service;
    use crate::patterns::adapter::{adapter, message, other_message};

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
