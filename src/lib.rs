mod utils;
use std::io::Cursor;
use std::io::Read;

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use utils::set_panic_hook;
use std::convert::TryFrom;
use tar::Archive;
use wasm_bindgen::prelude::*;

// https://github.com/open-policy-agent/opa/blob/main/bundle/bundle.go#L32
const BUNDLE_SIZE_LIMIT: usize = 1024 * 1024 * 128; // will limit bundle size to 128mb

#[derive(Debug)]
enum BundleFile {
    RegoExt,
    WasmFile,
    DataFile,
    YamlDataFile,
    YmlDataFile,
}

impl BundleFile {
    fn to_value(&self) -> &str {
        match *self {
            BundleFile::DataFile => "data.json",
            BundleFile::RegoExt => ".rego",
            BundleFile::WasmFile => "policy.wasm",
            BundleFile::YamlDataFile => "data.yaml",
            BundleFile::YmlDataFile => "data.yml",
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Bundle {
    pub data: String,
    pub wasm_module: Vec<u8>,
}

#[wasm_bindgen]
pub async fn decode_opa_bundle(data: &[u8]) -> Result<JsValue, JsError> {
    set_panic_hook();

    if data.len() > BUNDLE_SIZE_LIMIT {
        return Err(JsError::new("maximum bundle size limit is 128mb"));
    }

    let data = Cursor::new(data);
    let tar = GzDecoder::new(data);
    let mut archive = Archive::new(tar);

    let mut decoded_bundle = Bundle::default();

    archive
        .entries()
        .expect("cannot find entries")
        .for_each(|file| {
            let mut file = file.expect("unable to get entry");
            let size = file.header().size().expect("corrupted file header");
            let size = usize::try_from(size).expect("unable to convert size");

            let mut buf = vec![0; size];
            file.read_exact(&mut buf).expect("read failed");
            let path = file.header().path().unwrap();

            match path.file_name().unwrap().to_str().unwrap() {
                // TODO: handle bundle cases with multiple data files and or policies
                path if path == BundleFile::YamlDataFile.to_value() => {
                    decoded_bundle.data = String::from_utf8_lossy(&buf).into_owned()
                }
                path if path == BundleFile::YmlDataFile.to_value() => {
                    decoded_bundle.data = String::from_utf8_lossy(&buf).into_owned()
                }
                path if path == BundleFile::DataFile.to_value() => {
                    decoded_bundle.data = String::from_utf8_lossy(&buf).into_owned()
                }
                path if path == BundleFile::WasmFile.to_value() => decoded_bundle.wasm_module = buf,
                _ => {},
            }
        });

    Ok(serde_wasm_bindgen::to_value(&decoded_bundle).unwrap())
}
