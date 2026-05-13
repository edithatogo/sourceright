async function loadJson(path) {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`Failed to load ${path}: ${response.status}`);
  }
  return response.json();
}

function metric(label, value) {
  return `<div class="metric"><strong>${value}</strong><span>${label}</span></div>`;
}

function renderReport(report) {
  const summary = report.summary;
  document.querySelector("#status").textContent =
    `${report.report_type} / ${report.schema_version}`;
  document.querySelector("#report-note").textContent =
    "Synthetic sample data only. One reference is verified, one is queued for review, and this page does not call live providers or require API keys.";
  document.querySelector("#metrics").innerHTML = [
    metric("References", summary.total_references),
    metric("Verified", summary.verified_references),
    metric("Review queue", summary.review_queue_count),
    metric("AI-risk signals", summary.ai_risk_issue_count),
    metric("Warnings", summary.warning_count),
    metric("Errors", summary.error_count)
  ].join("");

  const issues = report.issues.length
    ? report.issues.map((issue) => `
      <section class="issue">
        <strong class="severity-${issue.severity}">${issue.severity}</strong>
        <div>${issue.message}</div>
        <div class="issue-meta">${issue.category} / ${issue.code} / ${issue.reference_id ?? "whole-report"}</div>
      </section>
    `).join("")
    : "<p>No open issues in the sample report.</p>";
  document.querySelector("#issues").innerHTML = issues;
}

function renderJournal(report) {
  document.querySelector("#journal").innerHTML = `
    <dt>Submission</dt><dd>${report.submission_id}</dd>
    <dt>Platform</dt><dd>${report.platform}</dd>
    <dt>Status</dt><dd>${report.status}</dd>
    <dt>Checklist</dt><dd>${report.author_action_checklist.join("<br>")}</dd>
  `;
}

Promise.all([
  loadJson("sample/reference-report.json"),
  loadJson("sample/journal-screening.json")
])
  .then(([referenceReport, journalReport]) => {
    renderReport(referenceReport);
    renderJournal(journalReport);
  })
  .catch((error) => {
    document.querySelector("#status").textContent = error.message;
  });
