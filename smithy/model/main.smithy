$version: "2.0"

namespace com.minerva

use aws.protocols#restJson1
use smithy.framework#ValidationException

@restJson1
@httpBearerAuth
service DatasetService {
    version: "2023-12-03"
    resources: [Dataset]
    operations: [HealthCheck, Signin]
}

@readonly
@http(uri: "/health", method: "GET")
@auth([])
operation HealthCheck {
    input := {
        @required
        @httpHeader("x-message")
        message: String
    }
    output := {
        @required
        message: String
    }
    errors: [ValidationException, ServerError]
}

/// Signin to get a token.
@http(uri: "/signin", method: "POST")
@auth([])
operation Signin {
    input := {
        @required
        username: String
        @required
        password: String
    }
    output := {
        @required
        token: String
    }
    errors: [ValidationException, UnauthorizedError, ForbiddenError, ThrottlingError]
}
