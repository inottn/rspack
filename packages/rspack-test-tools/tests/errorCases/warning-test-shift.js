/** @type {import('../..').TErrorCaseConfig} */
module.exports = {
	description: "Testing proxy methods on warnings: test shift&unshift",
	options() {
		return {
			entry: "./require.main.require",
			plugins: [
				compiler => {
					compiler.hooks.afterCompile.tap(
						"test shift and unshift",
						compilation => {
							compilation.warnings.shift();
							compilation.warnings.unshift(new Error("test unshift"));
						}
					);
				}
			]
		};
	},
	async check(diagnostics) {
		expect(diagnostics).toMatchInlineSnapshot(`
		Object {
		  "errors": Array [],
		  "warnings": Array [
		    Object {
		      "code": "Error",
		      "message": "  ⚠ test unshift\\n",
		      "moduleTrace": Array [],
		      "stack": "Error: test unshift\\n    at <TEST_TOOLS_ROOT>/tests/errorCases/warning-test-shift.js<LINE_COL>\\n    at Object.fn (<RSPACK_ROOT>/dist/index.js<LINE_COL>)\\n    at next (<ROOT>/node_modules/<PNPM_INNER>/@rspack/lite-tapable/dist/index.js<LINE_COL>)\\n    at AsyncSeriesHook.callAsyncStageRange (<ROOT>/node_modules/<PNPM_INNER>/@rspack/lite-tapable/dist/index.js<LINE_COL>)\\n    at AsyncSeriesHook.callAsync (<ROOT>/node_modules/<PNPM_INNER>/@rspack/lite-tapable/dist/index.js<LINE_COL>)\\n    at <RSPACK_ROOT>/dist/index.js<LINE_COL>",
		    },
		  ],
		}
	`);
	}
};
