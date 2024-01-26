// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  DatasetInfo,
  GetDatasetInput,
} from "../models/models_0";
import {
  de_GetDatasetCommand,
  se_GetDatasetCommand,
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
 * The input for {@link GetDatasetCommand}.
 */
export interface GetDatasetCommandInput extends GetDatasetInput {}
/**
 * @public
 *
 * The output of {@link GetDatasetCommand}.
 */
export interface GetDatasetCommandOutput extends DatasetInfo, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, GetDatasetCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, GetDatasetCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // GetDatasetInput
 *   id: "STRING_VALUE", // required
 * };
 * const command = new GetDatasetCommand(input);
 * const response = await client.send(command);
 * // { // DatasetInfo
 * //   name: "STRING_VALUE", // required
 * //   table_name: "STRING_VALUE", // required
 * //   lastModified: new Date("TIMESTAMP"), // required
 * //   size: Number("long"), // required
 * //   fields: [ // DatasetFieldList // required
 * //     { // DatasetField
 * //       name: "STRING_VALUE", // required
 * //       type: "STRING_VALUE", // required
 * //       nullable: true || false, // required
 * //     },
 * //   ],
 * // };
 *
 * ```
 *
 * @param GetDatasetCommandInput - {@link GetDatasetCommandInput}
 * @returns {@link GetDatasetCommandOutput}
 * @see {@link GetDatasetCommandInput} for command's `input` shape.
 * @see {@link GetDatasetCommandOutput} for command's `response` shape.
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
export class GetDatasetCommand extends $Command.classBuilder<GetDatasetCommandInput, GetDatasetCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "GetDataset", {

  })
  .n("DatasetServiceClient", "GetDatasetCommand")
  .f(void 0, void 0)
  .ser(se_GetDatasetCommand)
  .de(de_GetDatasetCommand)
.build() {
}
