// smithy-typescript generated code
import { DatasetServiceClient } from "../DatasetServiceClient";
import {
  ListDatasetCommand,
  ListDatasetCommandInput,
  ListDatasetCommandOutput,
} from "../commands/ListDatasetCommand";
import { DatasetServicePaginationConfiguration } from "./Interfaces";
import { createPaginator } from "@smithy/core";
import { Paginator } from "@smithy/types";

/**
 * @public
 */
export const paginateListDataset: (
    config: DatasetServicePaginationConfiguration,
    input: ListDatasetCommandInput,
    ...rest: any[]
) => Paginator<ListDatasetCommandOutput> =
    createPaginator<DatasetServicePaginationConfiguration, ListDatasetCommandInput, ListDatasetCommandOutput>(
        DatasetServiceClient,
        ListDatasetCommand,
        "nextToken",
        "nextToken",
        "size"
    );
