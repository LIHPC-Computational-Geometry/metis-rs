/* Include metis, but add const modifiers to pointer arguments where relevant,
 * so that the generated Rust bindings can use shared references.
 * As far as I can tell, METIS does not modify these, or otherwise does not
 * require exclusive access to the data behind those pointers.
 */

#define METIS_MeshToDual rs_METIS_MeshToDual
#define METIS_PartGraphKway rs_METIS_PartGraphKway
#define METIS_PartGraphRecursive rs_METIS_PartGraphRecursive
#define METIS_PartMeshDual rs_METIS_PartMeshDual
#define METIS_PartMeshNodal rs_METIS_PartMeshNodal

#include <metis.h>

#undef METIS_MeshToDual
#undef METIS_PartGraphKway
#undef METIS_PartGraphRecursive
#undef METIS_PartMeshDual
#undef METIS_PartMeshNodal

#ifdef __cplusplus
extern "C" {
#endif

METIS_API(int)
METIS_MeshToDual(const idx_t *ne, const idx_t *nn, const idx_t *eptr,
                 const idx_t *eind, const idx_t *ncommon, const idx_t *numflag,
                 idx_t **r_xadj, idx_t **r_adjncy);

METIS_API(int)
METIS_PartGraphKway(const idx_t *nvtxs, const idx_t *ncon, const idx_t *xadj,
                    const idx_t *adjncy, const idx_t *vwgt, const idx_t *vsize,
                    const idx_t *adjwgt, const idx_t *nparts,
                    const real_t *tpwgts, const real_t *ubvec,
                    const idx_t *options, idx_t *edgecut, idx_t *part);

METIS_API(int)
METIS_PartGraphRecursive(const idx_t *nvtxs, const idx_t *ncon,
                         const idx_t *xadj, const idx_t *adjncy,
                         const idx_t *vwgt, const idx_t *vsize,
                         const idx_t *adjwgt, const idx_t *nparts,
                         const real_t *tpwgts, const real_t *ubvec,
                         const idx_t *options, idx_t *edgecut, idx_t *part);

METIS_API(int)
METIS_PartMeshDual(const idx_t *ne, const idx_t *nn, const idx_t *eptr,
                   const idx_t *eind, const idx_t *vwgt, const idx_t *vsize,
                   const idx_t *ncommon, const idx_t *nparts,
                   const real_t *tpwgts, const idx_t *options, idx_t *objval,
                   idx_t *epart, idx_t *npart);

METIS_API(int)
METIS_PartMeshNodal(const idx_t *ne, const idx_t *nn, const idx_t *eptr,
                    const idx_t *eind, const idx_t *vwgt, const idx_t *vsize,
                    const idx_t *nparts, const real_t *tpwgts,
                    const idx_t *options, idx_t *objval, idx_t *epart,
                    idx_t *npart);

#ifdef __cplusplus
}
#endif
