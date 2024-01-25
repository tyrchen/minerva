// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  CreateDatasetInput,
  CreateDatasetOutput,
} from "../models/models_0";
import {
  de_CreateDatasetCommand,
  se_CreateDatasetCommand,
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
 * The input for {@link CreateDatasetCommand}.
 */
export interface CreateDatasetCommandInput extends CreateDatasetInput {}
/**
 * @public
 *
 * The output of {@link CreateDatasetCommand}.
 */
export interface CreateDatasetCommandOutput extends CreateDatasetOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, CreateDatasetCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, CreateDatasetCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // CreateDatasetInput
 *   name: "STRING_VALUE", // required
 *   sql: "STRING_VALUE", // required
 * };
 * const command = new CreateDatasetCommand(input);
 * const response = await client.send(command);
 * // { // CreateDatasetOutput
 * //   name: "STRING_VALUE", // required
 * // };
 *
 * ```
 *
 * @param CreateDatasetCommandInput - {@link CreateDatasetCommandInput}
 * @returns {@link CreateDatasetCommandOutput}
 * @see {@link CreateDatasetCommandInput} for command's `input` shape.
 * @see {@link CreateDatasetCommandOutput} for command's `response` shape.
 * @see {@link DatasetServiceClientResolvedConfig | config} for DatasetServiceClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
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
export class CreateDatasetCommand extends $Command.classBuilder<CreateDatasetCommandInput, CreateDatasetCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "CreateDataset", {

  })
  .n("DatasetServiceClient", "CreateDatasetCommand")
  .f(void 0, void 0)
  .ser(se_CreateDatasetCommand)
  .de(de_CreateDatasetCommand)
.build() {
}
