use metis::Graph;

fn main() -> Result<(), metis::Error> {
    // 5 - 3 - 4 - 0
    //     |   | /
    //     2 - 1
    let adjncy = [1, 4, 0, 2, 4, 1, 3, 2, 4, 5, 0, 1, 3, 3];
    let xadj = [0, 2, 5, 7, 10, 13, 14];

    // iterate over adjacent nodes
    let mut it = xadj
        .windows(2)
        .map(|x| &adjncy[x[0] as usize..x[1] as usize]);

    // node 0 is adjacent to nodes 1 and 4
    assert_eq!(it.next().unwrap(), &[1, 4]);
    // node 1 is adjacent to nodes 0, 2, and 4
    assert_eq!(it.next().unwrap(), &[0, 2, 4]);
    // node 2 is adjacent to nodes 1 and 3
    assert_eq!(it.next().unwrap(), &[1, 3]);
    // node 3 is adjacent to nodes 2, 4, and 5
    assert_eq!(it.next().unwrap(), &[2, 4, 5]);
    // node 4 is adjacent to nodes 0, 1, and 3
    assert_eq!(it.next().unwrap(), &[0, 1, 3]);
    // node 5 is adjacent to node 3
    assert_eq!(it.next().unwrap(), &[3]);
    assert!(it.next().is_none());

    let mut partition = vec![0; xadj.len() - 1];
    let n_parts = 2;
    Graph::new(1, n_parts, &xadj, &adjncy)?.part_kway(&mut partition)?;

    // group 0 consists out of node 0, 1 and 4
    // group 1 consists out of node 2, 3 and 5
    assert_eq!(&partition, &[0, 0, 1, 1, 0, 1]);

    let group = |group| {
        partition
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == group)
            .map(|(a, _)| a)
            .collect::<Vec<_>>()
    };
    // group 0 consists out of node 0, 1 and 4
    assert_eq!(&group(0), &[0, 1, 4]);
    // group 1 consists out of node 2, 3 and 5
    assert_eq!(&group(1), &[2, 3, 5]);

    Ok(())
}
