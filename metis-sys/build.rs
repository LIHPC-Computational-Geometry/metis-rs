use std::env;
use std::path::PathBuf;
use std::process;

#[cfg(feature = "vendored")]
fn build_lib() -> bindgen::Builder {
    const IDX_SIZE: usize = 32;
    const REAL_SIZE: usize = 32;

    let vendor = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../vendor");
    println!("cargo:rerun-if-changed={}", vendor.display());

    let mut build = cc::Build::new();
    build
        .define("IDXTYPEWIDTH", Some(IDX_SIZE.to_string().as_str()))
        .define("REALTYPEWIDTH", Some(REAL_SIZE.to_string().as_str()))
        .include(vendor.join("metis/include"));

    fn add_sources(build: &mut cc::Build, root: PathBuf, files: &[&str]) {
        build.files(files.iter().map(|src| root.join(src)));
        build.include(root);
    }

    add_sources(
        &mut build,
        vendor.join("metis/libmetis"),
        &[
            "auxapi.c",
            "balance.c",
            "bucketsort.c",
            "checkgraph.c",
            "coarsen.c",
            "compress.c",
            "contig.c",
            "debug.c",
            "fm.c",
            "fortran.c",
            "frename.c",
            "gklib.c",
            "graph.c",
            "initpart.c",
            "kmetis.c",
            "kwayfm.c",
            "kwayrefine.c",
            "mcutil.c",
            "mesh.c",
            "meshpart.c",
            "minconn.c",
            "mincover.c",
            "mmd.c",
            "ometis.c",
            "options.c",
            "parmetis.c",
            "pmetis.c",
            "refine.c",
            "separator.c",
            "sfm.c",
            "srefine.c",
            "stat.c",
            "timing.c",
            "util.c",
            "wspace.c",
        ],
    );

    add_sources(
        &mut build,
        vendor.join("GKlib"),
        &[
            "b64.c",
            "blas.c",
            "cache.c",
            "csr.c",
            "error.c",
            "evaluate.c",
            "fkvkselect.c",
            "fs.c",
            "getopt.c",
            "gk_util.c",
            "gkregex.c",
            "graph.c",
            "htable.c",
            "io.c",
            "itemsets.c",
            "mcore.c",
            "memory.c",
            "pqueue.c",
            "random.c",
            "rw.c",
            "seq.c",
            "sort.c",
            "string.c",
            "timers.c",
            "tokenizer.c",
        ],
    );

    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        add_sources(&mut build, vendor.join("GKlib/win32"), &["adapt.c"]);

        build
            .define("USE_GKREGEX", None)
            .define("WIN32", None)
            .define("__thread", Some("__declspec(thread)"))
            // .define("NDEBUG", None)
            // .define("NDEBUG2", None)
            ;

        build.opt_level(2);

        if target.contains("msvc") {
            build
                .define("MSC", None)
                .define("_CRT_SECURE_NO_WARNINGS", None)
                .flag("/Ob2");

            // force inclusion of math.h to make sure INFINITY is defined before gk_arch.h is parsed
            build.flag("/FImath.h");
        }
    } else if target.contains("linux") {
        build
            .define("LINUX", None)
            .define("_FILE_OFFSET_BITS", Some("64"));
    } else if target.contains("apple") {
        build.define("MACOS", None);
    }

    #[cfg(not(debug_assertions))]
    build.flag("-DNDEBUG").flag("-DNDEBUG2");

    // METIS triggers an infinite amount of warnings and showing them to users
    // downstream does not really help.
    build.warnings(false);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_dir = out_dir.join("lib");

    build.out_dir(&lib_dir);
    build.compile("metis");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=metis");
    println!("cargo:lib={}", lib_dir.display());
    println!("cargo:out={}", out_dir.display());

    bindgen::builder()
        .clang_arg(format!("-DIDXTYPEWIDTH={}", IDX_SIZE))
        .clang_arg(format!("-DREALTYPEWIDTH={}", REAL_SIZE))
        .header("../vendor/metis/include/metis.h")
}

#[cfg(not(feature = "vendored"))]
fn build_lib() -> bindgen::Builder {
    println!("cargo:rustc-link-lib=metis");
    bindgen::builder().header("wrapper.h")
}

fn main() {
    let bindings_builder = build_lib();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindings_builder
        .allowlist_function("METIS_.*")
        .allowlist_type("idx_t")
        .allowlist_type("real_t")
        .allowlist_type("rstatus_et")
        .allowlist_type("m.*_et")
        .allowlist_var("METIS_.*")
        .generate()
        .unwrap_or_else(|err| {
            eprintln!("Failed to generate bindings to METIS: {err}");
            process::exit(1);
        });

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(&out_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to write METIS bindings to {:?}: {}",
            out_path.display(),
            err,
        );
        process::exit(1);
    });
}
