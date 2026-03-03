pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>File Transfer</title>
<style>
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

html { background: #000; }

body {
  background: #000;
  color: #fff;
  font-family: -apple-system, 'SF Pro Display', 'SF Pro Text', BlinkMacSystemFont, sans-serif;
  min-height: 100dvh;
  -webkit-tap-highlight-color: transparent;
  -webkit-text-size-adjust: 100%;
  user-select: none;
  scroll-behavior: smooth;
  padding: calc(env(safe-area-inset-top, 0px) + 32px)
           calc(env(safe-area-inset-right, 0px) + 20px)
           calc(env(safe-area-inset-bottom, 0px) + 40px)
           calc(env(safe-area-inset-left, 0px) + 20px);
  max-width: 500px;
  margin: 0 auto;
}

/* ── Header ── */
.hdr { margin-bottom: 32px; }

.page-title {
  font-size: 34px;
  font-weight: 700;
  letter-spacing: -0.5px;
  line-height: 1.1;
  margin-bottom: 12px;
}

.live-pill {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  background: rgba(255,255,255,0.08);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 999px;
  padding: 6px 14px 6px 10px;
  font-family: 'SF Mono', ui-monospace, 'Courier New', monospace;
  font-size: 13px;
  color: rgba(255,255,255,0.6);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
}

.live-dot {
  width: 7px;
  height: 7px;
  background: #30D158;
  border-radius: 50%;
  flex-shrink: 0;
  animation: pulse 2s ease infinite;
}

@keyframes pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(48,209,88,0.5); }
  50%       { box-shadow: 0 0 0 7px rgba(48,209,88,0); }
}

/* ── Cards ── */
.card {
  background: rgba(255,255,255,0.06);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 22px;
  padding: 20px;
  margin-bottom: 16px;
  box-shadow: inset 0 1px 0 rgba(255,255,255,0.08);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
}

/* ── Drop zone ── */
.zone {
  border: 1.5px dashed rgba(255,255,255,0.18);
  border-radius: 16px;
  padding: 36px 16px;
  text-align: center;
  cursor: pointer;
  transition: transform 0.25s ease, border-color 0.25s ease, box-shadow 0.25s ease;
}

.zone.over {
  transform: scale(1.02);
  border-color: rgba(10,132,255,0.55);
  box-shadow: 0 0 0 4px rgba(10,132,255,0.15);
}

.zone input { display: none; }

.zone-icon {
  width: 56px;
  height: 56px;
  background: rgba(10,132,255,0.12);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 14px;
}

.zone-label {
  font-size: 16px;
  font-weight: 500;
  color: rgba(255,255,255,0.85);
  margin-bottom: 4px;
}

.zone-hint {
  font-size: 12px;
  color: rgba(255,255,255,0.3);
}

/* ── Selected file pill ── */
.sel-pill {
  display: none;
  align-items: center;
  gap: 10px;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.08);
  border-radius: 12px;
  padding: 11px 14px;
  margin-top: 12px;
}

.sel-pill.on { display: flex; }

.sel-fname {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  user-select: text;
  color: rgba(255,255,255,0.9);
}

.sel-fsize {
  font-size: 12px;
  color: rgba(255,255,255,0.4);
  white-space: nowrap;
  flex-shrink: 0;
}

/* ── Progress ring ── */
/* ── Hamster loader ── */
.ring-wrap {
  display: none;
  align-items: center;
  justify-content: center;
  padding: 28px 0;
}

.ring-wrap.on { display: flex; }

