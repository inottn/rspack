const svgToMiniDataURI = require("mini-svg-data-uri");
const mimeTypes = require("mime-types");

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	mode: "development",
	module: {
		// parser: {
		// 	asset: {
		// 		dataUrlCondition: (source, { filename }) => {
		// 			return filename.includes("?inline");
		// 		}
		// 	}
		// },
		generator: {
			asset: {
				dataUrl: {
					encoding: "base64",
				}
			},
			"asset/inline": {
				dataUrl: {
					encoding: false,
				}
			}
		},
		rules: [
			{
				test: /\.(png|svg)$/,
				type: "asset"
			},
			{
				test: /\.jpg$/,
				type: "asset/inline"
			}
		]
	}
};
