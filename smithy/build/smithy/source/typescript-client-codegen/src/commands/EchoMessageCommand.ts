// smithy-typescript generated code
import {
  EchoClientResolvedConfig,
  ServiceInputTypes,
  ServiceOutputTypes,
} from "../EchoClient";
import {
  EchoMessageInput,
  EchoMessageOutput,
} from "../models/models_0";
import {
  de_EchoMessageCommand,
  se_EchoMessageCommand,
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
 * The input for {@link EchoMessageCommand}.
 */
export interface EchoMessageCommandInput extends EchoMessageInput {}
/**
 * @public
 *
 * The output of {@link EchoMessageCommand}.
 */
export interface EchoMessageCommandOutput extends EchoMessageOutput, __MetadataBearer {}

/**
 * @public
 *
 * @example
 * Use a bare-bones client and the command you need to make an API call.
 * ```javascript
 * import { EchoClient, EchoMessageCommand } from "echo"; // ES Modules import
 * // const { EchoClient, EchoMessageCommand } = require("echo"); // CommonJS import
 * const client = new EchoClient(config);
 * const input = { // EchoMessageInput
 *   message: "STRING_VALUE", // required
 * };
 * const command = new EchoMessageCommand(input);
 * const response = await client.send(command);
 * // { // EchoMessageOutput
 * //   message: "STRING_VALUE", // required
 * // };
 *
 * ```
 *
 * @param EchoMessageCommandInput - {@link EchoMessageCommandInput}
 * @returns {@link EchoMessageCommandOutput}
 * @see {@link EchoMessageCommandInput} for command's `input` shape.
 * @see {@link EchoMessageCommandOutput} for command's `response` shape.
 * @see {@link EchoClientResolvedConfig | config} for EchoClient's `config` shape.
 *
 * @throws {@link ValidationException} (client fault)
 *  A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 *
 * @throws {@link EchoServiceException}
 * <p>Base exception class for all service exceptions from Echo service.</p>
 *
 */
export class EchoMessageCommand extends $Command.classBuilder<EchoMessageCommandInput, EchoMessageCommandOutput, EchoClientResolvedConfig, ServiceInputTypes, ServiceOutputTypes>()
      .m(function (this: any, Command: any, cs: any, config: EchoClientResolvedConfig, o: any) {
          return [

  getSerdePlugin(config, this.serialize, this.deserialize),
      ];
  })
  .s("EchoService", "EchoMessage", {

  })
  .n("EchoClient", "EchoMessageCommand")
  .f(void 0, void 0)
  .ser(se_EchoMessageCommand)
  .de(de_EchoMessageCommand)
.build() {
}