.wheel-and-hamster {
  --dur: 1s;
  position: relative;
  width: 8em;
  height: 8em;
  font-size: 14px;
}
.wheel, .hamster, .hamster div, .spoke { position: absolute; }
.wheel, .spoke { border-radius: 50%; top: 0; left: 0; width: 100%; height: 100%; }
.wheel {
  background: radial-gradient(100% 100% at center,hsla(0,0%,60%,0) 47.8%,hsl(0,0%,60%) 48%);
  z-index: 2;
}
.hamster {
  animation: hamster var(--dur) ease-in-out infinite;
  top: 50%; left: calc(50% - 3.5em);
  width: 7em; height: 3.75em;
  transform: rotate(4deg) translate(-0.8em,1.85em);
  transform-origin: 50% 0;
  z-index: 1;
}
.hamster__head {
  animation: hamsterHead var(--dur) ease-in-out infinite;
  background: hsl(30,90%,55%);
  border-radius: 70% 30% 0 100% / 40% 25% 25% 60%;
  box-shadow: 0 -0.25em 0 hsl(30,90%,80%) inset, 0.75em -1.55em 0 hsl(30,90%,90%) inset;
  top: 0; left: -2em; width: 2.75em; height: 2.5em;
  transform-origin: 100% 50%;
}
.hamster__ear {
  animation: hamsterEar var(--dur) ease-in-out infinite;
  background: hsl(0,90%,85%);
  border-radius: 50%;
  box-shadow: -0.25em 0 hsl(30,90%,55%) inset;
  top: -0.25em; right: -0.25em; width: 0.75em; height: 0.75em;
  transform-origin: 50% 75%;
}
.hamster__eye {
  animation: hamsterEye var(--dur) linear infinite;
  background-color: hsl(0,0%,0%);
  border-radius: 50%;
  top: 0.375em; left: 1.25em; width: 0.5em; height: 0.5em;
}
.hamster__nose {
  background: hsl(0,90%,75%);
  border-radius: 35% 65% 85% 15% / 70% 50% 50% 30%;
  top: 0.75em; left: 0; width: 0.2em; height: 0.25em;
}
.hamster__body {
  animation: hamsterBody var(--dur) ease-in-out infinite;
  background: hsl(30,90%,90%);
  border-radius: 50% 30% 50% 30% / 15% 60% 40% 40%;
  box-shadow: 0.1em 0.75em 0 hsl(30,90%,55%) inset, 0.15em -0.5em 0 hsl(30,90%,80%) inset;
  top: 0.25em; left: 2em; width: 4.5em; height: 3em;
  transform-origin: 17% 50%; transform-style: preserve-3d;
}
.hamster__limb--fr, .hamster__limb--fl {
  clip-path: polygon(0 0,100% 0,70% 80%,60% 100%,0% 100%,40% 80%);
  top: 2em; left: 0.5em; width: 1em; height: 1.5em; transform-origin: 50% 0;
}
.hamster__limb--fr {
  animation: hamsterFRLimb var(--dur) linear infinite;
  background: linear-gradient(hsl(30,90%,80%) 80%,hsl(0,90%,75%) 80%);
  transform: rotate(15deg) translateZ(-1px);
}
.hamster__limb--fl {
  animation: hamsterFLLimb var(--dur) linear infinite;
  background: linear-gradient(hsl(30,90%,90%) 80%,hsl(0,90%,85%) 80%);
  transform: rotate(15deg);
}
.hamster__limb--br, .hamster__limb--bl {
  border-radius: 0.75em 0.75em 0 0;
  clip-path: polygon(0 0,100% 0,100% 30%,70% 90%,70% 100%,30% 100%,40% 90%,0% 30%);
  top: 1em; left: 2.8em; width: 1.5em; height: 2.5em; transform-origin: 50% 30%;
}
.hamster__limb--br {
  animation: hamsterBRLimb var(--dur) linear infinite;
  background: linear-gradient(hsl(30,90%,80%) 90%,hsl(0,90%,75%) 90%);
  transform: rotate(-25deg) translateZ(-1px);
}
.hamster__limb--bl {
  animation: hamsterBLLimb var(--dur) linear infinite;
  background: linear-gradient(hsl(30,90%,90%) 90%,hsl(0,90%,85%) 90%);
  transform: rotate(-25deg);
}
.hamster__tail {
  animation: hamsterTail var(--dur) linear infinite;
  background: hsl(0,90%,85%);
  border-radius: 0.25em 50% 50% 0.25em;
  box-shadow: 0 -0.2em 0 hsl(0,90%,75%) inset;
  top: 1.5em; right: -0.5em; width: 1em; height: 0.5em;
  transform: rotate(30deg) translateZ(-1px);
  transform-origin: 0.25em 0.25em;
}
.spoke {
  animation: spoke var(--dur) linear infinite;
  background: radial-gradient(100% 100% at center,hsl(0,0%,60%) 4.8%,hsla(0,0%,60%,0) 5%),
    linear-gradient(hsla(0,0%,55%,0) 46.9%,hsl(0,0%,65%) 47% 52.9%,hsla(0,0%,65%,0) 53%) 50% 50% / 99% 99% no-repeat;
}
@keyframes hamster {
  from, to { transform: rotate(4deg) translate(-0.8em,1.85em); }
  50%       { transform: rotate(0)    translate(-0.8em,1.85em); }
}
@keyframes hamsterHead {
  from, 25%, 50%, 75%, to  { transform: rotate(0); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(8deg); }
}
@keyframes hamsterEye {
  from, 90%, to { transform: scaleY(1); }
  95%           { transform: scaleY(0); }
}
@keyframes hamsterEar {
  from, 25%, 50%, 75%, to    { transform: rotate(0); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(12deg); }
}
@keyframes hamsterBody {
  from, 25%, 50%, 75%, to    { transform: rotate(0); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-2deg); }
}
@keyframes hamsterFRLimb {
  from, 25%, 50%, 75%, to    { transform: rotate(50deg) translateZ(-1px); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-30deg) translateZ(-1px); }
}
@keyframes hamsterFLLimb {
  from, 25%, 50%, 75%, to    { transform: rotate(-30deg); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(50deg); }
}
@keyframes hamsterBRLimb {
  from, 25%, 50%, 75%, to    { transform: rotate(-60deg) translateZ(-1px); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(20deg) translateZ(-1px); }
}
@keyframes hamsterBLLimb {
  from, 25%, 50%, 75%, to    { transform: rotate(20deg); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(-60deg); }
}
@keyframes hamsterTail {
  from, 25%, 50%, 75%, to    { transform: rotate(30deg) translateZ(-1px); }
  12.5%, 37.5%, 62.5%, 87.5% { transform: rotate(10deg) translateZ(-1px); }
}
@keyframes spoke {
  from { transform: rotate(0); }
  to   { transform: rotate(-1turn); }
}

