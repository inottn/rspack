/** @type {import("@rspack/core").Configuration} */
module.exports = {
	builtins: {
		html: [
			{
				filename: "[name].[hash].html"
			},
			{
				template: "./index.html",
				filename: "[name].[hash].html"
			}
		]
	},
};
