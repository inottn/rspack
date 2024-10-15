/** @type {import('@rspack/core').Configuration} */
module.exports = {
	entry: "./src/index",
	target: 'web',
	node: false,
	output: {
		publicPath: '/'
	},
	experiments: {
		css: true,
	},
	module: {
		rules: [
			{
				test: /\.less$/,
				use: [
					{
						loader: "less-loader"
					}
				],
				type: "css",
				generator: {
					exportsOnly: false
				}
			}
		]
	}
};
