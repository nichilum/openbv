use crate::math::point::Point;

pub trait Hull {
    fn get_center(&self) -> Point;
    fn get_points(&self) -> &Vec<Point>;
}
