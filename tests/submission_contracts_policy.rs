use std::fs;
use std::path::Path;

use serde_json::Value;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn track_72_submission_requirements_contract_is_completed() {
    let metadata = read_json("conductor/tracks/72-submission-requirements-contracts/metadata.json");
    assert_eq!(
        metadata["status"], "completed",
        "track 72 should be completed"
    );

    let handoff = read(
        "conductor/tracks/72-submission-requirements-contracts/downstream-requirements-handoff.md",
    );
    let approval =
        read("conductor/tracks/72-submission-requirements-contracts/approval-boundary.md");
    let review = read("conductor/tracks/72-submission-requirements-contracts/review.md");

    for (id, marker) in [
        ("73", "73 MCP directory submission hardening"),
        ("74", "74 Citation-manager publication hardening"),
        ("75", "75 Journal-platform publication hardening"),
        ("76", "76 AI client extension publication hardening"),
        ("77", "77 VS Code and Open VSX publication hardening"),
    ] {
        assert!(
            handoff.contains(marker),
            "handoff missing downstream track {id}"
        );
    }

    assert!(approval.contains("external_submission_allowed: false"));
    assert!(approval.contains("explicit human consent"));
    assert!(review.contains("contracted"));
    assert!(review.contains("No external submission was created"));
}

#[test]
fn submission_contract_covers_requested_surfaces_and_evidence_gates() {
    let contract = read("conductor/submission-contracts.md");
    let docs = read("docs/src/submission-contracts.md");
    let docs_site = read("docs-site/src/content/docs/submission-contracts.md");

    for surface in [
        "Official MCP Registry",
        "Smithery",
        "Glama",
        "Zotero",
        "EndNote",
        "OJS/PKP",
        "arXiv submit-ce",
        "arXiv submission-core",
        "Claude Cowork",
        "Codex app",
        "GitHub Copilot",
        "Gemini CLI extensions",
        "Qwen CLI extensions",
        "VS Code / Open VSX",
    ] {
        assert!(contract.contains(surface), "missing surface {surface}");
    }

    for gate in [
        "Requirements searched",
        "Contracted",
        "Hardened local package",
        "Submission-ready",
        "Submitted",
        "Publicly accepted",
        "external issue or pull request URL",
        "explicit human approval gate",
    ] {
        assert!(contract.contains(gate), "missing gate {gate}");
    }

    assert!(docs.contains("Submission Contracts"));
    assert!(docs_site.contains("Submission Contracts"));
    assert!(docs.contains("External submissions require explicit approval"));
    assert!(docs_site.contains("External submissions require explicit approval"));
}

#[test]
fn submission_tracks_exist_for_registry_plugin_client_and_editor_maturity() {
    let tracks = read("conductor/tracks.md");
    let order = read("conductor/implementation-order.md");
    let ledger = read("conductor/evidence-ledger.json");

    for (id, marker) in [
        (
            "72-submission-requirements-contracts",
            "72 submission requirements contracts",
        ),
        (
            "73-mcp-directory-submission-hardening",
            "73 MCP directory submission hardening",
        ),
        (
            "74-citation-manager-publication-hardening",
            "74 citation-manager publication hardening",
        ),
        (
            "75-journal-platform-publication-hardening",
            "75 journal-platform publication hardening",
        ),
        (
            "76-ai-client-extension-publication-hardening",
            "76 AI client extension publication hardening",
        ),
        (
            "77-vscode-open-vsx-publication-hardening",
            "77 VS Code and Open VSX publication hardening",
        ),
    ] {
        let base = format!("conductor/tracks/{id}");
        assert!(Path::new(&format!("{base}/metadata.json")).exists());
        assert!(Path::new(&format!("{base}/spec.md")).exists());
        assert!(Path::new(&format!("{base}/plan.md")).exists());
        assert!(Path::new(&format!("{base}/test-matrix.md")).exists());
        assert!(tracks.contains(marker), "tracks.md missing {marker}");
        assert!(ledger.contains(id), "ledger missing {id}");
    }

    assert!(order.contains("Submission Hardening"));
    assert!(order.contains("Track 72"));
    assert!(order.contains("Track 77"));
}

