// smithy-typescript generated code
import {
  CreateDatasetCommandInput,
  CreateDatasetCommandOutput,
} from "./commands/CreateDatasetCommand";
import {
  GetDatasetCommandInput,
  GetDatasetCommandOutput,
} from "./commands/GetDatasetCommand";
import {
  HealthCheckCommandInput,
  HealthCheckCommandOutput,
} from "./commands/HealthCheckCommand";
import {
  ListDatasetCommandInput,
  ListDatasetCommandOutput,
} from "./commands/ListDatasetCommand";
import {
  QueryDatasetCommandInput,
  QueryDatasetCommandOutput,
} from "./commands/QueryDatasetCommand";
import {
  SampleDatasetCommandInput,
  SampleDatasetCommandOutput,
} from "./commands/SampleDatasetCommand";
import {
  SigninCommandInput,
  SigninCommandOutput,
} from "./commands/SigninCommand";
import { getRuntimeConfig as __getRuntimeConfig } from "./runtimeConfig";
import {
  RuntimeExtension,
  RuntimeExtensionsConfig,
  resolveRuntimeExtensions,
} from "./runtimeExtensions";
import {
  HostHeaderInputConfig,
  HostHeaderResolvedConfig,
  getHostHeaderPlugin,
  resolveHostHeaderConfig,
} from "@aws-sdk/middleware-host-header";
import { getLoggerPlugin } from "@aws-sdk/middleware-logger";
import { getRecursionDetectionPlugin } from "@aws-sdk/middleware-recursion-detection";
import {
  TokenInputConfig,
  TokenResolvedConfig,
  getTokenPlugin,
  resolveTokenConfig,
} from "@aws-sdk/middleware-token";
import {
  UserAgentInputConfig,
  UserAgentResolvedConfig,
  getUserAgentPlugin,
  resolveUserAgentConfig,
} from "@aws-sdk/middleware-user-agent";
import {
  CustomEndpointsInputConfig,
  CustomEndpointsResolvedConfig,
  resolveCustomEndpointsConfig,
} from "@smithy/config-resolver";
import { getContentLengthPlugin } from "@smithy/middleware-content-length";
import {
  RetryInputConfig,
  RetryResolvedConfig,
  getRetryPlugin,
  resolveRetryConfig,
} from "@smithy/middleware-retry";
import { HttpHandler as __HttpHandler } from "@smithy/protocol-http";
import {
  Client as __Client,
  DefaultsMode as __DefaultsMode,
  SmithyConfiguration as __SmithyConfiguration,
  SmithyResolvedConfiguration as __SmithyResolvedConfiguration,
} from "@smithy/smithy-client";
import {
  Provider,
  BodyLengthCalculator as __BodyLengthCalculator,
  CheckOptionalClientConfig as __CheckOptionalClientConfig,
  ChecksumConstructor as __ChecksumConstructor,
  Decoder as __Decoder,
  Encoder as __Encoder,
  HashConstructor as __HashConstructor,
  HttpHandlerOptions as __HttpHandlerOptions,
  Logger as __Logger,
  Provider as __Provider,
  StreamCollector as __StreamCollector,
  UrlParser as __UrlParser,
  UserAgent as __UserAgent,
} from "@smithy/types";

export { __Client }

/**
 * @public
 */
export type ServiceInputTypes =
  | CreateDatasetCommandInput
  | GetDatasetCommandInput
  | HealthCheckCommandInput
  | ListDatasetCommandInput
  | QueryDatasetCommandInput
  | SampleDatasetCommandInput
  | SigninCommandInput;

/**
 * @public
 */
export type ServiceOutputTypes =
  | CreateDatasetCommandOutput
  | GetDatasetCommandOutput
  | HealthCheckCommandOutput
  | ListDatasetCommandOutput
  | QueryDatasetCommandOutput
  | SampleDatasetCommandOutput
  | SigninCommandOutput;

/**
 * @public
 */
