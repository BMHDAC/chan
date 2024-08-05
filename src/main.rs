fn main() {
    println!("Hello, world!");
}

pub trait Future {
    fn talk();
    fn start_dating();
    fn get_married();
    fn have_kids();
    fn live_forever();
}
pub enum Us {
    You,
    Me,
}

impl Future for Us {
    fn friend_zone();
}
