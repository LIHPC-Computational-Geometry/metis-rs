//! A wrapper around Scotch bindings.
//!
//! This crate provides a thin but idiomatic API for libmetis.
//!
//! See [`Graph`] for a usage example.

#![deny(missing_docs)]

use metis_sys as m;
use std::convert::TryFrom;
use std::fmt;
use std::mem;
use std::os;
use std::ptr;
use std::slice;

pub mod option;

#[cfg(target_pointer_width = "16")]
compile_error!("METIS does not support 16-bit architectures");

/// Integer type used by METIS, can either be an [`i32`] or an [`i64`].
pub type Idx = m::idx_t;

/// Floating-point type used by METIS, can either be an [`f32`] or an [`f64`].
pub type Real = m::real_t;

/// The length of the `options` array.
///
/// See [`Graph::set_options`] for an example.  It is also used in
/// [`Mesh::set_options`].
pub const NOPTIONS: usize = m::METIS_NOPTIONS as usize;

/// Error type returned by METIS.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Input is invalid.
    ///
    /// These bindings should check for most input errors, if not all.
    Input,

    /// METIS hit an out-of-memory error.
    Memory,

    /// METIS returned an error but its meaning is unknown.
    Other,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Input => write!(f, "invalid input"),
            Error::Memory => write!(f, "out of memory"),
            Error::Other => write!(f, "METIS returned an error"),
        }
    }
}

/// The result of a partitioning.
pub type Result<T> = std::result::Result<T, Error>;

trait ErrorCode {
    /// Makes a [`Result`] from a return code (int) from METIS.
    fn wrap(self) -> Result<()>;
}

impl ErrorCode for m::rstatus_et {
    fn wrap(self) -> Result<()> {
        match self {
            m::rstatus_et_METIS_OK => Ok(()),
            m::rstatus_et_METIS_ERROR_INPUT => Err(Error::Input),
            m::rstatus_et_METIS_ERROR_MEMORY => Err(Error::Memory),
            m::rstatus_et_METIS_ERROR => Err(Error::Other),
            other => panic!("unexpected error code ({}) from METIS", other),
        }
    }
}

/// Builder structure to setup a graph partition computation.
///
/// This structure holds the required arguments for METIS to compute a
/// partition.  It also offers methods to easily set any optional argument.
///
/// # Example
///
/// ```rust
/// # use metis::Graph;
/// // Make a graph with two vertices and an edge between the two.
/// let xadj = &mut [0, 1, 2];
/// let adjncy = &mut [1, 0];
///
/// // Allocate the partition array which stores the partition of each vertex.
/// let mut part = [0, 0];
///
/// // There are one constraint and two parts.  The partitioning algorithm used
/// // is recursive bisection.  The k-way algorithm can also be used.
/// Graph::new(1, 2, xadj, adjncy)
///     .part_recursive(&mut part)
///     .unwrap();
///
/// // The two vertices are placed in different parts.
/// assert_ne!(part[0], part[1]);
/// ```
#[derive(Debug, PartialEq)]
pub struct Graph<'a> {
    /// The number of balancing constrains.
    ncon: Idx,

    /// The number of parts to partition the graph.
    nparts: Idx,

    /// The adjency structure of the graph (part 1).
    xadj: &'a mut [Idx],

    /// The adjency structure of the graph (part 2).
    ///
    /// Required size: xadj.last()
    adjncy: &'a mut [Idx],

    /// The computational weights of the vertices.
    ///
    /// Required size: ncon * (xadj.len()-1)
    vwgt: Option<&'a mut [Idx]>,

    /// The communication weights of the vertices.
    ///
    /// Required size: xadj.len()-1
    vsize: Option<&'a mut [Idx]>,

    /// The weight of the edges.
    ///
    /// Required size: xadj.last()
    adjwgt: Option<&'a mut [Idx]>,

    /// The target partition weights of the vertices.
    ///
    /// If `None` then the graph is equally divided among the partitions.
    ///
    /// Required size: ncon * nparts
    tpwgts: Option<&'a mut [Real]>,

    /// Imbalance tolerances for each constraint.
    ///
    /// Required size: ncon
    ubvec: Option<&'a mut [Real]>,

    /// Fine-tuning parameters.
    options: [Idx; NOPTIONS],
}

