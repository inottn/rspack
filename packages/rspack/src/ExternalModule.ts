import binding from "@rspack/binding";
import type { Source } from "webpack-sources";
import { JsSource } from "./util/source";

Object.defineProperty(binding.ExternalModule.prototype, "identifier", {
	enumerable: true,
	configurable: true,
	value(this: binding.ExternalModule): string {
		return this[binding.MODULE_IDENTIFIER_SYMBOL];
	}
});
Object.defineProperty(binding.ExternalModule.prototype, "originalSource", {
	enumerable: true,
	configurable: true,
	value(this: binding.ExternalModule) {
		const originalSource = this._originalSource();
		if (originalSource) {
			return JsSource.__from_binding(originalSource);
		}
		return null;
	}
});
Object.defineProperty(binding.ExternalModule.prototype, "emitFile", {
	enumerable: true,
	configurable: true,
	value(
		this: binding.ExternalModule,
		filename: string,
		source: Source,
		assetInfo?: binding.AssetInfo
	) {
		return this._emitFile(filename, JsSource.__to_binding(source), assetInfo);
	}
});

export { ExternalModule } from "@rspack/binding";
