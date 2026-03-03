pub const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>File Transfer</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  :root {
    --bg: #0f0f0f; --surface: #1a1a1a; --border: #2e2e2e;
    --accent: #3b82f6; --accent2: #22c55e; --text: #e5e5e5; --muted: #888;
  }
  body { background: var(--bg); color: var(--text);
         font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
         min-height: 100dvh; padding: 16px; }
  h1   { font-size: 1.4rem; font-weight: 700; margin-bottom: 4px; }
  .subtitle { color: var(--muted); font-size: .85rem; margin-bottom: 20px; }
  .card { background: var(--surface); border: 1px solid var(--border);
          border-radius: 12px; padding: 16px; margin-bottom: 16px; }
  .card h2 { font-size: 1rem; font-weight: 600; margin-bottom: 12px;
             display: flex; align-items: center; gap: 8px; }
  .drop-zone { border: 2px dashed var(--border); border-radius: 8px;
               padding: 28px 16px; text-align: center; cursor: pointer;
               transition: border-color .2s; }
  .drop-zone:hover, .drop-zone.over { border-color: var(--accent); }
  .drop-zone input { display: none; }
  .drop-zone p { color: var(--muted); font-size: .9rem; margin-top: 8px; }
  .btn { display: inline-flex; align-items: center; justify-content: center;
         gap: 6px; padding: 10px 18px; border-radius: 8px; border: none;
         font-size: .9rem; font-weight: 600; cursor: pointer;
         transition: opacity .15s; text-decoration: none; }
  .btn:hover { opacity: .85; }
  .btn-primary { background: var(--accent); color: #fff; width: 100%; margin-top: 10px; }
  .btn-success { background: var(--accent2); color: #fff; }
  .file-item { display: flex; align-items: center; justify-content: space-between;
               padding: 10px 0; border-bottom: 1px solid var(--border); gap: 10px; }
  .file-item:last-child { border-bottom: none; }
  .file-name { flex: 1; min-width: 0; }
  .file-name span { display: block; overflow: hidden; text-overflow: ellipsis;
                    white-space: nowrap; font-size: .9rem; }
  .file-size { color: var(--muted); font-size: .75rem; margin-top: 2px; }
  .dl-btn { flex-shrink: 0; }
  #progress-wrap { display: none; margin-top: 10px; }
  #progress-bar-bg { background: var(--border); border-radius: 99px; height: 8px; }
  #progress-bar { background: var(--accent); height: 8px; border-radius: 99px;
                  width: 0; transition: width .1s; }
  #progress-text { color: var(--muted); font-size: .8rem; margin-top: 6px; line-height: 1.7; }
  .hash-badge { font-family: monospace; font-size: .7rem; background: #1e3a5f;
                color: #93c5fd; padding: 2px 6px; border-radius: 4px;
                word-break: break-all; margin-left: 6px; }
  .empty { color: var(--muted); font-size: .9rem; text-align: center; padding: 16px 0; }
</style>
</head>
<body>
<h1> file upload and download</h1>
<p class="subtitle">Connected to TMPL_IP:TMPL_PORT</p>

<div class="card">
  <h2>Upload -></h2>
  <div class="drop-zone" id="drop-zone">
    <span style="font-size:2rem">📂</span>
    <p>Tap to choose files</p>
    <p style="font-size:.75rem;margin-top:4px">Max TMPL_MAX_MB MB upload</p>
    <input type="file" id="file-input" multiple>
  </div>
  <div id="selected-info" style="color:var(--muted);font-size:.85rem;margin-top:8px"></div>
  <div id="progress-wrap">
    <div id="progress-bar-bg"><div id="progress-bar"></div></div>
    <div id="progress-text"></div>
  </div>
  <button class="btn btn-primary" id="upload-btn">Upload</button>
</div>

<div class="card">
  <h2> Get from storage ->
    <span style="margin-left:auto;font-size:.8rem;color:var(--muted)">TMPL_FILE_COUNT file(s)</span>
  </h2>
  TMPL_FILES
</div>

<script>
const CHUNK_SIZE = TMPL_CHUNK_SIZE;

const zone  = document.getElementById('drop-zone');
const input = document.getElementById('file-input');
const info  = document.getElementById('selected-info');
const wrap  = document.getElementById('progress-wrap');
const bar   = document.getElementById('progress-bar');
const txt   = document.getElementById('progress-text');
const btn   = document.getElementById('upload-btn');

zone.addEventListener('click', () => input.click());
zone.addEventListener('dragover', e => { e.preventDefault(); zone.classList.add('over'); });
zone.addEventListener('dragleave', () => zone.classList.remove('over'));
zone.addEventListener('drop', e => {
  e.preventDefault(); zone.classList.remove('over');
  input.files = e.dataTransfer.files; updateInfo();
});
input.addEventListener('change', updateInfo);

function updateInfo() {
  const files = [...input.files];
  if (!files.length) { info.textContent = ''; return; }
  const total = files.reduce((a, f) => a + f.size, 0);
  info.textContent = files.length + ' file(s) — ' + fmtSize(total);
}
function fmtSize(b) {
  if (b < 1024) return b + ' B';
  if (b < 1048576) return (b / 1024).toFixed(1) + ' KB';
  return (b / 1048576).toFixed(1) + ' MB';
}
function esc(s) {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

btn.addEventListener('click', async () => {
  const files = [...input.files];
  if (!files.length) { alert('Please choose at least one file.'); return; }

  btn.disabled = true;
  wrap.style.display = 'block';
  bar.style.width = '0';
  txt.innerHTML = '';

  const results = [];

  for (let fi = 0; fi < files.length; fi++) {
    const file = files[fi];

    const id = await fetch('/upload/init', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: file.name,
    }).then(r => r.text());

    const totalChunks = Math.max(1, Math.ceil(file.size / CHUNK_SIZE));
    let done = 0;

    const updateBar = () => {
      const overall = (fi + done / totalChunks) / files.length;
      bar.style.width = Math.round(overall * 100) + '%';
      txt.innerHTML =
        results.map(r =>
          `✓ ${esc(r.name)} <span class="hash-badge">${r.hash.slice(0, 16)}…</span>`
        ).join('<br>') +
        (results.length ? '<br>' : '') +
        `${esc(file.name)}: ${done}/${totalChunks} chunks`;
    };
    updateBar();

    const uploads = [];
    for (let i = 0; i < totalChunks; i++) {
      const chunk = file.slice(i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE);
      uploads.push(
        fetch(`/upload/chunk/${id}/${i}`, { method: 'POST', body: chunk })
          .then(r => { if (!r.ok) throw new Error(`chunk ${i} failed`); done++; updateBar(); })
      );
    }
    await Promise.all(uploads);

    const res = await fetch(`/upload/complete/${id}`, { method: 'POST' });
    const hash = res.headers.get('X-File-Hash') || '(no hash)';
    results.push({ name: file.name, hash });
    updateBar();
  }

  bar.style.width = '100%';
  txt.innerHTML = results.map(r =>
    `✓ ${esc(r.name)} <span class="hash-badge">${r.hash.slice(0, 16)}…</span>`
  ).join('<br>');
  setTimeout(() => { window.location.href = '/'; }, 1800);
});
</script>
</body>
</html>"#;