impl<'a> Graph<'a> {
    /// Creates a new [`Graph`] object to be partitioned.
    ///
    /// # Panics
    ///
    /// This function panics if:
    /// - any of the arrays have a length that cannot be hold by an [`Idx`], or
    /// - `ncon` is not strictly greater than zero, or
    /// - `nparts` is not strictly greater than zero, or
    /// - `xadj` is empty, or
    /// - the length of `adjncy` is different than the last element of `xadj`.
    ///
    /// # Mutability
    ///
    /// While nothing should be modified by the [`Graph`] structure, METIS
    /// doesn't specify any `const` modifier, so everything must be mutable on
    /// Rust's side.
    pub fn new(ncon: Idx, nparts: Idx, xadj: &'a mut [Idx], adjncy: &'a mut [Idx]) -> Graph<'a> {
        assert!(0 < ncon, "ncon must be strictly greater than zero");
        assert!(0 < nparts, "nparts must be strictly greater than zero");
        let _ = Idx::try_from(xadj.len()).expect("xadj array larger than Idx::MAX");
        assert_ne!(xadj.len(), 0);
        let adjncy_len = Idx::try_from(adjncy.len()).expect("adjncy array larger than Idx::MAX");
        assert_eq!(adjncy_len, *xadj.last().unwrap());

