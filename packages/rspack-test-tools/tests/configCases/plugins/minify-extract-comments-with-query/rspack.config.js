const { rspack } = require("@rspack/core");

/**@type {import("@rspack/core").Configuration}*/
module.exports = {
	output: {
		filename: "bundle0.js?hash=[contenthash]"
	},
	optimization: {
		minimize: true,
		minimizer: [
			new rspack.SwcJsMinimizerRspackPlugin({
				extractComments: true
			})
		]
	}
};
