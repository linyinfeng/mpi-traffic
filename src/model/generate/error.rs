use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        OneWayRoadToEndIntersection {
            display("OneWay road to end intersection is not allowed currently")
        }
    }
}
