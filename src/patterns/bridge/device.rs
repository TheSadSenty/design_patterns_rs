pub(crate) trait Device {
    fn volume_up(&mut self);
    fn volume_down(&mut self);
    fn turn_up(&mut self);
    fn turn_down(&mut self);
    fn get_volume(&self) -> u8;
    fn get_status(&self) -> bool;
}
