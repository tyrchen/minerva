// smithy-typescript generated code
import {
  getHttpHandlerExtensionConfiguration,
  resolveHttpHandlerRuntimeConfig,
} from "@smithy/protocol-http";
import {
  getDefaultExtensionConfiguration,
  resolveDefaultRuntimeConfig,
} from "@smithy/smithy-client";
import { DatasetServiceExtensionConfiguration } from "./extensionConfiguration";

/**
 * @public
 */
export interface RuntimeExtension {
    configure(extensionConfiguration: DatasetServiceExtensionConfiguration): void;
}

/**
 * @public
 */
export interface RuntimeExtensionsConfig {
    extensions: RuntimeExtension[]
}

const asPartial = <T extends Partial<DatasetServiceExtensionConfiguration>>(t: T) => t;

/**
 * @internal
 */
export const resolveRuntimeExtensions = (
    runtimeConfig: any,
    extensions: RuntimeExtension[]
) => {
  const extensionConfiguration: DatasetServiceExtensionConfiguration = {
    ...asPartial(getDefaultExtensionConfiguration(runtimeConfig)),
    ...asPartial(getHttpHandlerExtensionConfiguration(runtimeConfig)),
  };

  extensions.forEach(extension => extension.configure(extensionConfiguration));

  return {
    ...runtimeConfig,
    ...resolveDefaultRuntimeConfig(extensionConfiguration),
    ...resolveHttpHandlerRuntimeConfig(extensionConfiguration),
  };
}
