import {
	BuiltinPluginName,
	type RawMinChunkSizePluginOptions
} from "@rspack/binding";

import { create } from "./base";

export const MinChunkSizePlugin = create(
	BuiltinPluginName.MinChunkSizePlugin,
	(options: RawMinChunkSizePluginOptions) => {
		return options;
	}
);
