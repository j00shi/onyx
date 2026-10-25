/** @type {import('stylelint').Config} */
/** See https://stylelint.io/user-guide/rules/ for options */
export default {
	extends: ["stylelint-config-standard-scss"],
	plugins: ["stylelint-prettier"],
	ignoreFiles: ["frontend/src/components/**/*", "**/*.css"],
	rules: {
		"prettier/prettier": true,
	},
};
