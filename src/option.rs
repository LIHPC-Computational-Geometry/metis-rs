//! Fine-tuning parameter types.
//!
//! For options that take an integer value, should this value be negative, the
//! default will be used, if any.

use crate::m;
use crate::Idx;

mod private {
    pub trait Sealed {}
}

/// Trait implemented by METIS' options.
///
/// See [`crate::Graph::set_options`] for an example.  It is also used in
/// [`crate::Mesh::set_options`].
pub trait Opt: private::Sealed {
    /// Index of the option in the array from [`crate::Graph::set_options`] and
    /// [`crate::Mesh::set_options`].
    const INDEX: usize;

    /// Convert the value into metis' format, for use with
    /// [`crate::Graph::set_options`] and [`crate::Mesh::set_options`].
    fn value(self) -> Idx;
}

/// Specifies the partitioning method.
pub enum PType {
    /// Multilevel recursive bisectioning.
    Rb,

    /// Multilevel k-way partitioning.
    Kway,
}

impl private::Sealed for PType {}
impl Opt for PType {
    const INDEX: usize = m::moptions_et_METIS_OPTION_PTYPE as usize;

    fn value(self) -> Idx {
        match self {
            PType::Rb => m::mptype_et_METIS_PTYPE_RB as Idx,
            PType::Kway => m::mptype_et_METIS_PTYPE_KWAY as Idx,
        }
    }
}

/// Specifies the type of objective.
pub enum ObjType {
    /// Edge-cut minimization.
    Cut,

    /// Total communication volume minimization.
    Vol,
}

impl private::Sealed for ObjType {}
impl Opt for ObjType {
    const INDEX: usize = m::moptions_et_METIS_OPTION_OBJTYPE as usize;

    fn value(self) -> Idx {
        match self {
            ObjType::Cut => m::mobjtype_et_METIS_OBJTYPE_CUT as Idx,
            ObjType::Vol => m::mobjtype_et_METIS_OBJTYPE_VOL as Idx,
        }
    }
}

/// Specifies the matching scheme to be used during coarsening.
pub enum CType {
    /// Random matching.
    Rm,

    /// Sorted heavy-edge matching.
    Shem,
}

impl private::Sealed for CType {}
impl Opt for CType {
    const INDEX: usize = m::moptions_et_METIS_OPTION_CTYPE as usize;

    fn value(self) -> Idx {
        match self {
            CType::Rm => m::mctype_et_METIS_CTYPE_RM as Idx,
            CType::Shem => m::mctype_et_METIS_CTYPE_SHEM as Idx,
        }
    }
}

/// Determines the algorithm used during initial partitioning.
pub enum IpType {
    /// Grows a bisection using a greedy strategy.
    Grow,

    /// Compute a bisection at random followed by a refinement.
    Random,

    /// Derives a separator from an edge cut.
    Edge,

    /// Grow a bisection using a greedy node-based strategy.
    Node,
}

impl private::Sealed for IpType {}
impl Opt for IpType {
    const INDEX: usize = m::moptions_et_METIS_OPTION_IPTYPE as usize;

    fn value(self) -> Idx {
        match self {
            IpType::Grow => m::miptype_et_METIS_IPTYPE_GROW as Idx,
            IpType::Random => m::miptype_et_METIS_IPTYPE_RANDOM as Idx,
            IpType::Edge => m::miptype_et_METIS_IPTYPE_EDGE as Idx,
            IpType::Node => m::miptype_et_METIS_IPTYPE_NODE as Idx,
        }
    }
}

/// Determines the algorithm used for refinement.
pub enum RType {
    /// FM-based cut refinement.
    Fm,

    /// Greedy-based cut and volume refinement.
    Greedy,

    /// Two-sided FM refinement.
    Sep2Sided,

    /// One-sided FM refinement.
    Sep1Sided,
}

impl private::Sealed for RType {}
impl Opt for RType {
    const INDEX: usize = m::moptions_et_METIS_OPTION_RTYPE as usize;

