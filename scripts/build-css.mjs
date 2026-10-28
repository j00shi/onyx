// Compiles the project's SCSS to the committed `.css` artifacts — the single
// source of truth, kept byte-identical to the VS Code Live Sass Compiler (see
// .vscode/settings.json). Don't use a bare `sass` CLI: a different Dart Sass
// version changes the output and skips Autoprefixer. Toolchain pinned in package.json.
//
//   node scripts/build-css.mjs           compile in place
//   node scripts/build-css.mjs --check   verify committed .css is current (CI)

import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import * as sass from "sass";
import postcss from "postcss";
import autoprefixer from "autoprefixer";

const srcDir = resolve(dirname(fileURLToPath(import.meta.url)), "../frontend/src");
const check = process.argv.includes("--check");

// Compilable entrypoints: non-partial `.scss`, skipping the vendored `components/` dir.
function* scssFiles(dir) {
	for (const e of readdirSync(dir, { withFileTypes: true })) {
		const p = join(dir, e.name);
		if (e.isDirectory()) {
			if (e.name !== "components") yield* scssFiles(p);
		} else if (e.name.endsWith(".scss") && !e.name.startsWith("_")) {
			yield p;
		}
	}
}

let stale = 0;
for (const scss of scssFiles(srcDir)) {
	const { css } = sass.compile(scss, { style: "compressed", sourceMap: false });
	// `from: undefined` keeps Autoprefixer from rewriting unrelated whitespace; trim
	// the trailing newline so output matches the extension exactly.
	const out = (await postcss([autoprefixer()]).process(css, { from: undefined })).css.replace(/\n+$/, "");
	const cssPath = scss.replace(/\.scss$/, ".css");

	if (!check) {
		writeFileSync(cssPath, out);
	} else if (readFileSync(cssPath, "utf8") !== out) {
		console.error(`stale: ${relative(srcDir, cssPath)}`);
		stale++;
	}
}

if (stale) {
	console.error(`\n${stale} .css file(s) out of date — run \`npm run build:css\`.`);
	process.exit(1);
}
