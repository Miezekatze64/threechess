const PATH = './target/wasm32-unknown-unknown/release/threechess.wasm';
var exports;

var ctx = document.getElementById('canvas').getContext('2d');
ctx.canvas.width = document.body.clientWidth;
ctx.canvas.height = document.body.clientHeight;
ctx.textAlign = 'left';
ctx.textBaseline = 'top';
ctx.font = '20px sans-serif';

addEventListener('resize', _ => {
  ctx.canvas.width = document.body.clientWidth;
  ctx.canvas.height = document.body.clientHeight;
  ctx.font = `20px sans-serif`;
  ctx.textAlign = 'left';
  ctx.textBaseline = 'top';
});

const MOUSE_UP_EVENT = 0;
var event_queue = [];

const event_queue_size = () => {
  let size = 0;
  for (a of event_queue) {
    size += a.buffer.byteLength;
  }
  return size;
};

addEventListener('mouseup', e => {
  if (e.button != 0) return;
  let x = e.clientX;
  let y = e.clientY;
  let data = new Uint8Array(new Uint32Array([MOUSE_UP_EVENT, e.button, x, y]).buffer)
  event_queue.push(data);
});


const get_string = (ptr, len) => {
  let arr = new Uint8Array(exports.memory.buffer);
  let str = '';
  for (let i = 0; i < len; i++) {
    str += String.fromCharCode(arr[ptr + i]);
  }
  return str;
}

load_sync = (url) => {
  var xhr = new XMLHttpRequest();
  xhr.open("GET", url, false);
  xhr.overrideMimeType("text/plain; charset=x-user-defined");
  xhr.send();
  URL.revokeObjectURL(url);
  var returnArray = [];
  for (let i = 0; i < xhr.responseText.length; i++) {
    returnArray.push(xhr.responseText.charCodeAt(i) & 0xff);
  }
  return returnArray;
}

const run_wasm = () => WebAssembly.instantiateStreaming(fetch(PATH), {
  'env': {
    '_log': (ptr, len) => {
      console.log(get_string(ptr, len));
    },
    '_elog': (ptr, len) => {
      console.error(get_string(ptr, len));
    },
    '_read_file': (path_ptr, path_len, buf_ptr, buf_len) => {
      let path = get_string(path_ptr, path_len);
      let buf = load_sync(path);
      console.assert(buf_len === buf.length, `${buf_len} !== ${buf.length}`);

      let mem = new Uint8Array(exports.memory.buffer, buf_ptr, buf_len);
      for (let i = 0; i < buf_len; i++) {
        mem[i] = buf[i];
      }
    },
    '_get_file_size': (path_ptr, path_len) => {
      let buf = load_sync(get_string(path_ptr, path_len));
      return buf.length;
    },
    '_set_draw_color': (r, g, b) => {
      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.strokeStyle = `rgb(${r}, ${g}, ${b})`;
    },
    '_clear': () => {
      ctx.fillRect(0, 0, ctx.canvas.clientWidth,
                   ctx.canvas.clientHeight);
    },
    '_win_size': (w_ptr, h_ptr) => {
      let mem = new Uint32Array(exports.memory.buffer);
      mem[w_ptr / 4] = ctx.canvas.width;
      mem[h_ptr / 4] = ctx.canvas.height;
    },
    '_event_queue_size': event_queue_size,
    '_get_event_queue': ptr => {
      const len = event_queue_size();
      let mem = new Uint8Array(exports.memory.buffer, ptr, len);
      let i = 0;
      for (e of event_queue) {
        for (let bi = 0; bi < e.buffer.byteLength; bi++) {
          mem[i++] = e[bi];
        }
      }

      event_queue = [];
    },
    '_text_size': (txt_ptr, txt_len, w_ptr, h_ptr) => {
      let text = get_string(txt_ptr, txt_len);
      let metrics = ctx.measureText(text);
      let actualHeight = metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;

      let mem = new Uint32Array(exports.memory.buffer);
      mem[w_ptr / 4] = metrics.width;
      mem[h_ptr / 4] = actualHeight;
    },
    '_render_text': (txt_ptr, txt_len, x, y, r, g, b) => {
      let txt = get_string(txt_ptr, txt_len);
      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.fillText(txt, x, y);
    },
    '_draw_point': (x, y) => {
      ctx.fillRect(x, y, 1, 1);
    },
    '_draw_line': (x1, y1, x2, y2) => {
      ctx.beginPath();
      ctx.moveTo(x1, y1);
      ctx.lineTo(x2, y2);
      ctx.stroke();
    },
    '_render_png': (data_ptr, data_len, x, y, w, h) => {
      let data = new Uint8Array(exports.memory.buffer, data_ptr, data_len);
      let str = '';
      for (let i = 0; i < data_len; i++) {
        str += String.fromCharCode(data[i]);
      }

      let b64 = btoa(str);
      let url = `data:image/png;base64,${b64}`;
      let img = new Image();
      img.src = url;

      ctx.drawImage(img, x, y, w, h);
    },
    '_present': () => {
      ctx.fill();
      ctx.stroke();
    }
  }
}).then(x => {
  exports = x.instance.exports;
  exports.init();

  setInterval(() => {
    exports.main_loop_step();
  }, 1000 / 60);
});

run_wasm();