    fn value(self) -> Idx {
        match self {
            RType::Fm => m::mrtype_et_METIS_RTYPE_FM as Idx,
            RType::Greedy => m::mrtype_et_METIS_RTYPE_GREEDY as Idx,
            RType::Sep2Sided => m::mrtype_et_METIS_RTYPE_SEP2SIDED as Idx,
            RType::Sep1Sided => m::mrtype_et_METIS_RTYPE_SEP1SIDED as Idx,
        }
    }
}

/// Specifies the number of different partitionings that it will compute. The
/// final partitioning is the one that achieves the best edgecut or
/// communication volume. Default is 1.
pub struct NCuts(pub Idx);

impl private::Sealed for NCuts {}
impl Opt for NCuts {
    const INDEX: usize = m::moptions_et_METIS_OPTION_NCUTS as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Specifies the number of different separators that it will compute at each
/// level of nested dissection.
///
/// The final separator that is used is the smallest one. Default is 1.
pub struct NSeps(pub Idx);

impl private::Sealed for NSeps {}
impl Opt for NSeps {
    const INDEX: usize = m::moptions_et_METIS_OPTION_NSEPS as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Used to indicate which numbering scheme is used for the adjacency structure
/// of a graph or the element-node structure of a mesh.
pub enum Numbering {
    /// C-style numbering which is assumed to start from 0.
    C,

    /// Fortran-style numbering which is assumed to start from 1.
    Fortran,
}

impl private::Sealed for Numbering {}
impl Opt for Numbering {
    const INDEX: usize = m::moptions_et_METIS_OPTION_NUMBERING as usize;

    fn value(self) -> Idx {
        match self {
            Numbering::C => 0,
            Numbering::Fortran => 1,
        }
    }
}

/// Specifies the number of iterations for the refinement algorithms at each
/// stage of the uncoarsening process.
///
/// Default is 10.
pub struct NIter(pub Idx);

impl private::Sealed for NIter {}
impl Opt for NIter {
    const INDEX: usize = m::moptions_et_METIS_OPTION_NITER as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Specifies the seed for the random number generator.
pub struct Seed(pub Idx);

impl private::Sealed for Seed {}
impl Opt for Seed {
    const INDEX: usize = m::moptions_et_METIS_OPTION_SEED as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Specifies that the partitioning routines should try to minimize the maximum
/// degree of the subdomain graph.
///
/// I.e., the graph in which each partition is a node, and edges connect
/// subdomains with a shared interface.
pub struct MinConn(pub bool);

impl private::Sealed for MinConn {}
impl Opt for MinConn {
    const INDEX: usize = m::moptions_et_METIS_OPTION_MINCONN as usize;

    fn value(self) -> Idx {
        self.0 as Idx
    }
}

/// Specifies that the coarsening will not perform any 2-hop matchings when the
/// standards matching approach fails to sufficiently coarsen the graph.
///
/// The 2-hop matching is very effective for graphs with power-law degree
/// distributions.
pub struct No2Hop(pub bool);

impl private::Sealed for No2Hop {}
impl Opt for No2Hop {
    const INDEX: usize = m::moptions_et_METIS_OPTION_NO2HOP as usize;

    fn value(self) -> Idx {
        self.0 as Idx
    }
}

/// Specifies that the partitioning routines should try to produce partitions
/// that are contiguous.
///
/// Note that if the input graph is not connected this option is ignored.
pub struct Contig(pub bool);

impl private::Sealed for Contig {}
impl Opt for Contig {
    const INDEX: usize = m::moptions_et_METIS_OPTION_CONTIG as usize;

    fn value(self) -> Idx {
        self.0 as Idx
    }
}

/// Specifies that the graph should be compressed by combining together vertices
/// that have identical adjacency lists.
pub struct Compress(pub bool);

impl private::Sealed for Compress {}
impl Opt for Compress {
    const INDEX: usize = m::moptions_et_METIS_OPTION_COMPRESS as usize;

    fn value(self) -> Idx {
        self.0 as Idx
    }
}

/// Specifies if the connected components of the graph should first be
/// identified and ordered separately.
pub struct CCOrder(pub bool);

impl private::Sealed for CCOrder {}
impl Opt for CCOrder {
    const INDEX: usize = m::moptions_et_METIS_OPTION_CCORDER as usize;