export interface ClientDefaults
  extends Partial<__SmithyResolvedConfiguration<__HttpHandlerOptions>> {
  /**
   * The HTTP handler to use. Fetch in browser and Https in Nodejs.
   */
  requestHandler?: __HttpHandler;

  /**
   * A constructor for a class implementing the {@link @smithy/types#ChecksumConstructor} interface
   * that computes the SHA-256 HMAC or checksum of a string or binary buffer.
   * @internal
   */
  sha256?: __ChecksumConstructor | __HashConstructor;

  /**
   * The function that will be used to convert strings into HTTP endpoints.
   * @internal
   */
  urlParser?: __UrlParser;

  /**
   * A function that can calculate the length of a request body.
   * @internal
   */
  bodyLengthChecker?: __BodyLengthCalculator;

  /**
   * A function that converts a stream into an array of bytes.
   * @internal
   */
  streamCollector?: __StreamCollector;

  /**
   * The function that will be used to convert a base64-encoded string to a byte array.
   * @internal
   */
  base64Decoder?: __Decoder;

  /**
   * The function that will be used to convert binary data to a base64-encoded string.
   * @internal
   */
  base64Encoder?: __Encoder;

  /**
   * The function that will be used to convert a UTF8-encoded string to a byte array.
   * @internal
   */
  utf8Decoder?: __Decoder;

  /**
   * The function that will be used to convert binary data to a UTF-8 encoded string.
   * @internal
   */
  utf8Encoder?: __Encoder;

  /**
   * The runtime environment.
   * @internal
   */
  runtime?: string;

  /**
   * Disable dynamically changing the endpoint of the client based on the hostPrefix
   * trait of an operation.
   */
  disableHostPrefix?: boolean;

  /**
   * Value for how many times a request will be made at most in case of retry.
   */
  maxAttempts?: number | __Provider<number>;

  /**
   * Specifies which retry algorithm to use.
   * @see https://docs.aws.amazon.com/AWSJavaScriptSDK/v3/latest/Package/-smithy-util-retry/Enum/RETRY_MODES/
   *
   */
  retryMode?: string | __Provider<string>;

  /**
   * Optional logger for logging debug/info/warn/error.
   */
  logger?: __Logger;

  /**
   * Optional extensions
   */
  extensions?: RuntimeExtension[];

  /**
   * The {@link @smithy/smithy-client#DefaultsMode} that will be used to determine how certain default configuration options are resolved in the SDK.
   */
  defaultsMode?: __DefaultsMode | __Provider<__DefaultsMode>;

  /**
   * The provider populating default tracking information to be sent with `user-agent`, `x-amz-user-agent` header
   * @internal
   */
  defaultUserAgentProvider?: Provider<__UserAgent>;

}

/**
 * @public
 */
export type DatasetServiceClientConfigType = Partial<__SmithyConfiguration<__HttpHandlerOptions>>
  & ClientDefaults
  & CustomEndpointsInputConfig
  & RetryInputConfig
  & HostHeaderInputConfig
  & TokenInputConfig
  & UserAgentInputConfig
/**
 * @public
 *
 *  The configuration interface of DatasetServiceClient class constructor that set the region, credentials and other options.
 */
export interface DatasetServiceClientConfig extends DatasetServiceClientConfigType {}

/**
 * @public
 */
export type DatasetServiceClientResolvedConfigType = __SmithyResolvedConfiguration<__HttpHandlerOptions>
  & Required<ClientDefaults>
  & RuntimeExtensionsConfig
  & CustomEndpointsResolvedConfig
  & RetryResolvedConfig
  & HostHeaderResolvedConfig
  & TokenResolvedConfig
  & UserAgentResolvedConfig
/**
 * @public
 *
 *  The resolved configuration interface of DatasetServiceClient class. This is resolved and normalized from the {@link DatasetServiceClientConfig | constructor configuration interface}.
 */
export interface DatasetServiceClientResolvedConfig extends DatasetServiceClientResolvedConfigType {}

/**
 * @public
 */
export class DatasetServiceClient extends __Client<
  __HttpHandlerOptions,
  ServiceInputTypes,
  ServiceOutputTypes,
  DatasetServiceClientResolvedConfig
> {
  /**
   * The resolved configuration of DatasetServiceClient class. This is resolved and normalized from the {@link DatasetServiceClientConfig | constructor configuration interface}.
   */
  readonly config: DatasetServiceClientResolvedConfig;

  constructor(...[configuration]: __CheckOptionalClientConfig<DatasetServiceClientConfig>) {
    let _config_0 = __getRuntimeConfig(configuration || {});
    let _config_1 = resolveCustomEndpointsConfig(_config_0);
    let _config_2 = resolveRetryConfig(_config_1);
    let _config_3 = resolveHostHeaderConfig(_config_2);
    let _config_4 = resolveTokenConfig(_config_3);
    let _config_5 = resolveUserAgentConfig(_config_4);
    let _config_6 = resolveRuntimeExtensions(_config_5, configuration?.extensions || []);
    super(_config_6);
    this.config = _config_6;
    this.middlewareStack.use(getRetryPlugin(this.config));
    this.middlewareStack.use(getContentLengthPlugin(this.config));
    this.middlewareStack.use(getHostHeaderPlugin(this.config));
    this.middlewareStack.use(getLoggerPlugin(this.config));
    this.middlewareStack.use(getRecursionDetectionPlugin(this.config));
    this.middlewareStack.use(getTokenPlugin(this.config));
    this.middlewareStack.use(getUserAgentPlugin(this.config));
  }

  /**
   * Destroy underlying resources, like sockets. It's usually not necessary to do this.
   * However in Node.js, it's best to explicitly shut down the client's agent when it is no longer needed.
   * Otherwise, sockets might stay open for quite a long time before the server terminates them.
   */
  destroy(): void {
    super.destroy();
  }
}
