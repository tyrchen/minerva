// smithy-typescript generated code
import {
  EchoMessageCommandInput,
  EchoMessageCommandOutput,
} from "../commands/EchoMessageCommand";
import {
  SigninCommandInput,
  SigninCommandOutput,
} from "../commands/SigninCommand";
import { EchoServiceException as __BaseException } from "../models/EchoServiceException";
import {
  ForbiddenError,
  ThrottlingError,
  UnauthorizedError,
  ValidationException,
} from "../models/models_0";
import { requestBuilder as rb } from "@smithy/core";
import {
  HttpRequest as __HttpRequest,
  HttpResponse as __HttpResponse,
} from "@smithy/protocol-http";
import {
  decorateServiceException as __decorateServiceException,
  expectNonNull as __expectNonNull,
  expectObject as __expectObject,
  expectString as __expectString,
  _json,
  collectBody,
  map,
  take,
  withBaseException,
} from "@smithy/smithy-client";
import {
  Endpoint as __Endpoint,
  ResponseMetadata as __ResponseMetadata,
  SerdeContext as __SerdeContext,
} from "@smithy/types";

/**
 * serializeAws_restJson1EchoMessageCommand
 */
export const se_EchoMessageCommand = async(
  input: EchoMessageCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
    'content-type': 'application/json',
  };
  b.bp("/echo");
  let body: any;
  body = JSON.stringify(take(input, {
    'message': [],
  }));
  b.m("POST")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1SigninCommand
 */
export const se_SigninCommand = async(
  input: SigninCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
    'content-type': 'application/json',
  };
  b.bp("/signin");
  let body: any;
  body = JSON.stringify(take(input, {
    'password': [],
    'username': [],
  }));
  b.m("POST")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * deserializeAws_restJson1EchoMessageCommand
 */
export const de_EchoMessageCommand = async(
  output: __HttpResponse,
  context: __SerdeContext
): Promise<EchoMessageCommandOutput> => {
  if (output.statusCode !== 200 && output.statusCode >= 300) {
    return de_EchoMessageCommandError(output, context);
  }
  const contents: any = map({
    $metadata: deserializeMetadata(output),
  });
  const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
  const doc = take(data, {
    'message': __expectString,
  });
  Object.assign(contents, doc);
  return contents;
}

/**
 * deserializeAws_restJson1EchoMessageCommandError
 */