    fn value(self) -> Idx {
        self.0 as Idx
    }
}

/// Specifies the minimum degree of the vertices that will be ordered last.
///
/// If the specified value is `x > 0`, then any vertices with a degree greater
/// than `0.1*x*(average degree)` are removed from the graph, an ordering of the
/// rest of the vertices is computed, and an overall ordering is computed by
/// ordering the removed vertices at the end of the overall ordering.  For
/// example if `x == 40`, and the average degree is 5, then the algorithmwill
/// remove all vertices with degree greater than 20. The vertices that are
/// removed are ordered last (i.e., they are automatically placed in the
/// top-level separator). Good values are often in the range of 60 to 200 (i.e.,
/// 6 to 20 times more than the average). Default value is 0, indicating that no
/// vertices are removed.
///
/// Used to control whether or not the ordering algorithm should remove any
/// vertices with high degree (i.e., dense columns). This is particularly
/// helpful for certain classes of LP matrices, in which there a few vertices
/// that are connected to many other vertices. By removing these vertices prior
/// to ordering, the quality and the amount of time required to do the ordering
/// improves.
pub struct PFactor(pub Idx);

impl private::Sealed for PFactor {}
impl Opt for PFactor {
    const INDEX: usize = m::moptions_et_METIS_OPTION_PFACTOR as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Specifies the maximum allowed load imbalance among the partitions.
///
/// A value of `x` indicates that the allowed load imbalance is `(1 + x)/1000`.
/// The load imbalance for the `j`th constraint is defined to be
/// `max_i(w[j,i])/t[j,i])`, where `w[j,i]` is the fraction of the overall
/// weight of the `j`th constraint that is assigned to the`i`th partition and
/// `t[j,i]` is the desired target weight of the `j`th constraint for the `i`th
/// partition (i.e., that specified via `-tpwgts`). For `-ptype=rb`, the default
/// value is 1 (i.e., load imbalance of 1.001) and for `-ptype=kway`, the
/// default value is 30 (i.e., load imbalance of 1.03).
pub struct UFactor(pub Idx);

impl private::Sealed for UFactor {}
impl Opt for UFactor {
    const INDEX: usize = m::moptions_et_METIS_OPTION_UFACTOR as usize;

    fn value(self) -> Idx {
        self.0
    }
}

/// Specifies the amount of progress/debugging information will be printed
/// during the execution of the algorithms.
///
/// The default value is false for every field (no debugging/progress
/// information).
pub struct DbgLvl {
    /// Prints various diagnostic messages.
    pub info: bool,

    /// Performs timing analysis.
    pub time: bool,

    /// Displays various statistics during coarsening.
    pub coarsen: bool,

    /// Displays various statistics during refinement.
    pub refine: bool,

    /// Displays various statistics during initial partitioning.
    pub ipart: bool,

    /// Display detailed information about vertex moves during refinement.
    pub move_info: bool,

    /// Display detailed information about vertex separators.
    pub sep_info: bool,

    /// Display information related to the minimization of subdomain
    /// connectivity.
    pub conn_info: bool,

    /// Display information related to the elimination of connected components.
    pub contig_info: bool,
}

impl private::Sealed for DbgLvl {}
impl Opt for DbgLvl {
    const INDEX: usize = m::moptions_et_METIS_OPTION_DBGLVL as usize;

    fn value(self) -> Idx {
        let mut dbglvl = 0;
        if self.info {
            dbglvl |= 1;
        }
        if self.time {
            dbglvl |= 2;
        }
        if self.coarsen {
            dbglvl |= 4;
        }
        if self.refine {
            dbglvl |= 8;
        }
        if self.ipart {
            dbglvl |= 16;
        }
        if self.move_info {
            dbglvl |= 32;
        }
        if self.sep_info {
            dbglvl |= 64;
        }
        if self.conn_info {
            dbglvl |= 128;
        }
        if self.contig_info {
            dbglvl |= 256;
        }
        dbglvl
    }
}
