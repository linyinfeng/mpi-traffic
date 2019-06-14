use mpi::{
    collective::{CommunicatorCollectives, Root},
    datatype::PartitionMut,
    topology::Rank,
};
use quick_error::quick_error;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, num::TryFromIntError, ops::Range};

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

pub fn bincode_broadcast<R, T>(rank: Rank, root: R, item: &mut T) -> Result<(), CommunicationError>
where
    R: Root,
    T: Serialize + for<'a> Deserialize<'a>,
{
    if rank == root.root_rank() {
        let mut serialized = bincode::serialize(&*item)?;
        let mut length = serialized.len();
        root.broadcast_into(&mut length);
        root.broadcast_into(&mut serialized[..]);
    } else {
        let mut length = 0usize;
        root.broadcast_into(&mut length);
        let mut buffer = vec![0u8; length];
        root.broadcast_into(&mut buffer[..]);
        *item = bincode::deserialize(&buffer[..])?;
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Division {
    first: usize,
    last: usize,
}

impl Division {
    pub fn new(count: usize, rank: Rank, size: Rank) -> Self {
        assert!(rank < size);
        let rank = rank as usize;
        let size = size as usize;
        let basic_count = if count % size == 0 {
            count / size
        } else {
            count / size + 1
        };
        Division {
            first: basic_count * rank,
            last: count.min(basic_count * (rank + 1)),
        }
    }

    pub fn range(&self) -> Range<usize> {
        self.first..self.last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_division() {
        let count = 10;
        let size = 3;
        assert_eq!(
            Division::new(count, 0, size),
            Division { first: 0, last: 4 }
        );
        assert_eq!(
            Division::new(count, 1, size),
            Division { first: 4, last: 8 }
        );
        assert_eq!(
            Division::new(count, 2, size),
            Division { first: 8, last: 10 }
        );
    }

    #[test]
    #[should_panic]
    fn division_invalid_rank() {
        let count = 10;
        let size = 3;
        assert_eq!(
            Division::new(count, 3, size),
            Division { first: 8, last: 10 }
        );
    }
}
