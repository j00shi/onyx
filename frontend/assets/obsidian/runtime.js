/*
 * Onyx — Obsidian plugin compatibility runtime.
 *
 * This file is NOT part of any Obsidian plugin. It is Onyx's own
 * re-implementation of the subset of the `obsidian` module API that plugins
 * import at runtime. An unmodified, esbuild-bundled plugin does
 * `require("obsidian")`; we hand it this object so its calls to `addRibbonIcon`,
 * `new Notice(...)`, `new Modal(...)`, settings builders, commands, etc. drive
 * Onyx's own UI (the shell DOM that the Dioxus `Workspace` component renders).
 *
 * Scope: enough fidelity for the official obsidian-sample-plugin — ribbon icons,
 * Notices/toasts, Modals, a command palette, a status bar, and a settings tab
 * built with the `Setting` builder. The surface is intentionally broader than
 * the sample needs (toggles, buttons, dropdowns, a few DOM helpers) so other
 * simple plugins have a chance of working too.
 */
(function () {
	"use strict";

	if (window.OnyxObsidian && window.OnyxObsidian.__ready) {
		return;
	}

	// ---------------------------------------------------------------------
	// DOM prototype extensions that Obsidian adds and plugins rely on.
	// ---------------------------------------------------------------------
	function applyElInfo(el, o) {
		if (o == null) return el;
		if (typeof o === "string") {
			el.className = o;
			return el;
		}
		if (o.cls != null) el.className = Array.isArray(o.cls) ? o.cls.join(" ") : o.cls;
		if (o.text != null) el.textContent = o.text;
		if (o.title != null) el.title = o.title;
		if (o.value != null) el.value = o.value;
		if (o.type != null) el.type = o.type;
		if (o.placeholder != null) el.placeholder = o.placeholder;
		if (o.href != null) el.href = o.href;
		if (o.attr) {
			for (const k in o.attr) {
				if (o.attr[k] != null) el.setAttribute(k, o.attr[k]);
			}
		}
		return el;
	}

	function createEl(tag, o, cb) {
		const el = document.createElement(tag);
		applyElInfo(el, o);
		// `this` is the parent when called as a prototype method.
		if (this && typeof this.appendChild === "function") this.appendChild(el);
		if (typeof cb === "function") cb(el);
		return el;
	}

	function define(proto, name, fn) {
		if (!proto[name]) {
			Object.defineProperty(proto, name, {
				value: fn,
				writable: true,
				configurable: true,
				enumerable: false,
			});
		}
	}

	for (const proto of [Element.prototype, DocumentFragment.prototype]) {
		define(proto, "createEl", createEl);
		define(proto, "createDiv", function (o, cb) {
			return createEl.call(this, "div", o, cb);
		});
		define(proto, "createSpan", function (o, cb) {
			return createEl.call(this, "span", o, cb);
		});
		define(proto, "empty", function () {
			while (this.firstChild) this.removeChild(this.firstChild);
			return this;
		});
		define(proto, "setText", function (val) {
			if (val instanceof Node) {
				while (this.firstChild) this.removeChild(this.firstChild);
				this.appendChild(val);
			} else {
				this.textContent = val == null ? "" : String(val);
			}
			return this;
		});
		define(proto, "appendText", function (val) {
			this.appendChild(document.createTextNode(val));
			return this;
		});
		define(proto, "addClass", function (...c) {
			this.classList.add(...c.filter(Boolean));
			return this;
		});
		define(proto, "removeClass", function (...c) {
			this.classList.remove(...c.filter(Boolean));
			return this;
		});
		define(proto, "toggleClass", function (c, on) {
			this.classList.toggle(c, on);
			return this;
		});
		define(proto, "hasClass", function (c) {
			return this.classList.contains(c);
		});
		define(proto, "setAttr", function (k, v) {
			if (v == null || v === false) this.removeAttribute(k);
			else this.setAttribute(k, v === true ? "" : v);
			return this;
		});
		define(proto, "setAttrs", function (obj) {
			for (const k in obj) this.setAttr(k, obj[k]);
			return this;
		});
		define(proto, "getAttr", function (k) {
			return this.getAttribute(k);
		});
		define(proto, "setCssStyles", function (styles) {
			Object.assign(this.style, styles);
			return this;
		});
		define(proto, "detach", function () {
			if (this.parentNode) this.parentNode.removeChild(this);
			return this;
		});
	}

	// Global element factories, also provided by Obsidian.
	window.createEl = function (tag, o, cb) {
		return createEl.call(null, tag, o, cb);
	};
	window.createDiv = function (o, cb) {
		return createEl.call(null, "div", o, cb);
	};
	window.createSpan = function (o, cb) {
		return createEl.call(null, "span", o, cb);
	};
	window.createFragment = function (cb) {
		const f = document.createDocumentFragment();
		if (cb) cb(f);
		return f;
	};

	// Obsidian exposes the active document/window as globals so plugins can work
	// inside pop-out windows. We only have one window.
	window.activeDocument = document;
	window.activeWindow = window;

	// ---------------------------------------------------------------------
	// Icons. Obsidian ships Lucide; we inline the handful the sample needs and
	// fall back to a glyph for anything else.
	// ---------------------------------------------------------------------
	const SVG_NS = "http://www.w3.org/2000/svg";
	const ICONS = {
		dice:
			'<path d="M3 3h18v18H3z"/><circle cx="8" cy="8" r="1.3" fill="currentColor" stroke="none"/><circle cx="16" cy="8" r="1.3" fill="currentColor" stroke="none"/><circle cx="8" cy="16" r="1.3" fill="currentColor" stroke="none"/><circle cx="16" cy="16" r="1.3" fill="currentColor" stroke="none"/><circle cx="12" cy="12" r="1.3" fill="currentColor" stroke="none"/>',
		settings:
			'<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>',
		search: '<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>',
		command:
			'<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"/>',
	};

	function setIcon(el, name) {
		el.empty();
		const svgSrc = ICONS[name];
		if (svgSrc) {
			const svg = document.createElementNS(SVG_NS, "svg");
			svg.setAttribute("viewBox", "0 0 24 24");
			svg.setAttribute("width", "18");
			svg.setAttribute("height", "18");
			svg.setAttribute("fill", "none");
			svg.setAttribute("stroke", "currentColor");
			svg.setAttribute("stroke-width", "2");
			svg.setAttribute("stroke-linecap", "round");
			svg.setAttribute("stroke-linejoin", "round");
			svg.classList.add("svg-icon", "lucide-" + name);
			svg.innerHTML = svgSrc;
			el.appendChild(svg);
		} else {
			const span = document.createElement("span");
			span.textContent = (name || "?").charAt(0).toUpperCase();
			el.appendChild(span);
		}
	}
	window.setIcon = setIcon;

	// ---------------------------------------------------------------------
	// Shell lookups. The Dioxus `Workspace` component renders these static,
	// never-updated mount points; we populate them.
	// ---------------------------------------------------------------------
	const shell = {
		ribbon: () => document.getElementById("onyx-ribbon"),
		statusbar: () => document.getElementById("onyx-statusbar"),
		notices: () => document.getElementById("onyx-notice-container"),
		modalRoot: () => document.getElementById("onyx-modal-root"),
		editor: () => document.getElementById("onyx-editor"),
	};

	// ---------------------------------------------------------------------
	// Component: lifecycle + registration bookkeeping (Plugin, Modal, etc.).
	// ---------------------------------------------------------------------
	class Component {
		constructor() {
			this._children = [];
			this._cleanups = [];
			this._loaded = false;
		}
		load() {
			if (this._loaded) return;
			this._loaded = true;
			this.onload();
			for (const c of this._children) c.load();
		}
		onload() {}
		unload() {
			if (!this._loaded) return;
			this._loaded = false;
			for (const c of this._children) c.unload();
			this._children.length = 0;
			for (const fn of this._cleanups.splice(0)) {
				try {
					fn();
				} catch (e) {
					console.error("[onyx] cleanup failed", e);
				}
			}
			this.onunload();
		}
		onunload() {}
		addChild(c) {
			this._children.push(c);
			if (this._loaded) c.load();
			return c;
		}
		removeChild(c) {
			const i = this._children.indexOf(c);
			if (i >= 0) {
				this._children.splice(i, 1);
				c.unload();
			}
			return c;
		}
		register(cb) {
			this._cleanups.push(cb);
		}
		registerEvent(eventRef) {
			if (eventRef && typeof eventRef.unload === "function") {
				this._cleanups.push(() => eventRef.unload());
			}
		}
		registerDomEvent(el, type, cb, opts) {
			el.addEventListener(type, cb, opts);
			this._cleanups.push(() => el.removeEventListener(type, cb, opts));
		}
		registerInterval(id) {
			this._cleanups.push(() => window.clearInterval(id));
			return id;
		}
	}

	// Events: a minimal emitter so `app.workspace.on(...)` etc. don't throw.
	class Events {
		constructor() {
			this._handlers = {};
		}
		on(name, cb) {
			(this._handlers[name] = this._handlers[name] || []).push(cb);
			const ref = { name, cb, unload: () => this.offref(ref) };
			return ref;
		}
		offref(ref) {
			const arr = this._handlers[ref.name];
			if (arr) {
				const i = arr.indexOf(ref.cb);
				if (i >= 0) arr.splice(i, 1);
			}
		}
		off(name, cb) {
			const arr = this._handlers[name];
			if (arr) {
				const i = arr.indexOf(cb);
				if (i >= 0) arr.splice(i, 1);
			}
		}
		trigger(name, ...args) {
			for (const cb of this._handlers[name] || []) {
				try {
					cb(...args);
				} catch (e) {
					console.error("[onyx] event handler failed", e);
				}
			}
		}
	}

	// ---------------------------------------------------------------------
	// Notice (toast).
	// ---------------------------------------------------------------------
	class Notice {
		constructor(message, duration) {
			const container = shell.notices();
			this.noticeEl = document.createElement("div");
			this.noticeEl.className = "notice";
			this.setMessage(message);
			if (container) container.appendChild(this.noticeEl);
			requestAnimationFrame(() => this.noticeEl.classList.add("mod-visible"));
			this.noticeEl.addEventListener("click", () => this.hide());
			const ms = duration === undefined ? 4000 : duration;
			if (ms > 0) this._timer = window.setTimeout(() => this.hide(), ms);
		}
		setMessage(message) {
			if (message instanceof Node) {
				this.noticeEl.textContent = "";
				this.noticeEl.appendChild(message);
			} else {
				this.noticeEl.textContent = message == null ? "" : String(message);
			}
			return this;
		}
		hide() {
			if (this._timer) window.clearTimeout(this._timer);
			const el = this.noticeEl;
			el.classList.remove("mod-visible");
			el.classList.add("mod-hiding");
			window.setTimeout(() => el.remove(), 250);
		}
	}

	// ---------------------------------------------------------------------
	// Modal.
	// ---------------------------------------------------------------------
	class Modal {
		constructor(app) {
			this.app = app;
			this.scope = new Scope();
			this.containerEl = document.createElement("div");
			this.containerEl.className = "modal-container mod-dim";

			this.bgEl = document.createElement("div");
			this.bgEl.className = "modal-bg";

			this.modalEl = document.createElement("div");
			this.modalEl.className = "modal";

			this.closeEl = document.createElement("div");
			this.closeEl.className = "modal-close-button";

			this.titleEl = document.createElement("div");
			this.titleEl.className = "modal-title";

			this.contentEl = document.createElement("div");
			this.contentEl.className = "modal-content";

			this.modalEl.appendChild(this.closeEl);
			this.modalEl.appendChild(this.titleEl);
			this.modalEl.appendChild(this.contentEl);
			this.containerEl.appendChild(this.bgEl);
			this.containerEl.appendChild(this.modalEl);

			this.closeEl.addEventListener("click", () => this.close());
			this.bgEl.addEventListener("click", () => this.close());
			this._onKey = (evt) => {
				if (evt.key === "Escape") {
					evt.preventDefault();
					this.close();
				}
			};
		}
		open() {
			const root = shell.modalRoot();
			if (root) root.appendChild(this.containerEl);
			document.addEventListener("keydown", this._onKey);
			this.onOpen();
			requestAnimationFrame(() => this.containerEl.classList.add("mod-visible"));
			return this;
		}
		close() {
			document.removeEventListener("keydown", this._onKey);
			this.onClose();
			this.containerEl.remove();
			return this;
		}
		onOpen() {}
		onClose() {}
		setTitle(title) {
			this.titleEl.textContent = title;
			return this;
		}
		setContent(content) {
			this.contentEl.empty();
			this.contentEl.setText(content);
			return this;
		}
	}

	// Keymap scope stub — plugins construct these; we don't route hotkeys through them.
	class Scope {
		constructor() {
			this.keys = [];
		}
		register(mods, key, fn) {
			const h = { mods, key, fn };
			this.keys.push(h);
			return h;
		}
		unregister(h) {
			const i = this.keys.indexOf(h);
			if (i >= 0) this.keys.splice(i, 1);
		}
	}

	// ---------------------------------------------------------------------
	// Setting builder + the field components it can add.
	// ---------------------------------------------------------------------
	class ValueComponent {
		constructor(el) {
			this.el = el;
			this.disabled = false;
		}
		then(cb) {
			cb(this);
			return this;
		}
		setDisabled(d) {
			this.disabled = d;
			this.el.disabled = d;
			return this;
		}
	}

	class TextComponent extends ValueComponent {
		constructor(containerEl, type) {
			const input = document.createElement("input");
			input.type = type || "text";
			input.className = type === "search" ? "search-input" : "";
			containerEl.appendChild(input);
			super(input);
			this.inputEl = input;
		}
		setPlaceholder(p) {
			this.inputEl.placeholder = p;
			return this;
		}
		getValue() {
			return this.inputEl.value;
		}
		setValue(v) {
			this.inputEl.value = v == null ? "" : v;
			return this;
		}
		onChange(cb) {
			this.inputEl.addEventListener("input", () => cb(this.inputEl.value));
			return this;
		}
	}

	class TextAreaComponent extends ValueComponent {
		constructor(containerEl) {
			const input = document.createElement("textarea");
			containerEl.appendChild(input);
			super(input);
			this.inputEl = input;
		}
		setPlaceholder(p) {
			this.inputEl.placeholder = p;
			return this;
		}
		getValue() {
			return this.inputEl.value;
		}
		setValue(v) {
			this.inputEl.value = v == null ? "" : v;
			return this;
		}
		onChange(cb) {
			this.inputEl.addEventListener("input", () => cb(this.inputEl.value));
			return this;
		}
	}

	class ToggleComponent extends ValueComponent {
		constructor(containerEl) {
			const wrap = document.createElement("div");
			wrap.className = "checkbox-container";
			const input = document.createElement("input");
			input.type = "checkbox";
			wrap.appendChild(input);
			containerEl.appendChild(wrap);
			super(input);
			this.toggleEl = wrap;
			input.addEventListener("change", () => {
				wrap.classList.toggle("is-enabled", input.checked);
			});
		}
		getValue() {
			return this.el.checked;
		}
		setValue(v) {
			this.el.checked = !!v;
			this.toggleEl.classList.toggle("is-enabled", !!v);
			return this;
		}
		onChange(cb) {
			this.el.addEventListener("change", () => cb(this.el.checked));
			return this;
		}
	}

	class ButtonComponent extends ValueComponent {
		constructor(containerEl) {
			const btn = document.createElement("button");
			containerEl.appendChild(btn);
			super(btn);
			this.buttonEl = btn;
		}
		setButtonText(t) {
			this.buttonEl.textContent = t;
			return this;
		}
		setCta() {
			this.buttonEl.classList.add("mod-cta");
			return this;
		}
		setWarning() {
			this.buttonEl.classList.add("mod-warning");
			return this;
		}
		setIcon(name) {
			setIcon(this.buttonEl, name);
			return this;
		}
		setTooltip(t) {
			this.buttonEl.title = t;
			return this;
		}
		onClick(cb) {
			this.buttonEl.addEventListener("click", cb);
			return this;
		}
	}

	class DropdownComponent extends ValueComponent {
		constructor(containerEl) {
			const sel = document.createElement("select");
			sel.className = "dropdown";
			containerEl.appendChild(sel);
			super(sel);
			this.selectEl = sel;
		}
		addOption(value, display) {
			const o = document.createElement("option");
			o.value = value;
			o.textContent = display;
			this.selectEl.appendChild(o);
			return this;
		}
		addOptions(record) {
			for (const k in record) this.addOption(k, record[k]);
			return this;
		}
		getValue() {
			return this.selectEl.value;
		}
		setValue(v) {
			this.selectEl.value = v;
			return this;
		}
		onChange(cb) {
			this.selectEl.addEventListener("change", () => cb(this.selectEl.value));
			return this;
		}
	}

	class Setting {
		constructor(containerEl) {
			this.settingEl = document.createElement("div");
			this.settingEl.className = "setting-item";

			this.infoEl = document.createElement("div");
			this.infoEl.className = "setting-item-info";
			this.nameEl = document.createElement("div");
			this.nameEl.className = "setting-item-name";
			this.descEl = document.createElement("div");
			this.descEl.className = "setting-item-description";
			this.infoEl.appendChild(this.nameEl);
			this.infoEl.appendChild(this.descEl);

			this.controlEl = document.createElement("div");
			this.controlEl.className = "setting-item-control";

			this.settingEl.appendChild(this.infoEl);
			this.settingEl.appendChild(this.controlEl);
			containerEl.appendChild(this.settingEl);
		}
		setName(name) {
			this.nameEl.textContent = name;
			return this;
		}
		setDesc(desc) {
			this.descEl.textContent = desc;
			return this;
		}
		setHeading() {
			this.settingEl.classList.add("setting-item-heading");
			return this;
		}
		setClass(cls) {
			this.settingEl.classList.add(cls);
			return this;
		}
		setDisabled(d) {
			this.settingEl.classList.toggle("is-disabled", d);
			return this;
		}
		addText(cb) {
			cb(new TextComponent(this.controlEl));
			return this;
		}
		addSearch(cb) {
			cb(new TextComponent(this.controlEl, "search"));
			return this;
		}
		addTextArea(cb) {
			cb(new TextAreaComponent(this.controlEl));
			return this;
		}
		addToggle(cb) {
			cb(new ToggleComponent(this.controlEl));
			return this;
		}
		addButton(cb) {
			cb(new ButtonComponent(this.controlEl));
			return this;
		}
		addExtraButton(cb) {
			cb(new ButtonComponent(this.controlEl));
			return this;
		}
		addDropdown(cb) {
			cb(new DropdownComponent(this.controlEl));
			return this;
		}
	}

	// ---------------------------------------------------------------------
	// Setting tab base.
	// ---------------------------------------------------------------------
	class PluginSettingTab {
		constructor(app, plugin) {
			this.app = app;
			this.plugin = plugin;
			this.containerEl = document.createElement("div");
			this.containerEl.className = "vertical-tab-content";
		}
		display() {}
		hide() {
			this.containerEl.empty();
		}
	}

	// ---------------------------------------------------------------------
	// Views. We model the note editor as the one active MarkdownView.
	// ---------------------------------------------------------------------
	class View {}
	class ItemView extends View {}
	class MarkdownView extends View {
		constructor(editor) {
			super();
			this.editor = editor;
		}
		getViewType() {
			return "markdown";
		}
	}
	class MarkdownFileInfo {}

	class Editor {
		constructor(el) {
			this.el = el;
		}
		getValue() {
			return this.el.value;
		}
		setValue(v) {
			this.el.value = v;
		}
		getSelection() {
			return this.el.value.substring(this.el.selectionStart, this.el.selectionEnd);
		}
		somethingSelected() {
			return this.el.selectionStart !== this.el.selectionEnd;
		}
		replaceSelection(replacement) {
			const start = this.el.selectionStart;
			const end = this.el.selectionEnd;
			const v = this.el.value;
			this.el.value = v.slice(0, start) + replacement + v.slice(end);
			const caret = start + replacement.length;
			this.el.selectionStart = this.el.selectionEnd = caret;
			this.el.focus();
			this.el.dispatchEvent(new Event("input", { bubbles: true }));
		}
		getCursor() {
			return { line: 0, ch: this.el.selectionStart };
		}
		setCursor(pos) {
			const ch = typeof pos === "number" ? pos : pos.ch;
			this.el.selectionStart = this.el.selectionEnd = ch;
		}
		focus() {
			this.el.focus();
		}
	}

	// ---------------------------------------------------------------------
	// Plugin base.
	// ---------------------------------------------------------------------
	class Plugin extends Component {
		constructor(app, manifest) {
			super();
			this.app = app;
			this.manifest = manifest;
		}
		addRibbonIcon(icon, title, callback) {
			const ribbon = shell.ribbon();
			const el = document.createElement("div");
			el.className = "side-dock-ribbon-action";
			el.setAttribute("aria-label", title);
			el.setAttribute("data-tooltip-position", "right");
			setIcon(el, icon);
			el.addEventListener("click", (evt) => callback(evt));
			if (ribbon) ribbon.appendChild(el);
			this.register(() => el.remove());
			return el;
		}
		addStatusBarItem() {
			const bar = shell.statusbar();
			const el = document.createElement("div");
			el.className = "status-bar-item plugin-" + (this.manifest ? this.manifest.id : "unknown");
			if (bar) bar.appendChild(el);
			this.register(() => el.remove());
			return el;
		}
		addCommand(command) {
			registry.commands.set(command.id, { ...command, pluginId: this.manifest.id });
			this.register(() => registry.commands.delete(command.id));
			return command;
		}
		removeCommand(id) {
			registry.commands.delete(id);
		}
		addSettingTab(tab) {
			registry.settingTabs.set(this.manifest.id, tab);
			this.register(() => registry.settingTabs.delete(this.manifest.id));
			return tab;
		}
		async loadData() {
			try {
				const raw = window.localStorage.getItem(this._dataKey());
				return raw ? JSON.parse(raw) : null;
			} catch (e) {
				console.error("[onyx] loadData failed", e);
				return null;
			}
		}
		async saveData(data) {
			try {
				window.localStorage.setItem(this._dataKey(), JSON.stringify(data));
			} catch (e) {
				console.error("[onyx] saveData failed", e);
			}
		}
		_dataKey() {
			return "onyx-plugin-data:" + (this.manifest ? this.manifest.id : "unknown");
		}
	}

	// ---------------------------------------------------------------------
	// The app object handed to every plugin.
	// ---------------------------------------------------------------------
	function buildApp() {
		const editorEl = shell.editor();
		const editor = editorEl ? new Editor(editorEl) : null;
		const activeView = editor ? new MarkdownView(editor) : null;

		const workspace = Object.assign(new Events(), {
			activeEditor: activeView,
			getActiveViewOfType(type) {
				if (type === MarkdownView && activeView) return activeView;
				if (type === View && activeView) return activeView;
				return null;
			},
			getActiveFile() {
				return null;
			},
			getLeaf() {
				return null;
			},
			getLeavesOfType() {
				return [];
			},
			onLayoutReady(cb) {
				cb();
			},
			_editor: editor,
			_activeView: activeView,
		});

		return {
			workspace,
			vault: Object.assign(new Events(), {
				getName: () => "Onyx Vault",
				adapter: {},
				getFiles: () => [],
				getAbstractFileByPath: () => null,
			}),
			metadataCache: Object.assign(new Events(), {
				getFileCache: () => null,
			}),
			keymap: { pushScope() {}, popScope() {} },
			scope: new Scope(),
			lastEvent: null,
		};
	}

	// ---------------------------------------------------------------------
	// Registry + command palette + settings UI (Onyx-side, generic).
	// ---------------------------------------------------------------------
	const registry = {
		app: null,
		plugins: new Map(),
		commands: new Map(),
		settingTabs: new Map(),
	};

	function runCommand(cmd) {
		const ws = registry.app.workspace;
		if (cmd.checkCallback) {
			if (cmd.checkCallback(true)) cmd.checkCallback(false);
			return;
		}
		if (cmd.editorCallback) {
			cmd.editorCallback(ws._editor, ws._activeView);
			return;
		}
		if (cmd.editorCheckCallback) {
			if (cmd.editorCheckCallback(true, ws._editor, ws._activeView)) {
				cmd.editorCheckCallback(false, ws._editor, ws._activeView);
			}
			return;
		}
		if (cmd.callback) cmd.callback();
	}

	function commandAvailable(cmd) {
		try {
			if (cmd.checkCallback) return !!cmd.checkCallback(true);
			if (cmd.editorCheckCallback) {
				const ws = registry.app.workspace;
				return !!cmd.editorCheckCallback(true, ws._editor, ws._activeView);
			}
		} catch (e) {
			return false;
		}
		return true;
	}

	let paletteEl = null;
	function closeCommandPalette() {
		if (paletteEl) {
			paletteEl.remove();
			paletteEl = null;
		}
	}
	function openCommandPalette() {
		closeCommandPalette();
		const root = shell.modalRoot();
		if (!root) return;

		const container = document.createElement("div");
		container.className = "modal-container mod-dim";
		const bg = document.createElement("div");
		bg.className = "modal-bg";
		const modal = document.createElement("div");
		modal.className = "modal prompt";

		const inputWrap = document.createElement("div");
		inputWrap.className = "prompt-input-container";
		const input = document.createElement("input");
		input.className = "prompt-input";
		input.type = "text";
		input.placeholder = "Select a command…";
		inputWrap.appendChild(input);

		const results = document.createElement("div");
		results.className = "prompt-results";

		modal.appendChild(inputWrap);
		modal.appendChild(results);
		container.appendChild(bg);
		container.appendChild(modal);
		root.appendChild(container);
		paletteEl = container;

		function render(filter) {
			results.empty();
			const q = (filter || "").toLowerCase();
			const items = [...registry.commands.values()].filter(
				(c) => commandAvailable(c) && c.name.toLowerCase().includes(q),
			);
			if (items.length === 0) {
				const empty = document.createElement("div");
				empty.className = "suggestion-empty";
				empty.textContent = "No matching commands";
				results.appendChild(empty);
				return;
			}
			for (const cmd of items) {
				const item = document.createElement("div");
				item.className = "suggestion-item";
				const title = document.createElement("span");
				title.className = "suggestion-title";
				title.textContent = cmd.name;
				const owner = document.createElement("span");
				owner.className = "suggestion-aux";
				owner.textContent = registry.plugins.get(cmd.pluginId)
					? registry.plugins.get(cmd.pluginId).manifest.name
					: cmd.pluginId;
				item.appendChild(title);
				item.appendChild(owner);
				item.addEventListener("mousedown", (e) => {
					e.preventDefault();
					closeCommandPalette();
					runCommand(cmd);
				});
				results.appendChild(item);
			}
		}

		input.addEventListener("input", () => render(input.value));
		input.addEventListener("keydown", (e) => {
			if (e.key === "Escape") closeCommandPalette();
		});
		bg.addEventListener("click", closeCommandPalette);
		render("");
		input.focus();
	}

	let settingsEl = null;
	function closeSettings() {
		if (settingsEl) {
			settingsEl.remove();
			settingsEl = null;
		}
	}
	function openSettings() {
		closeSettings();
		const root = shell.modalRoot();
		if (!root) return;

		const container = document.createElement("div");
		container.className = "modal-container mod-dim mod-settings";
		const bg = document.createElement("div");
		bg.className = "modal-bg";
		const modal = document.createElement("div");
		modal.className = "modal mod-settings-window";

		const title = document.createElement("div");
		title.className = "modal-title";
		title.textContent = "Settings";
		const closeBtn = document.createElement("div");
		closeBtn.className = "modal-close-button";

		const content = document.createElement("div");
		content.className = "modal-content settings-content";

		if (registry.settingTabs.size === 0) {
			const empty = document.createElement("div");
			empty.className = "setting-item-description";
			empty.textContent = "No plugin settings registered.";
			content.appendChild(empty);
		}
		for (const tab of registry.settingTabs.values()) {
			const header = document.createElement("h3");
			header.className = "settings-plugin-header";
			header.textContent = tab.plugin && tab.plugin.manifest ? tab.plugin.manifest.name : "Plugin";
			content.appendChild(header);
			tab.containerEl.empty();
			content.appendChild(tab.containerEl);
			try {
				tab.display();
			} catch (e) {
				console.error("[onyx] setting tab display failed", e);
			}
		}

		modal.appendChild(closeBtn);
		modal.appendChild(title);
		modal.appendChild(content);
		container.appendChild(bg);
		container.appendChild(modal);
		root.appendChild(container);
		settingsEl = container;

		closeBtn.addEventListener("click", closeSettings);
		bg.addEventListener("click", closeSettings);
	}

	// ---------------------------------------------------------------------
	// Plugin loading. `code` is an esbuild CJS bundle that does
	// require("obsidian"); we feed it this module and run its onload().
	// ---------------------------------------------------------------------
	const OBSIDIAN = {
		Component,
		Events,
		Plugin,
		Notice,
		Modal,
		Scope,
		Setting,
		PluginSettingTab,
		ValueComponent,
		TextComponent,
		TextAreaComponent,
		ToggleComponent,
		ButtonComponent,
		DropdownComponent,
		View,
		ItemView,
		MarkdownView,
		MarkdownFileInfo,
		Editor,
		setIcon,
		Platform: { isDesktop: true, isMobile: false, isDesktopApp: false },
		normalizePath: (p) => p.replace(/\\/g, "/").replace(/\/+/g, "/"),
		requestUrl: () => Promise.reject(new Error("requestUrl is not supported in Onyx")),
	};

	function enablePlugin(manifest, code, css) {
		if (registry.plugins.has(manifest.id)) {
			console.warn("[onyx] plugin already enabled:", manifest.id);
			return registry.plugins.get(manifest.id);
		}
		if (css) {
			const styleEl = document.createElement("style");
			styleEl.setAttribute("data-onyx-plugin", manifest.id);
			styleEl.textContent = css;
			document.head.appendChild(styleEl);
		}

		const module = { exports: {} };
		const require = (name) => {
			if (name === "obsidian") return OBSIDIAN;
			throw new Error("[onyx] plugin required unsupported module: " + name);
		};
		const factory = new Function(
			"module",
			"exports",
			"require",
			"process",
			"global",
			code,
		);
		factory(module, module.exports, require, { platform: "web", env: {}, argv: [] }, window);

		const PluginClass = module.exports.default || module.exports;
		if (typeof PluginClass !== "function") {
			throw new Error("[onyx] plugin did not export a Plugin class");
		}

		const instance = new PluginClass(registry.app, manifest);
		registry.plugins.set(manifest.id, instance);
		instance.load();
		console.log("[onyx] enabled plugin:", manifest.name, "(" + manifest.id + ")");
		return instance;
	}

	function disablePlugin(id) {
		const inst = registry.plugins.get(id);
		if (inst) {
			inst.unload();
			registry.plugins.delete(id);
		}
		const styleEl = document.head.querySelector('style[data-onyx-plugin="' + id + '"]');
		if (styleEl) styleEl.remove();
	}

	// Global command-palette hotkey (Mod-P), like Obsidian.
	document.addEventListener("keydown", (e) => {
		if ((e.metaKey || e.ctrlKey) && !e.shiftKey && (e.key === "p" || e.key === "P")) {
			e.preventDefault();
			openCommandPalette();
		}
	});

	// ---------------------------------------------------------------------
	// Public surface used by the Dioxus bootstrap and shell buttons.
	// ---------------------------------------------------------------------
	registry.app = null;
	window.OnyxObsidian = {
		__ready: true,
		obsidian: OBSIDIAN,
		registry,
		ensureApp() {
			if (!registry.app) registry.app = buildApp();
			return registry.app;
		},
		enablePlugin(manifest, code, css) {
			this.ensureApp();
			return enablePlugin(manifest, code, css);
		},
		disablePlugin,
		openCommandPalette,
		openSettings,
		listCommands() {
			return [...registry.commands.values()].map((c) => ({ id: c.id, name: c.name }));
		},
	};

	console.log("[onyx] Obsidian compatibility runtime ready");
})();
