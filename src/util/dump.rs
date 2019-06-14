use mpi::topology::Communicator;
use std::fmt::Debug;

pub fn dump<Comm, T>(communicator: Comm, path: &str, item: &T)
where
    Comm: Communicator,
    T: Debug,
{
    use std::io::Write;
    let mut file = std::fs::File::create(format!("{}/{}.txt", path, communicator.rank())).unwrap();
    file.write_all(format!("{:#?}", item).as_bytes()).unwrap();
}