/* ── Upload button ── */
.btn-send {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  margin-top: 14px;
  padding: 14px 20px;
  border: none;
  border-radius: 999px;
  background: #0A84FF;
  color: #fff;
  font-family: inherit;
  font-size: 17px;
  font-weight: 600;
  letter-spacing: -0.2px;
  cursor: pointer;
  overflow: hidden;
  -webkit-tap-highlight-color: transparent;
  transition: transform 0.1s cubic-bezier(0.25, 0.46, 0.45, 0.94),
              background 0.2s ease;
}

.btn-send:active { transform: scale(0.97); }
.btn-send.cancel { background: #FF453A; }

.btn-icon-wrap { display: flex; align-items: center; }

.btn-icon svg {
  display: block;
  transform-origin: center center;
  transition: transform 0.3s ease-in-out;
}

.btn-label {
  display: block;
  margin-left: 0.4em;
  transition: transform 0.4s ease-in-out, opacity 0.3s ease-in-out;
}

.btn-send:not(.cancel):hover .btn-icon {
  animation: fly-1 0.6s ease-in-out infinite alternate;
}
.btn-send:not(.cancel):hover .btn-icon svg {
  transform: translateX(0.9em) rotate(45deg) scale(1.1);
}
.btn-send:not(.cancel):hover .btn-label {
  transform: translateX(30em);
  opacity: 0;
}

@keyframes fly-1 {
  from { transform: translateY(0.1em); }
  to   { transform: translateY(-0.1em); }
}

/* Cancel state — hide the icon so text is centred */
.btn-send.cancel .btn-icon-wrap { display: none; }
.btn-send.cancel .btn-label     { margin-left: 0; }

/* ── File list header ── */
.list-hdr {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 4px;
}

.list-title {
  font-size: 17px;
  font-weight: 600;
  letter-spacing: -0.2px;
}

.count-pill {
  font-size: 12px;
  color: rgba(255,255,255,0.4);
  background: rgba(255,255,255,0.07);
  border-radius: 999px;
  padding: 3px 10px;
}

/* ── File rows ── */
.file-list { margin-top: 8px; }

.file-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 8px;
  margin: 0 -8px;
  border-radius: 10px;
  border-bottom: 0.5px solid rgba(255,255,255,0.06);
  transition: background 0.1s;
}

