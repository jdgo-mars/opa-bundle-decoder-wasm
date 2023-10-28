// TODO:
// Test file limit
// Test invalid format
// Test yaml 
// Test yml

#[cfg(test)]
mod tests {

use opa_bundle_decoder_wasm::{decode_opa_bundle, Bundle};
use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn decode_opa_bundle_works() -> Result<(), wasm_bindgen::JsValue> {
        let bundle = include_bytes!("../_fixtures_/bundle.tar.gz");
        let wasm_policy = include_bytes!("../_fixtures_/policy.wasm");
        let data = include_bytes!("../_fixtures_/data.json");

        let response = decode_opa_bundle(bundle).await?;
        let response: Bundle = serde_wasm_bindgen::from_value(response).map_err(|e| e.to_string())?;

        assert_eq!(String::from_utf8_lossy(data), response.data);
        assert_eq!(wasm_policy.to_vec(), response.wasm_module);

        Ok(())
    }
}
