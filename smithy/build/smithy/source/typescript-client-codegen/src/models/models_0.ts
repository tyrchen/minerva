// smithy-typescript generated code
import { EchoServiceException as __BaseException } from "./EchoServiceException";
import { ExceptionOptionType as __ExceptionOptionType } from "@smithy/smithy-client";

/**
 * @public
 */
export interface EchoMessageInput {
  message: string | undefined;
}

/**
 * @public
 */
export interface EchoMessageOutput {
  message: string | undefined;
}

/**
 * @public
 * Describes one specific validation failure for an input member.
 */
export interface ValidationExceptionField {
  /**
   * @public
   * A JSONPointer expression to the structure member whose value failed to satisfy the modeled constraints.
   */
  path: string | undefined;

  /**
   * @public
   * A detailed description of the validation failure.
   */
  message: string | undefined;
}

/**
 * @public
 * A standard error for input validation failures.
 * This should be thrown by services when a member of the input structure
 * falls outside of the modeled or documented constraints.
 */
export class ValidationException extends __BaseException {
  readonly name: "ValidationException" = "ValidationException";
  readonly $fault: "client" = "client";
  /**
   * @public
   * A list of specific failures encountered while validating the input.
   * A member can appear in this list more than once if it failed to satisfy multiple constraints.
   */
  fieldList?: (ValidationExceptionField)[];

  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<ValidationException, __BaseException>) {
    super({
      name: "ValidationException",
      $fault: "client",
      ...opts
    });
    Object.setPrototypeOf(this, ValidationException.prototype);
    this.fieldList = opts.fieldList;
  }
}

/**
 * @public
 * Forbidden error.
 */
export class ForbiddenError extends __BaseException {
  readonly name: "ForbiddenError" = "ForbiddenError";
  readonly $fault: "client" = "client";
  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<ForbiddenError, __BaseException>) {
    super({
      name: "ForbiddenError",
      $fault: "client",
      ...opts
    });
    Object.setPrototypeOf(this, ForbiddenError.prototype);
  }
}

/**
 * @public
 */
export interface SigninInput {
  username: string | undefined;
  password: string | undefined;
}

/**
 * @public
 */
export interface SigninOutput {
  token: string | undefined;
}

/**
 * @public
 * Throttling error.
 */
export class ThrottlingError extends __BaseException {
  readonly name: "ThrottlingError" = "ThrottlingError";
  readonly $fault: "client" = "client";
  $retryable = {
  };
  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<ThrottlingError, __BaseException>) {
    super({
      name: "ThrottlingError",
      $fault: "client",
      ...opts
    });
    Object.setPrototypeOf(this, ThrottlingError.prototype);
  }
}

/**
 * @public
 * Unauthorized error.
 */
export class UnauthorizedError extends __BaseException {
  readonly name: "UnauthorizedError" = "UnauthorizedError";
  readonly $fault: "client" = "client";
  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<UnauthorizedError, __BaseException>) {
    super({
      name: "UnauthorizedError",
      $fault: "client",
      ...opts
    });
    Object.setPrototypeOf(this, UnauthorizedError.prototype);
  }
}
