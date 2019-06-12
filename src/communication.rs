use mpi::{datatype::PartitionMut, traits::CommunicatorCollectives};
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