.file-row:last-child { border-bottom: none; }
.file-row:active { background: rgba(255,255,255,0.05); }

/* File type icons */
.fi {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 9px;
  font-family: 'SF Mono', ui-monospace, monospace;
  font-weight: 700;
  letter-spacing: 0.3px;
  flex-shrink: 0;
  color: rgba(255,255,255,0.95);
}

.fi-img { background: #0A84FF; }
.fi-vid { background: #BF5AF2; }
.fi-aud { background: #FF9F0A; }
.fi-doc { background: #FF453A; }
.fi-arc { background: #FFD60A; color: rgba(0,0,0,0.75); }
.fi-gen { background: rgba(255,255,255,0.12); }

.finfo { flex: 1; min-width: 0; }

.fname {
  display: block;
  font-size: 15px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 3px;
  user-select: text;
  color: rgba(255,255,255,0.95);
}

.fsize {
  display: inline-block;
  font-size: 11px;
  color: rgba(255,255,255,0.4);
  background: rgba(255,255,255,0.07);
  border-radius: 999px;
  padding: 2px 8px;
}

/* ── Download button ── */
.btn-dl {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  background: #30D158;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  text-decoration: none;
  -webkit-tap-highlight-color: transparent;
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.btn-dl:active {
  transform: scale(0.86);
  transition: transform 0.08s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

/* ── Empty state ── */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 36px 16px 24px;
  gap: 8px;
}

.empty-state svg { opacity: 0.2; margin-bottom: 6px; }

.empty-title {
  font-size: 17px;
  font-weight: 600;
  color: rgba(255,255,255,0.5);
}

.empty-sub {
  font-size: 13px;
  color: rgba(255,255,255,0.25);
  text-align: center;
  line-height: 1.5;
  max-width: 220px;
}
</style>
</head>
<body>

<!-- Header -->
<div class="hdr">
  <div class="page-title">File Transfer</div>
  <div class="live-pill">
    <div class="live-dot"></div>TMPL_IP:TMPL_PORT
  </div>
</div>

<!-- Upload card -->
<div class="card">
  <div class="zone" id="zone">
    <input type="file" id="file-input" multiple>
    <div class="zone-icon">
      <svg width="26" height="26" viewBox="0 0 26 26" fill="none">
        <path d="M13 20V6M7 12l6-7 6 7" stroke="#0A84FF" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M4 22h18" stroke="#0A84FF" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </div>
    <div class="zone-label">Tap to choose files</div>
    <div class="zone-hint">Max TMPL_MAX_MB MB per file</div>
  </div>

  <div class="sel-pill" id="sel-pill">
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" style="flex-shrink:0;opacity:0.35">
      <rect x="1.5" y="1.5" width="12" height="12" rx="2" stroke="white" stroke-width="1.5"/>
    </svg>
    <span class="sel-fname" id="sel-fname"></span>
    <span class="sel-fsize" id="sel-fsize"></span>
  </div>

  <div class="ring-wrap" id="ring-wrap">
    <div class="wheel-and-hamster" aria-label="Uploading…" role="img">
      <div class="wheel"></div>
      <div class="hamster">
        <div class="hamster__body">
          <div class="hamster__head">
            <div class="hamster__ear"></div>
            <div class="hamster__eye"></div>
            <div class="hamster__nose"></div>
          </div>
          <div class="hamster__limb hamster__limb--fr"></div>
          <div class="hamster__limb hamster__limb--fl"></div>
          <div class="hamster__limb hamster__limb--br"></div>
          <div class="hamster__limb hamster__limb--bl"></div>
          <div class="hamster__tail"></div>
        </div>
      </div>
      <div class="spoke"></div>
    </div>
  </div>

  <button class="btn-send" id="upload-btn">
    <div class="btn-icon-wrap">
      <div class="btn-icon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="20" height="20">
          <path fill="none" d="M0 0h24v24H0z"/>
          <path fill="currentColor" d="M1.946 9.315c-.522-.174-.527-.455.01-.634l19.087-6.362c.529-.176.832.12.684.638l-5.454 19.086c-.15.529-.455.547-.679.045L12 14l6-8-8 6-8.054-2.685z"/>
        </svg>
      </div>
    </div>
    <span class="btn-label">Upload</span>
  </button>
</div>

<!-- Files card -->
<div class="card">
  <div class="list-hdr">
    <span class="list-title">Files</span>
    <span class="count-pill">TMPL_FILE_COUNT</span>
  </div>
  <div class="file-list">TMPL_FILES</div>
</div>

<script>
const CHUNK = TMPL_CHUNK_SIZE;

const zone    = document.getElementById('zone');
const inp     = document.getElementById('file-input');
const selPill = document.getElementById('sel-pill');
const selFn   = document.getElementById('sel-fname');
const selFs   = document.getElementById('sel-fsize');
const ringW   = document.getElementById('ring-wrap');
const btn     = document.getElementById('upload-btn');

let files = [];
let ctrl  = null;
let busy  = false;
let dragN = 0;

function fmtSize(b) {
  if (b < 1024)    return b + ' B';
  if (b < 1048576) return (b / 1024).toFixed(1) + ' KB';
  return (b / 1048576).toFixed(1) + ' MB';
}

function showSel() {
  if (!files.length) { selPill.classList.remove('on'); return; }
  const total = files.reduce((s, f) => s + f.size, 0);
  selFn.textContent = files.length === 1
    ? files[0].name
    : files.length + ' files selected';
  selFs.textContent = fmtSize(total);
  selPill.classList.add('on');
}

function reset() {
  busy = false;
  ctrl = null;
  btn.querySelector('.btn-label').textContent = 'Upload';
  btn.classList.remove('cancel');
  ringW.classList.remove('on');
  inp.value = '';
  files = [];
  selPill.classList.remove('on');
}

zone.addEventListener('click', () => { if (!busy) inp.click(); });
inp.addEventListener('change', () => { files = [...inp.files]; showSel(); });

zone.addEventListener('dragenter', e => {
  e.preventDefault(); dragN++;
  zone.classList.add('over');
});
zone.addEventListener('dragleave', () => {
  dragN--;
  if (dragN <= 0) { dragN = 0; zone.classList.remove('over'); }
});
zone.addEventListener('dragover', e => e.preventDefault());
zone.addEventListener('drop', e => {
  e.preventDefault(); dragN = 0; zone.classList.remove('over');
  if (!busy) { files = [...e.dataTransfer.files]; showSel(); }
});

btn.addEventListener('click', async () => {
  if (busy) {
    if (ctrl) ctrl.abort();
    return;
  }
  if (!files.length) {
    zone.style.borderColor = 'rgba(255,69,58,0.6)';
    setTimeout(() => { zone.style.borderColor = ''; }, 700);
    return;
  }

  busy = true;
  ctrl = new AbortController();
  btn.querySelector('.btn-label').textContent = 'Cancel';
  btn.classList.add('cancel');
  selPill.classList.remove('on');
  ringW.classList.add('on');

  try {
    for (const file of files) {
      const id = await fetch('/upload/init', {
        method: 'POST',
        headers: { 'Content-Type': 'text/plain' },
        body: file.name,
        signal: ctrl.signal,
      }).then(r => r.text());

      const n = Math.max(1, Math.ceil(file.size / CHUNK));

      await Promise.all(
        Array.from({ length: n }, (_, i) =>
          fetch(`/upload/chunk/${id}/${i}`, {
            method: 'POST',
            body: file.slice(i * CHUNK, (i + 1) * CHUNK),
            signal: ctrl.signal,
          }).then(r => {
            if (!r.ok) throw new Error('chunk ' + i + ' failed');
          })
        )
      );

      await fetch(`/upload/complete/${id}`, {
        method: 'POST',
        signal: ctrl.signal,
      });
    }

    setTimeout(() => { window.location.href = '/'; }, 900);
  } catch (e) {
    if (e.name !== 'AbortError') console.error('Upload failed:', e);
    reset();
  }
});
</script>
</body>
</html>"##;
