const { decode_opa_bundle } = require("opa-bundle-decode-wasm");
const fs = require("fs");
const path = require("path");

(async function () {
  const bData = await fs.readFileSync(path.join(__dirname, "..", "..", "_fixtures_", "bundle.tar.gz"));

  console.time("decode");
  const decoded = await decode_opa_bundle(bData);
  console.timeEnd("decode");
  console.log(decoded);
})();
