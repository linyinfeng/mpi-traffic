use crate::model::board::IntersectionContext;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NoIntersectionPattern(context: IntersectionContext) {
            display("No pattern implemented for generating intersection with context {:?}", context)
        }
    }
}
