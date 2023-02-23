const PATH = './target/wasm32-unknown-unknown/debug/threechess.wasm';
var exports;

const get_string = (ptr, len) => {
  let arr = new Uint8Array(exports.memory.buffer);
  let str = '';
  for (let i = 0; i < len; i++) {
    str += String.fromCharCode(arr[ptr + i]);
  }
  return str;
}

WebAssembly.instantiateStreaming(fetch(PATH), {
  'env': {
    '_log': (ptr, len) => {
      console.log(get_string(ptr, len));
    },
    '_elog': (ptr, len) => {
      console.error(get_string(ptr, len));
    }
  }
}).then(x => {
  exports = x.instance.exports;
  exports.main();
});
