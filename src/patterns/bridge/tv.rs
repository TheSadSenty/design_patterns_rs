use crate::patterns::bridge::device;

pub(crate) struct Tv {
    pub(crate) volume: u8,
    pub(crate) status: bool,
}
impl device::Device for Tv {
    fn volume_up(&mut self) {
        if let Some(i) = self.volume.checked_add(10) {
            if i <= 100 {
                self.volume = i;
            }
        }
    }
    fn volume_down(&mut self) {
        if let Some(i) = self.volume.checked_sub(10) {
            self.volume = i;
        }
    }
    fn turn_up(&mut self) {
        self.status = true;
    }
    fn turn_down(&mut self) {
        self.status = false;
    }
    fn get_volume(&self) -> u8 {
        self.volume
    }
    fn get_status(&self) -> bool {
        self.status
    }
}
