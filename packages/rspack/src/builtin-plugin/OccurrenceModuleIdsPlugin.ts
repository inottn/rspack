import {
	BuiltinPluginName,
	type RawOccurrenceModuleIdsPluginOptions
} from "@rspack/binding";

import { create } from "./base";

export const OccurrenceModuleIdsPlugin = create(
	BuiltinPluginName.OccurrenceModuleIdsPlugin,
	(options?: RawOccurrenceModuleIdsPluginOptions) => ({ ...options }),
	"compilation"
);
