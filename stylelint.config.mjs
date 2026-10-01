/** @type {import('stylelint').Config} */
export default {
	extends: ["stylelint-config-standard-scss"],
	plugins: ["stylelint-prettier"],
	ignoreFiles: ["ui/src/standard/**/*", "**/*.css"],
	rules: {
		// Prettier owns formatting; surface any disagreement as a lint error.
		"prettier/prettier": true,

		// Strict, but relaxed where the project's conventions differ.
		"scss/at-rule-no-unknown": true,
		"no-descending-specificity": null,
		"custom-property-pattern": null,
		"selector-class-pattern": null,
		"scss/dollar-variable-pattern": null,
		// `$position/$size` in the `background` shorthand is CSS syntax, not division.
		"scss/operator-no-unspaced": null,
	},
};