#[test]
fn arxiv_submission_tracks_are_granular_and_high_maturity() {
    let contract = read("conductor/submission-contracts.md");
    let requirements = read("conductor/requirements.md");
    let tracks = read("conductor/tracks.md");
    let order = read("conductor/implementation-order.md");
    let ledger = read("conductor/evidence-ledger.json");

    for (id, marker) in [
        (
            "78-arxiv-upstream-requirements-recon",
            "78 arXiv upstream requirements reconnaissance",
        ),
        (
            "79-arxiv-submit-ce-maturity-hardening",
            "79 arXiv submit-ce maturity hardening",
        ),
        (
            "80-arxiv-submission-core-maturity-hardening",
            "80 arXiv submission-core maturity hardening",
        ),
        (
            "81-arxiv-upstream-submission-and-acceptance",
            "81 arXiv upstream submission and acceptance",
        ),
    ] {
        let base = format!("conductor/tracks/{id}");
        let spec = read(&format!("{base}/spec.md"));
        let plan = read(&format!("{base}/plan.md"));
        let matrix = read(&format!("{base}/test-matrix.md"));
        let metadata_text = read(&format!("{base}/metadata.json"));
        let metadata: serde_json::Value =
            serde_json::from_str(&metadata_text).expect("metadata should parse");

        assert!(tracks.contains(marker), "tracks.md missing {marker}");
        assert!(ledger.contains(id), "ledger missing {id}");
        assert!(spec.contains("maturity"));
        assert!(spec.contains("stability"));
        assert!(spec.contains("testing"));
        assert!(plan.contains("Do not submit upstream"));
        assert!(!plan.contains("acceptance gates pass"));
        assert!(matrix.contains("Default-CI"));
        let dependencies = metadata["dependencies"]
            .as_array()
            .expect("metadata dependencies")
            .iter()
            .filter_map(|value| value.as_str())
            .collect::<Vec<_>>();
        match id {
            "78-arxiv-upstream-requirements-recon" => {
                assert!(dependencies.contains(&"71-arxiv-submission-platform-adapters"));
                assert!(dependencies.contains(&"72-submission-requirements-contracts"));
            }
            "79-arxiv-submit-ce-maturity-hardening"
            | "80-arxiv-submission-core-maturity-hardening" => {
                assert!(dependencies.contains(&"71-arxiv-submission-platform-adapters"));
                assert!(dependencies.contains(&"78-arxiv-upstream-requirements-recon"));
            }
            "81-arxiv-upstream-submission-and-acceptance" => {
                assert!(dependencies.contains(&"78-arxiv-upstream-requirements-recon"));
                assert!(dependencies.contains(&"79-arxiv-submit-ce-maturity-hardening"));
                assert!(dependencies.contains(&"80-arxiv-submission-core-maturity-hardening"));
            }
            _ => unreachable!("unexpected arXiv track id"),
        }
    }

    for marker in [
        "arXiv/submit-ce",
        "arXiv/arxiv-submission-core",
        "maturity, stability, and testing gates",
        "Sourceright does not submit papers or mutate arXiv state",
    ] {
        assert!(contract.contains(marker), "contract missing {marker}");
    }

    assert!(order.contains("Track 78"));
    assert!(order.contains("Track 81"));
    assert!(requirements.contains("Submission requirements contracts"));
    assert!(requirements.contains("arXiv upstream submission contracts"));
    assert!(requirements.contains("72"));
    assert!(requirements.contains("78, 79, 80, 81"));

    let track_81_metadata =
        read("conductor/tracks/81-arxiv-upstream-submission-and-acceptance/metadata.json");
    assert!(track_81_metadata.contains("conductor/submission-contracts.md"));
    assert!(!track_81_metadata.contains("69-marketplace-submission-evidence"));
}

