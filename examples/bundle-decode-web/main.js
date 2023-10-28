import { default as wasm, decode_opa_bundle } from "opa-bundle-decode-wasm";

wasm().then(async (module) => { 
  const resp = await fetch("bundle", {
  }).catch((err) => {
    throw new Error(err);
  });
  let bData = await resp.arrayBuffer();
  bData = new Uint8Array(bData);

  console.time("decode");
  const decoded = await decode_opa_bundle(bData);
  console.timeEnd("decode");
  console.log(decoded);
});
