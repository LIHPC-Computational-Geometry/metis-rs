//! Partitions computed by the vendored METIS do not depend on the platform.
//!
//! METIS uses random numbers, e.g. to choose its initial partitions. The vendored build uses
//! GKlib's own random number generator instead of the C library's `rand()`, whose sequence differs
//! between platforms, and disables floating-point contraction, which depends on the target and
//! compiler. Hence, a given seed yields the same partition on every platform. The expected
//! partitions were computed once and must be reproduced on all CI platforms.
#![cfg(not(feature = "use-system"))]

use metis::{option, Graph, Idx};

const SIDE: usize = 10;

/// A `SIDE` x `SIDE` grid, in which each vertex is connected to its horizontal and vertical
/// neighbors. Vertex `row * SIDE + col` lies in row `row` and column `col`.
fn grid() -> (Vec<Idx>, Vec<Idx>) {
    let mut xadj = vec![0];
    let mut adjncy = Vec::new();
    for row in 0..SIDE {
        for col in 0..SIDE {
            let vertex = row * SIDE + col;
            if row > 0 {
                adjncy.push((vertex - SIDE) as Idx);
            }
            if col > 0 {
                adjncy.push((vertex - 1) as Idx);
            }
            if col + 1 < SIDE {
                adjncy.push((vertex + 1) as Idx);
            }
            if row + 1 < SIDE {
                adjncy.push((vertex + SIDE) as Idx);
            }
            xadj.push(adjncy.len() as Idx);
        }
    }
    (xadj, adjncy)
}

// Both partitioning methods are checked in a single test, because the state of the random number
// generator is shared by the whole process and tests run in parallel.
#[test]
fn partitions_do_not_depend_on_the_platform() {
    let (xadj, adjncy) = grid();

    let mut kway = vec![0; SIDE * SIDE];
    Graph::new(1, 4, &xadj, &adjncy)
        .unwrap()
        .set_option(option::Seed(42))
        .part_kway(&mut kway)
        .unwrap();

    let mut recursive = vec![0; SIDE * SIDE];
    Graph::new(1, 4, &xadj, &adjncy)
        .unwrap()
        .set_option(option::Seed(42))
        .part_recursive(&mut recursive)
        .unwrap();

    assert_eq!(EXPECTED_KWAY.to_vec(), kway);
    assert_eq!(EXPECTED_RECURSIVE.to_vec(), recursive);
}

/// The partition of each vertex of the grid, one row per line.
#[rustfmt::skip]
const EXPECTED_KWAY: [Idx; SIDE * SIDE] = [
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    0, 0, 0, 0, 0, 3, 3, 3, 3, 3,
    0, 0, 0, 0, 0, 3, 3, 3, 3, 3,
    0, 0, 0, 0, 0, 3, 3, 3, 3, 3,
    0, 0, 0, 0, 0, 3, 3, 3, 3, 3,
    0, 0, 0, 0, 0, 3, 3, 3, 3, 3,
];

/// The partition of each vertex of the grid, one row per line.
#[rustfmt::skip]
const EXPECTED_RECURSIVE: [Idx; SIDE * SIDE] = [
    3, 3, 3, 3, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 3, 3, 0, 0, 0, 0, 0,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
];
