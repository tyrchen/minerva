// smithy-typescript generated code
import {
  CreateDatasetCommandInput,
  CreateDatasetCommandOutput,
} from "../commands/CreateDatasetCommand";
import {
  GetDatasetCommandInput,
  GetDatasetCommandOutput,
} from "../commands/GetDatasetCommand";
import {
  HealthCheckCommandInput,
  HealthCheckCommandOutput,
} from "../commands/HealthCheckCommand";
import {
  ListDatasetCommandInput,
  ListDatasetCommandOutput,
} from "../commands/ListDatasetCommand";
import {
  QueryDatasetCommandInput,
  QueryDatasetCommandOutput,
} from "../commands/QueryDatasetCommand";
import {
  SampleDatasetCommandInput,
  SampleDatasetCommandOutput,
} from "../commands/SampleDatasetCommand";
import {
  SigninCommandInput,
  SigninCommandOutput,
} from "../commands/SigninCommand";
import { DatasetServiceServiceException as __BaseException } from "../models/DatasetServiceServiceException";
import {
  ClickhouseQueryError,
  DatasetInfo,
  ForbiddenError,
  NotFoundError,
  ServerError,
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
  expectLong as __expectLong,
  expectNonNull as __expectNonNull,
  expectNumber as __expectNumber,
  expectObject as __expectObject,
  expectString as __expectString,
  extendedEncodeURIComponent as __extendedEncodeURIComponent,
  parseEpochTimestamp as __parseEpochTimestamp,
  resolvedPath as __resolvedPath,
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
 * serializeAws_restJson1CreateDatasetCommand
 */
export const se_CreateDatasetCommand = async(
  input: CreateDatasetCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
    'content-type': 'application/json',
  };
  b.bp("/datasets");
  let body: any;
  body = JSON.stringify(take(input, {
    'name': [],
    'sql': [],
  }));
  b.m("POST")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1GetDatasetCommand
 */
export const se_GetDatasetCommand = async(
  input: GetDatasetCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
  };
  b.bp("/datasets/{id}");
  b.p('id', () => input.id!, '{id}', false)
  let body: any;
  b.m("GET")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1HealthCheckCommand
 */
export const se_HealthCheckCommand = async(
  input: HealthCheckCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = map({}, isSerializableHeaderValue, {
    [_xm]: input[_m]!,
  });
  b.bp("/health");
  let body: any;
  b.m("GET")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1ListDatasetCommand
 */
export const se_ListDatasetCommand = async(
  input: ListDatasetCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
  };
  b.bp("/datasets");
  const query: any = map({
    [_l]: [() => input.size !== void 0, () => (input[_s]!.toString())],
    [_nT]: [,input[_nT]!],
  });
  let body: any;
  b.m("GET")
  .h(headers)
  .q(query)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1QueryDatasetCommand
 */
export const se_QueryDatasetCommand = async(
  input: QueryDatasetCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
    'content-type': 'application/json',
  };
  b.bp("/datasets/{id}/query");
  b.p('id', () => input.id!, '{id}', false)
  let body: any;
  body = JSON.stringify(take(input, {
    'sql': [],
  }));
  b.m("POST")
  .h(headers)
  .b(body);
  return b.build();
}

/**
 * serializeAws_restJson1SampleDatasetCommand
 */
export const se_SampleDatasetCommand = async(
  input: SampleDatasetCommandInput,
  context: __SerdeContext
): Promise<__HttpRequest> => {
  const b = rb(input, context);
  const headers: any = {
  };
  b.bp("/datasets/{id}/sample");
  b.p('id', () => input.id!, '{id}', false)
  let body: any;
  b.m("GET")
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
 * deserializeAws_restJson1CreateDatasetCommand
 */
export const de_CreateDatasetCommand = async(
  output: __HttpResponse,
  context: __SerdeContext
): Promise<CreateDatasetCommandOutput> => {
  if (output.statusCode !== 201 && output.statusCode >= 300) {
    return de_CreateDatasetCommandError(output, context);
  }
  const contents: any = map({
    $metadata: deserializeMetadata(output),
    [_n]: [, output.headers[_xdn]],
  });
  await collectBody(output.body, context);
  return contents;
}

/**
 * deserializeAws_restJson1CreateDatasetCommandError
 */
const de_CreateDatasetCommandError = async(
  output: __HttpResponse,
  context: __SerdeContext,
): Promise<CreateDatasetCommandOutput> => {
  const parsedOutput: any = {
    ...output,
    body: await parseErrorBody(output.body, context)
  };
  const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
  switch (errorCode) {
    case "ServerError":
    case "com.minerva#ServerError":
      throw await de_ServerErrorRes(parsedOutput, context);
    case "ThrottlingError":
    case "com.minerva#ThrottlingError":
      throw await de_ThrottlingErrorRes(parsedOutput, context);
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
   * deserializeAws_restJson1GetDatasetCommand
   */
  export const de_GetDatasetCommand = async(
    output: __HttpResponse,
    context: __SerdeContext
  ): Promise<GetDatasetCommandOutput> => {
    if (output.statusCode !== 200 && output.statusCode >= 300) {
      return de_GetDatasetCommandError(output, context);
    }
    const contents: any = map({
      $metadata: deserializeMetadata(output),
    });
    const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
    const doc = take(data, {
      'fields': _json,
      'lastModified': _ => __expectNonNull(__parseEpochTimestamp(__expectNumber(_))),
      'name': __expectString,
      'size': __expectLong,
      'tableName': __expectString,
    });
    Object.assign(contents, doc);
    return contents;
  }

  /**
   * deserializeAws_restJson1GetDatasetCommandError
   */
  const de_GetDatasetCommandError = async(
    output: __HttpResponse,
    context: __SerdeContext,
  ): Promise<GetDatasetCommandOutput> => {
    const parsedOutput: any = {
      ...output,
      body: await parseErrorBody(output.body, context)
    };
    const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
    switch (errorCode) {
      case "NotFoundError":
      case "com.minerva#NotFoundError":
        throw await de_NotFoundErrorRes(parsedOutput, context);
      case "ServerError":
      case "com.minerva#ServerError":
        throw await de_ServerErrorRes(parsedOutput, context);
      case "ThrottlingError":
      case "com.minerva#ThrottlingError":
        throw await de_ThrottlingErrorRes(parsedOutput, context);
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
     * deserializeAws_restJson1HealthCheckCommand
     */
    export const de_HealthCheckCommand = async(
      output: __HttpResponse,
      context: __SerdeContext
    ): Promise<HealthCheckCommandOutput> => {
      if (output.statusCode !== 200 && output.statusCode >= 300) {
        return de_HealthCheckCommandError(output, context);
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
     * deserializeAws_restJson1HealthCheckCommandError
     */
    const de_HealthCheckCommandError = async(
      output: __HttpResponse,
      context: __SerdeContext,
    ): Promise<HealthCheckCommandOutput> => {
      const parsedOutput: any = {
        ...output,
        body: await parseErrorBody(output.body, context)
      };
      const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
      switch (errorCode) {
        case "ServerError":
        case "com.minerva#ServerError":
          throw await de_ServerErrorRes(parsedOutput, context);
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
       * deserializeAws_restJson1ListDatasetCommand
       */
      export const de_ListDatasetCommand = async(
        output: __HttpResponse,
        context: __SerdeContext
      ): Promise<ListDatasetCommandOutput> => {
        if (output.statusCode !== 200 && output.statusCode >= 300) {
          return de_ListDatasetCommandError(output, context);
        }
        const contents: any = map({
          $metadata: deserializeMetadata(output),
        });
        const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
        const doc = take(data, {
          'items': _ => de_DatasetList(_, context),
          'nextToken': __expectString,
        });
        Object.assign(contents, doc);
        return contents;
      }

      /**
       * deserializeAws_restJson1ListDatasetCommandError
       */
      const de_ListDatasetCommandError = async(
        output: __HttpResponse,
        context: __SerdeContext,
      ): Promise<ListDatasetCommandOutput> => {
        const parsedOutput: any = {
          ...output,
          body: await parseErrorBody(output.body, context)
        };
        const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
        switch (errorCode) {
          case "ServerError":
          case "com.minerva#ServerError":
            throw await de_ServerErrorRes(parsedOutput, context);
          case "ThrottlingError":
          case "com.minerva#ThrottlingError":
            throw await de_ThrottlingErrorRes(parsedOutput, context);
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
         * deserializeAws_restJson1QueryDatasetCommand
         */
        export const de_QueryDatasetCommand = async(
          output: __HttpResponse,
          context: __SerdeContext
        ): Promise<QueryDatasetCommandOutput> => {
          if (output.statusCode !== 200 && output.statusCode >= 300) {
            return de_QueryDatasetCommandError(output, context);
          }
          const contents: any = map({
            $metadata: deserializeMetadata(output),
          });
          const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
          const doc = take(data, {
            'data': context.base64Decoder,
          });
          Object.assign(contents, doc);
          return contents;
        }

        /**
         * deserializeAws_restJson1QueryDatasetCommandError
         */
        const de_QueryDatasetCommandError = async(
          output: __HttpResponse,
          context: __SerdeContext,
        ): Promise<QueryDatasetCommandOutput> => {
          const parsedOutput: any = {
            ...output,
            body: await parseErrorBody(output.body, context)
          };
          const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
          switch (errorCode) {
            case "ClickhouseQueryError":
            case "com.minerva#ClickhouseQueryError":
              throw await de_ClickhouseQueryErrorRes(parsedOutput, context);
            case "NotFoundError":
            case "com.minerva#NotFoundError":
              throw await de_NotFoundErrorRes(parsedOutput, context);
            case "ServerError":
            case "com.minerva#ServerError":
              throw await de_ServerErrorRes(parsedOutput, context);
            case "ThrottlingError":
            case "com.minerva#ThrottlingError":
              throw await de_ThrottlingErrorRes(parsedOutput, context);
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
           * deserializeAws_restJson1SampleDatasetCommand
           */
          export const de_SampleDatasetCommand = async(
            output: __HttpResponse,
            context: __SerdeContext
          ): Promise<SampleDatasetCommandOutput> => {
            if (output.statusCode !== 200 && output.statusCode >= 300) {
              return de_SampleDatasetCommandError(output, context);
            }
            const contents: any = map({
              $metadata: deserializeMetadata(output),
            });
            const data: Record<string, any> = __expectNonNull((__expectObject(await parseBody(output.body, context))), "body");
            const doc = take(data, {
              'data': context.base64Decoder,
            });
            Object.assign(contents, doc);
            return contents;
          }

          /**
           * deserializeAws_restJson1SampleDatasetCommandError
           */
          const de_SampleDatasetCommandError = async(
            output: __HttpResponse,
            context: __SerdeContext,
          ): Promise<SampleDatasetCommandOutput> => {
            const parsedOutput: any = {
              ...output,
              body: await parseErrorBody(output.body, context)
            };
            const errorCode = loadRestJsonErrorCode(output, parsedOutput.body);
            switch (errorCode) {
              case "NotFoundError":
              case "com.minerva#NotFoundError":
                throw await de_NotFoundErrorRes(parsedOutput, context);
              case "ServerError":
              case "com.minerva#ServerError":
                throw await de_ServerErrorRes(parsedOutput, context);
              case "ThrottlingError":
              case "com.minerva#ThrottlingError":
                throw await de_ThrottlingErrorRes(parsedOutput, context);
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
                case "com.minerva#ForbiddenError":
                  throw await de_ForbiddenErrorRes(parsedOutput, context);
                case "ThrottlingError":
                case "com.minerva#ThrottlingError":
                  throw await de_ThrottlingErrorRes(parsedOutput, context);
                case "UnauthorizedError":
                case "com.minerva#UnauthorizedError":
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
               * deserializeAws_restJson1ClickhouseQueryErrorRes
               */
              const de_ClickhouseQueryErrorRes = async (
                parsedOutput: any,
                context: __SerdeContext
              ): Promise<ClickhouseQueryError> => {
                const contents: any = map({
                });
                const data: any = parsedOutput.body;
                const doc = take(data, {
                  'message': __expectString,
                });
                Object.assign(contents, doc);
                const exception = new ClickhouseQueryError({
                  $metadata: deserializeMetadata(parsedOutput),
                  ...contents
                });
                return __decorateServiceException(exception, parsedOutput.body);
              };

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
               * deserializeAws_restJson1NotFoundErrorRes
               */
              const de_NotFoundErrorRes = async (
                parsedOutput: any,
                context: __SerdeContext
              ): Promise<NotFoundError> => {
                const contents: any = map({
                });
                const data: any = parsedOutput.body;
                const doc = take(data, {
                  'message': __expectString,
                });
                Object.assign(contents, doc);
                const exception = new NotFoundError({
                  $metadata: deserializeMetadata(parsedOutput),
                  ...contents
                });
                return __decorateServiceException(exception, parsedOutput.body);
              };

              /**
               * deserializeAws_restJson1ServerErrorRes
               */
              const de_ServerErrorRes = async (
                parsedOutput: any,
                context: __SerdeContext
              ): Promise<ServerError> => {
                const contents: any = map({
                });
                const data: any = parsedOutput.body;
                const doc = take(data, {
                  'code': __expectString,
                  'message': __expectString,
                });
                Object.assign(contents, doc);
                const exception = new ServerError({
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

              // de_DatasetField omitted.

              // de_DatasetFieldList omitted.

              /**
               * deserializeAws_restJson1DatasetInfo
               */
              const de_DatasetInfo = (
                output: any,
                context: __SerdeContext
              ): DatasetInfo => {
                return take(output, {
                  'fields': _json,
                  'lastModified': (_: any) => __expectNonNull(__parseEpochTimestamp(__expectNumber(_))),
                  'name': __expectString,
                  'size': __expectLong,
                  'tableName': __expectString,
                }) as any;
              }

              /**
               * deserializeAws_restJson1DatasetList
               */
              const de_DatasetList = (
                output: any,
                context: __SerdeContext
              ): (DatasetInfo)[] => {
                const retVal = (output || []).filter((e: any) => e != null).map((entry: any) => {
                  return de_DatasetInfo(entry, context);
                });
                return retVal;
              }

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

              const _l = "limit";
              const _m = "message";
              const _n = "name";
              const _nT = "nextToken";
              const _s = "size";
              const _xdn = "x-dataset-name";
              const _xm = "x-message";

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
