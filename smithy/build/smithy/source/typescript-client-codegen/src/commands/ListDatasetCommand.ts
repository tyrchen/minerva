// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  ListDatasetInput,
  ListDatasetOutput,
} from "../models/models_0";
import {
  de_ListDatasetCommand,
  se_ListDatasetCommand,
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
 * The input for {@link ListDatasetCommand}.
 */
export interface ListDatasetCommandInput extends ListDatasetInput {}
/**
 * @public
 *
 * The output of {@link ListDatasetCommand}.
 */
export interface ListDatasetCommandOutput extends ListDatasetOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, ListDatasetCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, ListDatasetCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // ListDatasetInput
 *   size: Number("int"),
 *   nextToken: "STRING_VALUE",
 * };
 * const command = new ListDatasetCommand(input);
 * const response = await client.send(command);
 * // { // ListDatasetOutput
 * //   items: [ // DatasetList // required
 * //     { // DatasetInfo
 * //       name: "STRING_VALUE", // required
 * //       lastModified: new Date("TIMESTAMP"), // required
 * //       size: Number("long"), // required
 * //       fields: [ // DatasetFieldList // required
 * //         { // DatasetField
 * //           name: "STRING_VALUE", // required
 * //           type: "STRING_VALUE", // required
 * //         },
 * //       ],
 * //     },
 * //   ],
 * //   nextToken: "STRING_VALUE",
 * // };
 *
 * ```
 *
 * @param ListDatasetCommandInput - {@link ListDatasetCommandInput}
 * @returns {@link ListDatasetCommandOutput}
 * @see {@link ListDatasetCommandInput} for command's `input` shape.
 * @see {@link ListDatasetCommandOutput} for command's `response` shape.
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
export class ListDatasetCommand extends $Command.classBuilder<ListDatasetCommandInput, ListDatasetCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "ListDataset", {

  })
  .n("DatasetServiceClient", "ListDatasetCommand")
  .f(void 0, void 0)
  .ser(se_ListDatasetCommand)
  .de(de_ListDatasetCommand)
.build() {
}
