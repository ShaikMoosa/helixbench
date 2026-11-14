// HelixBench Design System — Figma Plugin API builder.
// Recreates the product UI: token variables, foundations, component library
// (variants + states), all six screens, every dialog, states/flows and specs.
// Tokens mirror apps/desktop_gpui/src/theme.rs exactly.

const UI = 'Segoe UI';
const MONO = 'Consolas';

const HEX = {
  BG: 'f7f9fa', PANEL: 'ffffff', SURFACE: 'f1f4f6', SURFACE2: 'e4e9ee',
  OVERLAY: 'ffffff', BORDER: 'dde3e8', BORDER_STRONG: 'b9c4cd',
  TEXT: '1a2430', MUTED: '5a6b7c', FAINT: '8fa0ae',
  ACCENT: '0e9aa8', WHITE: 'ffffff',
  TOPBAR: '0d1b2a', TOPBAR_TEXT: 'e8eef4', TOPBAR_MUTED: '9db0c0',
  BASE_A: '2e9e5b', BASE_T: 'd94f46', BASE_C: '2f7fd6', BASE_G: 'd99a2b', BASE_N: '8b97a7',
  OK: '2f9e57', WARN: 'c08a20', DANGER: 'd64545', INFO: '2f7fd6',
  ORF_TOP: 'd97b3f', ORF_BOTTOM: '2e9e5b',
  SCRIM: '1a2430', SELECT_FILL: '0e9aa8', SEARCH_DNA: '0e9aa8', SEARCH_PROT: '2f7fd6',
  ACCENT_HOVER: '12b0c0', ACCENT_PRESS: '0b8391', DANGER_HOVER: 'e05e5e', DANGER_PRESS: 'bb3a3a',
};
const OPACITY = { SCRIM: 0.35, SELECT_FILL: 0.16, SEARCH_DNA: 0.22, SEARCH_PROT: 0.22 };

let col = null, mode = null;
const V = {};

function rgb(hex) {
  const n = parseInt(hex, 16);
  return { r: ((n >> 16) & 255) / 255, g: ((n >> 8) & 255) / 255, b: (n & 255) / 255 };
}
function solid(key, opacity) {
  const o = opacity !== undefined ? opacity : (OPACITY[key] !== undefined ? OPACITY[key] : 1);
  return { type: 'SOLID', color: rgb(HEX[key]), opacity: o };
}
function vp(key, opacity) {
  const p = solid(key, opacity);
  if (V[key]) return figma.variables.setBoundVariableForPaint(p, 'color', V[key]);
  return p;
}

async function loadFonts() {
  for (const s of ['Regular', 'Semibold', 'Bold']) await figma.loadFontAsync({ family: UI, style: s });
  for (const s of ['Regular', 'Bold']) await figma.loadFontAsync({ family: MONO, style: s });
}
function uiStyle(s) { return s === 'Medium' || s === 'Semi Bold' ? 'Semibold' : (s || 'Regular'); }

function setupVariables() {
  try {
    col = figma.variables.createVariableCollection('HB Tokens');
    mode = col.defaultModeId;
    col.renameMode(mode, 'Light');
    for (const key of Object.keys(HEX)) {
      const v = figma.variables.createVariable('color/' + key.toLowerCase(), col, 'COLOR');
      v.setValueForMode(mode, rgb(HEX[key]));
      V[key] = v;
    }
    const floats = {
      'space/1': 4, 'space/2': 8, 'space/3': 12, 'space/4': 16, 'space/5': 20, 'space/6': 24, 'space/8': 32,
      'radius/outer': 8, 'radius/inner': 6, 'radius/small': 4,
      'type/xs': 11, 'type/sm': 12, 'type/base': 13, 'type/md': 14, 'type/lg': 16, 'type/xl': 20,
      'control/h': 32, 'control/h-sm': 28, 'row/h': 36, 'sidebar/w': 184,
    };
    for (const [name, val] of Object.entries(floats)) {
      const v = figma.variables.createVariable(name, col, 'FLOAT');
      v.setValueForMode(mode, val);
      V[name] = v;
    }
  } catch (e) {
    console.log('Variables unavailable on this plan; using plain paints. ' + e);
    col = null;
  }
}

function setupTextStyles() {
  const scale = [['XS', 11], ['SM', 12], ['Base', 13], ['MD', 14], ['LG', 16], ['XL', 20]];
  for (const [name, size] of scale) {
    const st = figma.createTextStyle();
    st.name = 'HB/Text/' + name;
    st.fontName = { family: UI, style: 'Regular' };
    st.fontSize = size;
    st.lineHeight = { value: Math.round(size * 1.4), unit: 'PIXELS' };
  }
  const mono = figma.createTextStyle();
  mono.name = 'HB/Mono/Base';
  mono.fontName = { family: MONO, style: 'Regular' };
  mono.fontSize = 13;
  mono.lineHeight = { value: 18, unit: 'PIXELS' };
}

// --- node helpers -----------------------------------------------------------
function box(name, o = {}) {
  const f = figma.createFrame();
  f.name = name;
  if (o.w && o.h) f.resize(o.w, o.h);
  f.layoutMode = o.absolute ? 'NONE' : (o.dir || 'VERTICAL');
  if (!o.absolute) {
    f.itemSpacing = o.gap || 0;
    const p = o.pad || 0;
    f.paddingTop = f.paddingBottom = o.padY !== undefined ? o.padY : p;
    f.paddingLeft = f.paddingRight = o.padX !== undefined ? o.padX : p;
    f.primaryAxisSizingMode = o.growMain ? 'FIXED' : 'AUTO';
    f.counterAxisSizingMode = o.growCross ? 'FIXED' : 'AUTO';
    if (o.align) f.counterAxisAlignItems = o.align;
    if (o.justify) f.primaryAxisAlignItems = o.justify;
  }
  f.fills = o.fill ? [vp(o.fill, o.fillOpacity)] : [];
  if (o.stroke) { f.strokes = [vp(o.stroke)]; f.strokeWeight = o.strokeWeight || 1; }
  if (o.radius) f.cornerRadius = o.radius;
  f.clipsContent = o.clip !== false;
  if (o.opacity !== undefined) f.opacity = o.opacity;
  return f;
}
function row(name, o = {}) { return box(name, Object.assign({}, o, { dir: 'HORIZONTAL' })); }
function column(name, o = {}) { return box(name, Object.assign({}, o, { dir: 'VERTICAL' })); }
function add(parent, ...kids) { for (const k of kids) parent.appendChild(k); return parent; }
function txt(chars, o = {}) {
  const t = figma.createText();
  const st = uiStyle(o.style);
  t.fontName = { family: o.mono ? MONO : UI, style: o.mono ? (st === 'Bold' ? 'Bold' : 'Regular') : st };
  t.characters = chars;
  t.fontSize = o.size || 13;
  t.fills = [vp(o.color || 'TEXT', o.opacity)];
  if (o.lh) t.lineHeight = { value: o.lh, unit: 'PIXELS' };
  t.textAutoResize = o.auto || 'WIDTH_AND_HEIGHT';
  if (o.align) t.textAlignHorizontal = o.align;
  return t;
}
function spacer(w, h) { const f = figma.createFrame(); f.name = 'spacer'; f.resize(w || 1, h || 1); f.fills = []; return f; }
function grow(node) { node.layoutGrow = 1; return node; }
function dot(key, d) { const e = figma.createEllipse(); e.name = 'dot'; e.resize(d, d); e.fills = [vp(key)]; return e; }
function rect(name, w, h, key, radius, opacity) {
  const r = figma.createRectangle(); r.name = name; r.resize(w, h);
  r.fills = [vp(key, opacity)]; if (radius) r.cornerRadius = radius; return r;
}
function comp(name) { const c = figma.createComponent(); c.name = name; return c; }

