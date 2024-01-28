// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  QueryDatasetInput,
  QueryDatasetOutput,
} from "../models/models_0";
import {
  de_QueryDatasetCommand,
  se_QueryDatasetCommand,
} from "../protocols/Aws_restJson1";
import { getSerdePlugin } from "@smithy/middleware-serde";
import { Command as $Command } from "@smithy/smithy-client";
import { MetadataBearer as __MetadataBearer } from "@smithy/types";

/**
 * @public
 */
export { __MetadataBearer, $Command };
/**
 * @public
 *
 * The input for {@link QueryDatasetCommand}.
 */
export interface QueryDatasetCommandInput extends QueryDatasetInput {}
/**
 * @public
 *
 * The output of {@link QueryDatasetCommand}.
 */
export interface QueryDatasetCommandOutput extends QueryDatasetOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, QueryDatasetCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, QueryDatasetCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // QueryDatasetInput
 *   id: "STRING_VALUE", // required
 *   sql: "STRING_VALUE", // required
 * };
 * const command = new QueryDatasetCommand(input);
 * const response = await client.send(command);
 * // { // QueryDatasetOutput
 * //   data: "BLOB_VALUE", // required
 * // };
 *
 * ```
 *
 * @param QueryDatasetCommandInput - {@link QueryDatasetCommandInput}
 * @returns {@link QueryDatasetCommandOutput}
 * @see {@link QueryDatasetCommandInput} for command's `input` shape.
 * @see {@link QueryDatasetCommandOutput} for command's `response` shape.
 * @see {@link DatasetServiceClientResolvedConfig | config} for DatasetServiceClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 *
 * @throws {@link ClickhouseQueryError} (client fault)
 *
 * @throws {@link ThrottlingError} (client fault)
 *  Throttling error.
 *
 * @throws {@link ServerError} (server fault)
 *  Server error.
 *
 * @throws {@link DatasetServiceServiceException}
 * <p>Base exception class for all service exceptions from DatasetService service.</p>
 *
 */
export class QueryDatasetCommand extends $Command.classBuilder<QueryDatasetCommandInput, QueryDatasetCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "QueryDataset", {

  })
  .n("DatasetServiceClient", "QueryDatasetCommand")
  .f(void 0, void 0)
  .ser(se_QueryDatasetCommand)
  .de(de_QueryDatasetCommand)
.build() {
}
