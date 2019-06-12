use mpi::{
    collective::{CommunicatorCollectives, Root},
    datatype::PartitionMut,
    topology::Rank,
};
use quick_error::quick_error;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, num::TryFromIntError};

quick_error! {
    #[derive(Debug)]
    pub enum CommunicationError {
        Bincode(err: bincode::Error) {
            from()
            display("Bincode error: {}", err)
        }
        IntegerConversion(err: TryFromIntError) {
            from()
            display("Integer conversion error: {}", err)
        }
    }
}

pub fn bincode_all_gather_varcount<Comm, T>(
    comm: Comm,
    send: &T,
) -> Result<Vec<T>, CommunicationError>
where
    Comm: CommunicatorCollectives,
    T: Serialize + for<'a> Deserialize<'a>,
{
    let size = comm.size() as usize;
    let serialized = bincode::serialize(send)?;
    let local_count: i32 = serialized.len().try_into()?;
    let mut counts = vec![0i32; size];
    let mut displacements = vec![0i32; size];
    comm.all_gather_into(&local_count, &mut counts[..]);
    {
        let mut place = 0i32;
        for (i, c) in counts.iter().enumerate() {
            displacements[i] = place;
            place += c;
        }
    }
    let buffer_size: usize = counts.iter().map(|n| *n as usize).sum();
    let mut receive_buffer = vec![0u8; buffer_size];
    let mut partition = PartitionMut::new(&mut receive_buffer[..], &counts[..], &displacements[..]);
    comm.all_gather_varcount_into(&serialized[..], &mut partition);
    displacements
        .iter()
        .zip(counts.iter())
        .map(|(d, c)| {
            let d = *d as usize;
            let c = *c as usize;
            let single = &receive_buffer[d..d + c];
            bincode::deserialize(single).map_err(CommunicationError::Bincode)
        })
        .collect()
}

pub fn bincode_broadcast<R, T>(
    rank: Rank,
    root: R,
    send_item: Option<T>,
) -> Result<T, CommunicationError>
where
    R: Root,
    T: Serialize + for<'a> Deserialize<'a>,
{
    match send_item {
        Some(item) => {
            assert_eq!(root.root_rank(), rank);
            let mut serialized = bincode::serialize(&item)?;
            let mut length = serialized.len();
            root.broadcast_into(&mut length);
            root.broadcast_into(&mut serialized[..]);
            Ok(item)
        },
        None => {
            assert_ne!(root.root_rank(), rank);
            let mut length = 0usize;
            root.broadcast_into(&mut length);
            let mut buffer = vec![0u8; length];
            root.broadcast_into(&mut buffer[..]);
            let item = bincode::deserialize(&buffer[..])?;
            Ok(item)
        },
    }
}
