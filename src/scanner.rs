use std::env;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
pub fn print_dir_name() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}


pub fn scan_directory() -> std :: io :: Result<()>{
    let dir_iter = std :: fs :: read_dir(std :: env :: current_dir()?)?;
    for entry in dir_iter {
        let entry = entry?;
        let path = entry.path();
        let entry_name = entry.file_name();
        let ext = Path :: new(&entry_name).extension();

        if path.is_dir() {
            println!("Folder Name: {}", entry_name.display());
        }
        else {
            print!("File Name: {}", entry_name.display());
            match ext {
                Some(ext) => {println!("    |   Extension: {}", ext.display());}
                None => {}
            }
        }
    }

    Ok(())
}

pub fn categorize_entries() -> std :: io :: Result<()> {
    let dir_iter = std :: fs :: read_dir(std :: env :: current_dir()?)?;

    let mut extension_map: HashMap<&str, &str> = HashMap::new();
    let categories = [
        // Images
        ("Photos", [
            "jpg", "jpeg", "jpe", "jfif", "pjpeg", "pjp",
            "png", "gif", "webp", "bmp", "dib", "tif", "tiff",
            "svg", "svgz", "ico", "cur", "icns", "heic", "heif",
            "avif", "jxl", "apng", "raw", "cr2", "cr3", "nef",
            "nrw", "arw", "srf", "sr2", "dng", "orf", "rw2",
            "raf", "pef", "srw", "x3f", "psd", "psb", "xcf",
            "ai", "eps", "emf", "wmf", "exr", "hdr", "dds",
            "tga", "pcx", "ppm", "pgm", "pbm", "pnm", "fits",
            "jp2", "j2k", "jpf", "jpx", "jpm", "mj2",
            "bpg", "qoi", "jng", "mng", "pict", "pct"
        ].as_slice()),

        // Audio
        ("Audio", [
            "mp3", "wav", "wave", "flac", "aac", "ogg", "oga",
            "m4a", "m4b", "m4p", "wma", "aiff", "aif", "aifc",
            "alac", "opus", "amr", "awb", "ac3", "eac3",
            "dts", "mka", "mid", "midi", "kar", "rmi",
            "ape", "wv", "tta", "tak", "mpc", "mpp", "mp+",
            "au", "snd", "voc", "vox", "gsm", "ra", "ram",
            "rm", "spx", "caf", "dsf", "dff", "pcm",
            "raw", "8svx", "cda", "mod", "xm", "it", "s3m",
            "mt2", "umx", "sf2", "sfz", "vstpreset"
        ].as_slice()),

        // Videos
        ("Videos", [
            "mp4", "m4v", "mkv", "avi", "mov", "qt", "webm",
            "wmv", "flv", "f4v", "f4p", "f4a", "f4b",
            "mpeg", "mpg", "mpe", "mpv", "m1v", "m2v",
            "m2ts", "mts", "ts", "m2t", "vob", "ogv", "ogg",
            "3gp", "3g2", "asf", "rm", "rmvb", "divx",
            "xvid", "mxf", "roq", "nsv", "yuv", "y4m",
            "amv", "drc", "gifv", "mjpeg", "mjpg",
            "hevc", "h264", "h265", "av1", "ivf",
            "wtv", "dvr-ms", "bik", "bk2", "smk"
        ].as_slice()),

        // Documents
        ("Documents", [
            "pdf", "doc", "docx", "docm", "dot", "dotx",
            "dotm", "txt", "rtf", "odt", "ott", "tex",
            "latex", "ltx", "bib", "md", "markdown", "rst",
            "wpd", "wps", "pages", "abw", "zabw",
            "ppt", "pptx", "pptm", "pps", "ppsx", "ppsm",
            "pot", "potx", "potm", "odp", "otp",
            "xls", "xlsx", "xlsm", "xlsb", "xlsb",
            "xlt", "xltx", "xltm", "ods", "ots",
            "csv", "tsv", "tab", "dif", "sylk",
            "numbers", "key", "keynote", "one", "onepkg",
            "onetoc2", "vsd", "vsdx", "vss", "vssx",
            "vst", "vstx", "odg", "odf", "fodt",
            "fods", "fodp", "fodg", "pages"
        ].as_slice()),

        // Programming and source code
        ("Code", [
            "rs", "c", "h", "cpp", "cc", "cxx", "hpp",
            "hh", "hxx", "m", "mm", "cs", "csx",
            "java", "class", "jar", "py", "pyw", "pyi",
            "pyc", "pyo", "pyd", "js", "jsx", "mjs",
            "cjs", "ts", "tsx", "mts", "cts",
            "html", "htm", "xhtml", "css", "scss",
            "sass", "less", "styl", "json", "jsonc",
            "json5", "xml", "xsd", "xsl", "xslt",
            "yaml", "yml", "toml", "ini", "cfg",
            "conf", "properties", "env", "sh", "bash",
            "zsh", "fish", "bat", "cmd", "ps1",
            "psm1", "psd1", "php", "phtml", "phar",
            "rb", "erb", "rake", "go", "mod", "sum",
            "swift", "kt", "kts", "dart", "scala",
            "sc", "groovy", "gradle", "clj", "cljs",
            "cljc", "edn", "ex", "exs", "erl", "hrl",
            "hs", "lhs", "elm", "pl", "pm", "t",
            "lua", "r", "rmd", "jl", "ipynb",
            "sql", "prisma", "graphql", "gql",
            "proto", "thrift", "avsc", "fbs",
            "sol", "vy", "move", "v", "sv", "vh",
            "vhd", "vhdl", "asm", "s", "S",
            "wasm", "wat", "wast", "zig", "nim",
            "nims", "cr", "d", "di", "f", "f90",
            "f95", "f03", "for", "ftn", "cob",
            "cbl", "adb", "ads", "pas", "pp",
            "p", "pro", "lisp", "lsp", "scm",
            "ss", "rkt", "ml", "mli", "re",
            "rei", "fs", "fsi", "fsx", "exs",
            "vue", "svelte", "astro", "graphqls",
            "make", "mk", "cmake", "ninja",
            "gradle", "pom", "sln", "csproj",
            "vbproj", "fsproj", "vcxproj",
            "xcodeproj", "xcworkspace"
        ].as_slice()),

        // Compressed and archive files
        ("Archives", [
            "zip", "zipx", "rar", "7z", "tar",
            "gz", "gzip", "bz", "bz2", "bzip2",
            "xz", "lz", "lzma", "z", "zst",
            "zstd", "lz4", "br", "brotli",
            "tgz", "tbz", "tbz2", "txz", "tlz",
            "taz", "tzst", "tz", "jar", "war",
            "ear", "apk", "aab", "ipa", "cab",
            "arj", "ace", "alz", "arc", "ark",
            "car", "dar", "lha", "lzh", "lzx",
            "pak", "partimg", "pea", "pax",
            "paq", "paq8", "qpress", "shar",
            "sh", "sit", "sitx", "sqx", "tar.gz",
            "tar.bz2", "tar.xz", "tar.zst",
            "wim", "esd", "xar", "cpio",
            "deb", "rpm", "dmg", "iso"
        ].as_slice()),

        // Executable and application packages
        ("Applications", [
            "exe", "msi", "msp", "mst", "com",
            "scr", "bat", "cmd", "app", "appx",
            "appxbundle", "msix", "msixbundle",
            "apk", "aab", "xapk", "apks",
            "ipa", "deb", "udeb", "rpm",
            "dmg", "pkg", "mpkg", "flatpak",
            "snap", "appimage", "run", "bin",
            "sh", "elf", "out", "so", "dll",
            "dylib", "sys", "drv", "ocx",
            "ax", "cpl", "gadget", "hta",
            "jar", "jnlp", "war", "ear",
            "wasm", "vbs", "vbe", "js",
            "jse", "wsf", "wsh", "ps1"
        ].as_slice()),

        // Database files
        ("Databases", [
            "db", "db3", "sqlite", "sqlite3",
            "s3db", "sl3", "mdb", "accdb",
            "accde", "accdr", "accdt",
            "mdf", "ldf", "ndf", "bak",
            "dbf", "fdb", "gdb", "ib",
            "ibd", "frm", "myd", "myi",
            "ibdata", "ibdata1", "rdb",
            "redis", "aof", "bson", "realm",
            "pdb", "sdf", "edb", "nsf",
            "ntf", "ora", "dmp", "dump",
            "sql", "pgsql", "parquet", "orc",
            "avro", "arrow", "feather",
            "h5", "hdf5", "hdf", "rds",
            "rda", "rdata", "mat"
        ].as_slice()),

        // Configuration and system files
        ("Configuration", [
            "ini", "cfg", "conf", "config",
            "toml", "yaml", "yml", "json",
            "jsonc", "xml", "env", "properties",
            "plist", "reg", "desktop", "service",
            "socket", "target", "mount", "timer",
            "path", "automount", "network",
            "netdev", "rules", "policy", "manifest",
            "lock", "editorconfig", "gitignore",
            "gitattributes", "gitmodules",
            "npmrc", "nvmrc", "yarnrc",
            "browserslistrc", "babelrc", "eslintrc",
            "prettierrc", "dockerignore",
            "dockerfile", "makefile", "vagrantfile",
            "procfile", "gemfile", "rakefile",
            "gradle", "htaccess", "htpasswd",
            "npmignore", "npm-shrinkwrap"
        ].as_slice()),

        // Fonts
        ("Fonts", [
            "ttf", "otf", "woff", "woff2",
            "eot", "fon", "fnt", "pfa",
            "pfb", "afm", "pfm", "sfd",
            "bdf", "pcf", "snf", "dfont",
            "suit", "lwfn", "gxf", "cff",
            "vfb", "ufo", "designspace",
            "glyphs", "vfc"
        ].as_slice()),

        // 3D models and scenes
        ("3D Models", [
            "obj", "fbx", "stl", "gltf",
            "glb", "usd", "usda", "usdc",
            "usdz", "blend", "ma", "mb",
            "max", "3ds", "dae", "collada",
            "abc", "alembic", "ply", "off",
            "3mf", "amf", "wrl", "vrml",
            "x3d", "iges", "igs", "step",
            "stp", "sat", "sab", "skp",
            "c4d", "lwo", "lws", "x",
            "mesh", "msh", "vdb", "vox",
            "unity", "unitypackage", "uasset",
            "umap", "godot", "tscn", "tres",
            "gltf", "material", "mtl"
        ].as_slice()),

        // CAD and engineering
        ("CAD", [
            "dwg", "dxf", "dwt", "dws",
            "dgn", "rvt", "rfa", "rte",
            "rft", "ifc", "ifczip", "ifcxml",
            "step", "stp", "iges", "igs",
            "sat", "sab", "sldprt", "sldasm",
            "slddrw", "prt", "asm", "drw",
            "ipt", "iam", "idw", "f3d",
            "f3z", "3dm", "skp", "pln",
            "pla", "mod", "fcstd", "brd",
            "sch", "pcb", "gbr", "gerber",
            "gcode", "nc", "tap", "drl",
            "stl", "3mf", "amf"
        ].as_slice()),

        // Design and publishing
        ("Design", [
            "psd", "psb", "ai", "eps",
            "indd", "indt", "idml", "afdesign",
            "afphoto", "afpub", "sketch",
            "fig", "xd", "xcf", "kra",
            "ora", "clip", "sai", "sai2",
            "procreate", "cdr", "cmx", "cpt",
            "vsd", "vsdx", "drawio", "excalidraw",
            "canva", "pub", "qxd", "qxp",
            "pmd", "sla", "svg", "pdf"
        ].as_slice()),

        // Ebooks and digital publications
        ("Ebooks", [
            "epub", "mobi", "azw", "azw3",
            "azw4", "kfx", "prc", "pdb",
            "fb2", "fb3", "lit", "lrf",
            "lrx", "pml", "pmlz", "rb",
            "tcr", "djvu", "djv", "cbz",
            "cbr", "cb7", "cbt", "cba",
            "opf", "ncx", "acsm", "ibooks"
        ].as_slice()),

        // Email and messaging
        ("Email", [
            "eml", "msg", "oft", "pst",
            "ost", "mbox", "mbx", "emlx",
            "ics", "ical", "ifb", "vcf",
            "vcard", "contact", "wab",
            "olk", "dbx", "msf", "wdseml"
        ].as_slice()),

        // Backup and recovery
        ("Backups", [
            "bak", "backup", "old", "orig",
            "tmp", "temp", "bkp", "bup",
            "001", "002", "003", "incremental",
            "full", "dmp", "snapshot",
            "vhd", "vhdx", "vmdk", "vdi",
            "qcow", "qcow2", "ova", "ovf",
            "wbcat", "bkf", "tib", "tibx",
            "mrimg", "gho", "ghs", "adi",
            "ab", "crypt14", "crypt15"
        ].as_slice()),

        // Disk images and virtual drives
        ("Disk Images", [
            "iso", "img", "ima", "dmg",
            "sparseimage", "sparsebundle",
            "vhd", "vhdx", "vmdk", "vdi",
            "hdd", "qcow", "qcow2", "vfd",
            "wim", "esd", "swm", "bin",
            "cue", "nrg", "mdf", "mds",
            "ccd", "sub", "isz", "daa",
            "cso", "ciso", "udf", "toast",
            "vpk", "xci", "nsp", "gdi",
            "cdi", "chd", "wbfs"
        ].as_slice()),

        // Scientific and research data
        ("Scientific Data", [
            "fits", "fit", "fts", "hdf",
            "hdf4", "hdf5", "nc", "netcdf",
            "grib", "grb", "grb2", "cdf",
            "mat", "mat4", "mat5", "mat7",
            "npy", "npz", "pkl", "pickle",
            "sav", "por", "dta", "sas7bdat",
            "rds", "rdata", "rda", "feather",
            "parquet", "arrow", "zarr", "root",
            "h5ad", "loom", "vcf", "bcf",
            "bam", "sam", "cram", "fasta",
            "fa", "fna", "faa", "fastq",
            "fq", "gb", "gbk", "gff",
            "gff3", "bed", "wig", "bigwig",
            "bw", "cool", "hic"
        ].as_slice()),

        // Geographic and mapping data
        ("GIS", [
            "shp", "shx", "dbf", "prj",
            "geojson", "json", "topojson",
            "kml", "kmz", "gpx", "gpx",
            "gml", "gdb", "gpkg", "sqlite",
            "mbtiles", "pbf", "osm", "dem",
            "tif", "tiff", "vrt", "asc",
            "grd", "img", "jp2", "ecw",
            "sid", "las", "laz", "e57",
            "wkt", "wkb", "fgb", "flatgeobuf"
        ].as_slice()),

        // Subtitles and transcripts
        ("Subtitles", [
            "srt", "vtt", "ass", "ssa",
            "sub", "idx", "smi", "sami",
            "scc", "ttml", "dfxp", "stl",
            "sbv", "lrc", "cap", "usf",
            "aqt", "jss", "pjs", "rt"
        ].as_slice()),

        // Game assets and save files
        ("Game Files", [
            "sav", "save", "dat", "pak",
            "wad", "bsp", "map", "dem",
            "mdl", "md2", "md3", "md5",
            "vpk", "gcf", "ncf", "acf",
            "uasset", "umap", "unity3d",
            "assetbundle", "assets", "bundle",
            "bank", "fsb", "wem", "bnk",
            "locres", "strings", "resx",
            "resource", "resources"
        ].as_slice()),

        // Logs and reports
        ("Logs", [
            "log", "logs", "out", "err",
            "trace", "dump", "stacktrace",
            "crash", "dmp", "core", "journal",
            "audit", "report", "stats",
            "csv", "tsv", "txt"
        ].as_slice()),

        // Certificates and security
        ("Security", [
            "pem", "crt", "cer", "der",
            "p7b", "p7c", "p7s", "p12",
            "pfx", "key", "pub", "asc",
            "gpg", "pgp", "sig", "csr",
            "jks", "keystore", "truststore",
            "kdb", "spc", "sst", "crl",
            "kbx", "gpg-agent-info"
        ].as_slice()),

        // Web and internet resources
        ("Web", [
            "html", "htm", "xhtml", "css",
            "js", "mjs", "cjs", "ts",
            "jsx", "tsx", "wasm", "map",
            "webmanifest", "manifest", "appcache",
            "sw", "serviceworker", "htaccess",
            "htpasswd", "robots", "sitemap",
            "rss", "atom", "xml", "xsl",
            "xslt", "woff", "woff2", "eot",
            "ttf", "otf", "webp", "avif"
        ].as_slice()),

        // Virtual machines and containers
        ("Virtualization", [
            "ova", "ovf", "vmdk", "vdi",
            "vhd", "vhdx", "qcow", "qcow2",
            "vbox", "vbox-prev", "vbox-extpack",
            "vmx", "vmsd", "vmsn", "vmss",
            "vmsw", "nvram", "vma", "vma.zst",
            "dockerfile", "containerfile",
            "tar", "oci", "sif", "simg",
            "box", "wsl", "wslconfig"
        ].as_slice()),

        // Machine learning and AI
        ("AI and Machine Learning", [
            "onnx", "pt", "pth", "pkl",
            "pickle", "h5", "hdf5", "keras",
            "pb", "pbtxt", "tflite", "lite",
            "mlmodel", "mlpackage", "safetensors",
            "gguf", "ggml", "model", "ckpt",
            "checkpoint", "index", "meta",
            "data", "params", "weights",
            "engine", "plan", "trt",
            "npy", "npz", "joblib"
        ].as_slice()),

        // Temporary and cache files
        ("Temporary and Cache", [
            "tmp", "temp", "cache", "bak",
            "old", "swp", "swo", "swn",
            "lock", "lck", "part", "crdownload",
            "download", "partial", "cachedata",
            "dmp", "thumb", "thumbs",
            "ds_store", "spotlight-v100"
        ].as_slice()),

        // Miscellaneous
        ("Miscellaneous", [
            "dat", "bin", "data", "blob",
            "raw", "unknown", "file", "res",
            "resource", "resources", "pack",
            "index", "meta", "metadata",
            "manifest", "checksum", "md5",
            "sha1", "sha256", "sha512",
            "sum", "sig", "patch", "diff",
            "delta", "torrent", "nfo", "sfv",
            "cue", "url", "webloc", "lnk",
            "desktop", "shortcut"
        ].as_slice()),
    ];

    for (category, extensions) in categories {
        for extension in extensions {
            extension_map.insert(extension, category);
        }
    }
    let mut categories : HashMap<String, Vec<String>> = HashMap :: new();
    for entry in dir_iter {
        let entry = entry?;
        let path = entry.path();
        let entry_name = entry.file_name().into_string().unwrap();
        let ext = Path :: new(&entry_name).extension();
        if path.is_dir() {
            categories.entry("Folders".to_string()).or_default().push(entry_name);
        }
        else {
            match ext {
                Some(ext) => {
                    let category: &str = extension_map
                        .get(ext.to_str().unwrap())
                        .copied()
                        .unwrap_or("Others");
                    categories.entry(category.to_string()).or_default().push(entry_name);
                }
                None =>{
                    categories.entry("Others".to_string()).or_default().push(entry_name);
                }
            }
        }

    }
    // Sorting before displaying

    let mut sorted_categories : Vec<_> = categories.iter().collect();
    sorted_categories.sort_by_key(|(category, _)| *category);
    for (category, entries) in sorted_categories {
        println!("========================================================================================================================");
        println!("{}", category.to_ascii_uppercase());
        println!("========================================================================================================================");
        println!();
        let mut i = 1i32;
        for entry in entries {
            println!("{}. {}", i, entry);
            i += 1;
        }
        println!();
    }

    Ok(())
}