function variantSet(parent, name, combos, builder) {
  const list = [];
  for (const props of combos) {
    const c = builder(props);
    c.name = Object.entries(props).map(([k, v]) => k + '=' + v).join(', ');
    parent.appendChild(c);
    list.push(c);
  }
  const set = figma.combineAsVariants(list, parent);
  set.name = name;
  set.layoutMode = 'VERTICAL';
  set.itemSpacing = 12;
  set.paddingTop = set.paddingBottom = 16;
  set.paddingLeft = set.paddingRight = 16;
  set.fills = [vp('PANEL')];
  set.cornerRadius = 8;
  return set;
}
function section(parent, title, desc) {
  const s = column(title, { gap: 8, pad: 24, fill: 'BG', radius: 8 });
  add(s, txt(title, { size: 20, style: 'Semi Bold' }));
  if (desc) add(s, txt(desc, { size: 13, color: 'MUTED' }));
  parent.appendChild(s);
  return s;
}

// --- component builders -----------------------------------------------------
function buttonFill(kind, state) {
  if (state === 'Disabled') return { fill: 'SURFACE2', stroke: null, text: 'FAINT' };
  if (kind === 'Primary') {
    const f = state === 'Hover' ? 'ACCENT_HOVER' : state === 'Pressed' ? 'ACCENT_PRESS' : 'ACCENT';
    return { fill: f, stroke: state === 'Focus' ? 'WHITE' : null, text: 'BG' };
  }
  if (kind === 'Danger') {
    const f = state === 'Hover' ? 'DANGER_HOVER' : state === 'Pressed' ? 'DANGER_PRESS' : 'DANGER';
    return { fill: f, stroke: state === 'Focus' ? 'WHITE' : null, text: 'WHITE' };
  }
  if (kind === 'Outline') {
    const f = state === 'Hover' ? 'SURFACE2' : state === 'Pressed' ? 'SURFACE' : null;
    return { fill: f, stroke: state === 'Focus' ? 'ACCENT' : 'BORDER_STRONG', text: 'TEXT' };
  }
  const f = state === 'Hover' ? 'SURFACE2' : state === 'Pressed' ? 'SURFACE' : 'SURFACE';
  return { fill: f, stroke: state === 'Focus' ? 'ACCENT' : null, text: 'TEXT' };
}
function buildButton(p) {
  const c = comp('Button');
  const s = buttonFill(p.kind, p.state);
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 6; c.paddingLeft = c.paddingRight = 12;
  c.paddingTop = c.paddingBottom = 0; c.resize(96, 32); c.primaryAxisSizingMode = 'FIXED';
  c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER'; c.primaryAxisAlignItems = 'CENTER';
  c.cornerRadius = 6;
  c.fills = s.fill ? [vp(s.fill)] : [];
  if (s.stroke) { c.strokes = [vp(s.stroke)]; c.strokeWeight = p.state === 'Focus' ? 2 : 1; }
  if (p.state === 'Disabled') c.opacity = 0.6;
  add(c, txt('Button', { size: 13, style: 'Medium', color: s.text }));
  return c;
}
function buildIconButton(p) {
  const c = comp('IconButton');
  c.resize(28, 28); c.cornerRadius = 6; c.layoutMode = 'NONE';
  const fill = p.state === 'Hover' ? 'SURFACE2' : p.state === 'Pressed' ? 'SURFACE' : 'SURFACE';
  c.fills = p.state === 'Disabled' ? [] : [vp(fill)];
  if (p.state === 'Focus') { c.strokes = [vp('ACCENT')]; c.strokeWeight = 2; }
  if (p.state === 'Disabled') c.opacity = 0.5;
  const ic = p.tone === 'Accent' ? 'ACCENT' : 'MUTED';
  const h = rect('i-h', 12, 2, ic, 1); h.x = 8; h.y = 13;
  const v = rect('i-v', 2, 12, ic, 1); v.x = 13; v.y = 8;
  add(c, h, v);
  return c;
}
function buildInput(p) {
  const c = comp('TextInput');
  c.layoutMode = 'VERTICAL'; c.itemSpacing = 4; c.fills = [];
  add(c, txt('Label', { size: 12, style: 'Medium', color: 'MUTED' }));
  const field = row('field', { w: 260, h: 32, radius: 6, align: 'CENTER', padX: 10, fill: 'SURFACE' });
  field.primaryAxisSizingMode = 'FIXED'; field.counterAxisSizingMode = 'FIXED';
  let stroke = 'BORDER', value = 'Value', vc = 'TEXT';
  if (p.state === 'Default') { value = 'Placeholder'; vc = 'FAINT'; }
  if (p.state === 'Focus') stroke = 'ACCENT';
  if (p.state === 'Error') stroke = 'DANGER';
  field.strokes = [vp(stroke)]; field.strokeWeight = p.state === 'Focus' ? 2 : 1;
  if (p.state === 'Disabled') field.opacity = 0.5;
  add(field, grow(txt(value, { size: 13, color: vc })));
  add(c, field);
  if (p.state === 'Error') add(c, txt('Required field', { size: 11, color: 'DANGER' }));
  return c;
}
function buildCheckbox(p) {
  const c = comp('Checkbox');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.counterAxisAlignItems = 'CENTER'; c.fills = [];
  const b = box('box', { w: 16, h: 16, radius: 4, absolute: true });
  b.resize(16, 16);
  if (p.checked === 'Unchecked') { b.fills = [vp('SURFACE')]; b.strokes = [vp('BORDER_STRONG')]; b.strokeWeight = 1; }
  else { b.fills = [vp('ACCENT')]; }
  if (p.checked === 'Checked') { const t = txt('✓', { size: 11, style: 'Bold', color: 'BG' }); t.x = 3; t.y = 1; b.appendChild(t); }
  if (p.checked === 'Indeterminate') { const d = rect('dash', 8, 2, 'BG', 1); d.x = 4; d.y = 7; b.appendChild(d); }
  if (p.state === 'Disabled') c.opacity = 0.5;
  if (p.state === 'Hover') { b.strokes = [vp('ACCENT')]; b.strokeWeight = 1; }
  add(c, b, txt('Option', { size: 13, color: p.state === 'Disabled' ? 'FAINT' : 'TEXT' }));
  return c;
}
function buildSwitch(p) {
  const c = comp('Switch');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.counterAxisAlignItems = 'CENTER'; c.fills = [];
  const track = box('track', { w: 34, h: 20, radius: 10, absolute: true });
  track.resize(34, 20);
  track.fills = [vp(p.on === 'On' ? 'ACCENT' : 'SURFACE2')];
  const knob = dot('WHITE', 16); knob.x = p.on === 'On' ? 16 : 2; knob.y = 2;
  track.appendChild(knob);
  if (p.state === 'Disabled') c.opacity = 0.5;
  add(c, track, txt(p.on === 'On' ? 'On' : 'Off', { size: 13, color: 'MUTED' }));
  return c;
}
function buildRadio(p) {
  const c = comp('Radio');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.counterAxisAlignItems = 'CENTER'; c.fills = [];
  const outer = box('outer', { w: 16, h: 16, radius: 8, absolute: true });
  outer.fills = [vp('SURFACE')]; outer.strokes = [vp(p.selected === 'Selected' ? 'ACCENT' : 'BORDER_STRONG')]; outer.strokeWeight = 1;
  if (p.selected === 'Selected') { const inner = dot('ACCENT', 8); inner.x = 4; inner.y = 4; outer.appendChild(inner); }
  add(c, outer, txt('Choice', { size: 13 }));
  return c;
}
function buildSelect(p) {
  const c = comp('Select');
  c.layoutMode = 'VERTICAL'; c.itemSpacing = 4; c.fills = [];
  const field = row('field', { w: 240, h: 32, radius: 6, align: 'CENTER', padX: 10, fill: 'SURFACE', stroke: p.open === 'Open' ? 'ACCENT' : 'BORDER' });
  field.primaryAxisSizingMode = 'FIXED'; field.counterAxisSizingMode = 'FIXED';
  add(field, grow(txt('Standard', { size: 13 })), txt('▾', { size: 12, color: 'MUTED' }));
  add(c, field);
  if (p.open === 'Open') {
    const panel = column('options', { w: 240, radius: 6, fill: 'OVERLAY', stroke: 'BORDER', clip: true });
    panel.primaryAxisSizingMode = 'FIXED';
    const opts = ['Standard', 'Vertebrate Mitochondrial', 'Bacterial / Plastid'];
    opts.forEach((o, i) => {
      const r = row('opt', { w: 240, h: 30, padX: 10, align: 'CENTER', fill: i === 0 ? 'SURFACE2' : null });
      r.primaryAxisSizingMode = 'FIXED'; r.counterAxisSizingMode = 'FIXED';
      add(r, txt(o, { size: 13, color: i === 0 ? 'TEXT' : 'MUTED' }));
      panel.appendChild(r);
    });
    add(c, panel);
  }
  return c;
}
function buildTab(p) {
  const c = comp('ViewTab');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 6; c.paddingLeft = c.paddingRight = 12; c.resize(110, 36);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.fills = p.state === 'Active' ? [vp('SURFACE')] : [];
  const color = p.state === 'Active' ? 'ACCENT' : p.state === 'Hover' ? 'TEXT' : 'MUTED';
  add(c, txt('Map', { size: 13, style: p.state === 'Active' ? 'Medium' : 'Regular', color }));
  if (p.state === 'Active') {
    c.strokes = [vp('ACCENT')];
    c.strokeTopWeight = 0; c.strokeLeftWeight = 0; c.strokeRightWeight = 0; c.strokeBottomWeight = 2;
  }
  return c;
}
function buildMenuItem(p) {
  const c = comp('MenuItem');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.paddingLeft = c.paddingRight = 10; c.resize(220, 30);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.fills = p.state === 'Hover' ? [vp('SURFACE2')] : [];
  if (p.state === 'Disabled') c.opacity = 0.5;
  if (p.checked === 'Checked') add(c, txt('✓', { size: 12, color: 'ACCENT' }));
  add(c, grow(txt('Menu item', { size: 13, color: p.state === 'Disabled' ? 'FAINT' : 'TEXT' })));
  add(c, txt('Ctrl+S', { size: 11, color: 'FAINT' }));
  return c;
}
function buildTableRow(p) {
  const c = comp('TableRow');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 0; c.resize(640, 36);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.fills = p.state === 'Selected' ? [vp('SELECT_FILL')] : p.state === 'Hover' ? [vp('SURFACE2')] : [];
  const cells = [['demo_cds', 180, 'TEXT'], ['CDS', 90, 'MUTED'], ['101..200', 110, 'MUTED'], ['+1', 60, 'MUTED'], ['GFP reporter', 200, 'MUTED']];
  for (const [label, w, color] of cells) {
    const cell = row('cell', { w, h: 36, padX: 8, align: 'CENTER' });
    cell.primaryAxisSizingMode = 'FIXED'; cell.counterAxisSizingMode = 'FIXED';
    add(cell, txt(label, { size: 13, color }));
    c.appendChild(cell);
  }
  return c;
}
function buildTableHeader(p) {
  const c = comp('TableHeader');
  c.layoutMode = 'HORIZONTAL'; c.resize(640, 32); c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED';
  c.counterAxisAlignItems = 'CENTER'; c.fills = [vp('PANEL')];
  const cols = [['Name', 180], ['Type', 90], ['Range', 110], ['Strand', 60], ['Product', 200]];
  cols.forEach(([label, w], i) => {
    const cell = row('h', { w, h: 32, padX: 8, align: 'CENTER', gap: 4 });
    cell.primaryAxisSizingMode = 'FIXED'; cell.counterAxisSizingMode = 'FIXED';
    const active = (p.sort === 'Asc' || p.sort === 'Desc') && i === 0;
    add(cell, txt(label, { size: 11, style: 'Semi Bold', color: active ? 'ACCENT' : 'FAINT' }));
    if (active) add(cell, txt(p.sort === 'Asc' ? '↑' : '↓', { size: 11, color: 'ACCENT' }));
    c.appendChild(cell);
  });
  return c;
}
function buildTag(p) {
  const c = comp('Tag');
  const map = { CDS: 'BASE_C', promoter: 'BASE_A', terminator: 'BASE_T', origin: 'BASE_N', marker: 'BASE_G' };
  const key = map[p.kind] || 'BASE_N';
  c.layoutMode = 'HORIZONTAL'; c.paddingLeft = c.paddingRight = 6; c.resize(70, 20);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.cornerRadius = 4; c.fills = [vp(key, 0.18)];
  add(c, txt(p.kind, { size: 11, style: 'Medium', color: key }));
  return c;
}
function buildToast(p) {
  const c = comp('Toast');
  const tone = { Info: 'INFO', Success: 'OK', Warning: 'WARN', Error: 'DANGER' }[p.tone];
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.paddingLeft = c.paddingRight = 12; c.resize(320, 44);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.cornerRadius = 8; c.fills = [vp('OVERLAY')]; c.strokes = [vp('BORDER')]; c.strokeWeight = 1;
  add(c, dot(tone, 8), grow(txt('Saved pHelixBench-Demo.hbx', { size: 13 })));
  return c;
}
function buildScrollbar(p) {
  const c = comp('Scrollbar');
  c.layoutMode = 'NONE';
  if (p.orient === 'Vertical') { c.resize(10, 160); } else { c.resize(160, 10); }
  c.fills = [vp('BG')];
  const thumb = rect('thumb', p.orient === 'Vertical' ? 6 : 48, p.orient === 'Vertical' ? 48 : 6,
    p.state === 'Hover' || p.state === 'Drag' ? 'BORDER_STRONG' : 'BORDER', 3);
  thumb.x = 2; thumb.y = 2;
  c.appendChild(thumb);
  return c;
}
function buildSidebarRow(p) {
  const c = comp('SidebarRow');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.paddingLeft = c.paddingRight = 10; c.resize(168, 32);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.cornerRadius = 6; c.fills = p.on === 'On' ? [vp('SURFACE2')] : [];
  add(c, dot(p.on === 'On' ? 'ACCENT' : 'FAINT', 8), txt('Features', { size: 13, color: p.on === 'On' ? 'TEXT' : 'MUTED' }));
  return c;
}
function buildFindBar(p) {
  const c = comp('FindBar');
  c.layoutMode = 'HORIZONTAL'; c.itemSpacing = 8; c.paddingLeft = c.paddingRight = 10; c.resize(560, 40);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER';
  c.cornerRadius = 8; c.fills = [vp('OVERLAY')]; c.strokes = [vp('BORDER')]; c.strokeWeight = 1;
  const modes = ['DNA', 'Protein', 'Name'];
  for (const m of modes) {
    const active = m === p.mode;
    const chip = row('mode', { h: 24, padX: 8, radius: 4, align: 'CENTER', fill: active ? 'SURFACE2' : null });
    add(chip, txt(m, { size: 11, style: 'Medium', color: active ? 'ACCENT' : 'MUTED' }));
    c.appendChild(chip);
  }
  const field = row('q', { w: 160, h: 26, radius: 4, padX: 8, align: 'CENTER', fill: 'SURFACE', stroke: 'BORDER' });
  field.primaryAxisSizingMode = 'FIXED'; field.counterAxisSizingMode = 'FIXED';
  add(field, txt(p.mode === 'DNA' ? 'GAATTC' : p.mode === 'Protein' ? 'MKT' : 'demo', { size: 12, mono: p.mode !== 'Name' }));
  c.appendChild(field);
  add(c, txt('1 of 2', { size: 11, color: 'MUTED' }), txt('‹', { size: 14, color: 'MUTED' }), txt('›', { size: 14, color: 'MUTED' }), txt('✕', { size: 12, color: 'MUTED' }));
  return c;
}
function buildEmpty(p) {
  const c = comp('EmptyState');
  c.layoutMode = 'VERTICAL'; c.itemSpacing = 8; c.paddingTop = c.paddingBottom = 32; c.resize(360, 160);
  c.primaryAxisSizingMode = 'FIXED'; c.counterAxisSizingMode = 'FIXED'; c.counterAxisAlignItems = 'CENTER'; c.primaryAxisAlignItems = 'CENTER';
  c.fills = [];
  add(c, dot('FAINT', 24), txt('No ' + p.what + ' yet', { size: 14, style: 'Medium', color: 'MUTED' }),
    txt('Create one to get started.', { size: 12, color: 'FAINT' }));
  return c;
}

