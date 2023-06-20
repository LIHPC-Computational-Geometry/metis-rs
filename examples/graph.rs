use metis::Graph;

fn main() -> Result<(), metis::Error> {
    let xadj = &mut [0, 2, 5, 8, 11, 13, 16, 20, 24, 28, 31, 33, 36, 39, 42, 44];
    #[rustfmt::skip]
    let adjncy = &mut [
        1, 5,
        0, 2, 6,
        1, 3, 7,
        2, 4, 8,
        3, 9,
        0, 6, 10,
        1, 5, 7, 11,
        2, 6, 8, 12,
        3, 7, 9, 13,
        4, 8, 14,
        5, 11,
        6, 10, 12,
        7, 11, 13,
        8, 12, 14,
        9, 13,
    ];
    let mut part = vec![0x00; 15];
    Graph::new(1, 2, xadj, adjncy).part_recursive(&mut part)?;
    println!("{:?}", part);

    Ok(())
}
