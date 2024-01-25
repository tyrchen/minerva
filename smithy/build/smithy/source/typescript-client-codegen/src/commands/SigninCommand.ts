// smithy-typescript generated code
import {
  DatasetServiceClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../DatasetServiceClient";
import {
  SigninInput,
  SigninOutput,
} from "../models/models_0";
import {
  de_SigninCommand,
  se_SigninCommand,
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
 * The input for {@link SigninCommand}.
 */
export interface SigninCommandInput extends SigninInput {}
/**
 * @public
 *
 * The output of {@link SigninCommand}.
 */
export interface SigninCommandOutput extends SigninOutput, __MetadataBearer {}

/**
 * @public
 * Signin to get a token.
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { DatasetServiceClient, SigninCommand } from "@minerva/dataset-client"; // ES Modules import
 * // const { DatasetServiceClient, SigninCommand } = require("@minerva/dataset-client"); // CommonJS import
 * const client = new DatasetServiceClient(config);
 * const input = { // SigninInput
 *   username: "STRING_VALUE", // required
 *   password: "STRING_VALUE", // required
 * };
 * const command = new SigninCommand(input);
 * const response = await client.send(command);
 * // { // SigninOutput
 * //   token: "STRING_VALUE", // required
 * // };
 *
 * ```
 *
 * @param SigninCommandInput - {@link SigninCommandInput}
 * @returns {@link SigninCommandOutput}
 * @see {@link SigninCommandInput} for command's `input` shape.
 * @see {@link SigninCommandOutput} for command's `response` shape.
 * @see {@link DatasetServiceClientResolvedConfig | config} for DatasetServiceClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 *
 * @throws {@link UnauthorizedError} (client fault)
 *  Unauthorized error.
 *
 * @throws {@link ForbiddenError} (client fault)
 *  Forbidden error.
 *
 * @throws {@link ThrottlingError} (client fault)
 *  Throttling error.
 *
 * @throws {@link DatasetServiceServiceException}
 * <p>Base exception class for all service exceptions from DatasetService service.</p>
 *
 */
export class SigninCommand extends $Command.classBuilder<SigninCommandInput, SigninCommandOutput, DatasetServiceClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: DatasetServiceClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("DatasetService", "Signin", {

  })
  .n("DatasetServiceClient", "SigninCommand")
  .f(void 0, void 0)
  .ser(se_SigninCommand)
  .de(de_SigninCommand)
.build() {
}
