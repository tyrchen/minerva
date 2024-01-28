// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  SampleDatasetInput,
  SampleDatasetOutput,
} from "../models/models_0";
import {
  de_SampleDatasetCommand,
  se_SampleDatasetCommand,
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
 * The input for {@link SampleDatasetCommand}.
 */
export interface SampleDatasetCommandInput extends SampleDatasetInput {}
/**
 * @public
 *
 * The output of {@link SampleDatasetCommand}.
 */
export interface SampleDatasetCommandOutput extends SampleDatasetOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, SampleDatasetCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, SampleDatasetCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // SampleDatasetInput
 *   id: "STRING_VALUE", // required
 * };
 * const command = new SampleDatasetCommand(input);
 * const response = await client.send(command);
 * // { // SampleDatasetOutput
 * //   data: "BLOB_VALUE", // required
 * // };
 *
 * ```
 *
 * @param SampleDatasetCommandInput - {@link SampleDatasetCommandInput}
 * @returns {@link SampleDatasetCommandOutput}
 * @see {@link SampleDatasetCommandInput} for command's `input` shape.
 * @see {@link SampleDatasetCommandOutput} for command's `response` shape.
 * @see {@link DatasetServiceClientResolvedConfig | config} for DatasetServiceClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 *
 * @throws {@link NotFoundError} (client fault)
 *  Not found error.
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
export class SampleDatasetCommand extends $Command.classBuilder<SampleDatasetCommandInput, SampleDatasetCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "SampleDataset", {

  })
  .n("DatasetServiceClient", "SampleDatasetCommand")
  .f(void 0, void 0)
  .ser(se_SampleDatasetCommand)
  .de(de_SampleDatasetCommand)
.build() {
}