// --- chrome pieces ----------------------------------------------------------
function topBar() {
  const b = row('TopBar', { w: 1360, h: 40, fill: 'TOPBAR', align: 'CENTER', padX: 10, gap: 6 });
  b.primaryAxisSizingMode = 'FIXED'; b.counterAxisSizingMode = 'FIXED';
  add(b, dot('ACCENT', 10), txt('HelixBench', { size: 13, style: 'Medium', color: 'TOPBAR_TEXT' }));
  for (const m of ['File', 'Edit', 'View', 'Features', 'Primers', 'Enzymes', 'Tools', 'Help'])
    add(b, txt(m, { size: 13, color: 'TOPBAR_MUTED' }));
  add(b, grow(spacer(1, 1)), dot('ACCENT', 20), txt('Local workspace', { size: 12, color: 'TOPBAR_MUTED' }));
  return b;
}
function docTabs() {
  const b = row('DocTabs', { w: 1360, h: 40, fill: 'PANEL', align: 'CENTER', padX: 8, gap: 4 });
  b.primaryAxisSizingMode = 'FIXED'; b.counterAxisSizingMode = 'FIXED';
  const tab = row('doc', { h: 30, padX: 12, radius: 6, align: 'CENTER', gap: 8, fill: 'SURFACE', stroke: 'BORDER' });
  add(tab, dot('ACCENT', 8), txt('pHelixBench-Demo', { size: 12, color: 'TEXT' }), txt('✕', { size: 11, color: 'FAINT' }));
  add(b, tab, txt('+', { size: 14, color: 'MUTED' }), grow(spacer(1, 1)));
  const chip = row('topology', { h: 22, padX: 8, radius: 4, align: 'CENTER', fill: 'ACCENT', opacity: 0.12 });
  add(chip, txt('circular', { size: 11, color: 'ACCENT' }));
  b.appendChild(chip);
  return b;
}
function leftRail() {
  const s = column('LeftRail', { w: 48, fill: 'PANEL', pad: 8, gap: 6, align: 'CENTER' });
  s.counterAxisSizingMode = 'FIXED'; s.layoutAlign = 'STRETCH';
  const views = [['Map', true], ['Sequence', false], ['Features', false], ['Primers', false], ['Enzymes', false], ['History', false]];
  for (const [label, on] of views) {
    const r = box('rail', { w: 36, h: 36, radius: 6, fill: on ? 'SURFACE2' : null, align: 'CENTER', justify: 'CENTER' });
    add(r, dot(on ? 'ACCENT' : 'FAINT', 10));
    s.appendChild(r);
  }
  add(s, spacer(1, 6));
  for (let i = 0; i < 8; i++) { const r = box('act', { w: 36, h: 36, radius: 6, align: 'CENTER', justify: 'CENTER' }); add(r, dot('MUTED', 8)); s.appendChild(r); }
  return s;
}
function statusBar() {
  const b = row('StatusBar', { w: 1360, h: 37, fill: 'PANEL', align: 'CENTER', padX: 12, gap: 12 });
  b.primaryAxisSizingMode = 'FIXED'; b.counterAxisSizingMode = 'FIXED';
  add(b, txt('No selection', { size: 12, color: 'MUTED' }), txt('3021 bp · circular · GC 54.5%', { size: 12, color: 'MUTED' }),
    txt('13 features · 4 primers · 30 enzymes', { size: 12, color: 'MUTED' }), grow(spacer(1, 1)),
    dot('WARN', 6), txt('Unsaved', { size: 12, color: 'WARN' }));
  return b;
}
function seqPanel() {
  const p = column('SeqPanel', { w: 430, fill: 'PANEL', pad: 12, gap: 6 });
  p.counterAxisSizingMode = 'FIXED'; p.layoutAlign = 'STRETCH';
  const bases = 'CACAGTGGTTCCACGACAGATACAGACTTTCGGTTCAAG';
  for (let r = 0; r < 6; r++) {
    add(p, txt(bases, { size: 10, mono: true, color: 'MUTED' }));
    const bar = row('bar', { h: 12, radius: 3, fill: ['BASE_A', 'BASE_C', 'BASE_G', 'BASE_T'][r % 4], opacity: 0.5 });
    bar.layoutAlign = 'STRETCH';
    p.appendChild(bar);
    add(p, txt(bases.split('').reverse().join(''), { size: 10, mono: true, color: 'FAINT' }));
    add(p, txt((r * 60 + 50) + '', { size: 9, mono: true, color: 'FAINT' }));
  }
  return p;
}
function infoCards() {
  const r = row('Cards', { h: 150, gap: 10, pad: 10 });
  r.counterAxisSizingMode = 'FIXED'; r.layoutAlign = 'STRETCH';
  const mk = (title, lines) => {
    const c = column(title, { gap: 6, pad: 12, fill: 'PANEL', stroke: 'BORDER', radius: 8 });
    c.layoutGrow = 1; c.counterAxisSizingMode = 'FIXED';
    add(c, txt(title, { size: 12, style: 'Medium', color: 'TEXT' }));
    for (const l of lines) add(c, txt(l, { size: 11, color: 'MUTED' }));
    return c;
  };
  add(r, mk('Feature', ['T7 promoter', 'T7 RNA polymerase promoter']),
    mk('Properties', ['Type promoter', 'Range 97..116', 'Length 20 bp', 'Strand +1']),
    mk('Legend', ['T7 promoter', 'lac operator', 'RBS', 'demo_cds']));
  return r;
}
function editorSplit(content) {
  const split = row('split', { gap: 0 });
  split.layoutGrow = 1; split.counterAxisSizingMode = 'FIXED'; split.layoutAlign = 'STRETCH';
  const mapBox = column('map', { align: 'CENTER', justify: 'CENTER', gap: 8 });
  mapBox.layoutGrow = 1; mapBox.counterAxisSizingMode = 'FIXED';
  const ring = figma.createEllipse(); ring.resize(300, 300); ring.fills = []; ring.strokes = [vp('BORDER_STRONG')]; ring.strokeWeight = 2;
  mapBox.appendChild(ring);
  add(mapBox, txt('pHelixBench-Demo', { size: 14, style: 'Medium', color: 'TEXT' }), txt('3021 bp · circular', { size: 11, color: 'MUTED' }));
  add(split, mapBox, seqPanel());
  content.appendChild(split);
  content.appendChild(infoCards());
}
function shell(active) {
  const f = column('Screen · ' + active, { w: 1360, h: 860, fill: 'BG', clip: true });
  f.primaryAxisSizingMode = 'FIXED'; f.counterAxisSizingMode = 'FIXED';
  add(f, topBar(), docTabs());
  const body = row('body', { gap: 0 });
  body.layoutGrow = 1; body.counterAxisSizingMode = 'FIXED'; body.layoutAlign = 'STRETCH';
  const content = column('content', { pad: active === 'Map' ? 0 : 16, gap: active === 'Map' ? 0 : 12 });
  content.layoutGrow = 1; content.counterAxisSizingMode = 'FIXED'; content.layoutAlign = 'STRETCH';
  add(body, leftRail(), content);
  add(f, body, statusBar());
  return { frame: f, content };
}

