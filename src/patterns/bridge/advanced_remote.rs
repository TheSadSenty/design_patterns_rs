use crate::patterns::bridge::device::Device;

pub(crate) struct AdvancedRemote {
    device: Box<dyn Device>,
}

impl AdvancedRemote {
    pub(crate) fn new(device: Box<dyn Device>) -> AdvancedRemote {
        AdvancedRemote { device }
    }
    fn volume_up(&mut self) {
        self.device.volume_up();
    }
    fn volume_down(&mut self) {
        self.device.volume_down();
    }
    pub(crate) fn turn_up(&mut self) {
        self.device.turn_up()
    }
    fn turn_down(&mut self) {
        self.device.turn_down()
    }

    pub(crate) fn mute(&mut self) {
        while self.device.get_volume() != 0 {
            self.device.volume_down();
        }
    }
    pub(crate) fn get_volume(&mut self) -> u8 {
        self.device.get_volume()
    }
    pub(crate) fn get_status(&mut self) -> bool {
        self.device.get_status()
    }
}
