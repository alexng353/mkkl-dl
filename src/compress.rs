use zip;

use crate::utils::zipdir::doit;

const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[cfg(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib"
))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(any(
    feature = "deflate",
    feature = "deflate-miniz",
    feature = "deflate-zlib"
)))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2: Option<zip::CompressionMethod> = None;

#[cfg(feature = "zstd")]
const METHOD_ZSTD: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Zstd);
#[cfg(not(feature = "zstd"))]
const METHOD_ZSTD: Option<zip::CompressionMethod> = None;

pub(crate) async fn compress(src_dir: &str, dst_file: &str) {
    // read the path, and get all the folders

    // let args: Vec<_> = std::env::args().collect();
    // if args.len() < 3 {
    //     println!(
    //         "Usage: {} <source_directory> <destination_zipfile>",
    //         args[0]
    //     );
    //     return;
    // }

    // let src_dir = &*args[2];
    // let dst_file = &*args[3];
    // println!("src_dir: {}", src_dir);
    // println!("dst_file: {}", dst_file);

    // get os

    for &method in [METHOD_STORED, METHOD_DEFLATED, METHOD_BZIP2, METHOD_ZSTD].iter() {
        if method.is_none() {
            continue;
        }
        match doit(src_dir, dst_file, method.unwrap()) {
            Ok(_) => println!("done: {} written to {}", src_dir, dst_file),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
