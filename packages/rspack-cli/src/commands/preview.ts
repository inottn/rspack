import path from "node:path";
import {
	type DevServer,
	type MultiRspackOptions,
	type RspackOptions,
	rspack
} from "@rspack/core";
import type yargs from "yargs";

import type { RspackCLI } from "../cli";
import type { RspackCommand, RspackPreviewCLIOptions } from "../types";
import { commonOptions, setDefaultNodeEnv } from "../utils/options";

const previewOptions = (yargs: yargs.Argv) => {
	yargs.positional("dir", {
		type: "string",
		describe: "directory want to preview"
	});
	return commonOptions(yargs).options({
		publicPath: {
			type: "string",
			describe: "static resource server path"
		},
		port: {
			type: "number",
			describe: "preview server port"
		},
		host: {
			type: "string",
			describe: "preview server host"
		},
		open: {
			type: "boolean",
			describe: "open browser"
		},
		// same as devServer.server
		server: {
			type: "string",
			describe: "Configuration items for the server."
		}
	});
};

const defaultRoot = "dist";
export class PreviewCommand implements RspackCommand {
	async apply(cli: RspackCLI): Promise<void> {
		cli.program.command(
			["preview [dir]", "preview", "p"],
			"run the rspack server for build output",
			previewOptions,
			async options => {
				setDefaultNodeEnv(options, "production");

				const rspackOptions = { ...options, argv: { ...options } };
				const { RspackDevServer } = await import("@rspack/dev-server");

				let config = await cli.loadConfig(rspackOptions);
				config = await getPreviewConfig(config, options);
				if (!Array.isArray(config)) {
					config = [config as RspackOptions];
				}

				config = config as MultiRspackOptions;

				// find the possible devServer config
				config = config.find(item => item.devServer) || config[0];

				const devServerOptions = config.devServer as DevServer;

				try {
					const compiler = rspack({ entry: {} });
					if (!compiler) return;
					const server = new RspackDevServer(devServerOptions, compiler);

					await server.start();
				} catch (error) {
					const logger = cli.getLogger();
					logger.error(error);

					process.exit(2);
				}
			}
		);
	}
}

// get the devServerOptions from the config
async function getPreviewConfig(
	item: RspackOptions | MultiRspackOptions,
	options: RspackPreviewCLIOptions
): Promise<RspackOptions | MultiRspackOptions> {
	const internalPreviewConfig = async (item: RspackOptions) => {
		// all of the options that a preview static server needs(maybe not all)
		item.devServer = {
			static: {
				directory: options.dir
					? path.join(item.context ?? process.cwd(), options.dir)
					: (item.output?.path ??
						path.join(item.context ?? process.cwd(), defaultRoot)),
				publicPath: options.publicPath ?? "/"
			},
			port: options.port ?? 8080,
			proxy: item.devServer?.proxy,
			host: options.host ?? item.devServer?.host,
			open: options.open ?? item.devServer?.open,
			server: options.server ?? item.devServer?.server,
			historyApiFallback: item.devServer?.historyApiFallback
		};
		return item;
	};

	if (Array.isArray(item)) {
		return Promise.all(item.map(internalPreviewConfig));
	}
	return internalPreviewConfig(item as RspackOptions);
}
