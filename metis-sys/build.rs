#[cfg(all(not(feature = "vendored"), not(feature = "use-system")))]
compile_error!(r#"either "use-system" or "vendored" must be enabled for `metis-sys`"#);

#[cfg(feature = "vendored")]
const IDX_SIZE: usize = 32;

#[cfg(feature = "vendored")]
const REAL_SIZE: usize = 32;

#[cfg(feature = "vendored")]
fn build_lib() {
    use std::env;
    use std::path::PathBuf;

    let vendor = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor");
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
            .define("__thread", Some("__declspec(thread)"));

        if target.contains("msvc") {
            build
                .define("MSC", None)
                .define("_CRT_SECURE_NO_WARNINGS", None);

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

    #[cfg(any(not(debug_assertions), feature = "force-optimize-vendor"))]
    build.define("NDEBUG", None).define("NDEBUG2", None);

    #[cfg(feature = "force-optimize-vendor")]
    build.no_default_flags(true).opt_level(3).debug(false);

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
}

#[cfg(not(feature = "vendored"))]
fn build_lib() {
    println!("cargo:rustc-link-lib=metis");
}

// Always generate bindings when running from a locally installed METIS library.
// When building directly from source (feature = "vendored"), only regenerate
// bindings on command (feature = "generate-bindings").
#[cfg(all(
    feature = "use-system",
    any(not(feature = "vendored"), feature = "generate-bindings")
))]
fn generate_bindings() {
    use std::env;
    use std::path::PathBuf;

    #[cfg(feature = "vendored")]
    let builder = bindgen::builder()
        .clang_arg(format!("-DIDXTYPEWIDTH={}", IDX_SIZE))
        .clang_arg(format!("-DREALTYPEWIDTH={}", REAL_SIZE))
        .header("../vendor/metis/include/metis.h");

    #[cfg(not(feature = "vendored"))]
    let builder = bindgen::builder().header("wrapper.h");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = builder
        .allowlist_function("METIS_.*")
        .allowlist_type("idx_t")
        .allowlist_type("real_t")
        .allowlist_type("rstatus_et")
        .allowlist_type("m.*_et")
        .allowlist_var("METIS_.*")
        .generate()
        .unwrap_or_else(|err| {
            eprintln!("Failed to generate bindings to METIS: {err}");
            std::process::exit(1);
        });

    let out_path = if cfg!(feature = "vendored") {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("gen/bindings.rs")
    } else {
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs")
    };

    bindings.write_to_file(&out_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to write METIS bindings to {:?}: {}",
            out_path.display(),
            err,
        );
        std::process::exit(1);
    });
}

#[cfg(not(all(
    feature = "use-system",
    any(not(feature = "vendored"), feature = "generate-bindings")
)))]
fn generate_bindings() {}

fn main() {
    build_lib();
    generate_bindings();
}
