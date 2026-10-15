use dioxus::prelude::*;

const WORKSPACE_CSS: Asset = asset!("./workspace.css");

// The Obsidian compatibility runtime and the vendored sample plugin. These are
// served as plain assets and loaded at runtime by the bootstrap script below.
const RUNTIME_JS: Asset = asset!("/assets/obsidian/runtime.js");
const PLUGIN_MAIN: Asset = asset!("/assets/plugins/sample-plugin/main.js");
const PLUGIN_MANIFEST: Asset = asset!("/assets/plugins/sample-plugin/manifest.json");
const PLUGIN_STYLES: Asset = asset!("/assets/plugins/sample-plugin/styles.css");

// Fetches the runtime (if not already present), then the plugin's manifest /
// bundle / styles, and hands them to the runtime to enable. URLs are read from
// `data-*` attributes on `#onyx-app` so this stays a plain string (no interpolation).
const BOOTSTRAP: &str = r"
(async () => {
    if (window.__onyxBooted) return;
    window.__onyxBooted = true;
    const app = document.getElementById('onyx-app');
    if (!app) { window.__onyxBooted = false; return; }

    if (!(window.OnyxObsidian && window.OnyxObsidian.__ready)) {
        const rt = await (await fetch(app.getAttribute('data-runtime'))).text();
        (0, eval)(rt);
    }
    window.OnyxObsidian.ensureApp();

    const manifest = await (await fetch(app.getAttribute('data-manifest'))).json();
    const code = await (await fetch(app.getAttribute('data-plugin'))).text();
    let css = '';
    try { css = await (await fetch(app.getAttribute('data-styles'))).text(); } catch (e) {}

    window.OnyxObsidian.enablePlugin(manifest, code, css);
})().catch((e) => console.error('[onyx] bootstrap failed', e));
";

const WELCOME_NOTE: &str = "# Welcome to Onyx\n\nThis note editor is a real `MarkdownView` as far as Obsidian \
plugins are concerned.\n\n## Try the sample plugin\n\n- Click the **dice** icon in the left ribbon → a Notice \
appears.\n- Open the **command palette** (top-right, or Cmd/Ctrl-P):\n    - *Open modal (simple)* opens a Modal.\n    \
- *Open modal (complex)* only runs while this editor is the active view.\n    - *Replace selected content* — select \
some text here first, then run it.\n- Open **Settings** (gear icon) to see the plugin's settings tab.\n- Note the \
*Status bar text* at the bottom — the plugin put it there.\n\nSelect this text and run \"Replace selected content\".\n";

/// An Obsidian-like workspace that hosts unmodified Obsidian plugins through
/// Onyx's compatibility runtime.
#[component]
pub fn Workspace() -> Element {
	use_effect(|| {
		let _ = document::eval(BOOTSTRAP);
	});

	rsx! {
		document::Link { rel: "stylesheet", href: WORKSPACE_CSS }

		div {
			id: "onyx-app",
			class: "onyx-app",
			"data-runtime": "{RUNTIME_JS}",
			"data-plugin": "{PLUGIN_MAIN}",
			"data-manifest": "{PLUGIN_MANIFEST}",
			"data-styles": "{PLUGIN_STYLES}",

			// Left ribbon — plugin ribbon icons are appended here.
			div { id: "onyx-ribbon", class: "onyx-ribbon side-dock-ribbon" }

			// File sidebar (static placeholder content).
			aside { class: "onyx-sidebar",
				div { class: "onyx-sidebar-header",
					span { "ONYX VAULT" }
					a { class: "onyx-home-link", href: "/", "← Home" }
				}
				nav { class: "onyx-file-list",
					div { class: "onyx-file is-active", "📄 Welcome.md" }
					div { class: "onyx-file", "📄 Ideas.md" }
					div { class: "onyx-file", "📄 Daily note.md" }
					div { class: "onyx-folder", "📁 Projects" }
				}
			}

			// Main editor column.
			main { class: "onyx-main",
				div { class: "onyx-titlebar",
					div { class: "onyx-tab-title", "Welcome.md" }
					div { class: "onyx-titlebar-actions",
						button {
							class: "onyx-action-btn",
							title: "Open command palette (Cmd/Ctrl-P)",
							onclick: |_| {
								let _ = document::eval(
									"window.OnyxObsidian && window.OnyxObsidian.openCommandPalette()",
								);
							},
							"⌘ Commands"
						}
						button {
							class: "onyx-action-btn",
							title: "Open settings",
							onclick: |_| {
								let _ = document::eval(
									"window.OnyxObsidian && window.OnyxObsidian.openSettings()",
								);
							},
							"⚙ Settings"
						}
					}
				}

				div { class: "onyx-editor-wrap",
					textarea {
						id: "onyx-editor",
						class: "onyx-editor",
						spellcheck: false,
						{WELCOME_NOTE}
					}
				}

				// Status bar — plugin status items are appended here.
				div { id: "onyx-statusbar", class: "onyx-statusbar status-bar" }
			}

			// Overlay hosts, kept outside the Dioxus-managed layout so the runtime
			// can append toasts and modals without fighting the virtual DOM.
			div { id: "onyx-notice-container", class: "notice-container" }
			div { id: "onyx-modal-root", class: "onyx-modal-root" }
		}
	}
}