#[test]
fn host_plugin_submission_tracks_are_scaffolded() {
    let tracks = read("conductor/tracks.md");
    let order = read("conductor/implementation-order.md");
    let ledger = read("conductor/evidence-ledger.json");
    let contract = read("conductor/submission-contracts.md");

    for (id, marker) in [
        (
            "83-vscode-open-vsx-submission-and-acceptance",
            "83 VS Code and Open VSX submission and acceptance",
        ),
        (
            "84-claude-desktop-package-submission-and-acceptance",
            "84 Claude Desktop package submission and acceptance",
        ),
        (
            "85-codex-app-package-submission-and-acceptance",
            "85 Codex app package submission and acceptance",
        ),
        (
            "86-github-copilot-extension-submission-and-acceptance",
            "86 GitHub Copilot extension submission and acceptance",
        ),
        (
            "87-gemini-cli-extension-submission-and-acceptance",
            "87 Gemini CLI extension submission and acceptance",
        ),
        (
            "88-qwen-cli-extension-submission-and-acceptance",
            "88 Qwen CLI extension submission and acceptance",
        ),
        (
            "89-opencode-plugin-submission-and-acceptance",
            "89 OpenCode plugin submission and acceptance",
        ),
        (
            "90-cline-mcp-marketplace-submission-and-acceptance",
            "90 Cline MCP Marketplace submission and acceptance",
        ),
    ] {
        let base = format!("conductor/tracks/{id}");
        assert!(Path::new(&format!("{base}/metadata.json")).exists());
        assert!(Path::new(&format!("{base}/spec.md")).exists());
        assert!(Path::new(&format!("{base}/plan.md")).exists());
        assert!(Path::new(&format!("{base}/test-matrix.md")).exists());
        assert!(Path::new(&format!("{base}/requirements-evidence.md")).exists());
        assert!(Path::new(&format!("{base}/submission-drafts.md")).exists());
        assert!(tracks.contains(marker), "tracks.md missing {marker}");
        assert!(ledger.contains(id), "ledger missing {id}");
    }

    assert!(order.contains("Host Plugin Submission and Acceptance"));
    assert!(order.contains("Track 83"));
    assert!(order.contains("Track 90"));
    assert!(contract.contains("OpenCode"));
    assert!(contract.contains("Cline MCP Marketplace"));
}