        Graph {
            ncon,
            nparts,
            xadj,
            adjncy,
            vwgt: None,
            vsize: None,
            adjwgt: None,
            tpwgts: None,
            ubvec: None,
            options: [-1; NOPTIONS],
        }
    }

    /// Sets the computational weights of the vertices.
    ///
    /// By default all vertices have the same weight.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `vwgt` is not `ncon` times the
    /// number of vertices.
    pub fn set_vwgt(mut self, vwgt: &'a mut [Idx]) -> Graph<'a> {
        let vwgt_len = Idx::try_from(vwgt.len()).expect("vwgt array too large");
        assert_eq!(vwgt_len, self.ncon * (self.xadj.len() as Idx - 1));
        self.vwgt = Some(vwgt);
        self
    }

    /// Sets the communication weights of the vertices.
    ///
    /// By default all vertices have the same communication weight.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `vsize` is not the number of
    /// vertices.
    pub fn set_vsize(mut self, vsize: &'a mut [Idx]) -> Graph<'a> {
        let vsize_len = Idx::try_from(vsize.len()).expect("vsize array too large");
        assert_eq!(vsize_len, self.xadj.len() as Idx - 1);
        self.vsize = Some(vsize);
        self
    }

    /// Sets the weights of the edges.
    ///
    /// By default all edges have the same weight.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `adjwgt` is not equal to the
    /// length of `adjncy`.
    pub fn set_adjwgt(mut self, adjwgt: &'a mut [Idx]) -> Graph<'a> {
        let adjwgt_len = Idx::try_from(adjwgt.len()).expect("adjwgt array too large");
        assert_eq!(adjwgt_len, *self.xadj.last().unwrap());
        self.adjwgt = Some(adjwgt);
        self
    }

    /// Sets the target partition weights for each part and constraint.
    ///
    /// By default the graph is divided equally.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `tpwgts` is not equal to `ncon`
    /// times `nparts`.
    pub fn set_tpwgts(mut self, tpwgts: &'a mut [Real]) -> Graph<'a> {
        let tpwgts_len = Idx::try_from(tpwgts.len()).expect("tpwgts array too large");
        assert_eq!(tpwgts_len, self.ncon * self.nparts);
        self.tpwgts = Some(tpwgts);
        self
    }

    /// Sets the load imbalance tolerance for each constraint.
    ///
    /// By default it equals to 1.001 if `ncon` equals 1 and 1.01 otherwise.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `ubvec` is not equal to `ncon`.
    pub fn set_ubvec(mut self, ubvec: &'a mut [Real]) -> Graph<'a> {
        let ubvec_len = Idx::try_from(ubvec.len()).expect("ubvec array too large");
        assert_eq!(ubvec_len, self.ncon);
        self.ubvec = Some(ubvec);
        self
    }

    /// Sets the fine-tuning parameters for this partitioning.
    ///
    /// When few options are to be set, [`Graph::set_option`] might be a
    /// better fit.
    ///
    /// See the [option] module for the list of available parameters.  Note that
    /// not all are applicable to a given partitioning method.  Refer to the
    /// documentation of METIS ([link]) for more info on this.
    ///
    /// [link]: http://glaros.dtc.umn.edu/gkhome/fetch/sw/metis/manual.pdf
    ///
    /// # Example
    ///
    /// ```rust
    /// # use metis::Graph;
    /// use metis::option::Opt;
    ///
    /// let xadj = &mut [0, 1, 2];
    /// let adjncy = &mut [1, 0];
    /// let mut part = [0, 0];
    ///
    /// // -1 is the default value.
    /// let mut options = [-1; metis::NOPTIONS];
    ///
    /// // four refinement iterations instead of the default 10.
    /// options[metis::option::NIter::INDEX] = 4;
    ///
    /// Graph::new(1, 2, xadj, adjncy)
    ///     .set_options(&options)
    ///     .part_recursive(&mut part)
    ///     .unwrap();
    ///
    /// // The two vertices are placed in different parts.
    /// assert_ne!(part[0], part[1]);
    /// ```
    pub fn set_options(mut self, options: &[Idx; NOPTIONS]) -> Graph<'a> {
        self.options.copy_from_slice(options);
        self
    }

    /// Sets a fine-tuning parameter for this partitioning.
    ///
    /// When options are to be set in batches, [`Graph::set_options`] might be a
    /// better fit.
    ///
    /// See the [option] module for the list of available parameters.  Note that
    /// not all are applicable to a given partitioning method.  Refer to the
    /// documentation of METIS ([link]) for more info on this.
    ///
    /// [link]: http://glaros.dtc.umn.edu/gkhome/fetch/sw/metis/manual.pdf
    ///
    /// # Example
    ///
    /// ```rust
    /// # use metis::Graph;
    /// let xadj = &mut [0, 1, 2];
    /// let adjncy = &mut [1, 0];
    /// let mut part = [0, 0];
    ///
    /// Graph::new(1, 2, xadj, adjncy)
    ///     .set_option(metis::option::NIter(4))
    ///     .part_recursive(&mut part)
    ///     .unwrap();
    ///
    /// // The two vertices are placed in different parts.
    /// assert_ne!(part[0], part[1]);
    /// ```
    pub fn set_option<O>(mut self, option: O) -> Graph<'a>
    where
        O: option::Opt,
    {
        self.options[O::INDEX] = option.value();
        self
    }

    /// Partition the graph using multilevel recursive bisection.
    ///
    /// Returns the edge-cut, the total communication volume of the
    /// partitioning solution.
    ///
    /// Equivalent of `METIS_PartGraphRecursive`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `part` is not the number of
    /// vertices.
    pub fn part_recursive(mut self, part: &mut [Idx]) -> Result<Idx> {
        let part_len = Idx::try_from(part.len()).expect("part array larger than Idx::MAX");
        assert_eq!(
            part_len,
            self.xadj.len() as Idx - 1,
            "part.len() must be equal to the number of vertices",
        );

        let nvtxs = &mut (self.xadj.len() as Idx - 1) as *mut Idx;
        let ncon = &mut self.ncon as *mut Idx;
        let xadj = self.xadj.as_mut_ptr();
        let adjncy = self.adjncy.as_mut_ptr();
        let vwgt = if let Some(vwgt) = self.vwgt {
            vwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let vsize = if let Some(vsize) = self.vsize {
            vsize.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let adjwgt = if let Some(adjwgt) = self.adjwgt {
            adjwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let nparts = &mut self.nparts as *mut Idx;
        let tpwgts = if let Some(tpwgts) = self.tpwgts {
            tpwgts.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let ubvec = if let Some(ubvec) = self.ubvec {
            ubvec.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let options = self.options.as_mut_ptr();
        let mut edgecut = mem::MaybeUninit::uninit();
        let part = part.as_mut_ptr();
        unsafe {
            m::METIS_PartGraphRecursive(
                nvtxs,
                ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                adjwgt,
                nparts,
                tpwgts,
                ubvec,
                options,
                edgecut.as_mut_ptr(),
                part,
            )
            .wrap()?;
            Ok(edgecut.assume_init())
        }
    }

    /// Partition the graph using multilevel k-way partitioning.
    ///
    /// Returns the edge-cut, the total communication volume of the
    /// partitioning solution.
    ///
    /// Equivalent of `METIS_PartGraphKway`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `part` is not the number of
    /// vertices.
    pub fn part_kway(mut self, part: &mut [Idx]) -> Result<Idx> {
        let part_len = Idx::try_from(part.len()).expect("part array larger than Idx::MAX");
        assert_eq!(
            part_len,
            self.xadj.len() as Idx - 1,
            "part.len() must be equal to the number of vertices",
        );

        let nvtxs = &mut (self.xadj.len() as Idx - 1) as *mut Idx;
        let ncon = &mut self.ncon as *mut Idx;
        let xadj = self.xadj.as_mut_ptr();
        let adjncy = self.adjncy.as_mut_ptr();
        let vwgt = if let Some(vwgt) = self.vwgt {
            vwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let vsize = if let Some(vsize) = self.vsize {
            vsize.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let adjwgt = if let Some(adjwgt) = self.adjwgt {
            adjwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let nparts = &mut self.nparts as *mut Idx;
        let tpwgts = if let Some(tpwgts) = self.tpwgts {
            tpwgts.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let ubvec = if let Some(ubvec) = self.ubvec {
            ubvec.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let options = self.options.as_mut_ptr();
        let mut edgecut = mem::MaybeUninit::uninit();
        let part = part.as_mut_ptr();
        unsafe {
            m::METIS_PartGraphKway(
                nvtxs,
                ncon,
                xadj,
                adjncy,
                vwgt,
                vsize,
                adjwgt,
                nparts,
                tpwgts,
                ubvec,
                options,
                edgecut.as_mut_ptr(),
                part,
            )
            .wrap()?;
            Ok(edgecut.assume_init())
        }
    }
}

/// Builder structure to setup a mesh partition computation.
///
/// This structure holds the required arguments for METIS to compute a
/// partition.  It also offers methods to easily set any optional argument.
///
/// # Example
///
/// Usage is fairly similar to [`Graph`].  Refer to its documentation for
/// details.
#[derive(Debug, PartialEq)]
pub struct Mesh<'a> {
    /// The number of nodes in the mesh.
    nn: Idx,

    /// The number of parts to partition the mesh.
    nparts: Idx,

    /// The number of nodes two elements must share for an edge to appear in the
    /// dual graph.
    ncommon: Idx,

    eptr: &'a mut [Idx], // mesh representation
    eind: &'a mut [Idx], // mesh repr

    /// The computational weights of the elements.
    ///
    /// Required size: ne
    vwgt: Option<&'a mut [Idx]>,

    /// The communication weights of the elements.
    ///
    /// Required size: ne
    vsize: Option<&'a mut [Idx]>,

    /// The target partition weights of the elements.
    ///
    /// If `None` then the mesh is equally divided among the partitions.
    ///
    /// Required size: nparts
    tpwgts: Option<&'a mut [Real]>,

    /// Fine-tuning parameters.
    options: [Idx; NOPTIONS],
}

impl<'a> Mesh<'a> {
    /// Creates a new [`Mesh`] object to be partitioned.
    ///
    /// # Panics
    ///
    /// This function panics if:
    /// - `nn` is not strictly greater than zero, or
    /// - `nparts` is not strictly greater than zero, or
    /// - `eptr` is empty, or
    /// - the length of `eind` is different than the last element of `eptr`, or
    /// - any of the arrays have a length that cannot be hold by an [`Idx`].
    ///
    /// # Mutability
    ///
    /// While nothing should be modified by the [`Mesh`] structure, METIS
    /// doesn't specify any `const` modifier, so everything must be mutable on
    /// Rust's side.
    pub fn new(nn: Idx, nparts: Idx, eptr: &'a mut [Idx], eind: &'a mut [Idx]) -> Mesh<'a> {
        assert!(0 < nn, "nn must be strictly greater than zero");
        assert!(0 < nparts, "nn must be strictly greater than zero");
        let _ = Idx::try_from(eptr.len()).expect("eptr array larger than Idx::MAX");
        assert_ne!(eptr.len(), 0);
        let eind_len = Idx::try_from(eind.len()).expect("eind array larger than Idx::MAX");
        assert_eq!(eind_len, *eptr.last().unwrap());

        Mesh {
            nn,
            nparts,
            ncommon: 1,
            eptr,
            eind,
            vwgt: None,
            vsize: None,
            tpwgts: None,
            options: [-1; NOPTIONS],
        }
    }

    /// Sets the computational weights of the elements.
    ///
    /// By default all elements have the same weight.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `vwgt` is not the number of
    /// elements.
    pub fn set_vwgt(mut self, vwgt: &'a mut [Idx]) -> Mesh<'a> {
        let vwgt_len = Idx::try_from(vwgt.len()).expect("vwgt array too large");
        assert_eq!(vwgt_len, self.eptr.len() as Idx - 1);
        self.vwgt = Some(vwgt);
        self
    }

    /// Sets the communication weights of the elements.
    ///
    /// By default all elements have the same communication weight.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `vsize` is not the number of
    /// elements.
    pub fn set_vsize(mut self, vsize: &'a mut [Idx]) -> Mesh<'a> {
        let vsize_len = Idx::try_from(vsize.len()).expect("vsize array too large");
        assert_eq!(vsize_len, self.eptr.len() as Idx - 1);
        self.vsize = Some(vsize);
        self
    }

    /// Sets the target partition weights for each part and constraint.
    ///
    /// By default the mesh is divided equally.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `tpwgts` is not equal to `nparts`.
    pub fn set_tpwgts(mut self, tpwgts: &'a mut [Real]) -> Mesh<'a> {
        let tpwgts_len = Idx::try_from(tpwgts.len()).expect("tpwgts array too large");
        assert_eq!(tpwgts_len, self.nparts);
        self.tpwgts = Some(tpwgts);
        self
    }

    /// Sets the fine-tuning parameters for this partitioning.
    ///
    /// When few options are to be set, [`Mesh::set_option`] might be a
    /// better fit.
    ///
    /// See the [option] module for the list of available parameters.  Note that
    /// not all are applicable to a given partitioning method.  Refer to the
    /// documentation of METIS ([link]) for more info on this.
    ///
    /// See [`Graph::set_options`] for a usage example.
    ///
    /// [link]: http://glaros.dtc.umn.edu/gkhome/fetch/sw/metis/manual.pdf
    pub fn set_options(mut self, options: &[Idx; NOPTIONS]) -> Mesh<'a> {
        self.options.copy_from_slice(options);
        self
    }

    /// Sets a fine-tuning parameter for this partitioning.
    ///
    /// When options are to be set in batches, [`Mesh::set_options`] might be a
    /// better fit.
    ///
    /// See the [option] module for the list of available parameters.  Note that
    /// not all are applicable to a given partitioning method.  Refer to the
    /// documentation of METIS ([link]) for more info on this.
    ///
    /// See [`Graph::set_option`] for a usage example.
    ///
    /// [link]: http://glaros.dtc.umn.edu/gkhome/fetch/sw/metis/manual.pdf
    pub fn set_option<O>(mut self, option: O) -> Mesh<'a>
    where
        O: option::Opt,
    {
        self.options[O::INDEX] = option.value();
        self
    }

    /// Partition the mesh using its dual graph.
    ///
    /// Returns the edge-cut, the total communication volume of the
    /// partitioning solution.
    ///
    /// Equivalent of `METIS_PartMeshDual`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `epart` is not the number of
    /// elements, or if `nparts`'s is not the number of nodes.
    pub fn part_dual(mut self, epart: &mut [Idx], npart: &mut [Idx]) -> Result<Idx> {
        let epart_len = Idx::try_from(epart.len()).expect("epart array larger than Idx::MAX");
        assert_eq!(
            epart_len,
            self.eptr.len() as Idx - 1,
            "epart.len() must be equal to the number of elements",
        );
        let npart_len = Idx::try_from(npart.len()).expect("npart array larger than Idx::MAX");
        assert_eq!(
            npart_len, self.nn,
            "npart.len() must be equal to the number of nodes",
        );

        let ne = &mut (self.eptr.len() as Idx - 1) as *mut Idx;
        let nn = &mut self.nn as *mut Idx;
        let vwgt = if let Some(vwgt) = self.vwgt {
            vwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let vsize = if let Some(vsize) = self.vsize {
            vsize.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let ncommon = &mut self.ncommon as *mut Idx;
        let nparts = &mut self.nparts as *mut Idx;
        let tpwgts = if let Some(tpwgts) = self.tpwgts {
            tpwgts.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let options = self.options.as_mut_ptr();
        let mut edgecut = mem::MaybeUninit::uninit();
        unsafe {
            m::METIS_PartMeshDual(
                ne,
                nn,
                self.eptr.as_mut_ptr(),
                self.eind.as_mut_ptr(),
                vwgt,
                vsize,
                ncommon,
                nparts,
                tpwgts,
                options,
                edgecut.as_mut_ptr(),
                epart.as_mut_ptr(),
                npart.as_mut_ptr(),
            )
            .wrap()?;
            Ok(edgecut.assume_init())
        }
    }

    /// Partition the mesh using its nodal graph.
    ///
    /// Returns the edge-cut, the total communication volume of the
    /// partitioning solution.
    ///
    /// Previous settings of `ncommon` are not used by this function.
    ///
    /// Equivalent of `METIS_PartMeshNodal`.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `epart` is not the number of
    /// elements, or if `nparts`'s is not the number of nodes.
    pub fn part_nodal(mut self, epart: &mut [Idx], npart: &mut [Idx]) -> Result<Idx> {
        let epart_len = Idx::try_from(epart.len()).expect("epart array larger than Idx::MAX");
        assert_eq!(
            epart_len,
            self.eptr.len() as Idx - 1,
            "epart.len() must be equal to the number of elements",
        );
        let npart_len = Idx::try_from(npart.len()).expect("npart array larger than Idx::MAX");
        assert_eq!(
            npart_len, self.nn,
            "npart.len() must be equal to the number of nodes",
        );

        let ne = &mut (self.eptr.len() as Idx - 1) as *mut Idx;
        let nn = &mut self.nn as *mut Idx;
        let vwgt = if let Some(vwgt) = self.vwgt {
            vwgt.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let vsize = if let Some(vsize) = self.vsize {
            vsize.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let nparts = &mut self.nparts as *mut Idx;
        let tpwgts = if let Some(tpwgts) = self.tpwgts {
            tpwgts.as_mut_ptr()
        } else {
            ptr::null_mut()
        };
        let options = self.options.as_mut_ptr();
        let mut edgecut = mem::MaybeUninit::uninit();
        unsafe {
            m::METIS_PartMeshNodal(
                ne,
                nn,
                self.eptr.as_mut_ptr(),
                self.eind.as_mut_ptr(),
                vwgt,
                vsize,
                nparts,
                tpwgts,
                options,
                edgecut.as_mut_ptr(),
                epart.as_mut_ptr(),
                npart.as_mut_ptr(),
            )
            .wrap()?;
            Ok(edgecut.assume_init())
        }
    }
}

/// The dual of a mesh.
///
/// Result of [`mesh_to_dual`].
#[derive(Debug, PartialEq, Eq)]
pub struct Dual {
    xadj: &'static mut [Idx],
    adjncy: &'static mut [Idx],
}

impl Dual {
    /// The adjacency index array.
    pub fn xadj(&self) -> &[Idx] {
        self.xadj
    }

    /// The adjacency array.
    pub fn adjncy(&self) -> &[Idx] {
        self.adjncy
    }

    /// The adjacency index array, and the adjacency array as mutable slices.
    pub fn as_mut(&mut self) -> (&mut [Idx], &mut [Idx]) {
        (self.xadj, self.adjncy)
    }
}

impl Drop for Dual {
    fn drop(&mut self) {
        unsafe {
            m::METIS_Free(self.xadj.as_mut_ptr() as *mut os::raw::c_void);
            m::METIS_Free(self.adjncy.as_mut_ptr() as *mut os::raw::c_void);
        }
    }
}

/// Generate the dual graph of a mesh.
///
/// # Panics
///
/// This function panics if:
///
/// - `eptr` is empty, or
/// - `eptr`'s length doesn't fit in [`Idx`].
pub fn mesh_to_dual(
    mut nn: Idx,
    eptr: &mut [Idx],
    eind: &mut [Idx],
    mut ncommon: Idx,
    mut numflag: Idx,
) -> Result<Dual> {
    let eptr_len = Idx::try_from(eptr.len()).expect("eptr array larger than Idx::MAX");
    assert_ne!(eptr_len, 0, "eptr cannot be empty");

    let ne = &mut (eptr_len - 1) as *mut Idx;
    let mut xadj = mem::MaybeUninit::uninit();
    let mut adjncy = mem::MaybeUninit::uninit();

    // SAFETY: METIS_MeshToDual allocates the xadj and adjncy arrays.
    // SAFETY: hopefully those arrays are of correct length.
    unsafe {
        m::METIS_MeshToDual(
            ne,
            &mut nn as *mut Idx,
            eptr.as_mut_ptr(),
            eind.as_mut_ptr(),
            &mut ncommon as *mut Idx,
            &mut numflag as *mut Idx,
            xadj.as_mut_ptr(),
            adjncy.as_mut_ptr(),
        )
        .wrap()?;
        let xadj = xadj.assume_init();
        let xadj = slice::from_raw_parts_mut(xadj, eptr.len());
        let adjncy = adjncy.assume_init();
        let adjncy = slice::from_raw_parts_mut(adjncy, xadj[xadj.len() - 1] as usize);
        Ok(Dual { xadj, adjncy })
    }
}
