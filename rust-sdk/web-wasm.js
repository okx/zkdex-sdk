const fs = require('fs');

const wasmFile = './dist/zkdex-web_bg.wasm';
const jsFile = './dist/zkdex-web.js';
// The output of wasm2js with a fixed import.
const asmJsFile = './zkdex-bundler_asm.js';

const wasmData = fs.readFileSync(wasmFile);

// Strings that are inserted automatically by wasm-pack, but
// break library in it's current implementation
const brokenStrings = [
    // This substring is unique, had to
    // write only part of line to make the RegExp works.
    // Probably will rewrite in the future
    `input = import.meta.url.replace`,
    `input = new URL`
];

let jsCode = fs.readFileSync(jsFile).toString();

// Commenting out broken strings
brokenStrings.forEach((str) => {
    jsCode = jsCode.replace(new RegExp(str, 'g'), '// ' + str);
});

jsCode += `
export let base64WasmCode = \`${wasmData.toString('base64')}\`;

function base64ToArrayBuffer(base64) {
  const binaryString = window.atob(base64);
  const length = binaryString.length;
  const bytes = new Uint8Array(length);

  for (let i = 0; i < length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes.buffer;
}

const wasmBytes = base64ToArrayBuffer(base64WasmCode);

const wasmResponseInit = {
  "status" : 200 ,
  "statusText" : "ok.",
  headers: {
    'Content-Type': 'application/wasm',
    'Content-Length': wasmBytes.length
  }
};

export function wasmSupported() {
  try {
    if (typeof WebAssembly === 'object') {
      return true;
    }
  } catch (e) {
  }
  return false;
}

export async function loadZKDEXCrypto(wasmFileUrl) {
  if (!wasmSupported()) {
    // Use the bundler build.
    return require(\'${asmJsFile}\');
  }
  if (!wasmFileUrl) {
    const wasmResponse = new Response(wasmBytes, wasmResponseInit);
    await init(wasmResponse);
  } else {
    await init(DefaultZksyncCryptoWasmURL);
  }
}
`;

fs.writeFileSync(jsFile, jsCode);
