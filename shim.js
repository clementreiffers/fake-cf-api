// build/worker/shim.mjs
import I from "./29bae8a03647eb9b55a10b5f2203b23d2b970081-index.wasm";
var $ = Object.defineProperty;
var z = (e, t) => {
  for (var n in t)
    $(e, n, { get: t[n], enumerable: true });
};
var d = {};
z(d, { IntoUnderlyingByteSource: () => v, IntoUnderlyingSink: () => M, IntoUnderlyingSource: () => T, MinifyConfig: () => q, PipeOptions: () => S, PolishConfig: () => X, QueuingStrategy: () => C, R2Range: () => L, ReadableStreamGetReaderOptions: () => F, RequestRedirect: () => Y, __wbg_buffer_610b70c8fd30da2d: () => pt, __wbg_buffer_cf65c07de34b9a08: () => Bt, __wbg_byobRequest_a3c74c3694777d1b: () => ut, __wbg_byteLength_1fef7842ca4200fa: () => lt, __wbg_byteOffset_ede786cfcf88d3dd: () => dt, __wbg_bytesliteral_efe7d360639bf32b: () => gt, __wbg_call_9495de66fdbe016b: () => $t, __wbg_cf_a70afe2b6e717440: () => wt, __wbg_close_045ed342139beb7d: () => ft, __wbg_close_a41954830b65c455: () => ct, __wbg_constructor_0c9828c8a7cf1dc6: () => Ht, __wbg_enqueue_3a8a8e67e44d2567: () => it, __wbg_error_f7214ae7db04600c: () => et, __wbg_formData_88b09253e29bc444: () => Ct, __wbg_get_5f1a91f013de2311: () => jt, __wbg_get_baf4855f9a986186: () => Lt, __wbg_headers_ab5251d2727ac41e: () => St, __wbg_instanceof_Error_749a7378f4439ee0: () => Dt, __wbg_instanceof_File_42d74276109d8f61: () => kt, __wbg_latitude_4e2c6213a28a64c7: () => ht, __wbg_length_27a2afe8ab42b09f: () => Yt, __wbg_log_7bb108d119bafbc1: () => xt, __wbg_longitude_71787e3c0f553c34: () => yt, __wbg_method_d1ee174c753ca2be: () => Tt, __wbg_name_4e66d4cfa3e9270a: () => zt, __wbg_new0_25059e40b1c02766: () => It, __wbg_new_15d3966e9981a196: () => Rt, __wbg_new_9d3a9ce4282a18a8: () => Pt, __wbg_new_a99726b0abef495b: () => Q, __wbg_new_f1c3a9c2533a55b8: () => Et, __wbg_new_f9876326328f45ed: () => Ft, __wbg_newwithbyteoffsetandlength_9fb2f11355ecadf5: () => Kt, __wbg_newwithlength_b56c882b57805732: () => Zt, __wbg_newwithoptbuffersourceandinit_4d2fa6d435ff2a63: () => At, __wbg_newwithoptreadablestreamandinit_a0b4dc209bd176be: () => Mt, __wbg_newwithoptstrandinit_1a4621d99c54e7c3: () => vt, __wbg_region_10ca3fc7adc99274: () => mt, __wbg_resolve_fd40f858d9db1a04: () => Ut, __wbg_respond_f4778bef04e912a6: () => bt, __wbg_set_17499e8aa4003ebd: () => Xt, __wbg_set_6aa458a4ebdb65cb: () => Gt, __wbg_set_a5d34c36a1a4ebd1: () => Ot, __wbg_stack_4931b18709aff089: () => tt, __wbg_then_ec5db6d509eb475f: () => Jt, __wbg_then_f753623316e2873a: () => Vt, __wbg_toString_4f53179351070600: () => Nt, __wbg_toString_cec163b212643722: () => Wt, __wbg_url_bd2775644ef804ec: () => qt, __wbg_view_d1a31268af734e5d: () => at, __wbindgen_cb_drop: () => ot, __wbindgen_closure_wrapper930: () => ne, __wbindgen_debug_string: () => Qt, __wbindgen_is_undefined: () => st, __wbindgen_memory: () => ee, __wbindgen_number_new: () => _t, __wbindgen_object_clone_ref: () => nt, __wbindgen_object_drop_ref: () => Z, __wbindgen_string_get: () => G, __wbindgen_string_new: () => rt, __wbindgen_throw: () => te, fetch: () => D });
function R() {
  return "bytes";
}
var N = new WebAssembly.Instance(I, { "./index_bg.js": d });
var r = N.exports;
var g = new Array(128).fill(void 0);
g.push(void 0, null, true, false);
function o(e) {
  return g[e];
}
var m = g.length;
function H(e) {
  e < 132 || (g[e] = m, m = e);
}
function p(e) {
  let t = o(e);
  return H(e), t;
}
var l = 0;
var k = null;
function E() {
  return (k === null || k.byteLength === 0) && (k = new Uint8Array(r.memory.buffer)), k;
}
var P = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var O = new P("utf-8");
var U = typeof O.encodeInto == "function" ? function(e, t) {
  return O.encodeInto(e, t);
} : function(e, t) {
  let n = O.encode(e);
  return t.set(n), { read: e.length, written: n.length };
};
function y(e, t, n) {
  if (n === void 0) {
    let f = O.encode(e), x = t(f.length);
    return E().subarray(x, x + f.length).set(f), l = f.length, x;
  }
  let _ = e.length, s = t(_), a = E(), u = 0;
  for (; u < _; u++) {
    let f = e.charCodeAt(u);
    if (f > 127)
      break;
    a[s + u] = f;
  }
  if (u !== _) {
    u !== 0 && (e = e.slice(u)), s = n(s, _, _ = u + e.length * 3);
    let f = E().subarray(s + u, s + _);
    u += U(e, f).written;
  }
  return l = u, s;
}
function b(e) {
  return e == null;
}
var j = null;
function i() {
  return (j === null || j.byteLength === 0) && (j = new Int32Array(r.memory.buffer)), j;
}
function c(e) {
  m === g.length && g.push(g.length + 1);
  let t = m;
  return m = g[t], g[t] = e, t;
}
var J = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var W = new J("utf-8", { ignoreBOM: true, fatal: true });
W.decode();
function h(e, t) {
  return W.decode(E().subarray(e, e + t));
}
function A(e) {
  let t = typeof e;
  if (t == "number" || t == "boolean" || e == null)
    return `${e}`;
  if (t == "string")
    return `"${e}"`;
  if (t == "symbol") {
    let s = e.description;
    return s == null ? "Symbol" : `Symbol(${s})`;
  }
  if (t == "function") {
    let s = e.name;
    return typeof s == "string" && s.length > 0 ? `Function(${s})` : "Function";
  }
  if (Array.isArray(e)) {
    let s = e.length, a = "[";
    s > 0 && (a += A(e[0]));
    for (let u = 1; u < s; u++)
      a += ", " + A(e[u]);
    return a += "]", a;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(e)), _;
  if (n.length > 1)
    _ = n[1];
  else
    return toString.call(e);
  if (_ == "Object")
    try {
      return "Object(" + JSON.stringify(e) + ")";
    } catch {
      return "Object";
    }
  return e instanceof Error ? `${e.name}: ${e.message}
${e.stack}` : _;
}
function V(e, t, n, _) {
  let s = { a: e, b: t, cnt: 1, dtor: n }, a = (...u) => {
    s.cnt++;
    let f = s.a;
    s.a = 0;
    try {
      return _(f, s.b, ...u);
    } finally {
      --s.cnt === 0 ? r.__wbindgen_export_2.get(s.dtor)(f, s.b) : s.a = f;
    }
  };
  return a.original = s, a;
}
function B(e, t, n) {
  r._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h58a662108f63987d(e, t, c(n));
}
function D(e, t, n) {
  let _ = r.fetch(c(e), c(t), c(n));
  return p(_);
}
function w(e, t) {
  try {
    return e.apply(this, t);
  } catch (n) {
    r.__wbindgen_exn_store(c(n));
  }
}
function K(e, t, n, _) {
  r.wasm_bindgen__convert__closures__invoke2_mut__h63c442130f2627f6(e, t, c(n), c(_));
}
var X = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var Y = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var v = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingbytesource_free(t);
  }
  get type() {
    let t = r.intounderlyingbytesource_type(this.ptr);
    return p(t);
  }
  get autoAllocateChunkSize() {
    return r.intounderlyingbytesource_autoAllocateChunkSize(this.ptr) >>> 0;
  }
  start(t) {
    r.intounderlyingbytesource_start(this.ptr, c(t));
  }
  pull(t) {
    let n = r.intounderlyingbytesource_pull(this.ptr, c(t));
    return p(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingbytesource_cancel(t);
  }
};
var M = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsink_free(t);
  }
  write(t) {
    let n = r.intounderlyingsink_write(this.ptr, c(t));
    return p(n);
  }
  close() {
    let t = this.__destroy_into_raw(), n = r.intounderlyingsink_close(t);
    return p(n);
  }
  abort(t) {
    let n = this.__destroy_into_raw(), _ = r.intounderlyingsink_abort(n, c(t));
    return p(_);
  }
};
var T = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_intounderlyingsource_free(t);
  }
  pull(t) {
    let n = r.intounderlyingsource_pull(this.ptr, c(t));
    return p(n);
  }
  cancel() {
    let t = this.__destroy_into_raw();
    r.intounderlyingsource_cancel(t);
  }
};
var q = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_minifyconfig_free(t);
  }
  get js() {
    return r.__wbg_get_minifyconfig_js(this.ptr) !== 0;
  }
  set js(t) {
    r.__wbg_set_minifyconfig_js(this.ptr, t);
  }
  get html() {
    return r.__wbg_get_minifyconfig_html(this.ptr) !== 0;
  }
  set html(t) {
    r.__wbg_set_minifyconfig_html(this.ptr, t);
  }
  get css() {
    return r.__wbg_get_minifyconfig_css(this.ptr) !== 0;
  }
  set css(t) {
    r.__wbg_set_minifyconfig_css(this.ptr, t);
  }
};
var S = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_pipeoptions_free(t);
  }
  get preventClose() {
    return r.pipeoptions_preventClose(this.ptr) !== 0;
  }
  get preventCancel() {
    return r.pipeoptions_preventCancel(this.ptr) !== 0;
  }
  get preventAbort() {
    return r.pipeoptions_preventAbort(this.ptr) !== 0;
  }
  get signal() {
    let t = r.pipeoptions_signal(this.ptr);
    return p(t);
  }
};
var C = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_queuingstrategy_free(t);
  }
  get highWaterMark() {
    return r.queuingstrategy_highWaterMark(this.ptr);
  }
};
var L = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_r2range_free(t);
  }
  get offset() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_offset(_, this.ptr);
      var t = i()[_ / 4 + 0], n = i()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(t) {
    r.__wbg_set_r2range_offset(this.ptr, !b(t), b(t) ? 0 : t);
  }
  get length() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_length(_, this.ptr);
      var t = i()[_ / 4 + 0], n = i()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(t) {
    r.__wbg_set_r2range_length(this.ptr, !b(t), b(t) ? 0 : t);
  }
  get suffix() {
    try {
      let _ = r.__wbindgen_add_to_stack_pointer(-16);
      r.__wbg_get_r2range_suffix(_, this.ptr);
      var t = i()[_ / 4 + 0], n = i()[_ / 4 + 1];
      return t === 0 ? void 0 : n >>> 0;
    } finally {
      r.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(t) {
    r.__wbg_set_r2range_suffix(this.ptr, !b(t), b(t) ? 0 : t);
  }
};
var F = class {
  __destroy_into_raw() {
    let t = this.ptr;
    return this.ptr = 0, t;
  }
  free() {
    let t = this.__destroy_into_raw();
    r.__wbg_readablestreamgetreaderoptions_free(t);
  }
  get mode() {
    let t = r.readablestreamgetreaderoptions_mode(this.ptr);
    return p(t);
  }
};
function Z(e) {
  p(e);
}
function G(e, t) {
  let n = o(t), _ = typeof n == "string" ? n : void 0;
  var s = b(_) ? 0 : y(_, r.__wbindgen_malloc, r.__wbindgen_realloc), a = l;
  i()[e / 4 + 1] = a, i()[e / 4 + 0] = s;
}
function Q() {
  let e = new Error();
  return c(e);
}
function tt(e, t) {
  let n = o(t).stack, _ = y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function et(e, t) {
  try {
    console.error(h(e, t));
  } finally {
    r.__wbindgen_free(e, t);
  }
}
function nt(e) {
  let t = o(e);
  return c(t);
}
function rt(e, t) {
  let n = h(e, t);
  return c(n);
}
function _t(e) {
  return c(e);
}
function ot(e) {
  let t = p(e).original;
  return t.cnt-- == 1 ? (t.a = 0, true) : false;
}
function st(e) {
  return o(e) === void 0;
}
function ct(e) {
  o(e).close();
}
function it(e, t) {
  o(e).enqueue(o(t));
}
function ut(e) {
  let t = o(e).byobRequest;
  return b(t) ? 0 : c(t);
}
function ft(e) {
  o(e).close();
}
function at(e) {
  let t = o(e).view;
  return b(t) ? 0 : c(t);
}
function bt(e, t) {
  o(e).respond(t >>> 0);
}
function pt(e) {
  let t = o(e).buffer;
  return c(t);
}
function dt(e) {
  return o(e).byteOffset;
}
function lt(e) {
  return o(e).byteLength;
}
function gt() {
  let e = R();
  return c(e);
}
function wt(e) {
  let t = o(e).cf;
  return b(t) ? 0 : c(t);
}
function ht(e, t) {
  let n = o(t).latitude;
  var _ = b(n) ? 0 : y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function yt(e, t) {
  let n = o(t).longitude;
  var _ = b(n) ? 0 : y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function mt(e, t) {
  let n = o(t).region;
  var _ = b(n) ? 0 : y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function xt(e) {
  console.log(o(e));
}
function kt(e) {
  let t;
  try {
    t = o(e) instanceof File;
  } catch {
    t = false;
  }
  return t;
}
function jt(e, t, n) {
  let _ = o(e).get(h(t, n));
  return c(_);
}
function Et() {
  return w(function() {
    let e = new Headers();
    return c(e);
  }, arguments);
}
function Ot() {
  return w(function(e, t, n, _, s) {
    o(e).set(h(t, n), h(_, s));
  }, arguments);
}
function At() {
  return w(function(e, t) {
    let n = new Response(o(e), o(t));
    return c(n);
  }, arguments);
}
function vt() {
  return w(function(e, t, n) {
    let _ = new Response(e === 0 ? void 0 : h(e, t), o(n));
    return c(_);
  }, arguments);
}
function Mt() {
  return w(function(e, t) {
    let n = new Response(o(e), o(t));
    return c(n);
  }, arguments);
}
function Tt(e, t) {
  let n = o(t).method, _ = y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function qt(e, t) {
  let n = o(t).url, _ = y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function St(e) {
  let t = o(e).headers;
  return c(t);
}
function Ct() {
  return w(function(e) {
    let t = o(e).formData();
    return c(t);
  }, arguments);
}
function Lt() {
  return w(function(e, t) {
    let n = Reflect.get(o(e), o(t));
    return c(n);
  }, arguments);
}
function Ft() {
  let e = new Object();
  return c(e);
}
function Dt(e) {
  let t;
  try {
    t = o(e) instanceof Error;
  } catch {
    t = false;
  }
  return t;
}
function Rt(e, t) {
  let n = new Error(h(e, t));
  return c(n);
}
function Wt(e) {
  let t = o(e).toString();
  return c(t);
}
function $t() {
  return w(function(e, t, n) {
    let _ = o(e).call(o(t), o(n));
    return c(_);
  }, arguments);
}
function zt(e) {
  let t = o(e).name;
  return c(t);
}
function It() {
  let e = new Date();
  return c(e);
}
function Nt(e) {
  let t = o(e).toString();
  return c(t);
}
function Ht(e) {
  let t = o(e).constructor;
  return c(t);
}
function Pt(e, t) {
  try {
    var n = { a: e, b: t }, _ = (a, u) => {
      let f = n.a;
      n.a = 0;
      try {
        return K(f, n.b, a, u);
      } finally {
        n.a = f;
      }
    };
    let s = new Promise(_);
    return c(s);
  } finally {
    n.a = n.b = 0;
  }
}
function Ut(e) {
  let t = Promise.resolve(o(e));
  return c(t);
}
function Jt(e, t) {
  let n = o(e).then(o(t));
  return c(n);
}
function Vt(e, t, n) {
  let _ = o(e).then(o(t), o(n));
  return c(_);
}
function Bt(e) {
  let t = o(e).buffer;
  return c(t);
}
function Kt(e, t, n) {
  let _ = new Uint8Array(o(e), t >>> 0, n >>> 0);
  return c(_);
}
function Xt(e, t, n) {
  o(e).set(o(t), n >>> 0);
}
function Yt(e) {
  return o(e).length;
}
function Zt(e) {
  let t = new Uint8Array(e >>> 0);
  return c(t);
}
function Gt() {
  return w(function(e, t, n) {
    return Reflect.set(o(e), o(t), o(n));
  }, arguments);
}
function Qt(e, t) {
  let n = A(o(t)), _ = y(n, r.__wbindgen_malloc, r.__wbindgen_realloc), s = l;
  i()[e / 4 + 1] = s, i()[e / 4 + 0] = _;
}
function te(e, t) {
  throw new Error(h(e, t));
}
function ee() {
  let e = r.memory;
  return c(e);
}
function ne(e, t, n) {
  let _ = V(e, t, 127, B);
  return c(_);
}
var ue = { fetch: D, scheduled: void 0, queue: void 0 };
export {
  v as IntoUnderlyingByteSource,
  M as IntoUnderlyingSink,
  T as IntoUnderlyingSource,
  q as MinifyConfig,
  S as PipeOptions,
  X as PolishConfig,
  C as QueuingStrategy,
  L as R2Range,
  F as ReadableStreamGetReaderOptions,
  Y as RequestRedirect,
  pt as __wbg_buffer_610b70c8fd30da2d,
  Bt as __wbg_buffer_cf65c07de34b9a08,
  ut as __wbg_byobRequest_a3c74c3694777d1b,
  lt as __wbg_byteLength_1fef7842ca4200fa,
  dt as __wbg_byteOffset_ede786cfcf88d3dd,
  gt as __wbg_bytesliteral_efe7d360639bf32b,
  $t as __wbg_call_9495de66fdbe016b,
  wt as __wbg_cf_a70afe2b6e717440,
  ft as __wbg_close_045ed342139beb7d,
  ct as __wbg_close_a41954830b65c455,
  Ht as __wbg_constructor_0c9828c8a7cf1dc6,
  it as __wbg_enqueue_3a8a8e67e44d2567,
  et as __wbg_error_f7214ae7db04600c,
  Ct as __wbg_formData_88b09253e29bc444,
  jt as __wbg_get_5f1a91f013de2311,
  Lt as __wbg_get_baf4855f9a986186,
  St as __wbg_headers_ab5251d2727ac41e,
  Dt as __wbg_instanceof_Error_749a7378f4439ee0,
  kt as __wbg_instanceof_File_42d74276109d8f61,
  ht as __wbg_latitude_4e2c6213a28a64c7,
  Yt as __wbg_length_27a2afe8ab42b09f,
  xt as __wbg_log_7bb108d119bafbc1,
  yt as __wbg_longitude_71787e3c0f553c34,
  Tt as __wbg_method_d1ee174c753ca2be,
  zt as __wbg_name_4e66d4cfa3e9270a,
  It as __wbg_new0_25059e40b1c02766,
  Rt as __wbg_new_15d3966e9981a196,
  Pt as __wbg_new_9d3a9ce4282a18a8,
  Q as __wbg_new_a99726b0abef495b,
  Et as __wbg_new_f1c3a9c2533a55b8,
  Ft as __wbg_new_f9876326328f45ed,
  Kt as __wbg_newwithbyteoffsetandlength_9fb2f11355ecadf5,
  Zt as __wbg_newwithlength_b56c882b57805732,
  At as __wbg_newwithoptbuffersourceandinit_4d2fa6d435ff2a63,
  Mt as __wbg_newwithoptreadablestreamandinit_a0b4dc209bd176be,
  vt as __wbg_newwithoptstrandinit_1a4621d99c54e7c3,
  mt as __wbg_region_10ca3fc7adc99274,
  Ut as __wbg_resolve_fd40f858d9db1a04,
  bt as __wbg_respond_f4778bef04e912a6,
  Xt as __wbg_set_17499e8aa4003ebd,
  Gt as __wbg_set_6aa458a4ebdb65cb,
  Ot as __wbg_set_a5d34c36a1a4ebd1,
  tt as __wbg_stack_4931b18709aff089,
  Jt as __wbg_then_ec5db6d509eb475f,
  Vt as __wbg_then_f753623316e2873a,
  Nt as __wbg_toString_4f53179351070600,
  Wt as __wbg_toString_cec163b212643722,
  qt as __wbg_url_bd2775644ef804ec,
  at as __wbg_view_d1a31268af734e5d,
  ot as __wbindgen_cb_drop,
  ne as __wbindgen_closure_wrapper930,
  Qt as __wbindgen_debug_string,
  st as __wbindgen_is_undefined,
  ee as __wbindgen_memory,
  _t as __wbindgen_number_new,
  nt as __wbindgen_object_clone_ref,
  Z as __wbindgen_object_drop_ref,
  G as __wbindgen_string_get,
  rt as __wbindgen_string_new,
  te as __wbindgen_throw,
  ue as default,
  D as fetch
};
//# sourceMappingURL=shim.js.map
