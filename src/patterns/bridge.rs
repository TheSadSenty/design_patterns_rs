mod advanced_remote;
mod device;
mod remote;
mod tv;
#[cfg(test)]
mod test {
    use crate::patterns::bridge::*;

    #[test]
    fn bridge() {
        let tv_1 = tv::Tv {
            volume: 40,
            status: false,
        };
        let mut remote = remote::Remote::new(Box::new(tv_1));
        remote.turn_up();
        remote.volume_up();
        remote.volume_up();
        println!("Remote {}", remote.get_status());
        println!("Remote {}", remote.get_volume());
        let tv_2 = tv::Tv {
            volume: 40,
            status: false,
        };
        let mut advanced_remote = advanced_remote::AdvancedRemote::new(Box::new(tv_2));
        advanced_remote.turn_up();
        advanced_remote.mute();
        println!("Advanced Remote {}", advanced_remote.get_status());
        println!("Advanced Remote {}", advanced_remote.get_volume());
    }
}
