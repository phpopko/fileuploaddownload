pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>FILE—TRANSFER</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@700&display=swap" rel="stylesheet">
<style>
*, *::before, *::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  border-radius: 0 !important;
  box-shadow: none !important;
}

:root { --bg: #F0EDE8; --fg: #0A0A0A; --ac: #FF3000; }
body.dark { --bg: #0A0A0A; --fg: #F0EDE8; }

html, body {
  background: var(--bg);
  color: var(--fg);
}

body {
  font-family: 'Courier New', Courier, monospace;
  font-size: 13px;
  line-height: 1.1;
  overflow-x: hidden;
  min-height: 100dvh;
}

hr {
  border: none;
  border-top: 1px solid currentColor;
  margin: 0;
}

/* ── TITLE ── */
/* Slams to the true left viewport edge, bleeds off-screen right on mobile */
.title {
  font-family: 'Space Grotesk', sans-serif;
  font-size: clamp(48px, 12vw, 120px);
  font-weight: 700;
  letter-spacing: -0.04em;
  line-height: 1.1;
  text-transform: uppercase;
  white-space: nowrap;
  display: block;
  padding: 20px 0 14px 0;
}

/* ── MAIN PADDED AREA ── */
.pg {
  padding: 0 24px calc(24px + env(safe-area-inset-bottom, 0px));
}

/* ── META BAR ── */
.meta-bar {
  display: flex;
  align-items: baseline;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  padding-bottom: 16px;
}

.meta-live { flex: 1; }
#live-ind  { margin-right: 6px; }

.btn-invert {
  background: none;
  border: none;
  font-family: 'Courier New', Courier, monospace;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--fg);
  cursor: pointer;
  padding: 0;
  flex-shrink: 0;
}
.btn-invert:hover { text-decoration: underline; }

/* ── GRID ── */
.grid {
  display: grid;
  grid-template-columns: 1fr;
}
@media (min-width: 760px) {
  .grid { grid-template-columns: 1fr 1fr; column-gap: 48px; }
}

.sec-label {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  padding: 20px 0 12px;
}

/* ── DROP ZONE ── */
.zone {
  border: 1px solid currentColor;
  width: 100%;
  height: 120px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: crosshair;
  font-family: 'Courier New', Courier, monospace;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  text-align: center;
  padding: 0 16px;
  user-select: none;
  -webkit-user-select: none;
}
.zone input { display: none; }
.zone.over  { background: var(--ac); color: #F0EDE8; border-color: var(--ac); }

/* ── SELECTED FILE INFO ── */
.sel-info {
  display: none;
  padding: 8px 0;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.02em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.sel-info.on { display: block; }

/* ── PROGRESS TRACK — 1px raw red line ── */
.prog-track {
  height: 1px;
  position: relative;
  margin: 2px 0;
  display: none;
  overflow: hidden;
}
.prog-track.on { display: block; }
.prog-track::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: var(--fg);
  opacity: 0.12;
}
.prog-fill {
  position: absolute;
  left: 0; top: 0;
  height: 100%;
  width: 0%;
  background: var(--ac);
  transition: width 0.08s linear;
}

/* ── UPLOAD BUTTON ── */
.btn-upload {
  display: block;
  width: 100%;
  height: 44px;
  margin-top: 10px;
  border: none;
  background: var(--fg);
  color: var(--bg);
  cursor: pointer;
  font-family: 'Space Grotesk', sans-serif;
  font-size: 13px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  line-height: 1.1;
  position: relative;
  overflow: hidden;
}
.btn-upload:hover .btn-upload-text { text-decoration: underline; }

.up-fill {
  position: absolute;
  left: 0; top: 0;
  height: 100%; width: 0%;
  background: var(--ac);
  z-index: 0;
  transition: width 0.08s linear;
}
.btn-upload-text {
  position: relative;
  z-index: 1;
}

/* ── FILE TABLE ── */
.sec-receive { overflow-x: auto; }

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  font-family: 'Courier New', Courier, monospace;
}

thead th {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  font-weight: normal;
  text-align: left;
  padding: 0 8px 8px 0;
  border-bottom: 1px solid currentColor;
  white-space: nowrap;
}
thead th.r { text-align: right; padding-right: 0; }

tbody tr          { border-bottom: 1px solid currentColor; }
tbody tr:last-child { border-bottom: none; }

td { padding: 9px 8px 9px 0; vertical-align: middle; }
td:last-child { padding-right: 0; }

.c-num  { width: 28px; font-size: 11px; opacity: 0.35; white-space: nowrap; }
.c-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 140px; }
.c-size { font-size: 11px; opacity: 0.55; white-space: nowrap; }
.c-type { font-size: 11px; white-space: nowrap; }
.c-dl, .c-del { text-align: right; white-space: nowrap; width: 1%; }

