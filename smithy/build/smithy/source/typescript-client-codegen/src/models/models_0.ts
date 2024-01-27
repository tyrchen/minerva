// smithy-typescript generated code
import { DatasetServiceServiceException as __BaseException } from "./DatasetServiceServiceException";
import { ExceptionOptionType as __ExceptionOptionType } from "@smithy/smithy-client";

/**
 * @public
 */
export interface CreateDatasetInput {
  name: string | undefined;
  sql: string | undefined;
}

/**
 * @public
 */
export interface CreateDatasetOutput {
  name: string | undefined;
}

/**
 * @public
 * @enum
 */
export const ErrorCode = {
  DATABASE: "database",
  INFER: "infer",
  NETWORK: "network",
  UNKNOWN: "unknown",
} as const
/**
 * @public
 */
export type ErrorCode = typeof ErrorCode[keyof typeof ErrorCode]

/**
 * @public
 * Server error.
 */
export class ServerError extends __BaseException {
  readonly name: "ServerError" = "ServerError";
  readonly $fault: "server" = "server";
  code: ErrorCode | undefined;
  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<ServerError, __BaseException>) {
    super({
      name: "ServerError",
      $fault: "server",
      ...opts
    });
    Object.setPrototypeOf(this, ServerError.prototype);
    this.code = opts.code;
  }
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
 */
export interface DatasetField {
  name: string | undefined;
  type: string | undefined;
  nullable: boolean | undefined;
}

/**
 * @public
 */
export interface DatasetInfo {
  name: string | undefined;
  tableName: string | undefined;
  lastModified: Date | undefined;
  size: number | undefined;
  fields: (DatasetField)[] | undefined;
}

/**
 * @public
 */
export interface GetDatasetInput {
  id: string | undefined;
}

/**
 * @public
 * Not found error.
 */
export class NotFoundError extends __BaseException {
  readonly name: "NotFoundError" = "NotFoundError";
  readonly $fault: "client" = "client";
  /**
   * @internal
   */
  constructor(opts: __ExceptionOptionType<NotFoundError, __BaseException>) {
    super({
      name: "NotFoundError",
      $fault: "client",
      ...opts
    });
    Object.setPrototypeOf(this, NotFoundError.prototype);
  }
}

/**
 * @public
 */
export interface ListDatasetInput {
  size?: number;
  nextToken?: string;
}

/**
 * @public
 */
export interface ListDatasetOutput {
  items: (DatasetInfo)[] | undefined;
  nextToken?: string;
}

/**
 * @public
 */
export interface QueryDatasetInput {
  id: string | undefined;
  sql: string | undefined;
}

/**
 * @public
 */
export interface QueryDatasetOutput {
  data: Uint8Array | undefined;
}

/**
 * @public
 */
export interface SampleDatasetInput {
  id: string | undefined;
}

/**
 * @public
 */
export interface SampleDatasetOutput {
  data: Uint8Array | undefined;
}

/**
 * @public
 */
export interface HealthCheckInput {
  message: string | undefined;
}

/**
 * @public
 */
export interface HealthCheckOutput {
  message: string | undefined;
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
