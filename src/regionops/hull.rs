pub trait Hull {
    fn get_center(&self) -> (u32, u32);
    fn get_points(&self) -> &Vec<(u32, u32)>;
}
