fetch('sample/reference-report.json').then(r => r.json()).then(data => {
  document.getElementById('out').textContent = JSON.stringify(data, null, 2);
}).catch(err => { document.getElementById('out').textContent = String(err); });
