use crate::communication::CommunicationError;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Communication(err: CommunicationError) {
            from()
            display("Communication error: {}", err)
        }
    }
}