// --- pages ------------------------------------------------------------------
function makePages() {
  const keys = ['foundations', 'components', 'screens', 'dialogs', 'states', 'specs'];
  const out = {};
  for (let i = 0; i < keys.length; i++) { const p = figma.createPage(); p.name = PAGE_NAMES[i + 1]; out[keys[i]] = p; }
  for (const p of [...figma.root.children]) if (!PAGE_NAMES.includes(p.name)) p.remove();
  return out;
}

function buildFoundations(page) {
  figma.currentPage = page;
  const root = column('Foundations', { gap: 24, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('Foundations', { size: 28, style: 'Bold' }), txt('Tokens mirrored from theme.rs · light laboratory editor', { size: 14, color: 'MUTED' }));

  const colors = section(root, 'Color variables', 'Bound to the "HB Tokens" collection (mode: Night).');
  const groups = [
    ['Neutral ramp', ['BG', 'PANEL', 'SURFACE', 'SURFACE2', 'OVERLAY', 'BORDER', 'BORDER_STRONG', 'TEXT', 'MUTED', 'FAINT']],
    ['Accent & washes', ['ACCENT', 'SELECT_FILL', 'SEARCH_DNA', 'SEARCH_PROT', 'SCRIM']],
    ['Fluorescence bases', ['BASE_A', 'BASE_T', 'BASE_C', 'BASE_G', 'BASE_N']],
    ['Semantic', ['OK', 'WARN', 'DANGER', 'INFO']],
    ['ORF strands', ['ORF_TOP', 'ORF_BOTTOM']],
  ];
  for (const [gname, keys] of groups) {
    const g = column(gname, { gap: 8 });
    add(g, txt(gname, { size: 14, style: 'Semi Bold' }));
    const r = row('swatches', { gap: 12 });
    for (const k of keys) {
      const sw = column(k, { gap: 6 });
      const chip = box('chip', { w: 96, h: 56, radius: 6, fill: k, stroke: 'BORDER' });
      add(sw, chip, txt(k, { size: 11, style: 'Medium' }), txt('#' + HEX[k], { size: 11, mono: true, color: 'MUTED' }));
      r.appendChild(sw);
    }
    g.appendChild(r);
    colors.appendChild(g);
  }

  const type = section(root, 'Typography', 'Segoe UI (UI) · Consolas (sequence). Scale 11–20.');
  for (const [name, size] of [['XS', 11], ['SM', 12], ['Base', 13], ['MD', 14], ['LG', 16], ['XL', 20]]) {
    const r = row('t', { gap: 16, align: 'CENTER' });
    add(r, txt(name + ' · ' + size + 'px', { size: 11, mono: true, color: 'FAINT' }), txt('The quick brown fox · ATCG', { size }));
    type.appendChild(r);
  }
  const monoRow = row('mono', { gap: 16, align: 'CENTER' });
  add(monoRow, txt('Mono · 13px', { size: 11, mono: true, color: 'FAINT' }), txt('ATCG GAATTC MKTLL', { size: 13, mono: true }));
  type.appendChild(monoRow);

  const space = section(root, 'Spacing & radii', '4px grid · nested radii 8 / 6 / 4.');
  const sr = row('spacing', { gap: 12, align: 'CENTER' });
  for (const n of [4, 8, 12, 16, 24, 32]) {
    const c = column('s' + n, { gap: 6, align: 'CENTER' });
    add(c, rect('b', n, n, 'ACCENT', 2), txt(n + '', { size: 11, mono: true, color: 'MUTED' }));
    sr.appendChild(c);
  }
  space.appendChild(sr);
  const rr = row('radii', { gap: 12, align: 'CENTER' });
  for (const n of [8, 6, 4]) {
    const c = column('r' + n, { gap: 6, align: 'CENTER' });
    add(c, box('b', { w: 56, h: 40, radius: n, fill: 'SURFACE', stroke: 'BORDER_STRONG' }), txt(n + '', { size: 11, mono: true, color: 'MUTED' }));
    rr.appendChild(c);
  }
  space.appendChild(rr);
}

function buildComponents(page) {
  figma.currentPage = page;
  const root = column('Components', { gap: 24, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('Component library', { size: 28, style: 'Bold' }), txt('Every interactive element with its variants and states.', { size: 14, color: 'MUTED' }));
  const states = ['Default', 'Hover', 'Pressed', 'Focus', 'Disabled'];

  const bSec = section(root, 'Button', 'kind × state.');
  const bCombos = [];
  for (const kind of ['Primary', 'Outline', 'Subtle', 'Danger']) for (const state of states) bCombos.push({ kind, state });
  variantSet(bSec, 'Button', bCombos, buildButton);

  const ibSec = section(root, 'IconButton', 'tone × state.');
  const ibCombos = [];
  for (const tone of ['Subtle', 'Accent']) for (const state of states) ibCombos.push({ tone, state });
  variantSet(ibSec, 'IconButton', ibCombos, buildIconButton);

  const inSec = section(root, 'TextInput', 'state.');
  variantSet(inSec, 'TextInput', states.map(s => ({ state: s })), buildInput);

  const cbSec = section(root, 'Checkbox', 'checked × state.');
  const cbCombos = [];
  for (const checked of ['Unchecked', 'Checked', 'Indeterminate']) for (const state of ['Default', 'Hover', 'Disabled']) cbCombos.push({ checked, state });
  variantSet(cbSec, 'Checkbox', cbCombos, buildCheckbox);

  const swSec = section(root, 'Switch', 'on × state.');
  const swCombos = [];
  for (const on of ['On', 'Off']) for (const state of ['Default', 'Disabled']) swCombos.push({ on, state });
  variantSet(swSec, 'Switch', swCombos, buildSwitch);

  const raSec = section(root, 'Radio', 'selected.');
  variantSet(raSec, 'Radio', [{ selected: 'Selected' }, { selected: 'Unselected' }], buildRadio);

  const seSec = section(root, 'Select / Combo', 'closed / open.');
  variantSet(seSec, 'Select', [{ open: 'Closed' }, { open: 'Open' }], buildSelect);

  const tabSec = section(root, 'ViewTab', 'state.');
  variantSet(tabSec, 'ViewTab', [{ state: 'Active' }, { state: 'Inactive' }, { state: 'Hover' }], buildTab);

  const miSec = section(root, 'MenuItem', 'state × checked.');
  const miCombos = [];
  for (const state of ['Default', 'Hover', 'Disabled']) for (const checked of ['Unchecked', 'Checked']) miCombos.push({ state, checked });
  variantSet(miSec, 'MenuItem', miCombos, buildMenuItem);

  const trSec = section(root, 'TableRow', 'state.');
  variantSet(trSec, 'TableRow', [{ state: 'Default' }, { state: 'Hover' }, { state: 'Selected' }], buildTableRow);

  const thSec = section(root, 'TableHeader', 'sort.');
  variantSet(thSec, 'TableHeader', [{ sort: 'None' }, { sort: 'Asc' }, { sort: 'Desc' }], buildTableHeader);

  const tagSec = section(root, 'Tag / feature chip', 'kind.');
  variantSet(tagSec, 'Tag', ['CDS', 'promoter', 'terminator', 'origin', 'marker'].map(k => ({ kind: k })), buildTag);

  const toSec = section(root, 'Toast', 'tone.');
  variantSet(toSec, 'Toast', ['Info', 'Success', 'Warning', 'Error'].map(t => ({ tone: t })), buildToast);

  const sbSec = section(root, 'Scrollbar', 'orient × state.');
  const sbCombos = [];
  for (const orient of ['Vertical', 'Horizontal']) for (const state of ['Default', 'Hover', 'Drag']) sbCombos.push({ orient, state });
  variantSet(sbSec, 'Scrollbar', sbCombos, buildScrollbar);

  const srSec = section(root, 'SidebarRow', 'on / off.');
  variantSet(srSec, 'SidebarRow', [{ on: 'On' }, { on: 'Off' }], buildSidebarRow);

  const fbSec = section(root, 'FindBar', 'mode.');
  variantSet(fbSec, 'FindBar', ['DNA', 'Protein', 'Name'].map(m => ({ mode: m })), buildFindBar);

  const emSec = section(root, 'EmptyState', 'what.');
  variantSet(emSec, 'EmptyState', ['features', 'primers', 'enzymes', 'history'].map(w => ({ what: w })), buildEmpty);
}

// --- screens ----------------------------------------------------------------
function seqRows(content, opts = {}) {
  const seq = 'ATGCGTAAAGGCTTAGCTAGCTAGGATCCGAATTCGGTACCTAGCTAGCATGCGTAAAGGCTTAGCTA';
  const comp = 'TACGCATT TCCGAATCGATCGATCCTAGGCTTAAGCCATGGATCGATCGTACGCATTTCCGAATCGAT'.replace(/ /g, '');
  const block = column('seq', { gap: 2 });
  const top = row('top', { gap: 0 });
  const bot = row('bottom', { gap: 0 });
  for (let i = 0; i < seq.length; i++) {
    const b = seq[i];
    const key = { A: 'BASE_A', T: 'BASE_T', C: 'BASE_C', G: 'BASE_G' }[b] || 'BASE_N';
    const hit = opts.find && i >= 30 && i < 36;
    const sel = opts.selection && i >= 12 && i < 20;
    const t = txt(b, { size: 13, mono: true, color: opts.colors === false ? 'TEXT' : key });
    const cell = box('b', { w: 12, h: 20, absolute: true, align: 'CENTER' });
    cell.resize(12, 20);
    if (sel) cell.fills = [vp('SELECT_FILL')];
    else if (hit) cell.fills = [vp('SEARCH_DNA')];
    cell.appendChild(t); t.x = 2; t.y = 2;
    top.appendChild(cell);
    const cb = comp[i] || 'N';
    const ckey = { A: 'BASE_A', T: 'BASE_T', C: 'BASE_C', G: 'BASE_G' }[cb] || 'BASE_N';
    const ct = txt(cb, { size: 13, mono: true, color: opts.colors === false ? 'FAINT' : ckey });
    const ccell = box('c', { w: 12, h: 20, absolute: true });
    ccell.resize(12, 20);
    if (sel) ccell.fills = [vp('SELECT_FILL')];
    ccell.appendChild(ct); ct.x = 2; ct.y = 2;
    bot.appendChild(ccell);
  }
  add(block, top, bot);
  content.appendChild(block);
}
function buildScreens(page) {
  figma.currentPage = page;
  const root = column('Screens', { gap: 40, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('Screens', { size: 28, style: 'Bold' }), txt('All six views at 1360×860 with live demo content.', { size: 14, color: 'MUTED' }));

  // Map
  {
    const { frame, content } = shell('Map');
    root.appendChild(frame);
    editorSplit(content);
  }
  // Sequence (default)
  {
    const { frame, content } = shell('Sequence');
    root.appendChild(frame);
    seqRows(content, {});
    seqRows(content, {});
  }
  // Sequence zoomed + find + selection
  {
    const { frame, content } = shell('Sequence');
    frame.name = 'Screen · Sequence (zoom + find + selection)';
    root.appendChild(frame);
    const fb = buildFindBar({ mode: 'DNA' }); content.appendChild(fb);
    seqRows(content, { find: true, selection: true });
    const zb = row('zoombar', { gap: 8, align: 'CENTER', fill: 'PANEL', padX: 12, h: 40 });
    zb.counterAxisSizingMode = 'FIXED';
    add(zb, txt('Zoom', { size: 12, color: 'MUTED' }), txt('+', { size: 14 }), txt('−', { size: 14 }), txt('Fit', { size: 12 }), txt('window 200 bp', { size: 11, mono: true, color: 'MUTED' }));
    content.appendChild(zb);
    const mini = rect('minimap', 700, 26, 'SURFACE', 4); content.appendChild(mini);
  }
  // Features
  {
    const { frame, content } = shell('Features');
    root.appendChild(frame);
    const tb = row('toolbar', { gap: 8, align: 'CENTER' });
    const addB = buildButton({ kind: 'Primary', state: 'Default' }); addB.name = 'Add Feature';
    tb.appendChild(addB);
    content.appendChild(tb);
    content.appendChild(buildTableHeader({ sort: 'Asc' }));
    for (let i = 0; i < 6; i++) content.appendChild(buildTableRow({ state: i === 1 ? 'Selected' : 'Default' }));
  }
  // Primers
  {
    const { frame, content } = shell('Primers');
    root.appendChild(frame);
    content.appendChild(buildTableHeader({ sort: 'None' }));
    for (let i = 0; i < 5; i++) content.appendChild(buildTableRow({ state: 'Default' }));
  }
  // Enzymes
  {
    const { frame, content } = shell('Enzymes');
    root.appendChild(frame);
    content.appendChild(buildTableHeader({ sort: 'None' }));
    for (let i = 0; i < 6; i++) content.appendChild(buildTableRow({ state: i === 2 ? 'Hover' : 'Default' }));
  }
  // History
  {
    const { frame, content } = shell('History');
    root.appendChild(frame);
    const cur = row('current', { h: 36, padX: 8, radius: 6, align: 'CENTER', gap: 8, fill: 'SURFACE2' });
    add(cur, dot('ACCENT', 8), txt('Current state', { size: 13, style: 'Medium' }));
    content.appendChild(cur);
    for (const label of ['Add feature GFP reporter', 'Color selection 100..200', 'Delete primer Demo-Rev', 'Open pHelixBench-Demo']) {
      const r = row('h', { h: 36, padX: 8, radius: 6, align: 'CENTER', gap: 8 });
      add(r, dot('FAINT', 6), txt(label, { size: 13, color: 'MUTED' }));
      content.appendChild(r);
    }
  }
}

// --- dialogs ----------------------------------------------------------------
function dialogShell(title, w) {
  const d = column('Dialog · ' + title, { w: w || 480, radius: 8, fill: 'OVERLAY', stroke: 'BORDER', clip: true });
  d.counterAxisSizingMode = 'FIXED';
  const head = row('head', { h: 46, padX: 16, align: 'CENTER', fill: 'PANEL' });
  head.counterAxisSizingMode = 'FIXED'; head.layoutAlign = 'STRETCH';
  add(head, grow(txt(title, { size: 14, style: 'Semi Bold' })), txt('✕', { size: 12, color: 'MUTED' }));
  d.appendChild(head);
  return d;
}
function buildDialogs(page) {
  figma.currentPage = page;
  const root = column('Dialogs', { gap: 32, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('Dialogs & modals', { size: 28, style: 'Bold' }), txt('Every modal with its controls and states. Native Open/Save dialogs are OS-provided.', { size: 14, color: 'MUTED' }));
  const grid = row('grid', { gap: 32, align: 'MIN' });
  root.appendChild(grid);
  const left = column('col', { gap: 32 });
  const right = column('col', { gap: 32 });
  add(grid, left, right);

  // Options
  {
    const d = dialogShell('Options', 480);
    const body = column('body', { pad: 16, gap: 10 });
    for (const [label, on] of [['Show features', true], ['Show enzymes', true], ['Show ORFs', false], ['Complement strand', true]]) {
      const r = row('opt', { gap: 8, align: 'CENTER' });
      const sw = buildSwitch({ on: on ? 'On' : 'Off', state: 'Default' });
      add(r, sw, txt(label, { size: 13 }));
      body.appendChild(r);
    }
    const gc = row('gc', { gap: 8, align: 'CENTER' });
    add(gc, txt('Genetic code', { size: 13, color: 'MUTED' }), buildSelect({ open: 'Closed' }));
    body.appendChild(gc);
    const orf = row('orf', { gap: 8, align: 'CENTER' });
    add(orf, txt('ORF min aa', { size: 13, color: 'MUTED' }), txt('−', { size: 14 }), txt('50', { size: 13, mono: true }), txt('+', { size: 14 }));
    body.appendChild(orf);
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    left.appendChild(d);
  }
  // Add Feature
  {
    const d = dialogShell('Add Feature', 480);
    const body = column('body', { pad: 16, gap: 12 });
    body.appendChild(buildInput({ state: 'Filled' }));
    body.appendChild(buildInput({ state: 'Default' }));
    const range = row('range', { gap: 8 });
    range.appendChild(buildInput({ state: 'Filled' }));
    range.appendChild(buildInput({ state: 'Filled' }));
    body.appendChild(range);
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    left.appendChild(d);
  }
  // Unsaved changes
  {
    const d = dialogShell('Unsaved Changes', 440);
    const body = column('body', { pad: 16, gap: 8 });
    add(body, txt('Save changes to pHelixBench-Demo before closing?', { size: 13 }),
      txt('Your edits will be lost if you discard.', { size: 12, color: 'MUTED' }));
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Danger', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    left.appendChild(d);
  }
  // Detect Common
  {
    const d = dialogShell('Detect Common Features', 520);
    const body = column('body', { pad: 16, gap: 8 });
    add(body, txt('12 found · 8 new', { size: 12, color: 'MUTED' }));
    const list = column('list', { gap: 2, fill: 'SURFACE', radius: 6, clip: true });
    for (const n of ['T7 promoter', 'lac operator', 'M13 fwd priming site', 'ori (demo)', 'Kozak sequence']) {
      const r = row('r', { h: 32, padX: 8, align: 'CENTER', gap: 8 });
      const cb = buildCheckbox({ checked: 'Checked', state: 'Default' });
      add(r, cb, txt(n, { size: 13 }));
      list.appendChild(r);
    }
    body.appendChild(list);
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    right.appendChild(d);
  }
  // Color Selection
  {
    const d = dialogShell('Color Selection', 440);
    const body = column('body', { pad: 16, gap: 12 });
    add(body, txt('Range 100..200 · both strands', { size: 12, color: 'MUTED' }));
    const sw = row('swatches', { gap: 8 });
    for (const k of ['BASE_A', 'BASE_T', 'BASE_C', 'BASE_G', 'ACCENT', 'DANGER']) {
      const chip = box('sw', { w: 32, h: 32, radius: 6, fill: k, stroke: k === 'ACCENT' ? 'WHITE' : 'BORDER', strokeWeight: k === 'ACCENT' ? 2 : 1 });
      sw.appendChild(chip);
    }
    body.appendChild(sw);
    const strands = row('strands', { gap: 8 });
    strands.appendChild(buildRadio({ selected: 'Selected' }));
    strands.appendChild(buildRadio({ selected: 'Unselected' }));
    body.appendChild(strands);
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    right.appendChild(d);
  }
  // Add Primer
  {
    const d = dialogShell('Add Primer', 480);
    const body = column('body', { pad: 16, gap: 12 });
    body.appendChild(buildInput({ state: 'Filled' }));
    body.appendChild(buildInput({ state: 'Filled' }));
    const tm = row('tm', { gap: 8, align: 'CENTER' });
    add(tm, txt('Tm 58.4°C · GC 52%', { size: 12, mono: true, color: 'MUTED' }));
    body.appendChild(tm);
    d.appendChild(body);
    const foot = row('foot', { h: 56, padX: 16, gap: 8, justify: 'END', align: 'CENTER', fill: 'PANEL' });
    foot.counterAxisSizingMode = 'FIXED'; foot.layoutAlign = 'STRETCH';
    add(foot, buildButton({ kind: 'Subtle', state: 'Default' }), buildButton({ kind: 'Primary', state: 'Default' }));
    d.appendChild(foot);
    right.appendChild(d);
  }
}

// --- states & flows ---------------------------------------------------------
function buildStates(page) {
  figma.currentPage = page;
  const root = column('States & Flows', { gap: 24, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('States & interaction flows', { size: 28, style: 'Bold' }),
    txt('Hover / pressed / focus / disabled, feedback, and primary user flows.', { size: 14, color: 'MUTED' }));

  const fb = section(root, 'Feedback toasts', 'Shown bottom-right after save / export / errors.');
  const tr = row('toasts', { gap: 12 });
  for (const t of ['Info', 'Success', 'Warning', 'Error']) tr.appendChild(buildToast({ tone: t }));
  fb.appendChild(tr);

  const empty = section(root, 'Empty / loading / error', 'List views when a document has no items.');
  const er = row('empties', { gap: 16 });
  er.appendChild(buildEmpty({ what: 'features' }));
  const load = column('loading', { gap: 8, align: 'CENTER', pad: 24 });
  add(load, dot('ACCENT', 20), txt('Loading…', { size: 13, color: 'MUTED' }));
  er.appendChild(load);
  const err = column('error', { gap: 8, align: 'CENTER', pad: 24 });
  add(err, dot('DANGER', 20), txt('Could not parse file', { size: 13, color: 'DANGER' }), txt('Unsupported or corrupted sequence.', { size: 12, color: 'MUTED' }));
  er.appendChild(err);
  empty.appendChild(er);

  const flows = section(root, 'Primary flows', 'Step-by-step interaction documentation.');
  const flowText = [
    'Open / create: Ctrl+O or File→Open (native dialog) → document loads → all views refresh → recent-files updated.',
    'Edit feature: Features tab → Add Feature → fill name/type/range → Create → row appears, History entry added, dirty • shown.',
    'Undo / redo: Ctrl+Z / Ctrl+Y → state reverts → History current-marker moves → buttons enable/disable at stack ends.',
    'Save: Ctrl+S → if pathless, native Save As → atomic .hbx write → toast "Saved" + status path + dirty cleared.',
    'Search: Ctrl+F → choose DNA/Protein/Name → type query → hits highlighted, "n of m" → ‹/› scrolls to hit on correct strand.',
    'Zoom: View→Zoom controls → Sequence zoom bar +/−/Fit/center → minimap drag pans, edge-drag resizes → Map shows zoom arc.',
    'Color selection: select range → Edit→Color Selection → pick swatch + strands → Apply → region tinted on both strands.',
    'Close guard: window close while dirty → Unsaved Changes dialog → Save / Discard / Cancel.',
  ];
  for (const f of flowText) add(flows, txt('• ' + f, { size: 13, color: 'MUTED', auto: 'HEIGHT' }));
}

// --- specs ------------------------------------------------------------------
function buildSpecs(page) {
  figma.currentPage = page;
  const root = column('Specs', { gap: 24, pad: 40, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('Specification & redlines', { size: 28, style: 'Bold' }), txt('Token tables and component behavior for handoff.', { size: 14, color: 'MUTED' }));

  const ct = section(root, 'Color token table', 'name · hex · usage.');
  const usage = {
    BG: 'app background', PANEL: 'chrome bars', SURFACE: 'cards / rows', SURFACE2: 'hover / selected',
    OVERLAY: 'dialog surface', BORDER: 'hairlines', BORDER_STRONG: 'emphasis lines', TEXT: 'primary text',
    MUTED: 'secondary text', FAINT: 'tertiary / disabled', ACCENT: 'single UI accent / selection',
    BASE_A: 'adenine', BASE_T: 'thymine', BASE_C: 'cytosine', BASE_G: 'guanine', BASE_N: 'ambiguous',
    OK: 'success', WARN: 'warning / unsaved', DANGER: 'destructive / error', INFO: 'info',
  };
  for (const [k, u] of Object.entries(usage)) {
    const r = row('t', { gap: 12, align: 'CENTER' });
    add(r, box('chip', { w: 20, h: 20, radius: 4, fill: k, stroke: 'BORDER' }),
      txt(k, { size: 12, mono: true }), txt('#' + HEX[k], { size: 12, mono: true, color: 'MUTED' }),
      txt(u, { size: 12, color: 'MUTED' }));
    ct.appendChild(r);
  }

  const comp = section(root, 'Component behavior', 'props & interaction notes.');
  const notes = [
    'Button: kind=Primary/Outline/Subtle/Danger; state=Default/Hover/Pressed/Focus/Disabled. Focus adds 2px ring; Disabled 60% opacity.',
    'TextInput: state=Default(placeholder)/Focus(2px accent)/Filled/Error(danger + helper)/Disabled.',
    'Checkbox: Unchecked/Checked/Indeterminate × Default/Hover/Disabled; 16px, radius 4.',
    'Switch: On/Off × Default/Disabled; 34×20 track, 16px knob.',
    'Select: Closed/Open; open shows option list, first option hover-highlighted.',
    'ViewTab: Active(accent + underline)/Inactive/Hover.',
    'MenuItem: Default/Hover/Disabled × Checked; shortcut hint right-aligned.',
    'TableRow: Default/Hover/Selected(selection wash); TableHeader sort None/Asc/Desc with arrow.',
    'Toast: Info/Success/Warning/Error; auto-dismiss; bottom-right.',
    'Scrollbar: custom 10px rail, 6px thumb radius 3; Hover/Drag lighten.',
    'FindBar: mode DNA/Protein/Name; query + "n of m" + prev/next/close.',
  ];
  for (const n of notes) add(comp, txt('• ' + n, { size: 13, color: 'MUTED', auto: 'HEIGHT' }));

  const layout = section(root, 'Layout redlines', 'Chrome geometry.');
  for (const n of ['Title bar 30px · Menu bar 34px · Tab bar 44px · Status bar 37px', 'Sidebar width 184px · content padding 16px', 'Row height 36px · header 32px · control height 32px (sm 28px)', 'Spacing on 4px grid · radii 8 outer / 6 inner / 4 small']) {
    add(layout, txt('• ' + n, { size: 13, color: 'MUTED' }));
  }
}

function buildCover() {
  const page = figma.createPage();
  page.name = PAGE_NAMES[0];
  figma.root.insertChild(0, page);
  figma.currentPage = page;
  const root = column('Cover', { gap: 16, pad: 60, fill: 'BG' });
  page.appendChild(root);
  add(root, txt('HelixBench', { size: 48, style: 'Bold', color: 'ACCENT' }),
    txt('Design system & UI specification', { size: 20, color: 'TEXT' }),
    txt('Local-first DNA & plasmid workbench · light laboratory editor · pure Rust (GPUI + egui)', { size: 14, color: 'MUTED' }),
    spacer(1, 16),
    txt('Pages', { size: 16, style: 'Semi Bold' }),
    txt('01 Foundations · 02 Components · 03 Screens · 04 Dialogs · 05 States & Flows · 06 Specs', { size: 13, color: 'MUTED' }),
    spacer(1, 8),
    txt('Tokens: HB Tokens variable collection (Light). Type: Segoe UI + Consolas. Grid 4px · radii 8/6/4.', { size: 12, color: 'FAINT' }));
}

const PAGE_NAMES = ['00 · Cover', '01 · Foundations', '02 · Components', '03 · Screens', '04 · Dialogs', '05 · States & Flows', '06 · Specs'];
function resetPrior() {
  for (const p of [...figma.root.children]) if (PAGE_NAMES.includes(p.name)) p.remove();
  if (figma.root.children.length === 0) figma.createPage();
  for (const st of figma.getLocalTextStyles()) if (st.name.startsWith('HB/')) st.remove();
  for (const c of figma.variables.getLocalVariableCollections()) if (c.name === 'HB Tokens') c.remove();
}

async function main() {
 try {
  await loadFonts();
  resetPrior();
  setupVariables();
  setupTextStyles();
  const pages = makePages();
  buildFoundations(pages.foundations);
  buildComponents(pages.components);
  buildScreens(pages.screens);
  buildDialogs(pages.dialogs);
  buildStates(pages.states);
  buildSpecs(pages.specs);
  buildCover();
  figma.notify('HelixBench design system built: 7 pages, components, variables.');
  figma.closePlugin();
 } catch (e) {
  figma.notify('HB plugin error: ' + (e && e.message ? e.message : e), { error: true });
  console.error(e);
  figma.closePlugin();
 }
}
main();