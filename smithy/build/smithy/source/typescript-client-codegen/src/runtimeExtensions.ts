// smithy-typescript generated code
import {
  getHttpHandlerExtensionConfiguration,
  resolveHttpHandlerRuntimeConfig,
} from "@smithy/protocol-http";
import {
  getDefaultExtensionConfiguration,
  resolveDefaultRuntimeConfig,
} from "@smithy/smithy-client";
import { EchoExtensionConfiguration } from "./extensionConfiguration";

/**
 * @public
 */
export interface RuntimeExtension {
    configure(extensionConfiguration: EchoExtensionConfiguration): void;
}

/**
 * @public
 */
export interface RuntimeExtensionsConfig {
    extensions: RuntimeExtension[]
}

const asPartial = <T extends Partial<EchoExtensionConfiguration>>(t: T) => t;

/**
 * @internal
 */
export const resolveRuntimeExtensions = (
    runtimeConfig: any,
    extensions: RuntimeExtension[]
) => {
  const extensionConfiguration: EchoExtensionConfiguration = {
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