#[test]
fn submission_requirements_inventory_is_machine_readable_and_complete() {
    let inventory = read_json("conductor/submission-requirements.json");
    let contract = read("conductor/submission-contracts.md");
    let track_82 = read("conductor/tracks/82-self-improving-submission-and-health-loop/spec.md");

    assert_eq!(
        inventory["schema_version"],
        "sourceright.submission_requirements.v1"
    );
    assert_eq!(
        inventory["$schema"],
        "../schemas/sourceright.submission-requirements.schema.json"
    );
    let schema = read_json("schemas/sourceright.submission-requirements.schema.json");
    assert_eq!(schema["$id"], inventory["$schema"]);
    assert!(
        inventory["repo_health_target"]
            .as_f64()
            .expect("repo health target should be numeric")
            >= 9.5
    );
    assert_eq!(
        inventory["self_improving_controls"]["readiness_script"],
        "scripts/verify-submission-readiness.ps1"
    );
    assert_eq!(
        inventory["self_improving_controls"]["ci_workflow"],
        ".github/workflows/submission-readiness.yml"
    );
    assert_eq!(
        inventory["self_improving_controls"]["packet_manifest"],
        "conductor/submission-packets/manifest.json"
    );

    let surfaces = inventory["surfaces"]
        .as_array()
        .expect("surfaces should be an array");
    let required_surfaces = [
        "official-mcp-registry",
        "smithery",
        "glama",
        "zotero",
        "endnote",
        "ojs-pkp",
        "arxiv-submit-ce",
        "arxiv-submission-core",
        "claude-cowork",
        "codex-app",
        "github-copilot",
        "gemini-cli-extensions",
        "qwen-cli-extensions",
        "vscode-open-vsx",
        "opencode",
        "cline",
    ];

    for required in required_surfaces {
        let surface = surfaces
            .iter()
            .find(|surface| surface["id"] == required)
            .unwrap_or_else(|| panic!("missing inventory surface {required}"));
        assert!(
            surface["approval_required"].as_bool().unwrap_or(false),
            "{required} should require approval"
        );
        assert!(
            !surface["external_submission_allowed"]
                .as_bool()
                .unwrap_or(true),
            "{required} should not allow external submission by default"
        );
        assert!(
            surface["requirements_sources"]
                .as_array()
                .is_some_and(|sources| !sources.is_empty()),
            "{required} should record at least one requirements source"
        );
        for source in surface["requirements_sources"]
            .as_array()
            .expect("requirements sources should be an array")
        {
            assert_eq!(
                source["status"], "searched",
                "{required} should only contain searched requirements sources"
            );
            assert_eq!(
                source["retrieved_at"], "2026-05-18",
                "{required} should record the current retrieval date"
            );
        }
        for gate in [
            "requirements_searched",
            "contracted",
            "hardened_local_package",
            "submission_ready",
            "submitted",
            "publicly_accepted",
        ] {
            assert!(
                surface["gates"].get(gate).is_some(),
                "{required} missing gate {gate}"
            );
        }

        let gates = &surface["gates"];
        let ordered_gates = [
            "requirements_searched",
            "contracted",
            "hardened_local_package",
            "submission_ready",
            "submitted",
            "publicly_accepted",
        ];
        for pair in ordered_gates.windows(2) {
            let later = pair[1];
            let earlier = pair[0];
            assert!(
                !gates[later].as_bool().unwrap_or(false)
                    || gates[earlier].as_bool().unwrap_or(false),
                "{required} sets {later} before {earlier}"
            );
        }

        let readiness_or_later = gates["submission_ready"].as_bool().unwrap_or(false)
            || gates["submitted"].as_bool().unwrap_or(false)
            || gates["publicly_accepted"].as_bool().unwrap_or(false);
        let blockers = surface["blockers"]
            .as_array()
            .expect("blockers should be an array");
        assert!(
            !readiness_or_later || blockers.is_empty(),
            "{required} claims readiness or later while blockers remain"
        );
    }

    assert!(contract.contains("conductor/submission-requirements.json"));
    assert!(track_82.contains("repo-health controls"));
}

#[test]
fn self_improving_submission_workflow_and_verifiers_are_registered() {
    let tracks = read("conductor/tracks.md");
    let ledger = read("conductor/evidence-ledger.json");
    let workflow = read(".github/workflows/submission-readiness.yml");
    let readiness = read("scripts/verify-submission-readiness.ps1");
    let live_evidence = read("scripts/verify-live-submission-evidence.ps1");
    let windows_gnu = read("scripts/verify-local-windows-gnu.ps1");
    let requirements = read("conductor/requirements.md");

    assert!(tracks.contains("82 self-improving submission and health loop"));
    assert!(ledger.contains("82-self-improving-submission-and-health-loop"));
    assert!(requirements.contains("Self-improving submission readiness"));
    assert!(requirements.contains("| Should | 82 |"));
    assert!(workflow.contains("Submission readiness"));
    assert!(workflow.contains("conductor/requirements.md"));
    assert!(workflow.contains("conductor/evidence-ledger.json"));
    assert!(workflow.contains("actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd"));
    assert!(workflow.contains("persist-credentials: false"));
    assert!(workflow.contains("verify-submission-readiness.ps1"));
    assert!(workflow.contains("cargo test --locked --test submission_contracts_policy"));
    assert!(readiness.contains("$schema"));
    assert!(readiness.contains("cannot set $gate before $priorGate"));
    assert!(readiness.contains("blocked surfaces"));
    assert!(readiness.contains("sources needing search or refresh"));
    assert!(readiness.contains("repo_health_target"));
    assert!(readiness.contains("external_submission_allowed"));
    assert!(readiness.contains("approval_required"));
    assert!(live_evidence.contains("sourceright.live_submission_evidence.v1"));
    assert!(live_evidence.contains("REPLACE_WITH"));
    assert!(live_evidence.contains("publicly_accepted"));
    assert!(windows_gnu.contains("function Invoke-CheckedCommand"));
    assert!(windows_gnu.contains("$LASTEXITCODE"));
    assert!(windows_gnu.contains("verify-submission-readiness.ps1"));
}

