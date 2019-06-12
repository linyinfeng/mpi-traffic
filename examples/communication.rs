use mpi::topology::Communicator;
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

    let root = world.process_at_rank(0);
    let local_data = if rank == root.rank() {
        Some(A {
            v: vec![1, 2, 3, 4, 5],
        })
    } else {
        None
    };
    let data = bincode_broadcast(rank, root, local_data).unwrap();
    println!("{:?}", data);
}
