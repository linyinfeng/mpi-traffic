use mpi::{collective::CommunicatorCollectives, topology::Communicator};
use mpi_traffic::communication::{bincode_all_gather_varcount, bincode_broadcast};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct A {
    pub v: Vec<i32>,
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    let local_data = A {
        v: vec![rank, rank + 1],
    };
    let data = bincode_all_gather_varcount(world, &local_data);
    println!("{:?}", data);

    world.barrier();

    let root = world.process_at_rank(0);
    let mut data = if rank == root.rank() {
        Some(A {
            v: vec![1, 2, 3, 4, 5],
        })
    } else {
        None
    };
    bincode_broadcast(rank, root, &mut data).unwrap();
    println!("{:?}", data);
}
