// smithy-typescript generated code
import {
  DatasetServiceClient,
  DatasetServiceClientConfig,
} from "./DatasetServiceClient";
import {
  CreateDatasetCommand,
  CreateDatasetCommandInput,
  CreateDatasetCommandOutput,
} from "./commands/CreateDatasetCommand";
import {
  GetDatasetCommand,
  GetDatasetCommandInput,
  GetDatasetCommandOutput,
} from "./commands/GetDatasetCommand";
import {
  HealthCheckCommand,
  HealthCheckCommandInput,
  HealthCheckCommandOutput,
} from "./commands/HealthCheckCommand";
import {
  ListDatasetCommand,
  ListDatasetCommandInput,
  ListDatasetCommandOutput,
} from "./commands/ListDatasetCommand";
import {
  QueryDatasetCommand,
  QueryDatasetCommandInput,
  QueryDatasetCommandOutput,
} from "./commands/QueryDatasetCommand";
import {
  SampleDatasetCommand,
  SampleDatasetCommandInput,
  SampleDatasetCommandOutput,
} from "./commands/SampleDatasetCommand";
import {
  SigninCommand,
  SigninCommandInput,
  SigninCommandOutput,
} from "./commands/SigninCommand";
import { createAggregatedClient } from "@smithy/smithy-client";
import { HttpHandlerOptions as __HttpHandlerOptions } from "@smithy/types";

const commands = {
  CreateDatasetCommand,
  GetDatasetCommand,
  HealthCheckCommand,
  ListDatasetCommand,
  QueryDatasetCommand,
  SampleDatasetCommand,
  SigninCommand,
}

export interface DatasetService {
  /**
   * @see {@link CreateDatasetCommand}
   */
  createDataset(
    args: CreateDatasetCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<CreateDatasetCommandOutput>;
  createDataset(
    args: CreateDatasetCommandInput,
    cb: (err: any, data?: CreateDatasetCommandOutput) => void
  ): void;
  createDataset(
    args: CreateDatasetCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: CreateDatasetCommandOutput) => void
  ): void;

  /**
   * @see {@link GetDatasetCommand}
   */
  getDataset(
    args: GetDatasetCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<GetDatasetCommandOutput>;
  getDataset(
    args: GetDatasetCommandInput,
    cb: (err: any, data?: GetDatasetCommandOutput) => void
  ): void;
  getDataset(
    args: GetDatasetCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: GetDatasetCommandOutput) => void
  ): void;

  /**
   * @see {@link HealthCheckCommand}
   */
  healthCheck(
    args: HealthCheckCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<HealthCheckCommandOutput>;
  healthCheck(
    args: HealthCheckCommandInput,
    cb: (err: any, data?: HealthCheckCommandOutput) => void
  ): void;
  healthCheck(
    args: HealthCheckCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: HealthCheckCommandOutput) => void
  ): void;

  /**
   * @see {@link ListDatasetCommand}
   */
  listDataset(
    args: ListDatasetCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<ListDatasetCommandOutput>;
  listDataset(
    args: ListDatasetCommandInput,
    cb: (err: any, data?: ListDatasetCommandOutput) => void
  ): void;
  listDataset(
    args: ListDatasetCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: ListDatasetCommandOutput) => void
  ): void;

  /**
   * @see {@link QueryDatasetCommand}
   */
  queryDataset(
    args: QueryDatasetCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<QueryDatasetCommandOutput>;
  queryDataset(
    args: QueryDatasetCommandInput,
    cb: (err: any, data?: QueryDatasetCommandOutput) => void
  ): void;
  queryDataset(
    args: QueryDatasetCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: QueryDatasetCommandOutput) => void
  ): void;

  /**
   * @see {@link SampleDatasetCommand}
   */
  sampleDataset(
    args: SampleDatasetCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<SampleDatasetCommandOutput>;
  sampleDataset(
    args: SampleDatasetCommandInput,
    cb: (err: any, data?: SampleDatasetCommandOutput) => void
  ): void;
  sampleDataset(
    args: SampleDatasetCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: SampleDatasetCommandOutput) => void
  ): void;

  /**
   * @see {@link SigninCommand}
   */
  signin(
    args: SigninCommandInput,
    options?: __HttpHandlerOptions,
  ): Promise<SigninCommandOutput>;
  signin(
    args: SigninCommandInput,
    cb: (err: any, data?: SigninCommandOutput) => void
  ): void;
  signin(
    args: SigninCommandInput,
    options: __HttpHandlerOptions,
    cb: (err: any, data?: SigninCommandOutput) => void
  ): void;

}

/**
 * @public
 */
export class DatasetService extends DatasetServiceClient implements DatasetService {}
createAggregatedClient(commands, DatasetService);