.c-empty {
  text-align: center;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  opacity: 0.35;
  padding: 28px 0;
}

a.a-dl {
  color: var(--fg);
  text-decoration: none;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.02em;
  font-family: 'Courier New', Courier, monospace;
}
a.a-dl:hover { text-decoration: underline; }

.btn-del {
  background: none;
  border: none;
  cursor: pointer;
  font-family: 'Courier New', Courier, monospace;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.02em;
  color: var(--fg);
  padding: 0;
  opacity: 0.35;
  line-height: 1.1;
}
.btn-del:hover { text-decoration: underline; opacity: 1; }

/* ── FOOTER ── */
.footer {
  padding-top: 20px;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  opacity: 0.35;
}
</style>
</head>
<body>

<span class="title">FILE&#x2014;TRANSFER</span>

<div class="pg">

  <div class="meta-bar">
    <span class="meta-live"><span id="live-ind">[LIVE]</span> TMPL_IP:TMPL_PORT</span>
    <button class="btn-invert" id="btn-invert">[INVERT]</button>
  </div>

  <hr>

  <div class="grid">

    <section>
      <div class="sec-label">01 / SEND</div>
      <div class="zone" id="zone">
        <input type="file" id="file-input" multiple>
        DROP FILE HERE OR CLICK TO SELECT
      </div>
      <div class="sel-info" id="sel-info"></div>
      <div class="prog-track" id="prog-track">
        <div class="prog-fill" id="prog-fill"></div>
      </div>
      <button class="btn-upload" id="upload-btn">
        <div class="up-fill" id="up-fill"></div>
        <span class="btn-upload-text" id="upload-text">UPLOAD &#x2191;</span>
      </button>
    </section>

    <section class="sec-receive">
      <div class="sec-label">02 / RECEIVE</div>
      <table>
        <thead>
          <tr>
            <th class="c-num">#</th>
            <th>FILENAME</th>
            <th>SIZE</th>
            <th>TYPE</th>
            <th class="r">&#x2193;</th>
            <th class="r">&#x00D7;</th>
          </tr>
        </thead>
        <tbody id="file-tbody">TMPL_FILES</tbody>
      </table>
    </section>

  </div>

  <hr>

  <div class="footer">MAX TMPL_MAX_MB MB PER UPLOAD &#x2014; RUST/AXUM &#x2014; LOCAL NETWORK ONLY</div>

</div>

<script>
const CHUNK = TMPL_CHUNK_SIZE;

// ── Invert
document.getElementById('btn-invert').addEventListener('click', () => {
  document.body.classList.toggle('dark');
});

// ── LIVE pulse (setInterval, no CSS animation)
const liveEl = document.getElementById('live-ind');
setInterval(() => {
  liveEl.style.opacity = liveEl.style.opacity === '0.3' ? '1' : '0.3';
}, 1500);

// ── Press flash: opacity 0.5 for 100ms
document.addEventListener('pointerdown', e => {
  const t = e.target.closest('.btn-upload, .btn-invert, .btn-del, .a-dl');
  if (!t || t.disabled) return;
  t.style.opacity = '0.5';
  setTimeout(() => { t.style.opacity = ''; }, 100);
});

// ── Upload
const zone       = document.getElementById('zone');
const inp        = document.getElementById('file-input');
const selInfo    = document.getElementById('sel-info');
const progTrack  = document.getElementById('prog-track');
const progFill   = document.getElementById('prog-fill');
const upFill     = document.getElementById('up-fill');
const uploadText = document.getElementById('upload-text');
const uploadBtn  = document.getElementById('upload-btn');

