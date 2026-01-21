"use strict";

/** @type {import("@rspack/core").Configuration[]} */
module.exports = {
	experiments: {
		outputModule: true
	},
	output: {
		module: true,
		chunkFormat: "module"
	}
};
