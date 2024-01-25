// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  HealthCheckInput,
  HealthCheckOutput,
} from "../models/models_0";
import {
  de_HealthCheckCommand,
  se_HealthCheckCommand,
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
 * The input for {@link HealthCheckCommand}.
 */
export interface HealthCheckCommandInput extends HealthCheckInput {}
/**
 * @public
 *
 * The output of {@link HealthCheckCommand}.
 */
export interface HealthCheckCommandOutput extends HealthCheckOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, HealthCheckCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, HealthCheckCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // HealthCheckInput
 *   message: "STRING_VALUE", // required
 * };
 * const command = new HealthCheckCommand(input);
 * const response = await client.send(command);
 * // { // HealthCheckOutput
 * //   message: "STRING_VALUE", // required
 * // };
 *
 * ```
 *
 * @param HealthCheckCommandInput - {@link HealthCheckCommandInput}
 * @returns {@link HealthCheckCommandOutput}
 * @see {@link HealthCheckCommandInput} for command's `input` shape.
 * @see {@link HealthCheckCommandOutput} for command's `response` shape.
 * @see {@link DatasetServiceClientResolvedConfig | config} for DatasetServiceClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 *
 * @throws {@link ServerError} (server fault)
 *  Server error.
 *
 * @throws {@link DatasetServiceServiceException}
 * <p>Base exception class for all service exceptions from DatasetService service.</p>
 *
 */
export class HealthCheckCommand extends $Command.classBuilder<HealthCheckCommandInput, HealthCheckCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "HealthCheck", {

  })
  .n("DatasetServiceClient", "HealthCheckCommand")
  .f(void 0, void 0)
  .ser(se_HealthCheckCommand)
  .de(de_HealthCheckCommand)
.build() {
}