let files = [], ctrl = null, busy = false, dragN = 0;

function fmtSize(b) {
  if (b < 1024)    return b + ' B';
  if (b < 1048576) return (b / 1024).toFixed(1) + ' KB';
  return (b / 1048576).toFixed(1) + ' MB';
}

function showSel() {
  if (!files.length) { selInfo.classList.remove('on'); return; }
  const total = files.reduce((s, f) => s + f.size, 0);
  selInfo.textContent = files.length === 1
    ? files[0].name + ' \u2014 ' + fmtSize(files[0].size) + ' \u2014 READY'
    : files.length + ' FILES \u2014 ' + fmtSize(total) + ' \u2014 READY';
  selInfo.classList.add('on');
}

function setPct(pct) {
  const p = Math.min(100, pct);
  progFill.style.width = p + '%';
  upFill.style.width   = p + '%';
  uploadText.textContent = Math.round(p) + '%';
}

function reset() {
  busy = false; ctrl = null;
  uploadText.textContent = 'UPLOAD \u2191';
  progFill.style.width = '0%';
  upFill.style.width   = '0%';
  progTrack.classList.remove('on');
  inp.value = ''; files = [];
  selInfo.classList.remove('on');
}

zone.addEventListener('click', () => { if (!busy) inp.click(); });
inp.addEventListener('change',  () => { files = [...inp.files]; showSel(); });

zone.addEventListener('dragenter', e => { e.preventDefault(); if (++dragN === 1) zone.classList.add('over'); });
zone.addEventListener('dragleave', () => { if (--dragN <= 0) { dragN = 0; zone.classList.remove('over'); } });
zone.addEventListener('dragover',  e => e.preventDefault());
zone.addEventListener('drop', e => {
  e.preventDefault(); dragN = 0; zone.classList.remove('over');
  if (!busy) { files = [...e.dataTransfer.files]; showSel(); }
});

uploadBtn.addEventListener('click', async () => {
  if (busy) { if (ctrl) ctrl.abort(); reset(); return; }
  if (!files.length) {
    zone.style.borderColor = 'var(--ac)';
    setTimeout(() => { zone.style.borderColor = ''; }, 700);
    return;
  }

  busy = true; ctrl = new AbortController();
  progTrack.classList.add('on');
  setPct(0);

  let total = 0, done = 0;
  for (const f of files) total += Math.max(1, Math.ceil(f.size / CHUNK));

  try {
    for (const file of files) {
      const id = await fetch('/upload/init', {
        method: 'POST',
        headers: { 'Content-Type': 'text/plain' },
        body: file.name,
        signal: ctrl.signal,
      }).then(r => r.text());

      const n = Math.max(1, Math.ceil(file.size / CHUNK));
      await Promise.all(Array.from({ length: n }, (_, i) =>
        fetch('/upload/chunk/' + id + '/' + i, {
          method: 'POST',
          body: file.slice(i * CHUNK, (i + 1) * CHUNK),
          signal: ctrl.signal,
        }).then(r => {
          if (!r.ok) throw new Error('chunk ' + i);
          done++;
          setPct(done / total * 100);
        })
      ));

      await fetch('/upload/complete/' + id, { method: 'POST', signal: ctrl.signal });
    }
    setPct(100);
    setTimeout(() => { window.location.href = '/'; }, 800);
  } catch(e) {
    if (e.name !== 'AbortError') console.error(e);
    reset();
  }
});

// ── Delete
async function delRow(btn) {
  const row  = btn.closest('tr');
  const name = row.dataset.name;
  btn.disabled = true;
  try {
    const r = await fetch('/delete/' + encodeURIComponent(name), { method: 'DELETE' });
    if (!r.ok) { btn.disabled = false; return; }
    row.remove();
    const rows = document.querySelectorAll('#file-tbody tr[data-name]');
    rows.forEach((r, i) => {
      const c = r.querySelector('.c-num');
      if (c) c.textContent = i + 1;
    });
    if (!rows.length) {
      document.getElementById('file-tbody').innerHTML =
        '<tr><td colspan="6" class="c-empty">\u2014 NO FILES \u2014</td></tr>';
    }
  } catch(e) { btn.disabled = false; }
}
</script>
</body>
</html>"##;