const de_EchoMessageCommandError = async(
  output: __HttpResponse,
  context: __SerdeContext,
): Promise<EchoMessageCommandOutput> => {
  const parsedOutput: any = {
    ...output,
    body: await parseErrorBody(output.body, context)
  };
  const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
  switch (errorCode) {
    case "ValidationException":
    case "smithy.framework#ValidationException":
      throw await de_ValidationExceptionRes(parsedOutput, context);
    default:
      const parsedBody = parsedOutput.body;
      return throwDefaultError({
        output,
        parsedBody,
        errorCode
      })
    }
  }

  /**
   * deserializeAws_restJson1SigninCommand
   */
  export const de_SigninCommand = async(
    output: __HttpResponse,
    context: __SerdeContext
  ): Promise<SigninCommandOutput> => {
    if (output.statusCode !== 200 && output.statusCode >= 300) {
      return de_SigninCommandError(output, context);
    }
    const contents: any = map({
      $metadata: deserializeMetadata(output),
    });
    const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
    const doc = take(data, {
      'token': __expectString,
    });
    Object.assign(contents, doc);
    return contents;
  }

  /**
   * deserializeAws_restJson1SigninCommandError
   */
  const de_SigninCommandError = async(
    output: __HttpResponse,
    context: __SerdeContext,
  ): Promise<SigninCommandOutput> => {
    const parsedOutput: any = {
      ...output,
      body: await parseErrorBody(output.body, context)
    };
    const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
    switch (errorCode) {
      case "ForbiddenError":
      case "com.example#ForbiddenError":
        throw await de_ForbiddenErrorRes(parsedOutput, context);
      case "ThrottlingError":
      case "com.example#ThrottlingError":
        throw await de_ThrottlingErrorRes(parsedOutput, context);
      case "UnauthorizedError":
      case "com.example#UnauthorizedError":
        throw await de_UnauthorizedErrorRes(parsedOutput, context);
      case "ValidationException":
      case "smithy.framework#ValidationException":
        throw await de_ValidationExceptionRes(parsedOutput, context);
      default:
        const parsedBody = parsedOutput.body;
        return throwDefaultError({
          output,
          parsedBody,
          errorCode
        })
      }
    }

    const throwDefaultError = withBaseException(__BaseException);
    /**
     * deserializeAws_restJson1ForbiddenErrorRes
     */
    const de_ForbiddenErrorRes = async (
      parsedOutput: any,
      context: __SerdeContext
    ): Promise<ForbiddenError> => {
      const contents: any = map({
      });
      const data: any = parsedOutput.body;
      const doc = take(data, {
        'message': __expectString,
      });
      Object.assign(contents, doc);
      const exception = new ForbiddenError({
        $metadata: deserializeMetadata(parsedOutput),
        ...contents
      });
      return __decorateServiceException(exception, parsedOutput.body);
    };

    /**
     * deserializeAws_restJson1ThrottlingErrorRes
     */
    const de_ThrottlingErrorRes = async (
      parsedOutput: any,
      context: __SerdeContext
    ): Promise<ThrottlingError> => {
      const contents: any = map({
      });
      const data: any = parsedOutput.body;
      const doc = take(data, {
        'message': __expectString,
      });
      Object.assign(contents, doc);
      const exception = new ThrottlingError({
        $metadata: deserializeMetadata(parsedOutput),
        ...contents
      });
      return __decorateServiceException(exception, parsedOutput.body);
    };

    /**
     * deserializeAws_restJson1UnauthorizedErrorRes
     */
    const de_UnauthorizedErrorRes = async (
      parsedOutput: any,
      context: __SerdeContext
    ): Promise<UnauthorizedError> => {
      const contents: any = map({
      });
      const data: any = parsedOutput.body;
      const doc = take(data, {
        'message': __expectString,
      });
      Object.assign(contents, doc);
      const exception = new UnauthorizedError({
        $metadata: deserializeMetadata(parsedOutput),
        ...contents
      });
      return __decorateServiceException(exception, parsedOutput.body);
    };

    /**
     * deserializeAws_restJson1ValidationExceptionRes
     */
    const de_ValidationExceptionRes = async (
      parsedOutput: any,
      context: __SerdeContext
    ): Promise<ValidationException> => {
      const contents: any = map({
      });
      const data: any = parsedOutput.body;
      const doc = take(data, {
        'fieldList': _json,
        'message': __expectString,
      });
      Object.assign(contents, doc);
      const exception = new ValidationException({
        $metadata: deserializeMetadata(parsedOutput),
        ...contents
      });
      return __decorateServiceException(exception, parsedOutput.body);
    };

    // de_ValidationExceptionField omitted.

    // de_ValidationExceptionFieldList omitted.

    const deserializeMetadata = (output: __HttpResponse): __ResponseMetadata => ({
      httpStatusCode: output.statusCode,
      requestId: output.headers["x-amzn-requestid"] ?? output.headers["x-amzn-request-id"] ?? output.headers["x-amz-request-id"],
      extendedRequestId: output.headers["x-amz-id-2"],
      cfId: output.headers["x-amz-cf-id"],
    });

    // Encode Uint8Array data into string with utf-8.
    const collectBodyString = (streamBody: any, context: __SerdeContext): Promise<string> => collectBody(streamBody, context).then(body => context.utf8Encoder(body))

    const isSerializableHeaderValue = (value: any): boolean =>
      value !== undefined &&
      value !== null &&
      value !== "" &&
      (!Object.getOwnPropertyNames(value).includes("length") ||
        value.length != 0) &&
      (!Object.getOwnPropertyNames(value).includes("size") || value.size != 0);

    const parseBody = (streamBody: any, context: __SerdeContext): any => collectBodyString(streamBody, context).then(encoded => {
      if (encoded.length) {
        return JSON.parse(encoded);
      }
      return {};
    });

    const parseErrorBody = async (errorBody: any, context: __SerdeContext) => {
      const value = await parseBody(errorBody, context);
      value.message = value.message ?? value.Message;
      return value;
    }

    /**
     * Load an error code for the aws.rest-json-1.1 protocol.
     */
    const loadRestJsonErrorCode = (output: __HttpResponse, data: any): string | undefined => {
      const findKey = (object: any, key: string) => Object.keys(object).find((k) => k.toLowerCase() === key.toLowerCase());

      const sanitizeErrorCode = (rawValue: string | number): string => {
        let cleanValue = rawValue;
        if (typeof cleanValue === "number") {
          cleanValue = cleanValue.toString();
        }
        if (cleanValue.indexOf(",") >= 0) {
          cleanValue = cleanValue.split(",")[0];
        }
        if (cleanValue.indexOf(":") >= 0) {
          cleanValue = cleanValue.split(":")[0];
        }
        if (cleanValue.indexOf("#") >= 0) {
          cleanValue = cleanValue.split("#")[1];
        }
        return cleanValue;
      };

      const headerKey = findKey(output.headers, "x-amzn-errortype");
      if (headerKey !== undefined) {
        return sanitizeErrorCode(output.headers[headerKey]);
      }

      if (data.code !== undefined) {
        return sanitizeErrorCode(data.code);
      }

      if (data["__type"] !== undefined) {
        return sanitizeErrorCode(data["__type"]);
      }
    };