#[test]
fn local_submission_packets_exist_for_each_surface_family() {
    let inventory = read_json("conductor/submission-requirements.json");
    let manifest = read_json("conductor/submission-packets/manifest.json");
    let schema = read_json("schemas/sourceright.submission-packets.schema.json");
    assert_eq!(schema["$id"], manifest["$schema"]);
    for status in [
        "blocked",
        "active-control",
        "ready-local",
        "submitted",
        "accepted",
    ] {
        assert!(
            schema["$defs"]["packet"]["properties"]["status"]["enum"]
                .as_array()
                .expect("packet status enum should be an array")
                .iter()
                .any(|value| value.as_str() == Some(status)),
            "packet schema should allow status {status}"
        );
    }
    assert_eq!(
        manifest["schema_version"],
        "sourceright.submission_packets.v1"
    );
    assert!(
        manifest["approval_rule"]
            .as_str()
            .expect("approval rule should be a string")
            .contains("explicit approval")
    );

    let packets = [
        (
            "conductor/submission-packets/mcp-directories.md",
            vec!["Official MCP Registry", "Smithery", "Glama"],
        ),
        (
            "conductor/submission-packets/citation-managers.md",
            vec!["Zotero", "EndNote"],
        ),
        (
            "conductor/submission-packets/journal-platforms.md",
            vec!["OJS/PKP", "arXiv Boundary"],
        ),
        (
            "conductor/submission-packets/arxiv-upstream.md",
            vec!["arXiv/submit-ce", "arXiv/arxiv-submission-core"],
        ),
        (
            "conductor/submission-packets/ai-client-extensions.md",
            vec![
                "Claude",
                "Codex",
                "GitHub Copilot",
                "Gemini CLI",
                "Qwen CLI",
            ],
        ),
        (
            "conductor/submission-packets/vscode-open-vsx.md",
            vec!["VS Code Marketplace", "Open VSX"],
        ),
        (
            "conductor/submission-packets/agent-workflow.md",
            vec!["Add A Workflow When", "Add A Skill Or Agent When"],
        ),
    ];

    let index = read("conductor/submission-packets/README.md");
    let remaining = read("conductor/submission-packets/remaining-live-actions.md");
    let live_evidence_template =
        read_json("conductor/submission-packets/live-evidence.template.json");
    let live_evidence_schema =
        read_json("schemas/sourceright.live-submission-evidence.schema.json");
    assert!(index.contains("Submission Packets"));
    assert!(index.contains("manifest.json"));
    assert!(index.contains("External issues, pull requests"));
    assert!(index.contains("explicit approval"));
    assert!(remaining.contains("Remaining Live Submission Actions"));
    assert!(remaining.contains("Smithery"));
    assert!(remaining.contains("Glama"));
    assert!(remaining.contains("PKP Plugin Gallery"));
    assert!(remaining.contains("arXiv `submit-ce`"));
    assert_eq!(
        live_evidence_template["$schema"],
        "sourceright.live-submission-evidence.v1"
    );
    assert_eq!(
        live_evidence_schema["$id"],
        "sourceright.live-submission-evidence.v1"
    );

    for (path, markers) in packets {
        assert!(Path::new(path).exists(), "missing packet {path}");
        let packet = read(path);
        for marker in markers {
            assert!(packet.contains(marker), "{path} missing {marker}");
        }
        if !path.ends_with("agent-workflow.md") {
            assert!(packet.contains("Requirements Evidence"));
            assert!(packet.contains("Local Gates"));
            assert!(packet.contains("Draft"));
            assert!(packet.contains("Approval Gate"));
        }
        assert!(packet.contains("## Blockers"));
    }

    let manifest_packets = manifest["packets"]
        .as_array()
        .expect("manifest packets should be an array");
    let inventory_surfaces = inventory["surfaces"]
        .as_array()
        .expect("inventory surfaces should be an array");
    let inventory_surface_ids = inventory_surfaces
        .iter()
        .filter_map(|surface| surface["id"].as_str())
        .collect::<Vec<_>>();
    let required_packet_ids = [
        "mcp-directories",
        "citation-managers",
        "journal-platforms",
        "arxiv-upstream",
        "ai-client-extensions",
        "vscode-open-vsx",
        "agent-workflow",
    ];
    let required_surface_ids = [
        "official-mcp-registry",
        "smithery",
        "glama",
        "zotero",
        "endnote",
        "ojs-pkp",
        "arxiv-submit-ce",
        "arxiv-submission-core",
        "claude-cowork",
        "codex-app",
        "github-copilot",
        "gemini-cli-extensions",
        "qwen-cli-extensions",
        "vscode-open-vsx",
        "opencode",
        "cline",
    ];

    for required in required_packet_ids {
        assert!(
            manifest_packets
                .iter()
                .any(|packet| packet["id"].as_str() == Some(required)),
            "missing packet manifest row {required}"
        );
    }

    let mut covered_surfaces = Vec::new();
    for packet in manifest_packets {
        let id = packet["id"].as_str().expect("packet id should be a string");
        for property in packet
            .as_object()
            .expect("packet should be an object")
            .keys()
        {
            assert!(
                [
                    "id",
                    "path",
                    "owning_tracks",
                    "surfaces",
                    "status",
                    "approval_required",
                    "local_validation",
                    "blockers",
                ]
                .contains(&property.as_str()),
                "{id} has unexpected packet property {property}"
            );
        }
        let path = packet["path"]
            .as_str()
            .expect("packet path should be a string");
        assert!(
            Path::new(path).exists(),
            "{id} references missing path {path}"
        );
        assert!(
            packet["approval_required"].as_bool().unwrap_or(false),
            "{id} must be approval-gated"
        );
        assert!(
            packet["local_validation"]
                .as_array()
                .is_some_and(|checks| !checks.is_empty()),
            "{id} must define local validation"
        );
        let blockers = packet["blockers"]
            .as_array()
            .expect("packet blockers should be an array");
        let packet_markdown = read(path);
        assert!(
            packet_markdown.contains("## Blockers"),
            "{id} markdown should include a blockers section"
        );
        if packet["status"].as_str() == Some("blocked") {
            assert!(!blockers.is_empty(), "{id} is blocked without blockers");
        }
        for blocker in blockers {
            let blocker = blocker.as_str().expect("blocker should be a string");
            assert!(
                packet_markdown.contains(blocker),
                "{id} markdown should mention manifest blocker {blocker}"
            );
        }
        if blockers.is_empty() {
            assert!(
                packet_markdown.contains("None."),
                "{id} markdown should record None. when no blockers remain"
            );
        }
        if matches!(
            packet["status"].as_str(),
            Some("submitted") | Some("accepted")
        ) {
            assert!(
                blockers.is_empty(),
                "{id} cannot be submitted or accepted while blockers remain"
            );
        }
        for surface in packet["surfaces"]
            .as_array()
            .expect("packet surfaces should be an array")
        {
            let surface_id = surface.as_str().expect("surface id should be a string");
            assert!(
                inventory_surface_ids.contains(&surface_id),
                "{id} references unknown surface {surface_id}"
            );
            covered_surfaces.push(surface_id);
        }
    }

    for surface_id in required_surface_ids {
        assert!(
            covered_surfaces.contains(&surface_id),
            "submission surface {surface_id} is not covered by a packet manifest row"
        );
    }
}
