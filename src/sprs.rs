use crate::Graph;
use crate::Idx;
use crate::Partition;
use std::ops::Deref;

/// Setup a partition of a graph encoded with an adjacency matrix from the
/// [sprs] crate.
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), metis::Error> {
/// use metis::Partition as _;
///
/// // Build this graph:
/// //
/// //     0 -- 1
/// //     |    |
/// //     3 -- 2
/// //
/// const NUM_VERTICES: usize = 4;
/// let mut a = sprs::TriMatI::new((NUM_VERTICES, NUM_VERTICES));
/// a.add_triplet(0, 1, 42);
/// a.add_triplet(1, 2, 42); // All edges have the
/// a.add_triplet(2, 3, 42); // same weight (42).
/// a.add_triplet(3, 0, 42);
/// let a = a.to_csr();
///
/// let mut partition = [0; NUM_VERTICES];
/// a.setup_partition(2) // Partition with 2 parts.
///     .set_vwgt(&[2, 2, 1, 1]) // Set weights on vertices.
///     .part_kway(&mut partition)?;
///
/// eprintln!("{:?}", partition);
/// # Ok(())
/// # }
/// ```
impl<IptrStorage, IndStorage, DataStorage> Partition
    for sprs::CsMatBase<Idx, Idx, IptrStorage, IndStorage, DataStorage, Idx>
where
    IptrStorage: Deref<Target = [Idx]>,
    IndStorage: Deref<Target = [Idx]>,
    DataStorage: Deref<Target = [Idx]>,
{
    fn setup_partition(&self, nparts: Idx) -> Graph<'_> {
        let (xadj, adjncy, data) = self.view().into_raw_storage();
        Graph::new(1, nparts, xadj, adjncy).set_adjwgt(data)
    }
}
