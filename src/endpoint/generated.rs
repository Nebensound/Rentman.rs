use super::{
    Endpoint, EndpointErrorSpec, EndpointParameterLocation, EndpointParameterSpec,
    EndpointParameterValueSpec, EndpointRequestSpec, EndpointResponseSpec, EndpointSpec,
    EndpointWithId, ExecutableEndpoint,
};
use crate::model::*;
use crate::model::{CollectionResponse, ItemResponse, NoContent, NoRequest};

const GETACCESSORYCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETACCESSORYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /accessories`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAccessoryCollectionEndpoint;

impl Endpoint for GetAccessoryCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AccessoryResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/accessories",
        operation_id: "getAccessoryCollection",
        tag: "accessories",
        parameters: GETACCESSORYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AccessoryResponse"),
        errors: GETACCESSORYCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEACCESSORYENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETEACCESSORYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /accessories/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteAccessoryEndpoint;

impl Endpoint for DeleteAccessoryEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/accessories/{id}",
        operation_id: "deleteAccessory",
        tag: "accessories",
        parameters: DELETEACCESSORYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEACCESSORYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteAccessoryEndpoint {}

const GETACCESSORYITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETACCESSORYITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /accessories/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAccessoryItemEndpoint;

impl Endpoint for GetAccessoryItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<AccessoryResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/accessories/{id}",
        operation_id: "getAccessoryItem",
        tag: "accessories",
        parameters: GETACCESSORYITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("AccessoryResponse"),
        errors: GETACCESSORYITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetAccessoryItemEndpoint {}

const UPDATEACCESSORYENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEACCESSORYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /accessories/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateAccessoryEndpoint;

impl Endpoint for UpdateAccessoryEndpoint {
    type Request = AccessoryRequest;
    type Response = ItemResponse<AccessoryResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/accessories/{id}",
        operation_id: "updateAccessory",
        tag: "accessories",
        parameters: UPDATEACCESSORYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AccessoryRequest"),
        response: EndpointResponseSpec::Item("AccessoryResponse"),
        errors: UPDATEACCESSORYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateAccessoryEndpoint {}

const GETACTUALCONTENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETACTUALCONTENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /actualcontent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetActualContentCollectionEndpoint;

impl Endpoint for GetActualContentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ActualContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/actualcontent",
        operation_id: "getActualContentCollection",
        tag: "actualcontent",
        parameters: GETACTUALCONTENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ActualContentResponse"),
        errors: GETACTUALCONTENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETACTUALCONTENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETACTUALCONTENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /actualcontent/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetActualContentItemEndpoint;

impl Endpoint for GetActualContentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ActualContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/actualcontent/{id}",
        operation_id: "getActualContentItem",
        tag: "actualcontent",
        parameters: GETACTUALCONTENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ActualContentResponse"),
        errors: GETACTUALCONTENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetActualContentItemEndpoint {}

const GETALTERNATIVECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETALTERNATIVECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /alternatives`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAlternativeCollectionEndpoint;

impl Endpoint for GetAlternativeCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AlternativeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/alternatives",
        operation_id: "getAlternativeCollection",
        tag: "alternatives",
        parameters: GETALTERNATIVECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AlternativeResponse"),
        errors: GETALTERNATIVECOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEALTERNATIVEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETEALTERNATIVEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /alternatives/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteAlternativeEndpoint;

impl Endpoint for DeleteAlternativeEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/alternatives/{id}",
        operation_id: "deleteAlternative",
        tag: "alternatives",
        parameters: DELETEALTERNATIVEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEALTERNATIVEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteAlternativeEndpoint {}

const GETALTERNATIVEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETALTERNATIVEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /alternatives/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAlternativeItemEndpoint;

impl Endpoint for GetAlternativeItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<AlternativeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/alternatives/{id}",
        operation_id: "getAlternativeItem",
        tag: "alternatives",
        parameters: GETALTERNATIVEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("AlternativeResponse"),
        errors: GETALTERNATIVEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetAlternativeItemEndpoint {}

const UPDATEALTERNATIVEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEALTERNATIVEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /alternatives/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateAlternativeEndpoint;

impl Endpoint for UpdateAlternativeEndpoint {
    type Request = AlternativeRequest;
    type Response = ItemResponse<AlternativeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/alternatives/{id}",
        operation_id: "updateAlternative",
        tag: "alternatives",
        parameters: UPDATEALTERNATIVEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AlternativeRequest"),
        response: EndpointResponseSpec::Item("AlternativeResponse"),
        errors: UPDATEALTERNATIVEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateAlternativeEndpoint {}

const GETAPPOINTMENTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETAPPOINTMENTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /appointmentcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAppointmentCrewCollectionEndpoint;

impl Endpoint for GetAppointmentCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AppointmentCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/appointmentcrew",
        operation_id: "getAppointmentCrewCollection",
        tag: "appointmentcrew",
        parameters: GETAPPOINTMENTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AppointmentCrewResponse"),
        errors: GETAPPOINTMENTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEAPPOINTMENTCREWENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETEAPPOINTMENTCREWENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /appointmentcrew/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteAppointmentCrewEndpoint;

impl Endpoint for DeleteAppointmentCrewEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/appointmentcrew/{id}",
        operation_id: "deleteAppointmentCrew",
        tag: "appointmentcrew",
        parameters: DELETEAPPOINTMENTCREWENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEAPPOINTMENTCREWENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteAppointmentCrewEndpoint {}

const GETAPPOINTMENTCREWITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETAPPOINTMENTCREWITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /appointmentcrew/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAppointmentCrewItemEndpoint;

impl Endpoint for GetAppointmentCrewItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<AppointmentCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/appointmentcrew/{id}",
        operation_id: "getAppointmentCrewItem",
        tag: "appointmentcrew",
        parameters: GETAPPOINTMENTCREWITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("AppointmentCrewResponse"),
        errors: GETAPPOINTMENTCREWITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetAppointmentCrewItemEndpoint {}

const UPDATEAPPOINTMENTCREWENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATEAPPOINTMENTCREWENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /appointmentcrew/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateAppointmentCrewEndpoint;

impl Endpoint for UpdateAppointmentCrewEndpoint {
    type Request = AppointmentCrewRequest;
    type Response = ItemResponse<AppointmentCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/appointmentcrew/{id}",
        operation_id: "updateAppointmentCrew",
        tag: "appointmentcrew",
        parameters: UPDATEAPPOINTMENTCREWENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AppointmentCrewRequest"),
        response: EndpointResponseSpec::Item("AppointmentCrewResponse"),
        errors: UPDATEAPPOINTMENTCREWENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateAppointmentCrewEndpoint {}

const GETAPPOINTMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETAPPOINTMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /appointments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAppointmentCollectionEndpoint;

impl Endpoint for GetAppointmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AppointmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/appointments",
        operation_id: "getAppointmentCollection",
        tag: "appointments",
        parameters: GETAPPOINTMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AppointmentResponse"),
        errors: GETAPPOINTMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATEAPPOINTMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATEAPPOINTMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /appointments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateAppointmentEndpoint;

impl Endpoint for CreateAppointmentEndpoint {
    type Request = AppointmentRequest;
    type Response = ItemResponse<AppointmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/appointments",
        operation_id: "createAppointment",
        tag: "appointments",
        parameters: CREATEAPPOINTMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AppointmentRequest"),
        response: EndpointResponseSpec::Item("AppointmentResponse"),
        errors: CREATEAPPOINTMENTENDPOINT_ERRORS,
    };
}

const DELETEAPPOINTMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETEAPPOINTMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /appointments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteAppointmentEndpoint;

impl Endpoint for DeleteAppointmentEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/appointments/{id}",
        operation_id: "deleteAppointment",
        tag: "appointments",
        parameters: DELETEAPPOINTMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEAPPOINTMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteAppointmentEndpoint {}

const GETAPPOINTMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETAPPOINTMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /appointments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAppointmentItemEndpoint;

impl Endpoint for GetAppointmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<AppointmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/appointments/{id}",
        operation_id: "getAppointmentItem",
        tag: "appointments",
        parameters: GETAPPOINTMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("AppointmentResponse"),
        errors: GETAPPOINTMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetAppointmentItemEndpoint {}

const UPDATEAPPOINTMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEAPPOINTMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /appointments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateAppointmentEndpoint;

impl Endpoint for UpdateAppointmentEndpoint {
    type Request = AppointmentRequest;
    type Response = ItemResponse<AppointmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/appointments/{id}",
        operation_id: "updateAppointment",
        tag: "appointments",
        parameters: UPDATEAPPOINTMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AppointmentRequest"),
        response: EndpointResponseSpec::Item("AppointmentResponse"),
        errors: UPDATEAPPOINTMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateAppointmentEndpoint {}

const GETAPPOINTMENTAPPOINTMENTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETAPPOINTMENTAPPOINTMENTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /appointments/{id}/appointmentcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetAppointmentAppointmentCrewCollectionEndpoint;

impl Endpoint for GetAppointmentAppointmentCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AppointmentCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/appointments/{id}/appointmentcrew",
        operation_id: "getAppointmentAppointmentCrewCollection",
        tag: "appointments",
        parameters: GETAPPOINTMENTAPPOINTMENTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AppointmentCrewResponse"),
        errors: GETAPPOINTMENTAPPOINTMENTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetAppointmentAppointmentCrewCollectionEndpoint {}

const CREATEAPPOINTMENTCREWENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATEAPPOINTMENTCREWENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /appointments/{id}/appointmentcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateAppointmentCrewEndpoint;

impl Endpoint for CreateAppointmentCrewEndpoint {
    type Request = AppointmentCrewRequest;
    type Response = ItemResponse<AppointmentCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/appointments/{id}/appointmentcrew",
        operation_id: "createAppointmentCrew",
        tag: "appointments",
        parameters: CREATEAPPOINTMENTCREWENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AppointmentCrewRequest"),
        response: EndpointResponseSpec::Item("AppointmentCrewResponse"),
        errors: CREATEAPPOINTMENTCREWENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateAppointmentCrewEndpoint {}

const GETCONTACTPERSONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCONTACTPERSONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contactpersons`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactPersonCollectionEndpoint;

impl Endpoint for GetContactPersonCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ContactPersonResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contactpersons",
        operation_id: "getContactPersonCollection",
        tag: "contactpersons",
        parameters: GETCONTACTPERSONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ContactPersonResponse"),
        errors: GETCONTACTPERSONCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETECONTACTPERSONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETECONTACTPERSONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /contactpersons/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteContactPersonEndpoint;

impl Endpoint for DeleteContactPersonEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/contactpersons/{id}",
        operation_id: "deleteContactPerson",
        tag: "contactpersons",
        parameters: DELETECONTACTPERSONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETECONTACTPERSONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteContactPersonEndpoint {}

const GETCONTACTPERSONITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTPERSONITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contactpersons/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactPersonItemEndpoint;

impl Endpoint for GetContactPersonItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ContactPersonResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contactpersons/{id}",
        operation_id: "getContactPersonItem",
        tag: "contactpersons",
        parameters: GETCONTACTPERSONITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ContactPersonResponse"),
        errors: GETCONTACTPERSONITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactPersonItemEndpoint {}

const UPDATECONTACTPERSONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATECONTACTPERSONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /contactpersons/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateContactPersonEndpoint;

impl Endpoint for UpdateContactPersonEndpoint {
    type Request = ContactPersonRequest;
    type Response = ItemResponse<ContactPersonResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/contactpersons/{id}",
        operation_id: "updateContactPerson",
        tag: "contactpersons",
        parameters: UPDATECONTACTPERSONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ContactPersonRequest"),
        response: EndpointResponseSpec::Item("ContactPersonResponse"),
        errors: UPDATECONTACTPERSONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateContactPersonEndpoint {}

const GETCONTACTPERSONFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTPERSONFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contactpersons/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactPersonFileFolderCollectionEndpoint;

impl Endpoint for GetContactPersonFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contactpersons/{id}/file_folders",
        operation_id: "getContactPersonFileFolderCollection",
        tag: "contactpersons",
        parameters: GETCONTACTPERSONFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETCONTACTPERSONFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactPersonFileFolderCollectionEndpoint {}

const GETCONTACTPERSONFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTPERSONFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contactpersons/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactPersonFileCollectionEndpoint;

impl Endpoint for GetContactPersonFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contactpersons/{id}/files",
        operation_id: "getContactPersonFileCollection",
        tag: "contactpersons",
        parameters: GETCONTACTPERSONFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETCONTACTPERSONFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactPersonFileCollectionEndpoint {}

const GETCONTACTPERSONTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTPERSONTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contactpersons/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactPersonTaskCollectionEndpoint;

impl Endpoint for GetContactPersonTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contactpersons/{id}/tasks",
        operation_id: "getContactPersonTaskCollection",
        tag: "contactpersons",
        parameters: GETCONTACTPERSONTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETCONTACTPERSONTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactPersonTaskCollectionEndpoint {}

const CREATETASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATETASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /contactpersons/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateTaskEndpoint;

impl Endpoint for CreateTaskEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/contactpersons/{id}/tasks",
        operation_id: "createTask",
        tag: "contactpersons",
        parameters: CREATETASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: CREATETASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateTaskEndpoint {}

const GETCONTACTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCONTACTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactCollectionEndpoint;

impl Endpoint for GetContactCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ContactResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts",
        operation_id: "getContactCollection",
        tag: "contacts",
        parameters: GETCONTACTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ContactResponse"),
        errors: GETCONTACTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATECONTACTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATECONTACTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /contacts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContactEndpoint;

impl Endpoint for CreateContactEndpoint {
    type Request = ContactRequest;
    type Response = ItemResponse<ContactResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/contacts",
        operation_id: "createContact",
        tag: "contacts",
        parameters: CREATECONTACTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ContactRequest"),
        response: EndpointResponseSpec::Item("ContactResponse"),
        errors: CREATECONTACTENDPOINT_ERRORS,
    };
}

const DELETECONTACTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETECONTACTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /contacts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteContactEndpoint;

impl Endpoint for DeleteContactEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/contacts/{id}",
        operation_id: "deleteContact",
        tag: "contacts",
        parameters: DELETECONTACTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETECONTACTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteContactEndpoint {}

const GETCONTACTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETCONTACTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactItemEndpoint;

impl Endpoint for GetContactItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ContactResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts/{id}",
        operation_id: "getContactItem",
        tag: "contacts",
        parameters: GETCONTACTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ContactResponse"),
        errors: GETCONTACTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactItemEndpoint {}

const UPDATECONTACTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATECONTACTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /contacts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateContactEndpoint;

impl Endpoint for UpdateContactEndpoint {
    type Request = ContactRequest;
    type Response = ItemResponse<ContactResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/contacts/{id}",
        operation_id: "updateContact",
        tag: "contacts",
        parameters: UPDATECONTACTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ContactRequest"),
        response: EndpointResponseSpec::Item("ContactResponse"),
        errors: UPDATECONTACTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateContactEndpoint {}

const GETCONTACTCONTACTPERSONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTCONTACTPERSONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts/{id}/contactpersons`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactContactPersonCollectionEndpoint;

impl Endpoint for GetContactContactPersonCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ContactPersonResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts/{id}/contactpersons",
        operation_id: "getContactContactPersonCollection",
        tag: "contacts",
        parameters: GETCONTACTCONTACTPERSONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ContactPersonResponse"),
        errors: GETCONTACTCONTACTPERSONCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactContactPersonCollectionEndpoint {}

const CREATECONTACTPERSONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATECONTACTPERSONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /contacts/{id}/contactpersons`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateContactPersonEndpoint;

impl Endpoint for CreateContactPersonEndpoint {
    type Request = ContactPersonRequest;
    type Response = ItemResponse<ContactPersonResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/contacts/{id}/contactpersons",
        operation_id: "createContactPerson",
        tag: "contacts",
        parameters: CREATECONTACTPERSONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ContactPersonRequest"),
        response: EndpointResponseSpec::Item("ContactPersonResponse"),
        errors: CREATECONTACTPERSONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateContactPersonEndpoint {}

const GETCONTACTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactFileFolderCollectionEndpoint;

impl Endpoint for GetContactFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts/{id}/file_folders",
        operation_id: "getContactFileFolderCollection",
        tag: "contacts",
        parameters: GETCONTACTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETCONTACTFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactFileFolderCollectionEndpoint {}

const GETCONTACTFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactFileCollectionEndpoint;

impl Endpoint for GetContactFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts/{id}/files",
        operation_id: "getContactFileCollection",
        tag: "contacts",
        parameters: GETCONTACTFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETCONTACTFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactFileCollectionEndpoint {}

const GETCONTACTTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTACTTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contacts/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContactTaskCollectionEndpoint;

impl Endpoint for GetContactTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contacts/{id}/tasks",
        operation_id: "getContactTaskCollection",
        tag: "contacts",
        parameters: GETCONTACTTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETCONTACTTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContactTaskCollectionEndpoint {}

const POSTCONTACTSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTCONTACTSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /contacts/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostContactsIdTasksEndpoint;

impl Endpoint for PostContactsIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/contacts/{id}/tasks",
        operation_id: "createTask",
        tag: "contacts",
        parameters: POSTCONTACTSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTCONTACTSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostContactsIdTasksEndpoint {}

const GETCONTRACTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCONTRACTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contracts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContractCollectionEndpoint;

impl Endpoint for GetContractCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ContractResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contracts",
        operation_id: "getContractCollection",
        tag: "contracts",
        parameters: GETCONTRACTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ContractResponse"),
        errors: GETCONTRACTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETCONTRACTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETCONTRACTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contracts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContractItemEndpoint;

impl Endpoint for GetContractItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ContractResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contracts/{id}",
        operation_id: "getContractItem",
        tag: "contracts",
        parameters: GETCONTRACTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ContractResponse"),
        errors: GETCONTRACTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContractItemEndpoint {}

const GETCONTRACTFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTRACTFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contracts/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContractFileCollectionEndpoint;

impl Endpoint for GetContractFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contracts/{id}/files",
        operation_id: "getContractFileCollection",
        tag: "contracts",
        parameters: GETCONTRACTFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETCONTRACTFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContractFileCollectionEndpoint {}

const GETCONTRACTINVOICELINECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTRACTINVOICELINECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contracts/{id}/invoicelines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContractInvoiceLineCollectionEndpoint;

impl Endpoint for GetContractInvoiceLineCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contracts/{id}/invoicelines",
        operation_id: "getContractInvoiceLineCollection",
        tag: "contracts",
        parameters: GETCONTRACTINVOICELINECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvoiceLineResponse"),
        errors: GETCONTRACTINVOICELINECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContractInvoiceLineCollectionEndpoint {}

const GETCONTRACTTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCONTRACTTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /contracts/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetContractTaskCollectionEndpoint;

impl Endpoint for GetContractTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/contracts/{id}/tasks",
        operation_id: "getContractTaskCollection",
        tag: "contracts",
        parameters: GETCONTRACTTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETCONTRACTTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetContractTaskCollectionEndpoint {}

const POSTCONTRACTSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTCONTRACTSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /contracts/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostContractsIdTasksEndpoint;

impl Endpoint for PostContractsIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/contracts/{id}/tasks",
        operation_id: "createTask",
        tag: "contracts",
        parameters: POSTCONTRACTSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTCONTRACTSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostContractsIdTasksEndpoint {}

const GETPROJECTCOSTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /costs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectCostCollectionEndpoint;

impl Endpoint for GetProjectCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/costs",
        operation_id: "getProjectCostCollection",
        tag: "costs",
        parameters: GETPROJECTCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCostResponse"),
        errors: GETPROJECTCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEPROJECTCOSTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETEPROJECTCOSTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /costs/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteProjectCostEndpoint;

impl Endpoint for DeleteProjectCostEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/costs/{id}",
        operation_id: "deleteProjectCost",
        tag: "costs",
        parameters: DELETEPROJECTCOSTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEPROJECTCOSTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteProjectCostEndpoint {}

const GETPROJECTCOSTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTCOSTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /costs/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectCostItemEndpoint;

impl Endpoint for GetProjectCostItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/costs/{id}",
        operation_id: "getProjectCostItem",
        tag: "costs",
        parameters: GETPROJECTCOSTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectCostResponse"),
        errors: GETPROJECTCOSTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectCostItemEndpoint {}

const UPDATEPROJECTCOSTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEPROJECTCOSTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /costs/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateProjectCostEndpoint;

impl Endpoint for UpdateProjectCostEndpoint {
    type Request = ProjectCostRequest;
    type Response = ItemResponse<ProjectCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/costs/{id}",
        operation_id: "updateProjectCost",
        tag: "costs",
        parameters: UPDATEPROJECTCOSTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectCostRequest"),
        response: EndpointResponseSpec::Item("ProjectCostResponse"),
        errors: UPDATEPROJECTCOSTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateProjectCostEndpoint {}

const GETCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewCollectionEndpoint;

impl Endpoint for GetCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew",
        operation_id: "getCrewCollection",
        tag: "crew",
        parameters: GETCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewResponse"),
        errors: GETCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETCREWITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETCREWITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewItemEndpoint;

impl Endpoint for GetCrewItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<CrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}",
        operation_id: "getCrewItem",
        tag: "crew",
        parameters: GETCREWITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("CrewResponse"),
        errors: GETCREWITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewItemEndpoint {}

const GETCREWAPPOINTMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWAPPOINTMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/appointments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewAppointmentCollectionEndpoint;

impl Endpoint for GetCrewAppointmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AppointmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/appointments",
        operation_id: "getCrewAppointmentCollection",
        tag: "crew",
        parameters: GETCREWAPPOINTMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AppointmentResponse"),
        errors: GETCREWAPPOINTMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewAppointmentCollectionEndpoint {}

const GETCREWCREWAVAILABILITYCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWCREWAVAILABILITYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/crewavailability`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewCrewAvailabilityCollectionEndpoint;

impl Endpoint for GetCrewCrewAvailabilityCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewAvailabilityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/crewavailability",
        operation_id: "getCrewCrewAvailabilityCollection",
        tag: "crew",
        parameters: GETCREWCREWAVAILABILITYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewAvailabilityResponse"),
        errors: GETCREWCREWAVAILABILITYCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewCrewAvailabilityCollectionEndpoint {}

const CREATECREWAVAILABILITYENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATECREWAVAILABILITYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /crew/{id}/crewavailability`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateCrewAvailabilityEndpoint;

impl Endpoint for CreateCrewAvailabilityEndpoint {
    type Request = CrewAvailabilityRequest;
    type Response = ItemResponse<CrewAvailabilityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/crew/{id}/crewavailability",
        operation_id: "createCrewAvailability",
        tag: "crew",
        parameters: CREATECREWAVAILABILITYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("CrewAvailabilityRequest"),
        response: EndpointResponseSpec::Item("CrewAvailabilityResponse"),
        errors: CREATECREWAVAILABILITYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateCrewAvailabilityEndpoint {}

const GETCREWCREWRATESCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWCREWRATESCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/crewrates`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewCrewRatesCollectionEndpoint;

impl Endpoint for GetCrewCrewRatesCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewRatesResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/crewrates",
        operation_id: "getCrewCrewRatesCollection",
        tag: "crew",
        parameters: GETCREWCREWRATESCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewRatesResponse"),
        errors: GETCREWCREWRATESCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewCrewRatesCollectionEndpoint {}

const GETCREWFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewFileFolderCollectionEndpoint;

impl Endpoint for GetCrewFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/file_folders",
        operation_id: "getCrewFileFolderCollection",
        tag: "crew",
        parameters: GETCREWFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETCREWFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewFileFolderCollectionEndpoint {}

const GETCREWFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewFileCollectionEndpoint;

impl Endpoint for GetCrewFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/files",
        operation_id: "getCrewFileCollection",
        tag: "crew",
        parameters: GETCREWFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETCREWFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewFileCollectionEndpoint {}

const GETCREWINVITATIONSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWINVITATIONSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/invitations`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewInvitationsCollectionEndpoint;

impl Endpoint for GetCrewInvitationsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvitationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/invitations",
        operation_id: "getCrewInvitationsCollection",
        tag: "crew",
        parameters: GETCREWINVITATIONSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvitationsResponse"),
        errors: GETCREWINVITATIONSCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewInvitationsCollectionEndpoint {}

const GETCREWTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crew/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewTaskCollectionEndpoint;

impl Endpoint for GetCrewTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crew/{id}/tasks",
        operation_id: "getCrewTaskCollection",
        tag: "crew",
        parameters: GETCREWTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETCREWTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewTaskCollectionEndpoint {}

const POSTCREWIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTCREWIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /crew/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostCrewIdTasksEndpoint;

impl Endpoint for PostCrewIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/crew/{id}/tasks",
        operation_id: "createTask",
        tag: "crew",
        parameters: POSTCREWIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTCREWIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostCrewIdTasksEndpoint {}

const GETCREWAVAILABILITYCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCREWAVAILABILITYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crewavailability`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewAvailabilityCollectionEndpoint;

impl Endpoint for GetCrewAvailabilityCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewAvailabilityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crewavailability",
        operation_id: "getCrewAvailabilityCollection",
        tag: "crewavailability",
        parameters: GETCREWAVAILABILITYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewAvailabilityResponse"),
        errors: GETCREWAVAILABILITYCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETECREWAVAILABILITYENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETECREWAVAILABILITYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /crewavailability/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteCrewAvailabilityEndpoint;

impl Endpoint for DeleteCrewAvailabilityEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/crewavailability/{id}",
        operation_id: "deleteCrewAvailability",
        tag: "crewavailability",
        parameters: DELETECREWAVAILABILITYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETECREWAVAILABILITYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteCrewAvailabilityEndpoint {}

const GETCREWAVAILABILITYITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWAVAILABILITYITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crewavailability/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewAvailabilityItemEndpoint;

impl Endpoint for GetCrewAvailabilityItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<CrewAvailabilityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crewavailability/{id}",
        operation_id: "getCrewAvailabilityItem",
        tag: "crewavailability",
        parameters: GETCREWAVAILABILITYITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("CrewAvailabilityResponse"),
        errors: GETCREWAVAILABILITYITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewAvailabilityItemEndpoint {}

const UPDATECREWAVAILABILITYENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATECREWAVAILABILITYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /crewavailability/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateCrewAvailabilityEndpoint;

impl Endpoint for UpdateCrewAvailabilityEndpoint {
    type Request = CrewAvailabilityRequest;
    type Response = ItemResponse<CrewAvailabilityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/crewavailability/{id}",
        operation_id: "updateCrewAvailability",
        tag: "crewavailability",
        parameters: UPDATECREWAVAILABILITYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("CrewAvailabilityRequest"),
        response: EndpointResponseSpec::Item("CrewAvailabilityResponse"),
        errors: UPDATECREWAVAILABILITYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateCrewAvailabilityEndpoint {}

const GETCREWRATESCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCREWRATESCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crewrates`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRatesCollectionEndpoint;

impl Endpoint for GetCrewRatesCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewRatesResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crewrates",
        operation_id: "getCrewRatesCollection",
        tag: "crewrates",
        parameters: GETCREWRATESCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewRatesResponse"),
        errors: GETCREWRATESCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETCREWRATESITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETCREWRATESITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /crewrates/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRatesItemEndpoint;

impl Endpoint for GetCrewRatesItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<CrewRatesResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/crewrates/{id}",
        operation_id: "getCrewRatesItem",
        tag: "crewrates",
        parameters: GETCREWRATESITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("CrewRatesResponse"),
        errors: GETCREWRATESITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewRatesItemEndpoint {}

const GETEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentCollectionEndpoint;

impl Endpoint for GetEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<EquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment",
        operation_id: "getEquipmentCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("EquipmentResponse"),
        errors: GETEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATEEQUIPMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATEEQUIPMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateEquipmentEndpoint;

impl Endpoint for CreateEquipmentEndpoint {
    type Request = EquipmentRequest;
    type Response = ItemResponse<EquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment",
        operation_id: "createEquipment",
        tag: "equipment",
        parameters: CREATEEQUIPMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("EquipmentRequest"),
        response: EndpointResponseSpec::Item("EquipmentResponse"),
        errors: CREATEEQUIPMENTENDPOINT_ERRORS,
    };
}

const GETEQUIPMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETEQUIPMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentItemEndpoint;

impl Endpoint for GetEquipmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<EquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}",
        operation_id: "getEquipmentItem",
        tag: "equipment",
        parameters: GETEQUIPMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("EquipmentResponse"),
        errors: GETEQUIPMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentItemEndpoint {}

const UPDATEEQUIPMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEEQUIPMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /equipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateEquipmentEndpoint;

impl Endpoint for UpdateEquipmentEndpoint {
    type Request = EquipmentRequest;
    type Response = ItemResponse<EquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/equipment/{id}",
        operation_id: "updateEquipment",
        tag: "equipment",
        parameters: UPDATEEQUIPMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("EquipmentRequest"),
        response: EndpointResponseSpec::Item("EquipmentResponse"),
        errors: UPDATEEQUIPMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateEquipmentEndpoint {}

const GETEQUIPMENTACCESSORYCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTACCESSORYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/accessories`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentAccessoryCollectionEndpoint;

impl Endpoint for GetEquipmentAccessoryCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AccessoryResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/accessories",
        operation_id: "getEquipmentAccessoryCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTACCESSORYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AccessoryResponse"),
        errors: GETEQUIPMENTACCESSORYCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentAccessoryCollectionEndpoint {}

const CREATEACCESSORYENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATEACCESSORYENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/accessories`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateAccessoryEndpoint;

impl Endpoint for CreateAccessoryEndpoint {
    type Request = AccessoryRequest;
    type Response = ItemResponse<AccessoryResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/accessories",
        operation_id: "createAccessory",
        tag: "equipment",
        parameters: CREATEACCESSORYENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AccessoryRequest"),
        response: EndpointResponseSpec::Item("AccessoryResponse"),
        errors: CREATEACCESSORYENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateAccessoryEndpoint {}

const GETEQUIPMENTALTERNATIVECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTALTERNATIVECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/alternatives`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentAlternativeCollectionEndpoint;

impl Endpoint for GetEquipmentAlternativeCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<AlternativeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/alternatives",
        operation_id: "getEquipmentAlternativeCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTALTERNATIVECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("AlternativeResponse"),
        errors: GETEQUIPMENTALTERNATIVECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentAlternativeCollectionEndpoint {}

const CREATEALTERNATIVEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATEALTERNATIVEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/alternatives`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateAlternativeEndpoint;

impl Endpoint for CreateAlternativeEndpoint {
    type Request = AlternativeRequest;
    type Response = ItemResponse<AlternativeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/alternatives",
        operation_id: "createAlternative",
        tag: "equipment",
        parameters: CREATEALTERNATIVEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("AlternativeRequest"),
        response: EndpointResponseSpec::Item("AlternativeResponse"),
        errors: CREATEALTERNATIVEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateAlternativeEndpoint {}

const GETEQUIPMENTEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/equipmentsetscontent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentEquipmentSetContentCollectionEndpoint;

impl Endpoint for GetEquipmentEquipmentSetContentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<EquipmentSetContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/equipmentsetscontent",
        operation_id: "getEquipmentEquipmentSetContentCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("EquipmentSetContentResponse"),
        errors: GETEQUIPMENTEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentEquipmentSetContentCollectionEndpoint {}

const CREATEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATEEQUIPMENTSETCONTENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/equipmentsetscontent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateEquipmentSetContentEndpoint;

impl Endpoint for CreateEquipmentSetContentEndpoint {
    type Request = EquipmentSetContentRequest;
    type Response = ItemResponse<EquipmentSetContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/equipmentsetscontent",
        operation_id: "createEquipmentSetContent",
        tag: "equipment",
        parameters: CREATEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("EquipmentSetContentRequest"),
        response: EndpointResponseSpec::Item("EquipmentSetContentResponse"),
        errors: CREATEEQUIPMENTSETCONTENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateEquipmentSetContentEndpoint {}

const GETEQUIPMENTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentFileFolderCollectionEndpoint;

impl Endpoint for GetEquipmentFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/file_folders",
        operation_id: "getEquipmentFileFolderCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETEQUIPMENTFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentFileFolderCollectionEndpoint {}

const GETEQUIPMENTFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentFileCollectionEndpoint;

impl Endpoint for GetEquipmentFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/files",
        operation_id: "getEquipmentFileCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETEQUIPMENTFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentFileCollectionEndpoint {}

const GETEQUIPMENTREPAIRCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTREPAIRCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/repairs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentRepairCollectionEndpoint;

impl Endpoint for GetEquipmentRepairCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<RepairResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/repairs",
        operation_id: "getEquipmentRepairCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTREPAIRCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("RepairResponse"),
        errors: GETEQUIPMENTREPAIRCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentRepairCollectionEndpoint {}

const GETEQUIPMENTSERIALNUMBERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTSERIALNUMBERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/serialnumbers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentSerialNumberCollectionEndpoint;

impl Endpoint for GetEquipmentSerialNumberCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SerialNumberResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/serialnumbers",
        operation_id: "getEquipmentSerialNumberCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTSERIALNUMBERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SerialNumberResponse"),
        errors: GETEQUIPMENTSERIALNUMBERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentSerialNumberCollectionEndpoint {}

const CREATESERIALNUMBERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATESERIALNUMBERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/serialnumbers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSerialNumberEndpoint;

impl Endpoint for CreateSerialNumberEndpoint {
    type Request = SerialNumberRequest;
    type Response = ItemResponse<SerialNumberResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/serialnumbers",
        operation_id: "createSerialNumber",
        tag: "equipment",
        parameters: CREATESERIALNUMBERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SerialNumberRequest"),
        response: EndpointResponseSpec::Item("SerialNumberResponse"),
        errors: CREATESERIALNUMBERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateSerialNumberEndpoint {}

const GETEQUIPMENTSTOCKMOVEMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTSTOCKMOVEMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/stockmovements`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentStockMovementCollectionEndpoint;

impl Endpoint for GetEquipmentStockMovementCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<StockMovementResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/stockmovements",
        operation_id: "getEquipmentStockMovementCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTSTOCKMOVEMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("StockMovementResponse"),
        errors: GETEQUIPMENTSTOCKMOVEMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentStockMovementCollectionEndpoint {}

const CREATESTOCKMOVEMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATESTOCKMOVEMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/stockmovements`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateStockMovementEndpoint;

impl Endpoint for CreateStockMovementEndpoint {
    type Request = StockMovementRequest;
    type Response = ItemResponse<StockMovementResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/stockmovements",
        operation_id: "createStockMovement",
        tag: "equipment",
        parameters: CREATESTOCKMOVEMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("StockMovementRequest"),
        response: EndpointResponseSpec::Item("StockMovementResponse"),
        errors: CREATESTOCKMOVEMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateStockMovementEndpoint {}

const GETEQUIPMENTSUPPLIERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTSUPPLIERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/suppliers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentSupplierCollectionEndpoint;

impl Endpoint for GetEquipmentSupplierCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SupplierResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/suppliers",
        operation_id: "getEquipmentSupplierCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTSUPPLIERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SupplierResponse"),
        errors: GETEQUIPMENTSUPPLIERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentSupplierCollectionEndpoint {}

const CREATESUPPLIERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATESUPPLIERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/suppliers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSupplierEndpoint;

impl Endpoint for CreateSupplierEndpoint {
    type Request = SupplierRequest;
    type Response = ItemResponse<SupplierResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/suppliers",
        operation_id: "createSupplier",
        tag: "equipment",
        parameters: CREATESUPPLIERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SupplierRequest"),
        response: EndpointResponseSpec::Item("SupplierResponse"),
        errors: CREATESUPPLIERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateSupplierEndpoint {}

const GETEQUIPMENTTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipment/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentTaskCollectionEndpoint;

impl Endpoint for GetEquipmentTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipment/{id}/tasks",
        operation_id: "getEquipmentTaskCollection",
        tag: "equipment",
        parameters: GETEQUIPMENTTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETEQUIPMENTTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentTaskCollectionEndpoint {}

const POSTEQUIPMENTIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTEQUIPMENTIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /equipment/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostEquipmentIdTasksEndpoint;

impl Endpoint for PostEquipmentIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/equipment/{id}/tasks",
        operation_id: "createTask",
        tag: "equipment",
        parameters: POSTEQUIPMENTIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTEQUIPMENTIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostEquipmentIdTasksEndpoint {}

const GETEQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETEQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipmentassignedserials`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentAssignedSerialsCollectionEndpoint;

impl Endpoint for GetEquipmentAssignedSerialsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<EquipmentAssignedSerialsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipmentassignedserials",
        operation_id: "getEquipmentAssignedSerialsCollection",
        tag: "equipmentassignedserials",
        parameters: GETEQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("EquipmentAssignedSerialsResponse"),
        errors: GETEQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETEQUIPMENTASSIGNEDSERIALSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTASSIGNEDSERIALSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipmentassignedserials/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentAssignedSerialsItemEndpoint;

impl Endpoint for GetEquipmentAssignedSerialsItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<EquipmentAssignedSerialsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipmentassignedserials/{id}",
        operation_id: "getEquipmentAssignedSerialsItem",
        tag: "equipmentassignedserials",
        parameters: GETEQUIPMENTASSIGNEDSERIALSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("EquipmentAssignedSerialsResponse"),
        errors: GETEQUIPMENTASSIGNEDSERIALSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentAssignedSerialsItemEndpoint {}

const GETEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipmentsetscontent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentSetContentCollectionEndpoint;

impl Endpoint for GetEquipmentSetContentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<EquipmentSetContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipmentsetscontent",
        operation_id: "getEquipmentSetContentCollection",
        tag: "equipmentsetscontent",
        parameters: GETEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("EquipmentSetContentResponse"),
        errors: GETEQUIPMENTSETCONTENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETEEQUIPMENTSETCONTENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /equipmentsetscontent/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteEquipmentSetContentEndpoint;

impl Endpoint for DeleteEquipmentSetContentEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/equipmentsetscontent/{id}",
        operation_id: "deleteEquipmentSetContent",
        tag: "equipmentsetscontent",
        parameters: DELETEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEEQUIPMENTSETCONTENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteEquipmentSetContentEndpoint {}

const GETEQUIPMENTSETCONTENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEQUIPMENTSETCONTENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /equipmentsetscontent/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetEquipmentSetContentItemEndpoint;

impl Endpoint for GetEquipmentSetContentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<EquipmentSetContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/equipmentsetscontent/{id}",
        operation_id: "getEquipmentSetContentItem",
        tag: "equipmentsetscontent",
        parameters: GETEQUIPMENTSETCONTENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("EquipmentSetContentResponse"),
        errors: GETEQUIPMENTSETCONTENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetEquipmentSetContentItemEndpoint {}

const UPDATEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATEEQUIPMENTSETCONTENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /equipmentsetscontent/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateEquipmentSetContentEndpoint;

impl Endpoint for UpdateEquipmentSetContentEndpoint {
    type Request = EquipmentSetContentRequest;
    type Response = ItemResponse<EquipmentSetContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/equipmentsetscontent/{id}",
        operation_id: "updateEquipmentSetContent",
        tag: "equipmentsetscontent",
        parameters: UPDATEEQUIPMENTSETCONTENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("EquipmentSetContentRequest"),
        response: EndpointResponseSpec::Item("EquipmentSetContentResponse"),
        errors: UPDATEEQUIPMENTSETCONTENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateEquipmentSetContentEndpoint {}

const GETEXTRAINPUTFIELDCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETEXTRAINPUTFIELDCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /extrainputfields`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetExtraInputFieldCollectionEndpoint;

impl Endpoint for GetExtraInputFieldCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ExtraInputFieldResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/extrainputfields",
        operation_id: "getExtraInputFieldCollection",
        tag: "extrainputfields",
        parameters: GETEXTRAINPUTFIELDCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ExtraInputFieldResponse"),
        errors: GETEXTRAINPUTFIELDCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETEXTRAINPUTFIELDITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETEXTRAINPUTFIELDITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /extrainputfields/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetExtraInputFieldItemEndpoint;

impl Endpoint for GetExtraInputFieldItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ExtraInputFieldResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/extrainputfields/{id}",
        operation_id: "getExtraInputFieldItem",
        tag: "extrainputfields",
        parameters: GETEXTRAINPUTFIELDITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ExtraInputFieldResponse"),
        errors: GETEXTRAINPUTFIELDITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetExtraInputFieldItemEndpoint {}

const GETFACTORGROUPSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFACTORGROUPSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /factorgroups`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactorGroupsCollectionEndpoint;

impl Endpoint for GetFactorGroupsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FactorGroupsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/factorgroups",
        operation_id: "getFactorGroupsCollection",
        tag: "factorgroups",
        parameters: GETFACTORGROUPSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FactorGroupsResponse"),
        errors: GETFACTORGROUPSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETFACTORGROUPSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFACTORGROUPSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /factorgroups/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactorGroupsItemEndpoint;

impl Endpoint for GetFactorGroupsItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FactorGroupsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/factorgroups/{id}",
        operation_id: "getFactorGroupsItem",
        tag: "factorgroups",
        parameters: GETFACTORGROUPSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FactorGroupsResponse"),
        errors: GETFACTORGROUPSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactorGroupsItemEndpoint {}

const GETFACTORGROUPSFACTORSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETFACTORGROUPSFACTORSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /factorgroups/{id}/factors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactorGroupsFactorsCollectionEndpoint;

impl Endpoint for GetFactorGroupsFactorsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FactorsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/factorgroups/{id}/factors",
        operation_id: "getFactorGroupsFactorsCollection",
        tag: "factorgroups",
        parameters: GETFACTORGROUPSFACTORSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FactorsResponse"),
        errors: GETFACTORGROUPSFACTORSCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactorGroupsFactorsCollectionEndpoint {}

const GETFACTORSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFACTORSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /factors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactorsCollectionEndpoint;

impl Endpoint for GetFactorsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FactorsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/factors",
        operation_id: "getFactorsCollection",
        tag: "factors",
        parameters: GETFACTORSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FactorsResponse"),
        errors: GETFACTORSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETFACTORSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFACTORSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /factors/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactorsItemEndpoint;

impl Endpoint for GetFactorsItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FactorsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/factors/{id}",
        operation_id: "getFactorsItem",
        tag: "factors",
        parameters: GETFACTORSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FactorsResponse"),
        errors: GETFACTORSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactorsItemEndpoint {}

const GETFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFileFolderCollectionEndpoint;

impl Endpoint for GetFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/file_folders",
        operation_id: "getFileFolderCollection",
        tag: "file_folders",
        parameters: GETFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETFILEFOLDERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFILEFOLDERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /file_folders/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFileFolderItemEndpoint;

impl Endpoint for GetFileFolderItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/file_folders/{id}",
        operation_id: "getFileFolderItem",
        tag: "file_folders",
        parameters: GETFILEFOLDERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FileFolderResponse"),
        errors: GETFILEFOLDERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFileFolderItemEndpoint {}

const GETFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFileCollectionEndpoint;

impl Endpoint for GetFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/files",
        operation_id: "getFileCollection",
        tag: "files",
        parameters: GETFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETFILECOLLECTIONENDPOINT_ERRORS,
    };
}

const GETFILEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFILEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /files/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFileItemEndpoint;

impl Endpoint for GetFileItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/files/{id}",
        operation_id: "getFileItem",
        tag: "files",
        parameters: GETFILEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FileResponse"),
        errors: GETFILEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFileItemEndpoint {}

const GETFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFolderCollectionEndpoint;

impl Endpoint for GetFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/folders",
        operation_id: "getFolderCollection",
        tag: "folders",
        parameters: GETFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FolderResponse"),
        errors: GETFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATEFOLDERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATEFOLDERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateFolderEndpoint;

impl Endpoint for CreateFolderEndpoint {
    type Request = FolderRequest;
    type Response = ItemResponse<FolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/folders",
        operation_id: "createFolder",
        tag: "folders",
        parameters: CREATEFOLDERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("FolderRequest"),
        response: EndpointResponseSpec::Item("FolderResponse"),
        errors: CREATEFOLDERENDPOINT_ERRORS,
    };
}

const GETFOLDERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFOLDERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /folders/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFolderItemEndpoint;

impl Endpoint for GetFolderItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/folders/{id}",
        operation_id: "getFolderItem",
        tag: "folders",
        parameters: GETFOLDERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FolderResponse"),
        errors: GETFOLDERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFolderItemEndpoint {}

const UPDATEFOLDERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEFOLDERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /folders/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFolderEndpoint;

impl Endpoint for UpdateFolderEndpoint {
    type Request = FolderRequest;
    type Response = ItemResponse<FolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/folders/{id}",
        operation_id: "updateFolder",
        tag: "folders",
        parameters: UPDATEFOLDERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("FolderRequest"),
        response: EndpointResponseSpec::Item("FolderResponse"),
        errors: UPDATEFOLDERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateFolderEndpoint {}

const GETINVITATIONSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETINVITATIONSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invitations`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInvitationsCollectionEndpoint;

impl Endpoint for GetInvitationsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvitationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invitations",
        operation_id: "getInvitationsCollection",
        tag: "invitations",
        parameters: GETINVITATIONSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvitationsResponse"),
        errors: GETINVITATIONSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETINVITATIONSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETINVITATIONSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invitations/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInvitationsItemEndpoint;

impl Endpoint for GetInvitationsItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<InvitationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invitations/{id}",
        operation_id: "getInvitationsItem",
        tag: "invitations",
        parameters: GETINVITATIONSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("InvitationsResponse"),
        errors: GETINVITATIONSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetInvitationsItemEndpoint {}

const GETINVOICELINECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETINVOICELINECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoicelines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInvoiceLineCollectionEndpoint;

impl Endpoint for GetInvoiceLineCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoicelines",
        operation_id: "getInvoiceLineCollection",
        tag: "invoicelines",
        parameters: GETINVOICELINECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvoiceLineResponse"),
        errors: GETINVOICELINECOLLECTIONENDPOINT_ERRORS,
    };
}

const GETINVOICELINEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETINVOICELINEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoicelines/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInvoiceLineItemEndpoint;

impl Endpoint for GetInvoiceLineItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoicelines/{id}",
        operation_id: "getInvoiceLineItem",
        tag: "invoicelines",
        parameters: GETINVOICELINEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("InvoiceLineResponse"),
        errors: GETINVOICELINEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetInvoiceLineItemEndpoint {}

const GETFACTUURCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETFACTUURCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurCollectionEndpoint;

impl Endpoint for GetFactuurCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FactuurResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices",
        operation_id: "getFactuurCollection",
        tag: "invoices",
        parameters: GETFACTUURCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FactuurResponse"),
        errors: GETFACTUURCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETFACTUURITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETFACTUURITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurItemEndpoint;

impl Endpoint for GetFactuurItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<FactuurResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices/{id}",
        operation_id: "getFactuurItem",
        tag: "invoices",
        parameters: GETFACTUURITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("FactuurResponse"),
        errors: GETFACTUURITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactuurItemEndpoint {}

const GETFACTUURFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETFACTUURFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurFileCollectionEndpoint;

impl Endpoint for GetFactuurFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices/{id}/files",
        operation_id: "getFactuurFileCollection",
        tag: "invoices",
        parameters: GETFACTUURFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETFACTUURFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactuurFileCollectionEndpoint {}

const GETFACTUURINVOICELINECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETFACTUURINVOICELINECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices/{id}/invoicelines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurInvoiceLineCollectionEndpoint;

impl Endpoint for GetFactuurInvoiceLineCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices/{id}/invoicelines",
        operation_id: "getFactuurInvoiceLineCollection",
        tag: "invoices",
        parameters: GETFACTUURINVOICELINECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvoiceLineResponse"),
        errors: GETFACTUURINVOICELINECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactuurInvoiceLineCollectionEndpoint {}

const GETFACTUURPAYMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETFACTUURPAYMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices/{id}/payments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurPaymentCollectionEndpoint;

impl Endpoint for GetFactuurPaymentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PaymentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices/{id}/payments",
        operation_id: "getFactuurPaymentCollection",
        tag: "invoices",
        parameters: GETFACTUURPAYMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PaymentResponse"),
        errors: GETFACTUURPAYMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactuurPaymentCollectionEndpoint {}

const CREATEPAYMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATEPAYMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /invoices/{id}/payments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatePaymentEndpoint;

impl Endpoint for CreatePaymentEndpoint {
    type Request = PaymentRequest;
    type Response = ItemResponse<PaymentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/invoices/{id}/payments",
        operation_id: "createPayment",
        tag: "invoices",
        parameters: CREATEPAYMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("PaymentRequest"),
        response: EndpointResponseSpec::Item("PaymentResponse"),
        errors: CREATEPAYMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreatePaymentEndpoint {}

const GETFACTUURTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETFACTUURTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /invoices/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetFactuurTaskCollectionEndpoint;

impl Endpoint for GetFactuurTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/invoices/{id}/tasks",
        operation_id: "getFactuurTaskCollection",
        tag: "invoices",
        parameters: GETFACTUURTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETFACTUURTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetFactuurTaskCollectionEndpoint {}

const POSTINVOICESIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTINVOICESIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /invoices/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostInvoicesIdTasksEndpoint;

impl Endpoint for PostInvoicesIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/invoices/{id}/tasks",
        operation_id: "createTask",
        tag: "invoices",
        parameters: POSTINVOICESIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTINVOICESIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostInvoicesIdTasksEndpoint {}

const GETLEAVEMUTATIONSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETLEAVEMUTATIONSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leavemutation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveMutationsCollectionEndpoint;

impl Endpoint for GetLeaveMutationsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<LeaveMutationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leavemutation",
        operation_id: "getLeaveMutationsCollection",
        tag: "leavemutation",
        parameters: GETLEAVEMUTATIONSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("LeaveMutationsResponse"),
        errors: GETLEAVEMUTATIONSCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATELEAVEMUTATIONSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATELEAVEMUTATIONSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /leavemutation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateLeaveMutationsEndpoint;

impl Endpoint for CreateLeaveMutationsEndpoint {
    type Request = LeaveMutationsRequest;
    type Response = ItemResponse<LeaveMutationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/leavemutation",
        operation_id: "createLeaveMutations",
        tag: "leavemutation",
        parameters: CREATELEAVEMUTATIONSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("LeaveMutationsRequest"),
        response: EndpointResponseSpec::Item("LeaveMutationsResponse"),
        errors: CREATELEAVEMUTATIONSENDPOINT_ERRORS,
    };
}

const GETLEAVEMUTATIONSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETLEAVEMUTATIONSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leavemutation/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveMutationsItemEndpoint;

impl Endpoint for GetLeaveMutationsItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<LeaveMutationsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leavemutation/{id}",
        operation_id: "getLeaveMutationsItem",
        tag: "leavemutation",
        parameters: GETLEAVEMUTATIONSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("LeaveMutationsResponse"),
        errors: GETLEAVEMUTATIONSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetLeaveMutationsItemEndpoint {}

const GETLEAVEREQUESTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETLEAVEREQUESTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leaverequest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveRequestCollectionEndpoint;

impl Endpoint for GetLeaveRequestCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<LeaveRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leaverequest",
        operation_id: "getLeaveRequestCollection",
        tag: "leaverequest",
        parameters: GETLEAVEREQUESTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("LeaveRequestResponse"),
        errors: GETLEAVEREQUESTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATELEAVEREQUESTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATELEAVEREQUESTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /leaverequest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateLeaveRequestEndpoint;

impl Endpoint for CreateLeaveRequestEndpoint {
    type Request = LeaveRequestRequest;
    type Response = ItemResponse<LeaveRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/leaverequest",
        operation_id: "createLeaveRequest",
        tag: "leaverequest",
        parameters: CREATELEAVEREQUESTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("LeaveRequestRequest"),
        response: EndpointResponseSpec::Item("LeaveRequestResponse"),
        errors: CREATELEAVEREQUESTENDPOINT_ERRORS,
    };
}

const GETLEAVEREQUESTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETLEAVEREQUESTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leaverequest/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveRequestItemEndpoint;

impl Endpoint for GetLeaveRequestItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<LeaveRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leaverequest/{id}",
        operation_id: "getLeaveRequestItem",
        tag: "leaverequest",
        parameters: GETLEAVEREQUESTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("LeaveRequestResponse"),
        errors: GETLEAVEREQUESTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetLeaveRequestItemEndpoint {}

const UPDATELEAVEREQUESTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATELEAVEREQUESTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /leaverequest/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateLeaveRequestEndpoint;

impl Endpoint for UpdateLeaveRequestEndpoint {
    type Request = LeaveRequestRequest;
    type Response = ItemResponse<LeaveRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/leaverequest/{id}",
        operation_id: "updateLeaveRequest",
        tag: "leaverequest",
        parameters: UPDATELEAVEREQUESTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("LeaveRequestRequest"),
        response: EndpointResponseSpec::Item("LeaveRequestResponse"),
        errors: UPDATELEAVEREQUESTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateLeaveRequestEndpoint {}

const GETLEAVEREQUESTTIMEREGISTRATIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETLEAVEREQUESTTIMEREGISTRATIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leaverequest/{id}/timeregistration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveRequestTimeRegistrationCollectionEndpoint;

impl Endpoint for GetLeaveRequestTimeRegistrationCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leaverequest/{id}/timeregistration",
        operation_id: "getLeaveRequestTimeRegistrationCollection",
        tag: "leaverequest",
        parameters: GETLEAVEREQUESTTIMEREGISTRATIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TimeRegistrationResponse"),
        errors: GETLEAVEREQUESTTIMEREGISTRATIONCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetLeaveRequestTimeRegistrationCollectionEndpoint {}

const CREATETIMEREGISTRATIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATETIMEREGISTRATIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /leaverequest/{id}/timeregistration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateTimeRegistrationEndpoint;

impl Endpoint for CreateTimeRegistrationEndpoint {
    type Request = TimeRegistrationRequest;
    type Response = ItemResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/leaverequest/{id}/timeregistration",
        operation_id: "createTimeRegistration",
        tag: "leaverequest",
        parameters: CREATETIMEREGISTRATIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TimeRegistrationRequest"),
        response: EndpointResponseSpec::Item("TimeRegistrationResponse"),
        errors: CREATETIMEREGISTRATIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateTimeRegistrationEndpoint {}

const GETLEAVETYPESCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETLEAVETYPESCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leavetypes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveTypesCollectionEndpoint;

impl Endpoint for GetLeaveTypesCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<LeaveTypesResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leavetypes",
        operation_id: "getLeaveTypesCollection",
        tag: "leavetypes",
        parameters: GETLEAVETYPESCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("LeaveTypesResponse"),
        errors: GETLEAVETYPESCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETLEAVETYPESITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETLEAVETYPESITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /leavetypes/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLeaveTypesItemEndpoint;

impl Endpoint for GetLeaveTypesItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<LeaveTypesResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/leavetypes/{id}",
        operation_id: "getLeaveTypesItem",
        tag: "leavetypes",
        parameters: GETLEAVETYPESITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("LeaveTypesResponse"),
        errors: GETLEAVETYPESITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetLeaveTypesItemEndpoint {}

const GETLEDGERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETLEDGERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /ledgercodes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLedgerCollectionEndpoint;

impl Endpoint for GetLedgerCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<LedgerResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/ledgercodes",
        operation_id: "getLedgerCollection",
        tag: "ledgercodes",
        parameters: GETLEDGERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("LedgerResponse"),
        errors: GETLEDGERCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETLEDGERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETLEDGERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /ledgercodes/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetLedgerItemEndpoint;

impl Endpoint for GetLedgerItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<LedgerResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/ledgercodes/{id}",
        operation_id: "getLedgerItem",
        tag: "ledgercodes",
        parameters: GETLEDGERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("LedgerResponse"),
        errors: GETLEDGERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetLedgerItemEndpoint {}

const GETPAYMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPAYMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /payments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPaymentCollectionEndpoint;

impl Endpoint for GetPaymentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PaymentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/payments",
        operation_id: "getPaymentCollection",
        tag: "payments",
        parameters: GETPAYMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PaymentResponse"),
        errors: GETPAYMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPAYMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPAYMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /payments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPaymentItemEndpoint;

impl Endpoint for GetPaymentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<PaymentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/payments/{id}",
        operation_id: "getPaymentItem",
        tag: "payments",
        parameters: GETPAYMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("PaymentResponse"),
        errors: GETPAYMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPaymentItemEndpoint {}

const UPDATEPAYMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEPAYMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /payments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatePaymentEndpoint;

impl Endpoint for UpdatePaymentEndpoint {
    type Request = PaymentRequest;
    type Response = ItemResponse<PaymentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/payments/{id}",
        operation_id: "updatePayment",
        tag: "payments",
        parameters: UPDATEPAYMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("PaymentRequest"),
        response: EndpointResponseSpec::Item("PaymentResponse"),
        errors: UPDATEPAYMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdatePaymentEndpoint {}

const GETPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectCrewCollectionEndpoint;

impl Endpoint for GetProjectCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectcrew",
        operation_id: "getProjectCrewCollection",
        tag: "projectcrew",
        parameters: GETPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCrewResponse"),
        errors: GETPROJECTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTCREWITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTCREWITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectcrew/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectCrewItemEndpoint;

impl Endpoint for GetProjectCrewItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectcrew/{id}",
        operation_id: "getProjectCrewItem",
        tag: "projectcrew",
        parameters: GETPROJECTCREWITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectCrewResponse"),
        errors: GETPROJECTCREWITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectCrewItemEndpoint {}

const GETPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectEquipmentCollectionEndpoint;

impl Endpoint for GetProjectEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectequipment",
        operation_id: "getProjectEquipmentCollection",
        tag: "projectequipment",
        parameters: GETPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentResponse"),
        errors: GETPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTEQUIPMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTEQUIPMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectequipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectEquipmentItemEndpoint;

impl Endpoint for GetProjectEquipmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectequipment/{id}",
        operation_id: "getProjectEquipmentItem",
        tag: "projectequipment",
        parameters: GETPROJECTEQUIPMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectEquipmentResponse"),
        errors: GETPROJECTEQUIPMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectEquipmentItemEndpoint {}

const GETPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectequipmentgroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectEquipmentGroupCollectionEndpoint;

impl Endpoint for GetProjectEquipmentGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectequipmentgroup",
        operation_id: "getProjectEquipmentGroupCollection",
        tag: "projectequipmentgroup",
        parameters: GETPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentGroupResponse"),
        errors: GETPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTEQUIPMENTGROUPITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTEQUIPMENTGROUPITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectequipmentgroup/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectEquipmentGroupItemEndpoint;

impl Endpoint for GetProjectEquipmentGroupItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectequipmentgroup/{id}",
        operation_id: "getProjectEquipmentGroupItem",
        tag: "projectequipmentgroup",
        parameters: GETPROJECTEQUIPMENTGROUPITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectEquipmentGroupResponse"),
        errors: GETPROJECTEQUIPMENTGROUPITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectEquipmentGroupItemEndpoint {}

const GETPROJECTEQUIPMENTGROUPPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTEQUIPMENTGROUPPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectequipmentgroup/{id}/projectequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectEquipmentGroupProjectEquipmentCollectionEndpoint;

impl Endpoint for GetProjectEquipmentGroupProjectEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectequipmentgroup/{id}/projectequipment",
        operation_id: "getProjectEquipmentGroupProjectEquipmentCollection",
        tag: "projectequipmentgroup",
        parameters: GETPROJECTEQUIPMENTGROUPPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentResponse"),
        errors: GETPROJECTEQUIPMENTGROUPPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectEquipmentGroupProjectEquipmentCollectionEndpoint {}

const GETPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctiongroups`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionGroupCollectionEndpoint;

impl Endpoint for GetProjectFunctionGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctiongroups",
        operation_id: "getProjectFunctionGroupCollection",
        tag: "projectfunctiongroups",
        parameters: GETPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionGroupResponse"),
        errors: GETPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTFUNCTIONGROUPITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFUNCTIONGROUPITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctiongroups/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionGroupItemEndpoint;

impl Endpoint for GetProjectFunctionGroupItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectFunctionGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctiongroups/{id}",
        operation_id: "getProjectFunctionGroupItem",
        tag: "projectfunctiongroups",
        parameters: GETPROJECTFUNCTIONGROUPITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectFunctionGroupResponse"),
        errors: GETPROJECTFUNCTIONGROUPITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFunctionGroupItemEndpoint {}

const GETPROJECTFUNCTIONGROUPPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTFUNCTIONGROUPPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctiongroups/{id}/projectfunctions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionGroupProjectFunctionCollectionEndpoint;

impl Endpoint for GetProjectFunctionGroupProjectFunctionCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctiongroups/{id}/projectfunctions",
        operation_id: "getProjectFunctionGroupProjectFunctionCollection",
        tag: "projectfunctiongroups",
        parameters: GETPROJECTFUNCTIONGROUPPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionResponse"),
        errors: GETPROJECTFUNCTIONGROUPPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFunctionGroupProjectFunctionCollectionEndpoint {}

const GETPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionCollectionEndpoint;

impl Endpoint for GetProjectFunctionCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctions",
        operation_id: "getProjectFunctionCollection",
        tag: "projectfunctions",
        parameters: GETPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionResponse"),
        errors: GETPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTFUNCTIONITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFUNCTIONITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctions/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionItemEndpoint;

impl Endpoint for GetProjectFunctionItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectFunctionResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctions/{id}",
        operation_id: "getProjectFunctionItem",
        tag: "projectfunctions",
        parameters: GETPROJECTFUNCTIONITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectFunctionResponse"),
        errors: GETPROJECTFUNCTIONITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFunctionItemEndpoint {}

const GETPROJECTFUNCTIONPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFUNCTIONPROJECTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctions/{id}/projectcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionProjectCrewCollectionEndpoint;

impl Endpoint for GetProjectFunctionProjectCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctions/{id}/projectcrew",
        operation_id: "getProjectFunctionProjectCrewCollection",
        tag: "projectfunctions",
        parameters: GETPROJECTFUNCTIONPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCrewResponse"),
        errors: GETPROJECTFUNCTIONPROJECTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFunctionProjectCrewCollectionEndpoint {}

const GETPROJECTFUNCTIONPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFUNCTIONPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectfunctions/{id}/projectvehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFunctionProjectVehicleCollectionEndpoint;

impl Endpoint for GetProjectFunctionProjectVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectVehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectfunctions/{id}/projectvehicles",
        operation_id: "getProjectFunctionProjectVehicleCollection",
        tag: "projectfunctions",
        parameters: GETPROJECTFUNCTIONPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectVehicleResponse"),
        errors: GETPROJECTFUNCTIONPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFunctionProjectVehicleCollectionEndpoint {}

const GETPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectrequestequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectRequestEquipmentCollectionEndpoint;

impl Endpoint for GetProjectRequestEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectRequestEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectrequestequipment",
        operation_id: "getProjectRequestEquipmentCollection",
        tag: "projectrequestequipment",
        parameters: GETPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectRequestEquipmentResponse"),
        errors: GETPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /projectrequestequipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteProjectRequestEquipmentEndpoint;

impl Endpoint for DeleteProjectRequestEquipmentEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/projectrequestequipment/{id}",
        operation_id: "deleteProjectRequestEquipment",
        tag: "projectrequestequipment",
        parameters: DELETEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteProjectRequestEquipmentEndpoint {}

const GETPROJECTREQUESTEQUIPMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTREQUESTEQUIPMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectrequestequipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectRequestEquipmentItemEndpoint;

impl Endpoint for GetProjectRequestEquipmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectRequestEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectrequestequipment/{id}",
        operation_id: "getProjectRequestEquipmentItem",
        tag: "projectrequestequipment",
        parameters: GETPROJECTREQUESTEQUIPMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectRequestEquipmentResponse"),
        errors: GETPROJECTREQUESTEQUIPMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectRequestEquipmentItemEndpoint {}

const UPDATEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /projectrequestequipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateProjectRequestEquipmentEndpoint;

impl Endpoint for UpdateProjectRequestEquipmentEndpoint {
    type Request = ProjectRequestEquipmentRequest;
    type Response = ItemResponse<ProjectRequestEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/projectrequestequipment/{id}",
        operation_id: "updateProjectRequestEquipment",
        tag: "projectrequestequipment",
        parameters: UPDATEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectRequestEquipmentRequest"),
        response: EndpointResponseSpec::Item("ProjectRequestEquipmentResponse"),
        errors: UPDATEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateProjectRequestEquipmentEndpoint {}

const GETPROJECTREQUESTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTREQUESTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectrequests`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectRequestCollectionEndpoint;

impl Endpoint for GetProjectRequestCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectrequests",
        operation_id: "getProjectRequestCollection",
        tag: "projectrequests",
        parameters: GETPROJECTREQUESTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectRequestResponse"),
        errors: GETPROJECTREQUESTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATEPROJECTREQUESTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATEPROJECTREQUESTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projectrequests`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectRequestEndpoint;

impl Endpoint for CreateProjectRequestEndpoint {
    type Request = ProjectRequestRequest;
    type Response = ItemResponse<ProjectRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projectrequests",
        operation_id: "createProjectRequest",
        tag: "projectrequests",
        parameters: CREATEPROJECTREQUESTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectRequestRequest"),
        response: EndpointResponseSpec::Item("ProjectRequestResponse"),
        errors: CREATEPROJECTREQUESTENDPOINT_ERRORS,
    };
}

const DELETEPROJECTREQUESTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETEPROJECTREQUESTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /projectrequests/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteProjectRequestEndpoint;

impl Endpoint for DeleteProjectRequestEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/projectrequests/{id}",
        operation_id: "deleteProjectRequest",
        tag: "projectrequests",
        parameters: DELETEPROJECTREQUESTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEPROJECTREQUESTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteProjectRequestEndpoint {}

const GETPROJECTREQUESTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTREQUESTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectrequests/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectRequestItemEndpoint;

impl Endpoint for GetProjectRequestItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectrequests/{id}",
        operation_id: "getProjectRequestItem",
        tag: "projectrequests",
        parameters: GETPROJECTREQUESTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectRequestResponse"),
        errors: GETPROJECTREQUESTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectRequestItemEndpoint {}

const UPDATEPROJECTREQUESTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATEPROJECTREQUESTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /projectrequests/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateProjectRequestEndpoint;

impl Endpoint for UpdateProjectRequestEndpoint {
    type Request = ProjectRequestRequest;
    type Response = ItemResponse<ProjectRequestResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/projectrequests/{id}",
        operation_id: "updateProjectRequest",
        tag: "projectrequests",
        parameters: UPDATEPROJECTREQUESTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectRequestRequest"),
        response: EndpointResponseSpec::Item("ProjectRequestResponse"),
        errors: UPDATEPROJECTREQUESTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateProjectRequestEndpoint {}

const GETPROJECTREQUESTPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTREQUESTPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectrequests/{id}/projectrequestequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectRequestProjectRequestEquipmentCollectionEndpoint;

impl Endpoint for GetProjectRequestProjectRequestEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectRequestEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectrequests/{id}/projectrequestequipment",
        operation_id: "getProjectRequestProjectRequestEquipmentCollection",
        tag: "projectrequests",
        parameters: GETPROJECTREQUESTPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectRequestEquipmentResponse"),
        errors: GETPROJECTREQUESTPROJECTREQUESTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectRequestProjectRequestEquipmentCollectionEndpoint {}

const CREATEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projectrequests/{id}/projectrequestequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectRequestEquipmentEndpoint;

impl Endpoint for CreateProjectRequestEquipmentEndpoint {
    type Request = ProjectRequestEquipmentRequest;
    type Response = ItemResponse<ProjectRequestEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projectrequests/{id}/projectrequestequipment",
        operation_id: "createProjectRequestEquipment",
        tag: "projectrequests",
        parameters: CREATEPROJECTREQUESTEQUIPMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectRequestEquipmentRequest"),
        response: EndpointResponseSpec::Item("ProjectRequestEquipmentResponse"),
        errors: CREATEPROJECTREQUESTEQUIPMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateProjectRequestEquipmentEndpoint {}

const GETPROJECTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectCollectionEndpoint;

impl Endpoint for GetProjectCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects",
        operation_id: "getProjectCollection",
        tag: "projects",
        parameters: GETPROJECTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectResponse"),
        errors: GETPROJECTCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATEPROJECTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATEPROJECTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectEndpoint;

impl Endpoint for CreateProjectEndpoint {
    type Request = ProjectRequest;
    type Response = ItemResponse<ProjectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects",
        operation_id: "createProject",
        tag: "projects",
        parameters: CREATEPROJECTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectRequest"),
        response: EndpointResponseSpec::Item("ProjectResponse"),
        errors: CREATEPROJECTENDPOINT_ERRORS,
    };
}

const GETPROJECTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectItemEndpoint;

impl Endpoint for GetProjectItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}",
        operation_id: "getProjectItem",
        tag: "projects",
        parameters: GETPROJECTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectResponse"),
        errors: GETPROJECTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectItemEndpoint {}

const GETPROJECTCONTRACTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTCONTRACTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/contracts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectContractCollectionEndpoint;

impl Endpoint for GetProjectContractCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ContractResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/contracts",
        operation_id: "getProjectContractCollection",
        tag: "projects",
        parameters: GETPROJECTCONTRACTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ContractResponse"),
        errors: GETPROJECTCONTRACTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectContractCollectionEndpoint {}

const GETPROJECTPROJECTCOSTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/costs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectCostCollectionEndpoint;

impl Endpoint for GetProjectProjectCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/costs",
        operation_id: "getProjectProjectCostCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCostResponse"),
        errors: GETPROJECTPROJECTCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectCostCollectionEndpoint {}

const CREATEPROJECTCOSTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATEPROJECTCOSTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects/{id}/costs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectCostEndpoint;

impl Endpoint for CreateProjectCostEndpoint {
    type Request = ProjectCostRequest;
    type Response = ItemResponse<ProjectCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects/{id}/costs",
        operation_id: "createProjectCost",
        tag: "projects",
        parameters: CREATEPROJECTCOSTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectCostRequest"),
        response: EndpointResponseSpec::Item("ProjectCostResponse"),
        errors: CREATEPROJECTCOSTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateProjectCostEndpoint {}

const GETPROJECTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFileFolderCollectionEndpoint;

impl Endpoint for GetProjectFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/file_folders",
        operation_id: "getProjectFileFolderCollection",
        tag: "projects",
        parameters: GETPROJECTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETPROJECTFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFileFolderCollectionEndpoint {}

const GETPROJECTFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectFileCollectionEndpoint;

impl Endpoint for GetProjectFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/files",
        operation_id: "getProjectFileCollection",
        tag: "projects",
        parameters: GETPROJECTFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETPROJECTFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectFileCollectionEndpoint {}

const GETPROJECTPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectCrewCollectionEndpoint;

impl Endpoint for GetProjectProjectCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectcrew",
        operation_id: "getProjectProjectCrewCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCrewResponse"),
        errors: GETPROJECTPROJECTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectCrewCollectionEndpoint {}

const GETPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectEquipmentCollectionEndpoint;

impl Endpoint for GetProjectProjectEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectequipment",
        operation_id: "getProjectProjectEquipmentCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentResponse"),
        errors: GETPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectEquipmentCollectionEndpoint {}

const GETPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectequipmentgroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectEquipmentGroupCollectionEndpoint;

impl Endpoint for GetProjectProjectEquipmentGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectequipmentgroup",
        operation_id: "getProjectProjectEquipmentGroupCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentGroupResponse"),
        errors: GETPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectEquipmentGroupCollectionEndpoint {}

const GETPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectfunctiongroups`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectFunctionGroupCollectionEndpoint;

impl Endpoint for GetProjectProjectFunctionGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectfunctiongroups",
        operation_id: "getProjectProjectFunctionGroupCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionGroupResponse"),
        errors: GETPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectFunctionGroupCollectionEndpoint {}

const CREATEPROJECTFUNCTIONGROUPENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATEPROJECTFUNCTIONGROUPENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects/{id}/projectfunctiongroups`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectFunctionGroupEndpoint;

impl Endpoint for CreateProjectFunctionGroupEndpoint {
    type Request = ProjectFunctionGroupRequest;
    type Response = ItemResponse<ProjectFunctionGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects/{id}/projectfunctiongroups",
        operation_id: "createProjectFunctionGroup",
        tag: "projects",
        parameters: CREATEPROJECTFUNCTIONGROUPENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectFunctionGroupRequest"),
        response: EndpointResponseSpec::Item("ProjectFunctionGroupResponse"),
        errors: CREATEPROJECTFUNCTIONGROUPENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateProjectFunctionGroupEndpoint {}

const GETPROJECTPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectfunctions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectFunctionCollectionEndpoint;

impl Endpoint for GetProjectProjectFunctionCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectfunctions",
        operation_id: "getProjectProjectFunctionCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTFUNCTIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionResponse"),
        errors: GETPROJECTPROJECTFUNCTIONCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectFunctionCollectionEndpoint {}

const CREATEPROJECTFUNCTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATEPROJECTFUNCTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects/{id}/projectfunctions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateProjectFunctionEndpoint;

impl Endpoint for CreateProjectFunctionEndpoint {
    type Request = ProjectFunctionRequest;
    type Response = ItemResponse<ProjectFunctionResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects/{id}/projectfunctions",
        operation_id: "createProjectFunction",
        tag: "projects",
        parameters: CREATEPROJECTFUNCTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("ProjectFunctionRequest"),
        response: EndpointResponseSpec::Item("ProjectFunctionResponse"),
        errors: CREATEPROJECTFUNCTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateProjectFunctionEndpoint {}

const GETPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/projectvehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectProjectVehicleCollectionEndpoint;

impl Endpoint for GetProjectProjectVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectVehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/projectvehicles",
        operation_id: "getProjectProjectVehicleCollection",
        tag: "projects",
        parameters: GETPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectVehicleResponse"),
        errors: GETPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectProjectVehicleCollectionEndpoint {}

const GETPROJECTQUOTATIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTQUOTATIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/quotes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectQuotationCollectionEndpoint;

impl Endpoint for GetProjectQuotationCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<QuotationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/quotes",
        operation_id: "getProjectQuotationCollection",
        tag: "projects",
        parameters: GETPROJECTQUOTATIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("QuotationResponse"),
        errors: GETPROJECTQUOTATIONCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectQuotationCollectionEndpoint {}

const GETPROJECTSUBPROJECTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTSUBPROJECTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/subprojects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectSubprojectCollectionEndpoint;

impl Endpoint for GetProjectSubprojectCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubprojectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/subprojects",
        operation_id: "getProjectSubprojectCollection",
        tag: "projects",
        parameters: GETPROJECTSUBPROJECTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubprojectResponse"),
        errors: GETPROJECTSUBPROJECTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectSubprojectCollectionEndpoint {}

const CREATESUBPROJECTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATESUBPROJECTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects/{id}/subprojects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSubprojectEndpoint;

impl Endpoint for CreateSubprojectEndpoint {
    type Request = SubprojectRequest;
    type Response = ItemResponse<SubprojectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects/{id}/subprojects",
        operation_id: "createSubproject",
        tag: "projects",
        parameters: CREATESUBPROJECTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SubprojectRequest"),
        response: EndpointResponseSpec::Item("SubprojectResponse"),
        errors: CREATESUBPROJECTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateSubprojectEndpoint {}

const GETPROJECTTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projects/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectTaskCollectionEndpoint;

impl Endpoint for GetProjectTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projects/{id}/tasks",
        operation_id: "getProjectTaskCollection",
        tag: "projects",
        parameters: GETPROJECTTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETPROJECTTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectTaskCollectionEndpoint {}

const POSTPROJECTSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTPROJECTSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /projects/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostProjectsIdTasksEndpoint;

impl Endpoint for PostProjectsIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/projects/{id}/tasks",
        operation_id: "createTask",
        tag: "projects",
        parameters: POSTPROJECTSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTPROJECTSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostProjectsIdTasksEndpoint {}

const GETPROJECTTYPECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTTYPECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projecttypes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectTypeCollectionEndpoint;

impl Endpoint for GetProjectTypeCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectTypeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projecttypes",
        operation_id: "getProjectTypeCollection",
        tag: "projecttypes",
        parameters: GETPROJECTTYPECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectTypeResponse"),
        errors: GETPROJECTTYPECOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTTYPEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPROJECTTYPEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projecttypes/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectTypeItemEndpoint;

impl Endpoint for GetProjectTypeItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectTypeResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projecttypes/{id}",
        operation_id: "getProjectTypeItem",
        tag: "projecttypes",
        parameters: GETPROJECTTYPEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectTypeResponse"),
        errors: GETPROJECTTYPEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectTypeItemEndpoint {}

const GETPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectvehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectVehicleCollectionEndpoint;

impl Endpoint for GetProjectVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectVehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectvehicles",
        operation_id: "getProjectVehicleCollection",
        tag: "projectvehicles",
        parameters: GETPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectVehicleResponse"),
        errors: GETPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPROJECTVEHICLEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPROJECTVEHICLEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /projectvehicles/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetProjectVehicleItemEndpoint;

impl Endpoint for GetProjectVehicleItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<ProjectVehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/projectvehicles/{id}",
        operation_id: "getProjectVehicleItem",
        tag: "projectvehicles",
        parameters: GETPROJECTVEHICLEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("ProjectVehicleResponse"),
        errors: GETPROJECTVEHICLEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetProjectVehicleItemEndpoint {}

const GETPURCHASEORDERCOSTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPURCHASEORDERCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseordercosts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderCostCollectionEndpoint;

impl Endpoint for GetPurchaseOrderCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PurchaseOrderCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseordercosts",
        operation_id: "getPurchaseOrderCostCollection",
        tag: "purchaseordercosts",
        parameters: GETPURCHASEORDERCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PurchaseOrderCostResponse"),
        errors: GETPURCHASEORDERCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPURCHASEORDERCOSTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERCOSTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseordercosts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderCostItemEndpoint;

impl Endpoint for GetPurchaseOrderCostItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<PurchaseOrderCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseordercosts/{id}",
        operation_id: "getPurchaseOrderCostItem",
        tag: "purchaseordercosts",
        parameters: GETPURCHASEORDERCOSTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("PurchaseOrderCostResponse"),
        errors: GETPURCHASEORDERCOSTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderCostItemEndpoint {}

const GETPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorderglobalcosts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderGlobalCostCollectionEndpoint;

impl Endpoint for GetPurchaseOrderGlobalCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PurchaseOrderGlobalCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorderglobalcosts",
        operation_id: "getPurchaseOrderGlobalCostCollection",
        tag: "purchaseorderglobalcosts",
        parameters: GETPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PurchaseOrderGlobalCostResponse"),
        errors: GETPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPURCHASEORDERGLOBALCOSTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERGLOBALCOSTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorderglobalcosts/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderGlobalCostItemEndpoint;

impl Endpoint for GetPurchaseOrderGlobalCostItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<PurchaseOrderGlobalCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorderglobalcosts/{id}",
        operation_id: "getPurchaseOrderGlobalCostItem",
        tag: "purchaseorderglobalcosts",
        parameters: GETPURCHASEORDERGLOBALCOSTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("PurchaseOrderGlobalCostResponse"),
        errors: GETPURCHASEORDERGLOBALCOSTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderGlobalCostItemEndpoint {}

const GETPURCHASEORDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETPURCHASEORDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderCollectionEndpoint;

impl Endpoint for GetPurchaseOrderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PurchaseOrderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders",
        operation_id: "getPurchaseOrderCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PurchaseOrderResponse"),
        errors: GETPURCHASEORDERCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETPURCHASEORDERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderItemEndpoint;

impl Endpoint for GetPurchaseOrderItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<PurchaseOrderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}",
        operation_id: "getPurchaseOrderItem",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("PurchaseOrderResponse"),
        errors: GETPURCHASEORDERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderItemEndpoint {}

const GETPURCHASEORDERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderFileFolderCollectionEndpoint;

impl Endpoint for GetPurchaseOrderFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/file_folders",
        operation_id: "getPurchaseOrderFileFolderCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETPURCHASEORDERFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderFileFolderCollectionEndpoint {}

const GETPURCHASEORDERFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderFileCollectionEndpoint;

impl Endpoint for GetPurchaseOrderFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/files",
        operation_id: "getPurchaseOrderFileCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETPURCHASEORDERFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderFileCollectionEndpoint {}

const GETPURCHASEORDERINVOICELINECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERINVOICELINECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/invoicelines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderInvoiceLineCollectionEndpoint;

impl Endpoint for GetPurchaseOrderInvoiceLineCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/invoicelines",
        operation_id: "getPurchaseOrderInvoiceLineCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERINVOICELINECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvoiceLineResponse"),
        errors: GETPURCHASEORDERINVOICELINECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderInvoiceLineCollectionEndpoint {}

const GETPURCHASEORDERPURCHASEORDERCOSTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERPURCHASEORDERCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/purchaseordercosts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderPurchaseOrderCostCollectionEndpoint;

impl Endpoint for GetPurchaseOrderPurchaseOrderCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PurchaseOrderCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/purchaseordercosts",
        operation_id: "getPurchaseOrderPurchaseOrderCostCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERPURCHASEORDERCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PurchaseOrderCostResponse"),
        errors: GETPURCHASEORDERPURCHASEORDERCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderPurchaseOrderCostCollectionEndpoint {}

const GETPURCHASEORDERPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETPURCHASEORDERPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/purchaseorderglobalcosts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderPurchaseOrderGlobalCostCollectionEndpoint;

impl Endpoint for GetPurchaseOrderPurchaseOrderGlobalCostCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<PurchaseOrderGlobalCostResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/purchaseorderglobalcosts",
        operation_id: "getPurchaseOrderPurchaseOrderGlobalCostCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("PurchaseOrderGlobalCostResponse"),
        errors: GETPURCHASEORDERPURCHASEORDERGLOBALCOSTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderPurchaseOrderGlobalCostCollectionEndpoint {}

const GETPURCHASEORDERTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETPURCHASEORDERTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /purchaseorders/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPurchaseOrderTaskCollectionEndpoint;

impl Endpoint for GetPurchaseOrderTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/purchaseorders/{id}/tasks",
        operation_id: "getPurchaseOrderTaskCollection",
        tag: "purchaseorders",
        parameters: GETPURCHASEORDERTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETPURCHASEORDERTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetPurchaseOrderTaskCollectionEndpoint {}

const POSTPURCHASEORDERSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTPURCHASEORDERSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /purchaseorders/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostPurchaseordersIdTasksEndpoint;

impl Endpoint for PostPurchaseordersIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/purchaseorders/{id}/tasks",
        operation_id: "createTask",
        tag: "purchaseorders",
        parameters: POSTPURCHASEORDERSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTPURCHASEORDERSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostPurchaseordersIdTasksEndpoint {}

const GETQUOTATIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETQUOTATIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /quotes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQuotationCollectionEndpoint;

impl Endpoint for GetQuotationCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<QuotationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/quotes",
        operation_id: "getQuotationCollection",
        tag: "quotes",
        parameters: GETQUOTATIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("QuotationResponse"),
        errors: GETQUOTATIONCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETQUOTATIONITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETQUOTATIONITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /quotes/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQuotationItemEndpoint;

impl Endpoint for GetQuotationItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<QuotationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/quotes/{id}",
        operation_id: "getQuotationItem",
        tag: "quotes",
        parameters: GETQUOTATIONITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("QuotationResponse"),
        errors: GETQUOTATIONITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetQuotationItemEndpoint {}

const GETQUOTATIONFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETQUOTATIONFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /quotes/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQuotationFileCollectionEndpoint;

impl Endpoint for GetQuotationFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/quotes/{id}/files",
        operation_id: "getQuotationFileCollection",
        tag: "quotes",
        parameters: GETQUOTATIONFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETQUOTATIONFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetQuotationFileCollectionEndpoint {}

const GETQUOTATIONINVOICELINECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETQUOTATIONINVOICELINECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /quotes/{id}/invoicelines`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQuotationInvoiceLineCollectionEndpoint;

impl Endpoint for GetQuotationInvoiceLineCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<InvoiceLineResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/quotes/{id}/invoicelines",
        operation_id: "getQuotationInvoiceLineCollection",
        tag: "quotes",
        parameters: GETQUOTATIONINVOICELINECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("InvoiceLineResponse"),
        errors: GETQUOTATIONINVOICELINECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetQuotationInvoiceLineCollectionEndpoint {}

const GETQUOTATIONTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETQUOTATIONTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /quotes/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetQuotationTaskCollectionEndpoint;

impl Endpoint for GetQuotationTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/quotes/{id}/tasks",
        operation_id: "getQuotationTaskCollection",
        tag: "quotes",
        parameters: GETQUOTATIONTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETQUOTATIONTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetQuotationTaskCollectionEndpoint {}

const POSTQUOTESIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTQUOTESIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /quotes/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostQuotesIdTasksEndpoint;

impl Endpoint for PostQuotesIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/quotes/{id}/tasks",
        operation_id: "createTask",
        tag: "quotes",
        parameters: POSTQUOTESIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTQUOTESIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostQuotesIdTasksEndpoint {}

const GETCREWRATEFACTORCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCREWRATEFACTORCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /ratefactors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRateFactorCollectionEndpoint;

impl Endpoint for GetCrewRateFactorCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewRateFactorResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/ratefactors",
        operation_id: "getCrewRateFactorCollection",
        tag: "ratefactors",
        parameters: GETCREWRATEFACTORCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewRateFactorResponse"),
        errors: GETCREWRATEFACTORCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETCREWRATEFACTORITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWRATEFACTORITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /ratefactors/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRateFactorItemEndpoint;

impl Endpoint for GetCrewRateFactorItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<CrewRateFactorResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/ratefactors/{id}",
        operation_id: "getCrewRateFactorItem",
        tag: "ratefactors",
        parameters: GETCREWRATEFACTORITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("CrewRateFactorResponse"),
        errors: GETCREWRATEFACTORITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewRateFactorItemEndpoint {}

const GETCREWRATECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETCREWRATECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /rates`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRateCollectionEndpoint;

impl Endpoint for GetCrewRateCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewRateResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/rates",
        operation_id: "getCrewRateCollection",
        tag: "rates",
        parameters: GETCREWRATECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewRateResponse"),
        errors: GETCREWRATECOLLECTIONENDPOINT_ERRORS,
    };
}

const GETCREWRATEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETCREWRATEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /rates/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRateItemEndpoint;

impl Endpoint for GetCrewRateItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<CrewRateResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/rates/{id}",
        operation_id: "getCrewRateItem",
        tag: "rates",
        parameters: GETCREWRATEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("CrewRateResponse"),
        errors: GETCREWRATEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewRateItemEndpoint {}

const GETCREWRATECREWRATEFACTORCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETCREWRATECREWRATEFACTORCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /rates/{id}/ratefactors`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetCrewRateCrewRateFactorCollectionEndpoint;

impl Endpoint for GetCrewRateCrewRateFactorCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<CrewRateFactorResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/rates/{id}/ratefactors",
        operation_id: "getCrewRateCrewRateFactorCollection",
        tag: "rates",
        parameters: GETCREWRATECREWRATEFACTORCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("CrewRateFactorResponse"),
        errors: GETCREWRATECREWRATEFACTORCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetCrewRateCrewRateFactorCollectionEndpoint {}

const GETREPAIRCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETREPAIRCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /repairs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRepairCollectionEndpoint;

impl Endpoint for GetRepairCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<RepairResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/repairs",
        operation_id: "getRepairCollection",
        tag: "repairs",
        parameters: GETREPAIRCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("RepairResponse"),
        errors: GETREPAIRCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETREPAIRITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETREPAIRITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /repairs/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRepairItemEndpoint;

impl Endpoint for GetRepairItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<RepairResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/repairs/{id}",
        operation_id: "getRepairItem",
        tag: "repairs",
        parameters: GETREPAIRITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("RepairResponse"),
        errors: GETREPAIRITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetRepairItemEndpoint {}

const GETREPAIRFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETREPAIRFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /repairs/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRepairFileFolderCollectionEndpoint;

impl Endpoint for GetRepairFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/repairs/{id}/file_folders",
        operation_id: "getRepairFileFolderCollection",
        tag: "repairs",
        parameters: GETREPAIRFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETREPAIRFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetRepairFileFolderCollectionEndpoint {}

const GETREPAIRFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETREPAIRFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /repairs/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRepairFileCollectionEndpoint;

impl Endpoint for GetRepairFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/repairs/{id}/files",
        operation_id: "getRepairFileCollection",
        tag: "repairs",
        parameters: GETREPAIRFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETREPAIRFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetRepairFileCollectionEndpoint {}

const GETREPAIRTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETREPAIRTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /repairs/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetRepairTaskCollectionEndpoint;

impl Endpoint for GetRepairTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/repairs/{id}/tasks",
        operation_id: "getRepairTaskCollection",
        tag: "repairs",
        parameters: GETREPAIRTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETREPAIRTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetRepairTaskCollectionEndpoint {}

const POSTREPAIRSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTREPAIRSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /repairs/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostRepairsIdTasksEndpoint;

impl Endpoint for PostRepairsIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/repairs/{id}/tasks",
        operation_id: "createTask",
        tag: "repairs",
        parameters: POSTREPAIRSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTREPAIRSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostRepairsIdTasksEndpoint {}

const GETSERIALNUMBERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSERIALNUMBERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberCollectionEndpoint;

impl Endpoint for GetSerialNumberCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SerialNumberResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers",
        operation_id: "getSerialNumberCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SerialNumberResponse"),
        errors: GETSERIALNUMBERCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETESERIALNUMBERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETESERIALNUMBERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /serialnumbers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteSerialNumberEndpoint;

impl Endpoint for DeleteSerialNumberEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/serialnumbers/{id}",
        operation_id: "deleteSerialNumber",
        tag: "serialnumbers",
        parameters: DELETESERIALNUMBERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETESERIALNUMBERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteSerialNumberEndpoint {}

const GETSERIALNUMBERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSERIALNUMBERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberItemEndpoint;

impl Endpoint for GetSerialNumberItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SerialNumberResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}",
        operation_id: "getSerialNumberItem",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SerialNumberResponse"),
        errors: GETSERIALNUMBERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberItemEndpoint {}

const UPDATESERIALNUMBERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATESERIALNUMBERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /serialnumbers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateSerialNumberEndpoint;

impl Endpoint for UpdateSerialNumberEndpoint {
    type Request = SerialNumberRequest;
    type Response = ItemResponse<SerialNumberResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/serialnumbers/{id}",
        operation_id: "updateSerialNumber",
        tag: "serialnumbers",
        parameters: UPDATESERIALNUMBERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SerialNumberRequest"),
        response: EndpointResponseSpec::Item("SerialNumberResponse"),
        errors: UPDATESERIALNUMBERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateSerialNumberEndpoint {}

const GETSERIALNUMBERACTUALCONTENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSERIALNUMBERACTUALCONTENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}/actualcontent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberActualContentCollectionEndpoint;

impl Endpoint for GetSerialNumberActualContentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ActualContentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}/actualcontent",
        operation_id: "getSerialNumberActualContentCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERACTUALCONTENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ActualContentResponse"),
        errors: GETSERIALNUMBERACTUALCONTENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberActualContentCollectionEndpoint {}

const GETSERIALNUMBEREQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSERIALNUMBEREQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}/equipmentassignedserials`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberEquipmentAssignedSerialsCollectionEndpoint;

impl Endpoint for GetSerialNumberEquipmentAssignedSerialsCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<EquipmentAssignedSerialsResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}/equipmentassignedserials",
        operation_id: "getSerialNumberEquipmentAssignedSerialsCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBEREQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("EquipmentAssignedSerialsResponse"),
        errors: GETSERIALNUMBEREQUIPMENTASSIGNEDSERIALSCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberEquipmentAssignedSerialsCollectionEndpoint {}

const GETSERIALNUMBERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSERIALNUMBERFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberFileFolderCollectionEndpoint;

impl Endpoint for GetSerialNumberFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}/file_folders",
        operation_id: "getSerialNumberFileFolderCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETSERIALNUMBERFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberFileFolderCollectionEndpoint {}

const GETSERIALNUMBERFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSERIALNUMBERFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberFileCollectionEndpoint;

impl Endpoint for GetSerialNumberFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}/files",
        operation_id: "getSerialNumberFileCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETSERIALNUMBERFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberFileCollectionEndpoint {}

const GETSERIALNUMBERTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSERIALNUMBERTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /serialnumbers/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSerialNumberTaskCollectionEndpoint;

impl Endpoint for GetSerialNumberTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/serialnumbers/{id}/tasks",
        operation_id: "getSerialNumberTaskCollection",
        tag: "serialnumbers",
        parameters: GETSERIALNUMBERTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETSERIALNUMBERTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSerialNumberTaskCollectionEndpoint {}

const POSTSERIALNUMBERSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTSERIALNUMBERSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /serialnumbers/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostSerialnumbersIdTasksEndpoint;

impl Endpoint for PostSerialnumbersIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/serialnumbers/{id}/tasks",
        operation_id: "createTask",
        tag: "serialnumbers",
        parameters: POSTSERIALNUMBERSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTSERIALNUMBERSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostSerialnumbersIdTasksEndpoint {}

const GETSTATUSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSTATUSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /statuses`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStatusCollectionEndpoint;

impl Endpoint for GetStatusCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<StatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/statuses",
        operation_id: "getStatusCollection",
        tag: "statuses",
        parameters: GETSTATUSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("StatusResponse"),
        errors: GETSTATUSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSTATUSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSTATUSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /statuses/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStatusItemEndpoint;

impl Endpoint for GetStatusItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<StatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/statuses/{id}",
        operation_id: "getStatusItem",
        tag: "statuses",
        parameters: GETSTATUSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("StatusResponse"),
        errors: GETSTATUSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetStatusItemEndpoint {}

const GETSTOCKLOCATIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSTOCKLOCATIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /stocklocations`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStockLocationCollectionEndpoint;

impl Endpoint for GetStockLocationCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<StockLocationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/stocklocations",
        operation_id: "getStockLocationCollection",
        tag: "stocklocations",
        parameters: GETSTOCKLOCATIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("StockLocationResponse"),
        errors: GETSTOCKLOCATIONCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSTOCKLOCATIONITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSTOCKLOCATIONITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /stocklocations/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStockLocationItemEndpoint;

impl Endpoint for GetStockLocationItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<StockLocationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/stocklocations/{id}",
        operation_id: "getStockLocationItem",
        tag: "stocklocations",
        parameters: GETSTOCKLOCATIONITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("StockLocationResponse"),
        errors: GETSTOCKLOCATIONITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetStockLocationItemEndpoint {}

const GETSTOCKLOCATIONVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSTOCKLOCATIONVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /stocklocations/{id}/vehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStockLocationVehicleCollectionEndpoint;

impl Endpoint for GetStockLocationVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/stocklocations/{id}/vehicles",
        operation_id: "getStockLocationVehicleCollection",
        tag: "stocklocations",
        parameters: GETSTOCKLOCATIONVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("VehicleResponse"),
        errors: GETSTOCKLOCATIONVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetStockLocationVehicleCollectionEndpoint {}

const CREATEVEHICLEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATEVEHICLEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /stocklocations/{id}/vehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateVehicleEndpoint;

impl Endpoint for CreateVehicleEndpoint {
    type Request = VehicleRequest;
    type Response = ItemResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/stocklocations/{id}/vehicles",
        operation_id: "createVehicle",
        tag: "stocklocations",
        parameters: CREATEVEHICLEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("VehicleRequest"),
        response: EndpointResponseSpec::Item("VehicleResponse"),
        errors: CREATEVEHICLEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateVehicleEndpoint {}

const GETSTOCKMOVEMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSTOCKMOVEMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /stockmovements`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStockMovementCollectionEndpoint;

impl Endpoint for GetStockMovementCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<StockMovementResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/stockmovements",
        operation_id: "getStockMovementCollection",
        tag: "stockmovements",
        parameters: GETSTOCKMOVEMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("StockMovementResponse"),
        errors: GETSTOCKMOVEMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETESTOCKMOVEMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETESTOCKMOVEMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /stockmovements/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteStockMovementEndpoint;

impl Endpoint for DeleteStockMovementEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/stockmovements/{id}",
        operation_id: "deleteStockMovement",
        tag: "stockmovements",
        parameters: DELETESTOCKMOVEMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETESTOCKMOVEMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteStockMovementEndpoint {}

const GETSTOCKMOVEMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSTOCKMOVEMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /stockmovements/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetStockMovementItemEndpoint;

impl Endpoint for GetStockMovementItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<StockMovementResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/stockmovements/{id}",
        operation_id: "getStockMovementItem",
        tag: "stockmovements",
        parameters: GETSTOCKMOVEMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("StockMovementResponse"),
        errors: GETSTOCKMOVEMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetStockMovementItemEndpoint {}

const UPDATESTOCKMOVEMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATESTOCKMOVEMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /stockmovements/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateStockMovementEndpoint;

impl Endpoint for UpdateStockMovementEndpoint {
    type Request = StockMovementRequest;
    type Response = ItemResponse<StockMovementResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/stockmovements/{id}",
        operation_id: "updateStockMovement",
        tag: "stockmovements",
        parameters: UPDATESTOCKMOVEMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("StockMovementRequest"),
        response: EndpointResponseSpec::Item("StockMovementResponse"),
        errors: UPDATESTOCKMOVEMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateStockMovementEndpoint {}

const GETSUBPROJECTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUBPROJECTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectCollectionEndpoint;

impl Endpoint for GetSubprojectCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubprojectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects",
        operation_id: "getSubprojectCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubprojectResponse"),
        errors: GETSUBPROJECTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSUBPROJECTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSUBPROJECTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectItemEndpoint;

impl Endpoint for GetSubprojectItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SubprojectResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}",
        operation_id: "getSubprojectItem",
        tag: "subprojects",
        parameters: GETSUBPROJECTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SubprojectResponse"),
        errors: GETSUBPROJECTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectItemEndpoint {}

const GETSUBPROJECTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectFileFolderCollectionEndpoint;

impl Endpoint for GetSubprojectFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/file_folders",
        operation_id: "getSubprojectFileFolderCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETSUBPROJECTFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectFileFolderCollectionEndpoint {}

const GETSUBPROJECTPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTPROJECTCREWCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/projectcrew`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectProjectCrewCollectionEndpoint;

impl Endpoint for GetSubprojectProjectCrewCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectCrewResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/projectcrew",
        operation_id: "getSubprojectProjectCrewCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTPROJECTCREWCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectCrewResponse"),
        errors: GETSUBPROJECTPROJECTCREWCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectProjectCrewCollectionEndpoint {}

const GETSUBPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/projectequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectProjectEquipmentCollectionEndpoint;

impl Endpoint for GetSubprojectProjectEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/projectequipment",
        operation_id: "getSubprojectProjectEquipmentCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentResponse"),
        errors: GETSUBPROJECTPROJECTEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectProjectEquipmentCollectionEndpoint {}

const GETSUBPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/projectequipmentgroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectProjectEquipmentGroupCollectionEndpoint;

impl Endpoint for GetSubprojectProjectEquipmentGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/projectequipmentgroup",
        operation_id: "getSubprojectProjectEquipmentGroupCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectEquipmentGroupResponse"),
        errors: GETSUBPROJECTPROJECTEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectProjectEquipmentGroupCollectionEndpoint {}

const GETSUBPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/projectfunctiongroups`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectProjectFunctionGroupCollectionEndpoint;

impl Endpoint for GetSubprojectProjectFunctionGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectFunctionGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/projectfunctiongroups",
        operation_id: "getSubprojectProjectFunctionGroupCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectFunctionGroupResponse"),
        errors: GETSUBPROJECTPROJECTFUNCTIONGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectProjectFunctionGroupCollectionEndpoint {}

const GETSUBPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subprojects/{id}/projectvehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubprojectProjectVehicleCollectionEndpoint;

impl Endpoint for GetSubprojectProjectVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<ProjectVehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subprojects/{id}/projectvehicles",
        operation_id: "getSubprojectProjectVehicleCollection",
        tag: "subprojects",
        parameters: GETSUBPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("ProjectVehicleResponse"),
        errors: GETSUBPROJECTPROJECTVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubprojectProjectVehicleCollectionEndpoint {}

const GETSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentalequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalEquipmentCollectionEndpoint;

impl Endpoint for GetSubrentalEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentalequipment",
        operation_id: "getSubrentalEquipmentCollection",
        tag: "subrentalequipment",
        parameters: GETSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalEquipmentResponse"),
        errors: GETSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSUBRENTALEQUIPMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALEQUIPMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentalequipment/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalEquipmentItemEndpoint;

impl Endpoint for GetSubrentalEquipmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SubrentalEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentalequipment/{id}",
        operation_id: "getSubrentalEquipmentItem",
        tag: "subrentalequipment",
        parameters: GETSUBRENTALEQUIPMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SubrentalEquipmentResponse"),
        errors: GETSUBRENTALEQUIPMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalEquipmentItemEndpoint {}

const GETSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentalequipmentgroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalEquipmentGroupCollectionEndpoint;

impl Endpoint for GetSubrentalEquipmentGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentalequipmentgroup",
        operation_id: "getSubrentalEquipmentGroupCollection",
        tag: "subrentalequipmentgroup",
        parameters: GETSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalEquipmentGroupResponse"),
        errors: GETSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSUBRENTALEQUIPMENTGROUPITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALEQUIPMENTGROUPITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentalequipmentgroup/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalEquipmentGroupItemEndpoint;

impl Endpoint for GetSubrentalEquipmentGroupItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SubrentalEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentalequipmentgroup/{id}",
        operation_id: "getSubrentalEquipmentGroupItem",
        tag: "subrentalequipmentgroup",
        parameters: GETSUBRENTALEQUIPMENTGROUPITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SubrentalEquipmentGroupResponse"),
        errors: GETSUBRENTALEQUIPMENTGROUPITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalEquipmentGroupItemEndpoint {}

const GETSUBRENTALEQUIPMENTGROUPSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSUBRENTALEQUIPMENTGROUPSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS:
    &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentalequipmentgroup/{id}/subrentalequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalEquipmentGroupSubrentalEquipmentCollectionEndpoint;

impl Endpoint for GetSubrentalEquipmentGroupSubrentalEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentalequipmentgroup/{id}/subrentalequipment",
        operation_id: "getSubrentalEquipmentGroupSubrentalEquipmentCollection",
        tag: "subrentalequipmentgroup",
        parameters: GETSUBRENTALEQUIPMENTGROUPSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalEquipmentResponse"),
        errors: GETSUBRENTALEQUIPMENTGROUPSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalEquipmentGroupSubrentalEquipmentCollectionEndpoint {}

const GETSUBRENTALCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUBRENTALCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalCollectionEndpoint;

impl Endpoint for GetSubrentalCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals",
        operation_id: "getSubrentalCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalResponse"),
        errors: GETSUBRENTALCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETSUBRENTALITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSUBRENTALITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalItemEndpoint;

impl Endpoint for GetSubrentalItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SubrentalResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}",
        operation_id: "getSubrentalItem",
        tag: "subrentals",
        parameters: GETSUBRENTALITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SubrentalResponse"),
        errors: GETSUBRENTALITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalItemEndpoint {}

const GETSUBRENTALFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalFileFolderCollectionEndpoint;

impl Endpoint for GetSubrentalFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}/file_folders",
        operation_id: "getSubrentalFileFolderCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETSUBRENTALFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalFileFolderCollectionEndpoint {}

const GETSUBRENTALFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalFileCollectionEndpoint;

impl Endpoint for GetSubrentalFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}/files",
        operation_id: "getSubrentalFileCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETSUBRENTALFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalFileCollectionEndpoint {}

const GETSUBRENTALSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}/subrentalequipment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalSubrentalEquipmentCollectionEndpoint;

impl Endpoint for GetSubrentalSubrentalEquipmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalEquipmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}/subrentalequipment",
        operation_id: "getSubrentalSubrentalEquipmentCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalEquipmentResponse"),
        errors: GETSUBRENTALSUBRENTALEQUIPMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalSubrentalEquipmentCollectionEndpoint {}

const GETSUBRENTALSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}/subrentalequipmentgroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalSubrentalEquipmentGroupCollectionEndpoint;

impl Endpoint for GetSubrentalSubrentalEquipmentGroupCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubrentalEquipmentGroupResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}/subrentalequipmentgroup",
        operation_id: "getSubrentalSubrentalEquipmentGroupCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubrentalEquipmentGroupResponse"),
        errors: GETSUBRENTALSUBRENTALEQUIPMENTGROUPCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalSubrentalEquipmentGroupCollectionEndpoint {}

const GETSUBRENTALTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUBRENTALTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subrentals/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubrentalTaskCollectionEndpoint;

impl Endpoint for GetSubrentalTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subrentals/{id}/tasks",
        operation_id: "getSubrentalTaskCollection",
        tag: "subrentals",
        parameters: GETSUBRENTALTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETSUBRENTALTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubrentalTaskCollectionEndpoint {}

const POSTSUBRENTALSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTSUBRENTALSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /subrentals/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostSubrentalsIdTasksEndpoint;

impl Endpoint for PostSubrentalsIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/subrentals/{id}/tasks",
        operation_id: "createTask",
        tag: "subrentals",
        parameters: POSTSUBRENTALSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTSUBRENTALSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostSubrentalsIdTasksEndpoint {}

const GETSUBTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUBTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subtasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubtaskCollectionEndpoint;

impl Endpoint for GetSubtaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubtaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subtasks",
        operation_id: "getSubtaskCollection",
        tag: "subtasks",
        parameters: GETSUBTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubtaskResponse"),
        errors: GETSUBTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETESUBTASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETESUBTASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /subtasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteSubtaskEndpoint;

impl Endpoint for DeleteSubtaskEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/subtasks/{id}",
        operation_id: "deleteSubtask",
        tag: "subtasks",
        parameters: DELETESUBTASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETESUBTASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteSubtaskEndpoint {}

const GETSUBTASKITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSUBTASKITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /subtasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSubtaskItemEndpoint;

impl Endpoint for GetSubtaskItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SubtaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/subtasks/{id}",
        operation_id: "getSubtaskItem",
        tag: "subtasks",
        parameters: GETSUBTASKITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SubtaskResponse"),
        errors: GETSUBTASKITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSubtaskItemEndpoint {}

const UPDATESUBTASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATESUBTASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /subtasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateSubtaskEndpoint;

impl Endpoint for UpdateSubtaskEndpoint {
    type Request = SubtaskRequest;
    type Response = ItemResponse<SubtaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/subtasks/{id}",
        operation_id: "updateSubtask",
        tag: "subtasks",
        parameters: UPDATESUBTASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SubtaskRequest"),
        response: EndpointResponseSpec::Item("SubtaskResponse"),
        errors: UPDATESUBTASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateSubtaskEndpoint {}

const GETSUPPLIERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETSUPPLIERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /suppliers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupplierCollectionEndpoint;

impl Endpoint for GetSupplierCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SupplierResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/suppliers",
        operation_id: "getSupplierCollection",
        tag: "suppliers",
        parameters: GETSUPPLIERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SupplierResponse"),
        errors: GETSUPPLIERCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETESUPPLIERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETESUPPLIERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /suppliers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteSupplierEndpoint;

impl Endpoint for DeleteSupplierEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/suppliers/{id}",
        operation_id: "deleteSupplier",
        tag: "suppliers",
        parameters: DELETESUPPLIERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETESUPPLIERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteSupplierEndpoint {}

const GETSUPPLIERITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETSUPPLIERITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /suppliers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupplierItemEndpoint;

impl Endpoint for GetSupplierItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<SupplierResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/suppliers/{id}",
        operation_id: "getSupplierItem",
        tag: "suppliers",
        parameters: GETSUPPLIERITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("SupplierResponse"),
        errors: GETSUPPLIERITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSupplierItemEndpoint {}

const UPDATESUPPLIERENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATESUPPLIERENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /suppliers/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateSupplierEndpoint;

impl Endpoint for UpdateSupplierEndpoint {
    type Request = SupplierRequest;
    type Response = ItemResponse<SupplierResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/suppliers/{id}",
        operation_id: "updateSupplier",
        tag: "suppliers",
        parameters: UPDATESUPPLIERENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SupplierRequest"),
        response: EndpointResponseSpec::Item("SupplierResponse"),
        errors: UPDATESUPPLIERENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateSupplierEndpoint {}

const GETSUPPLIERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUPPLIERFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /suppliers/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupplierFileFolderCollectionEndpoint;

impl Endpoint for GetSupplierFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/suppliers/{id}/file_folders",
        operation_id: "getSupplierFileFolderCollection",
        tag: "suppliers",
        parameters: GETSUPPLIERFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETSUPPLIERFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSupplierFileFolderCollectionEndpoint {}

const GETSUPPLIERFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUPPLIERFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /suppliers/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupplierFileCollectionEndpoint;

impl Endpoint for GetSupplierFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/suppliers/{id}/files",
        operation_id: "getSupplierFileCollection",
        tag: "suppliers",
        parameters: GETSUPPLIERFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETSUPPLIERFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSupplierFileCollectionEndpoint {}

const GETSUPPLIERTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETSUPPLIERTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /suppliers/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupplierTaskCollectionEndpoint;

impl Endpoint for GetSupplierTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/suppliers/{id}/tasks",
        operation_id: "getSupplierTaskCollection",
        tag: "suppliers",
        parameters: GETSUPPLIERTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETSUPPLIERTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetSupplierTaskCollectionEndpoint {}

const POSTSUPPLIERSIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const POSTSUPPLIERSIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /suppliers/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostSuppliersIdTasksEndpoint;

impl Endpoint for PostSuppliersIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/suppliers/{id}/tasks",
        operation_id: "createTask",
        tag: "suppliers",
        parameters: POSTSUPPLIERSIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTSUPPLIERSIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostSuppliersIdTasksEndpoint {}

const GETTASKASSIGNMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTASKASSIGNMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taskassignments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskAssignmentCollectionEndpoint;

impl Endpoint for GetTaskAssignmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskAssignmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taskassignments",
        operation_id: "getTaskAssignmentCollection",
        tag: "taskassignments",
        parameters: GETTASKASSIGNMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskAssignmentResponse"),
        errors: GETTASKASSIGNMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

const DELETETASKASSIGNMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETETASKASSIGNMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /taskassignments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTaskAssignmentEndpoint;

impl Endpoint for DeleteTaskAssignmentEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/taskassignments/{id}",
        operation_id: "deleteTaskAssignment",
        tag: "taskassignments",
        parameters: DELETETASKASSIGNMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETETASKASSIGNMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteTaskAssignmentEndpoint {}

const GETTASKASSIGNMENTITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTASKASSIGNMENTITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taskassignments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskAssignmentItemEndpoint;

impl Endpoint for GetTaskAssignmentItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TaskAssignmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taskassignments/{id}",
        operation_id: "getTaskAssignmentItem",
        tag: "taskassignments",
        parameters: GETTASKASSIGNMENTITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TaskAssignmentResponse"),
        errors: GETTASKASSIGNMENTITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskAssignmentItemEndpoint {}

const UPDATETASKASSIGNMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATETASKASSIGNMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /taskassignments/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateTaskAssignmentEndpoint;

impl Endpoint for UpdateTaskAssignmentEndpoint {
    type Request = TaskAssignmentRequest;
    type Response = ItemResponse<TaskAssignmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/taskassignments/{id}",
        operation_id: "updateTaskAssignment",
        tag: "taskassignments",
        parameters: UPDATETASKASSIGNMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskAssignmentRequest"),
        response: EndpointResponseSpec::Item("TaskAssignmentResponse"),
        errors: UPDATETASKASSIGNMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateTaskAssignmentEndpoint {}

const GETTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskCollectionEndpoint;

impl Endpoint for GetTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks",
        operation_id: "getTaskCollection",
        tag: "tasks",
        parameters: GETTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

const POSTTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const POSTTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostTasksEndpoint;

impl Endpoint for PostTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/tasks",
        operation_id: "createTask",
        tag: "tasks",
        parameters: POSTTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTTASKSENDPOINT_ERRORS,
    };
}

const DELETETASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETETASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /tasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTaskEndpoint;

impl Endpoint for DeleteTaskEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/tasks/{id}",
        operation_id: "deleteTask",
        tag: "tasks",
        parameters: DELETETASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETETASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteTaskEndpoint {}

const GETTASKITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETTASKITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskItemEndpoint;

impl Endpoint for GetTaskItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks/{id}",
        operation_id: "getTaskItem",
        tag: "tasks",
        parameters: GETTASKITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: GETTASKITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskItemEndpoint {}

const UPDATETASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATETASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /tasks/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateTaskEndpoint;

impl Endpoint for UpdateTaskEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/tasks/{id}",
        operation_id: "updateTask",
        tag: "tasks",
        parameters: UPDATETASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: UPDATETASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateTaskEndpoint {}

const GETTASKFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTASKFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskFileFolderCollectionEndpoint;

impl Endpoint for GetTaskFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks/{id}/file_folders",
        operation_id: "getTaskFileFolderCollection",
        tag: "tasks",
        parameters: GETTASKFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETTASKFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskFileFolderCollectionEndpoint {}

const GETTASKFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTASKFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskFileCollectionEndpoint;

impl Endpoint for GetTaskFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks/{id}/files",
        operation_id: "getTaskFileCollection",
        tag: "tasks",
        parameters: GETTASKFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETTASKFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskFileCollectionEndpoint {}

const GETTASKSUBTASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTASKSUBTASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks/{id}/subtasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskSubtaskCollectionEndpoint;

impl Endpoint for GetTaskSubtaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<SubtaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks/{id}/subtasks",
        operation_id: "getTaskSubtaskCollection",
        tag: "tasks",
        parameters: GETTASKSUBTASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("SubtaskResponse"),
        errors: GETTASKSUBTASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskSubtaskCollectionEndpoint {}

const CREATESUBTASKENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const CREATESUBTASKENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /tasks/{id}/subtasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateSubtaskEndpoint;

impl Endpoint for CreateSubtaskEndpoint {
    type Request = SubtaskRequest;
    type Response = ItemResponse<SubtaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/tasks/{id}/subtasks",
        operation_id: "createSubtask",
        tag: "tasks",
        parameters: CREATESUBTASKENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("SubtaskRequest"),
        response: EndpointResponseSpec::Item("SubtaskResponse"),
        errors: CREATESUBTASKENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateSubtaskEndpoint {}

const GETTASKTASKASSIGNMENTCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTASKTASKASSIGNMENTCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /tasks/{id}/taskassignments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskTaskAssignmentCollectionEndpoint;

impl Endpoint for GetTaskTaskAssignmentCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskAssignmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/tasks/{id}/taskassignments",
        operation_id: "getTaskTaskAssignmentCollection",
        tag: "tasks",
        parameters: GETTASKTASKASSIGNMENTCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskAssignmentResponse"),
        errors: GETTASKTASKASSIGNMENTCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskTaskAssignmentCollectionEndpoint {}

const CREATETASKASSIGNMENTENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const CREATETASKASSIGNMENTENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /tasks/{id}/taskassignments`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateTaskAssignmentEndpoint;

impl Endpoint for CreateTaskAssignmentEndpoint {
    type Request = TaskAssignmentRequest;
    type Response = ItemResponse<TaskAssignmentResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/tasks/{id}/taskassignments",
        operation_id: "createTaskAssignment",
        tag: "tasks",
        parameters: CREATETASKASSIGNMENTENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskAssignmentRequest"),
        response: EndpointResponseSpec::Item("TaskAssignmentResponse"),
        errors: CREATETASKASSIGNMENTENDPOINT_ERRORS,
    };
}

impl EndpointWithId for CreateTaskAssignmentEndpoint {}

const GETTASKSTATUSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTASKSTATUSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taskstatuses`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskStatusCollectionEndpoint;

impl Endpoint for GetTaskStatusCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskStatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taskstatuses",
        operation_id: "getTaskStatusCollection",
        tag: "taskstatuses",
        parameters: GETTASKSTATUSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskStatusResponse"),
        errors: GETTASKSTATUSCOLLECTIONENDPOINT_ERRORS,
    };
}

const CREATETASKSTATUSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const CREATETASKSTATUSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /taskstatuses`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateTaskStatusEndpoint;

impl Endpoint for CreateTaskStatusEndpoint {
    type Request = TaskStatusRequest;
    type Response = ItemResponse<TaskStatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/taskstatuses",
        operation_id: "createTaskStatus",
        tag: "taskstatuses",
        parameters: CREATETASKSTATUSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskStatusRequest"),
        response: EndpointResponseSpec::Item("TaskStatusResponse"),
        errors: CREATETASKSTATUSENDPOINT_ERRORS,
    };
}

const DELETETASKSTATUSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETETASKSTATUSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /taskstatuses/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTaskStatusEndpoint;

impl Endpoint for DeleteTaskStatusEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/taskstatuses/{id}",
        operation_id: "deleteTaskStatus",
        tag: "taskstatuses",
        parameters: DELETETASKSTATUSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETETASKSTATUSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteTaskStatusEndpoint {}

const GETTASKSTATUSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETTASKSTATUSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taskstatuses/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaskStatusItemEndpoint;

impl Endpoint for GetTaskStatusItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TaskStatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taskstatuses/{id}",
        operation_id: "getTaskStatusItem",
        tag: "taskstatuses",
        parameters: GETTASKSTATUSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TaskStatusResponse"),
        errors: GETTASKSTATUSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaskStatusItemEndpoint {}

const UPDATETASKSTATUSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATETASKSTATUSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /taskstatuses/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateTaskStatusEndpoint;

impl Endpoint for UpdateTaskStatusEndpoint {
    type Request = TaskStatusRequest;
    type Response = ItemResponse<TaskStatusResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/taskstatuses/{id}",
        operation_id: "updateTaskStatus",
        tag: "taskstatuses",
        parameters: UPDATETASKSTATUSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskStatusRequest"),
        response: EndpointResponseSpec::Item("TaskStatusResponse"),
        errors: UPDATETASKSTATUSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateTaskStatusEndpoint {}

const GETTAXCLASSCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTAXCLASSCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taxclasses`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaxClassCollectionEndpoint;

impl Endpoint for GetTaxClassCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaxClassResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taxclasses",
        operation_id: "getTaxClassCollection",
        tag: "taxclasses",
        parameters: GETTAXCLASSCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaxClassResponse"),
        errors: GETTAXCLASSCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETTAXCLASSITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETTAXCLASSITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /taxclasses/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTaxClassItemEndpoint;

impl Endpoint for GetTaxClassItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TaxClassResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/taxclasses/{id}",
        operation_id: "getTaxClassItem",
        tag: "taxclasses",
        parameters: GETTAXCLASSITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TaxClassResponse"),
        errors: GETTAXCLASSITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTaxClassItemEndpoint {}

const GETTIMEREGISTRATIONCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTIMEREGISTRATIONCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /timeregistration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationCollectionEndpoint;

impl Endpoint for GetTimeRegistrationCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistration",
        operation_id: "getTimeRegistrationCollection",
        tag: "timeregistration",
        parameters: GETTIMEREGISTRATIONCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TimeRegistrationResponse"),
        errors: GETTIMEREGISTRATIONCOLLECTIONENDPOINT_ERRORS,
    };
}

const POSTTIMEREGISTRATIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const POSTTIMEREGISTRATIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /timeregistration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostTimeregistrationEndpoint;

impl Endpoint for PostTimeregistrationEndpoint {
    type Request = TimeRegistrationRequest;
    type Response = ItemResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/timeregistration",
        operation_id: "createTimeRegistration",
        tag: "timeregistration",
        parameters: POSTTIMEREGISTRATIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TimeRegistrationRequest"),
        response: EndpointResponseSpec::Item("TimeRegistrationResponse"),
        errors: POSTTIMEREGISTRATIONENDPOINT_ERRORS,
    };
}

const DELETETIMEREGISTRATIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const DELETETIMEREGISTRATIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /timeregistration/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTimeRegistrationEndpoint;

impl Endpoint for DeleteTimeRegistrationEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/timeregistration/{id}",
        operation_id: "deleteTimeRegistration",
        tag: "timeregistration",
        parameters: DELETETIMEREGISTRATIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETETIMEREGISTRATIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteTimeRegistrationEndpoint {}

const GETTIMEREGISTRATIONITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTIMEREGISTRATIONITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /timeregistration/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationItemEndpoint;

impl Endpoint for GetTimeRegistrationItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistration/{id}",
        operation_id: "getTimeRegistrationItem",
        tag: "timeregistration",
        parameters: GETTIMEREGISTRATIONITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TimeRegistrationResponse"),
        errors: GETTIMEREGISTRATIONITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTimeRegistrationItemEndpoint {}

const UPDATETIMEREGISTRATIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const UPDATETIMEREGISTRATIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /timeregistration/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateTimeRegistrationEndpoint;

impl Endpoint for UpdateTimeRegistrationEndpoint {
    type Request = TimeRegistrationRequest;
    type Response = ItemResponse<TimeRegistrationResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/timeregistration/{id}",
        operation_id: "updateTimeRegistration",
        tag: "timeregistration",
        parameters: UPDATETIMEREGISTRATIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TimeRegistrationRequest"),
        response: EndpointResponseSpec::Item("TimeRegistrationResponse"),
        errors: UPDATETIMEREGISTRATIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateTimeRegistrationEndpoint {}

const GETTIMEREGISTRATIONFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTIMEREGISTRATIONFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /timeregistration/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationFileCollectionEndpoint;

impl Endpoint for GetTimeRegistrationFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistration/{id}/files",
        operation_id: "getTimeRegistrationFileCollection",
        tag: "timeregistration",
        parameters: GETTIMEREGISTRATIONFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETTIMEREGISTRATIONFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTimeRegistrationFileCollectionEndpoint {}

const GETTIMEREGISTRATIONTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_PARAMETERS:
    &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETTIMEREGISTRATIONTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] =
    &[
        EndpointErrorSpec {
            status: 400,
            description: "Bad request",
        },
        EndpointErrorSpec {
            status: 401,
            description: "Unauthorized",
        },
        EndpointErrorSpec {
            status: 404,
            description: "Not found",
        },
        EndpointErrorSpec {
            status: 500,
            description: "Something went wrong",
        },
        EndpointErrorSpec {
            status: 502,
            description: "Bad gateway",
        },
    ];

/// Typed endpoint for `GET /timeregistration/{id}/timeregistrationactivities`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationTimeRegistrationActivityCollectionEndpoint;

impl Endpoint for GetTimeRegistrationTimeRegistrationActivityCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TimeRegistrationActivityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistration/{id}/timeregistrationactivities",
        operation_id: "getTimeRegistrationTimeRegistrationActivityCollection",
        tag: "timeregistration",
        parameters: GETTIMEREGISTRATIONTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TimeRegistrationActivityResponse"),
        errors: GETTIMEREGISTRATIONTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTimeRegistrationTimeRegistrationActivityCollectionEndpoint {}

const GETTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /timeregistrationactivities`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationActivityCollectionEndpoint;

impl Endpoint for GetTimeRegistrationActivityCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TimeRegistrationActivityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistrationactivities",
        operation_id: "getTimeRegistrationActivityCollection",
        tag: "timeregistrationactivities",
        parameters: GETTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TimeRegistrationActivityResponse"),
        errors: GETTIMEREGISTRATIONACTIVITYCOLLECTIONENDPOINT_ERRORS,
    };
}

const GETTIMEREGISTRATIONACTIVITYITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETTIMEREGISTRATIONACTIVITYITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /timeregistrationactivities/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetTimeRegistrationActivityItemEndpoint;

impl Endpoint for GetTimeRegistrationActivityItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<TimeRegistrationActivityResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/timeregistrationactivities/{id}",
        operation_id: "getTimeRegistrationActivityItem",
        tag: "timeregistrationactivities",
        parameters: GETTIMEREGISTRATIONACTIVITYITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("TimeRegistrationActivityResponse"),
        errors: GETTIMEREGISTRATIONACTIVITYITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetTimeRegistrationActivityItemEndpoint {}

const GETVEHICLECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const GETVEHICLECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /vehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVehicleCollectionEndpoint;

impl Endpoint for GetVehicleCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/vehicles",
        operation_id: "getVehicleCollection",
        tag: "vehicles",
        parameters: GETVEHICLECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("VehicleResponse"),
        errors: GETVEHICLECOLLECTIONENDPOINT_ERRORS,
    };
}

const POSTVEHICLESENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[];

const POSTVEHICLESENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /vehicles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostVehiclesEndpoint;

impl Endpoint for PostVehiclesEndpoint {
    type Request = VehicleRequest;
    type Response = ItemResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/vehicles",
        operation_id: "createVehicle",
        tag: "vehicles",
        parameters: POSTVEHICLESENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("VehicleRequest"),
        response: EndpointResponseSpec::Item("VehicleResponse"),
        errors: POSTVEHICLESENDPOINT_ERRORS,
    };
}

const DELETEVEHICLEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const DELETEVEHICLEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `DELETE /vehicles/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteVehicleEndpoint;

impl Endpoint for DeleteVehicleEndpoint {
    type Request = NoRequest;
    type Response = NoContent;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "DELETE",
        path: "/vehicles/{id}",
        operation_id: "deleteVehicle",
        tag: "vehicles",
        parameters: DELETEVEHICLEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::NoContent,
        errors: DELETEVEHICLEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for DeleteVehicleEndpoint {}

const GETVEHICLEITEMENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const GETVEHICLEITEMENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /vehicles/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVehicleItemEndpoint;

impl Endpoint for GetVehicleItemEndpoint {
    type Request = NoRequest;
    type Response = ItemResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/vehicles/{id}",
        operation_id: "getVehicleItem",
        tag: "vehicles",
        parameters: GETVEHICLEITEMENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Item("VehicleResponse"),
        errors: GETVEHICLEITEMENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetVehicleItemEndpoint {}

const UPDATEVEHICLEENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const UPDATEVEHICLEENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `PUT /vehicles/{id}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateVehicleEndpoint;

impl Endpoint for UpdateVehicleEndpoint {
    type Request = VehicleRequest;
    type Response = ItemResponse<VehicleResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "PUT",
        path: "/vehicles/{id}",
        operation_id: "updateVehicle",
        tag: "vehicles",
        parameters: UPDATEVEHICLEENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("VehicleRequest"),
        response: EndpointResponseSpec::Item("VehicleResponse"),
        errors: UPDATEVEHICLEENDPOINT_ERRORS,
    };
}

impl EndpointWithId for UpdateVehicleEndpoint {}

const GETVEHICLEFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETVEHICLEFILEFOLDERCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /vehicles/{id}/file_folders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVehicleFileFolderCollectionEndpoint;

impl Endpoint for GetVehicleFileFolderCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileFolderResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/vehicles/{id}/file_folders",
        operation_id: "getVehicleFileFolderCollection",
        tag: "vehicles",
        parameters: GETVEHICLEFILEFOLDERCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileFolderResponse"),
        errors: GETVEHICLEFILEFOLDERCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetVehicleFileFolderCollectionEndpoint {}

const GETVEHICLEFILECOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETVEHICLEFILECOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /vehicles/{id}/files`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVehicleFileCollectionEndpoint;

impl Endpoint for GetVehicleFileCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<FileResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/vehicles/{id}/files",
        operation_id: "getVehicleFileCollection",
        tag: "vehicles",
        parameters: GETVEHICLEFILECOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("FileResponse"),
        errors: GETVEHICLEFILECOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetVehicleFileCollectionEndpoint {}

const GETVEHICLETASKCOLLECTIONENDPOINT_PARAMETERS: &[EndpointParameterSpec] =
    &[EndpointParameterSpec {
        name: "id",
        location: EndpointParameterLocation::Path,
        required: true,
        value: EndpointParameterValueSpec::Integer,
    }];

const GETVEHICLETASKCOLLECTIONENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `GET /vehicles/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetVehicleTaskCollectionEndpoint;

impl Endpoint for GetVehicleTaskCollectionEndpoint {
    type Request = NoRequest;
    type Response = CollectionResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "GET",
        path: "/vehicles/{id}/tasks",
        operation_id: "getVehicleTaskCollection",
        tag: "vehicles",
        parameters: GETVEHICLETASKCOLLECTIONENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::None,
        response: EndpointResponseSpec::Collection("TaskResponse"),
        errors: GETVEHICLETASKCOLLECTIONENDPOINT_ERRORS,
    };
}

impl EndpointWithId for GetVehicleTaskCollectionEndpoint {}

const POSTVEHICLESIDTASKSENDPOINT_PARAMETERS: &[EndpointParameterSpec] = &[EndpointParameterSpec {
    name: "id",
    location: EndpointParameterLocation::Path,
    required: true,
    value: EndpointParameterValueSpec::Integer,
}];

const POSTVEHICLESIDTASKSENDPOINT_ERRORS: &[EndpointErrorSpec] = &[
    EndpointErrorSpec {
        status: 400,
        description: "Bad request",
    },
    EndpointErrorSpec {
        status: 401,
        description: "Unauthorized",
    },
    EndpointErrorSpec {
        status: 404,
        description: "Not found",
    },
    EndpointErrorSpec {
        status: 500,
        description: "Something went wrong",
    },
    EndpointErrorSpec {
        status: 502,
        description: "Bad gateway",
    },
];

/// Typed endpoint for `POST /vehicles/{id}/tasks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostVehiclesIdTasksEndpoint;

impl Endpoint for PostVehiclesIdTasksEndpoint {
    type Request = TaskRequest;
    type Response = ItemResponse<TaskResponse>;

    const SPEC: EndpointSpec = EndpointSpec {
        method: "POST",
        path: "/vehicles/{id}/tasks",
        operation_id: "createTask",
        tag: "vehicles",
        parameters: POSTVEHICLESIDTASKSENDPOINT_PARAMETERS,
        request: EndpointRequestSpec::Json("TaskRequest"),
        response: EndpointResponseSpec::Item("TaskResponse"),
        errors: POSTVEHICLESIDTASKSENDPOINT_ERRORS,
    };
}

impl EndpointWithId for PostVehiclesIdTasksEndpoint {}

/// All operations in the checked-in Rentman API document.
pub const ALL_ENDPOINTS: &[EndpointSpec] = &[
    <GetAccessoryCollectionEndpoint as Endpoint>::SPEC,
    <DeleteAccessoryEndpoint as Endpoint>::SPEC,
    <GetAccessoryItemEndpoint as Endpoint>::SPEC,
    <UpdateAccessoryEndpoint as Endpoint>::SPEC,
    <GetActualContentCollectionEndpoint as Endpoint>::SPEC,
    <GetActualContentItemEndpoint as Endpoint>::SPEC,
    <GetAlternativeCollectionEndpoint as Endpoint>::SPEC,
    <DeleteAlternativeEndpoint as Endpoint>::SPEC,
    <GetAlternativeItemEndpoint as Endpoint>::SPEC,
    <UpdateAlternativeEndpoint as Endpoint>::SPEC,
    <GetAppointmentCrewCollectionEndpoint as Endpoint>::SPEC,
    <DeleteAppointmentCrewEndpoint as Endpoint>::SPEC,
    <GetAppointmentCrewItemEndpoint as Endpoint>::SPEC,
    <UpdateAppointmentCrewEndpoint as Endpoint>::SPEC,
    <GetAppointmentCollectionEndpoint as Endpoint>::SPEC,
    <CreateAppointmentEndpoint as Endpoint>::SPEC,
    <DeleteAppointmentEndpoint as Endpoint>::SPEC,
    <GetAppointmentItemEndpoint as Endpoint>::SPEC,
    <UpdateAppointmentEndpoint as Endpoint>::SPEC,
    <GetAppointmentAppointmentCrewCollectionEndpoint as Endpoint>::SPEC,
    <CreateAppointmentCrewEndpoint as Endpoint>::SPEC,
    <GetContactPersonCollectionEndpoint as Endpoint>::SPEC,
    <DeleteContactPersonEndpoint as Endpoint>::SPEC,
    <GetContactPersonItemEndpoint as Endpoint>::SPEC,
    <UpdateContactPersonEndpoint as Endpoint>::SPEC,
    <GetContactPersonFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetContactPersonFileCollectionEndpoint as Endpoint>::SPEC,
    <GetContactPersonTaskCollectionEndpoint as Endpoint>::SPEC,
    <CreateTaskEndpoint as Endpoint>::SPEC,
    <GetContactCollectionEndpoint as Endpoint>::SPEC,
    <CreateContactEndpoint as Endpoint>::SPEC,
    <DeleteContactEndpoint as Endpoint>::SPEC,
    <GetContactItemEndpoint as Endpoint>::SPEC,
    <UpdateContactEndpoint as Endpoint>::SPEC,
    <GetContactContactPersonCollectionEndpoint as Endpoint>::SPEC,
    <CreateContactPersonEndpoint as Endpoint>::SPEC,
    <GetContactFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetContactFileCollectionEndpoint as Endpoint>::SPEC,
    <GetContactTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostContactsIdTasksEndpoint as Endpoint>::SPEC,
    <GetContractCollectionEndpoint as Endpoint>::SPEC,
    <GetContractItemEndpoint as Endpoint>::SPEC,
    <GetContractFileCollectionEndpoint as Endpoint>::SPEC,
    <GetContractInvoiceLineCollectionEndpoint as Endpoint>::SPEC,
    <GetContractTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostContractsIdTasksEndpoint as Endpoint>::SPEC,
    <GetProjectCostCollectionEndpoint as Endpoint>::SPEC,
    <DeleteProjectCostEndpoint as Endpoint>::SPEC,
    <GetProjectCostItemEndpoint as Endpoint>::SPEC,
    <UpdateProjectCostEndpoint as Endpoint>::SPEC,
    <GetCrewCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewItemEndpoint as Endpoint>::SPEC,
    <GetCrewAppointmentCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewCrewAvailabilityCollectionEndpoint as Endpoint>::SPEC,
    <CreateCrewAvailabilityEndpoint as Endpoint>::SPEC,
    <GetCrewCrewRatesCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewFileCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewInvitationsCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostCrewIdTasksEndpoint as Endpoint>::SPEC,
    <GetCrewAvailabilityCollectionEndpoint as Endpoint>::SPEC,
    <DeleteCrewAvailabilityEndpoint as Endpoint>::SPEC,
    <GetCrewAvailabilityItemEndpoint as Endpoint>::SPEC,
    <UpdateCrewAvailabilityEndpoint as Endpoint>::SPEC,
    <GetCrewRatesCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewRatesItemEndpoint as Endpoint>::SPEC,
    <GetEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <CreateEquipmentEndpoint as Endpoint>::SPEC,
    <GetEquipmentItemEndpoint as Endpoint>::SPEC,
    <UpdateEquipmentEndpoint as Endpoint>::SPEC,
    <GetEquipmentAccessoryCollectionEndpoint as Endpoint>::SPEC,
    <CreateAccessoryEndpoint as Endpoint>::SPEC,
    <GetEquipmentAlternativeCollectionEndpoint as Endpoint>::SPEC,
    <CreateAlternativeEndpoint as Endpoint>::SPEC,
    <GetEquipmentEquipmentSetContentCollectionEndpoint as Endpoint>::SPEC,
    <CreateEquipmentSetContentEndpoint as Endpoint>::SPEC,
    <GetEquipmentFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetEquipmentFileCollectionEndpoint as Endpoint>::SPEC,
    <GetEquipmentRepairCollectionEndpoint as Endpoint>::SPEC,
    <GetEquipmentSerialNumberCollectionEndpoint as Endpoint>::SPEC,
    <CreateSerialNumberEndpoint as Endpoint>::SPEC,
    <GetEquipmentStockMovementCollectionEndpoint as Endpoint>::SPEC,
    <CreateStockMovementEndpoint as Endpoint>::SPEC,
    <GetEquipmentSupplierCollectionEndpoint as Endpoint>::SPEC,
    <CreateSupplierEndpoint as Endpoint>::SPEC,
    <GetEquipmentTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostEquipmentIdTasksEndpoint as Endpoint>::SPEC,
    <GetEquipmentAssignedSerialsCollectionEndpoint as Endpoint>::SPEC,
    <GetEquipmentAssignedSerialsItemEndpoint as Endpoint>::SPEC,
    <GetEquipmentSetContentCollectionEndpoint as Endpoint>::SPEC,
    <DeleteEquipmentSetContentEndpoint as Endpoint>::SPEC,
    <GetEquipmentSetContentItemEndpoint as Endpoint>::SPEC,
    <UpdateEquipmentSetContentEndpoint as Endpoint>::SPEC,
    <GetExtraInputFieldCollectionEndpoint as Endpoint>::SPEC,
    <GetExtraInputFieldItemEndpoint as Endpoint>::SPEC,
    <GetFactorGroupsCollectionEndpoint as Endpoint>::SPEC,
    <GetFactorGroupsItemEndpoint as Endpoint>::SPEC,
    <GetFactorGroupsFactorsCollectionEndpoint as Endpoint>::SPEC,
    <GetFactorsCollectionEndpoint as Endpoint>::SPEC,
    <GetFactorsItemEndpoint as Endpoint>::SPEC,
    <GetFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetFileFolderItemEndpoint as Endpoint>::SPEC,
    <GetFileCollectionEndpoint as Endpoint>::SPEC,
    <GetFileItemEndpoint as Endpoint>::SPEC,
    <GetFolderCollectionEndpoint as Endpoint>::SPEC,
    <CreateFolderEndpoint as Endpoint>::SPEC,
    <GetFolderItemEndpoint as Endpoint>::SPEC,
    <UpdateFolderEndpoint as Endpoint>::SPEC,
    <GetInvitationsCollectionEndpoint as Endpoint>::SPEC,
    <GetInvitationsItemEndpoint as Endpoint>::SPEC,
    <GetInvoiceLineCollectionEndpoint as Endpoint>::SPEC,
    <GetInvoiceLineItemEndpoint as Endpoint>::SPEC,
    <GetFactuurCollectionEndpoint as Endpoint>::SPEC,
    <GetFactuurItemEndpoint as Endpoint>::SPEC,
    <GetFactuurFileCollectionEndpoint as Endpoint>::SPEC,
    <GetFactuurInvoiceLineCollectionEndpoint as Endpoint>::SPEC,
    <GetFactuurPaymentCollectionEndpoint as Endpoint>::SPEC,
    <CreatePaymentEndpoint as Endpoint>::SPEC,
    <GetFactuurTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostInvoicesIdTasksEndpoint as Endpoint>::SPEC,
    <GetLeaveMutationsCollectionEndpoint as Endpoint>::SPEC,
    <CreateLeaveMutationsEndpoint as Endpoint>::SPEC,
    <GetLeaveMutationsItemEndpoint as Endpoint>::SPEC,
    <GetLeaveRequestCollectionEndpoint as Endpoint>::SPEC,
    <CreateLeaveRequestEndpoint as Endpoint>::SPEC,
    <GetLeaveRequestItemEndpoint as Endpoint>::SPEC,
    <UpdateLeaveRequestEndpoint as Endpoint>::SPEC,
    <GetLeaveRequestTimeRegistrationCollectionEndpoint as Endpoint>::SPEC,
    <CreateTimeRegistrationEndpoint as Endpoint>::SPEC,
    <GetLeaveTypesCollectionEndpoint as Endpoint>::SPEC,
    <GetLeaveTypesItemEndpoint as Endpoint>::SPEC,
    <GetLedgerCollectionEndpoint as Endpoint>::SPEC,
    <GetLedgerItemEndpoint as Endpoint>::SPEC,
    <GetPaymentCollectionEndpoint as Endpoint>::SPEC,
    <GetPaymentItemEndpoint as Endpoint>::SPEC,
    <UpdatePaymentEndpoint as Endpoint>::SPEC,
    <GetProjectCrewCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectCrewItemEndpoint as Endpoint>::SPEC,
    <GetProjectEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectEquipmentItemEndpoint as Endpoint>::SPEC,
    <GetProjectEquipmentGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectEquipmentGroupItemEndpoint as Endpoint>::SPEC,
    <GetProjectEquipmentGroupProjectEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionGroupItemEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionGroupProjectFunctionCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionItemEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionProjectCrewCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFunctionProjectVehicleCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectRequestEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <DeleteProjectRequestEquipmentEndpoint as Endpoint>::SPEC,
    <GetProjectRequestEquipmentItemEndpoint as Endpoint>::SPEC,
    <UpdateProjectRequestEquipmentEndpoint as Endpoint>::SPEC,
    <GetProjectRequestCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectRequestEndpoint as Endpoint>::SPEC,
    <DeleteProjectRequestEndpoint as Endpoint>::SPEC,
    <GetProjectRequestItemEndpoint as Endpoint>::SPEC,
    <UpdateProjectRequestEndpoint as Endpoint>::SPEC,
    <GetProjectRequestProjectRequestEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectRequestEquipmentEndpoint as Endpoint>::SPEC,
    <GetProjectCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectEndpoint as Endpoint>::SPEC,
    <GetProjectItemEndpoint as Endpoint>::SPEC,
    <GetProjectContractCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectCostCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectCostEndpoint as Endpoint>::SPEC,
    <GetProjectFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectFileCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectCrewCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectEquipmentGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectFunctionGroupCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectFunctionGroupEndpoint as Endpoint>::SPEC,
    <GetProjectProjectFunctionCollectionEndpoint as Endpoint>::SPEC,
    <CreateProjectFunctionEndpoint as Endpoint>::SPEC,
    <GetProjectProjectVehicleCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectQuotationCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectSubprojectCollectionEndpoint as Endpoint>::SPEC,
    <CreateSubprojectEndpoint as Endpoint>::SPEC,
    <GetProjectTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostProjectsIdTasksEndpoint as Endpoint>::SPEC,
    <GetProjectTypeCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectTypeItemEndpoint as Endpoint>::SPEC,
    <GetProjectVehicleCollectionEndpoint as Endpoint>::SPEC,
    <GetProjectVehicleItemEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderCostCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderCostItemEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderGlobalCostCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderGlobalCostItemEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderItemEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderFileCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderInvoiceLineCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderPurchaseOrderCostCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderPurchaseOrderGlobalCostCollectionEndpoint as Endpoint>::SPEC,
    <GetPurchaseOrderTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostPurchaseordersIdTasksEndpoint as Endpoint>::SPEC,
    <GetQuotationCollectionEndpoint as Endpoint>::SPEC,
    <GetQuotationItemEndpoint as Endpoint>::SPEC,
    <GetQuotationFileCollectionEndpoint as Endpoint>::SPEC,
    <GetQuotationInvoiceLineCollectionEndpoint as Endpoint>::SPEC,
    <GetQuotationTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostQuotesIdTasksEndpoint as Endpoint>::SPEC,
    <GetCrewRateFactorCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewRateFactorItemEndpoint as Endpoint>::SPEC,
    <GetCrewRateCollectionEndpoint as Endpoint>::SPEC,
    <GetCrewRateItemEndpoint as Endpoint>::SPEC,
    <GetCrewRateCrewRateFactorCollectionEndpoint as Endpoint>::SPEC,
    <GetRepairCollectionEndpoint as Endpoint>::SPEC,
    <GetRepairItemEndpoint as Endpoint>::SPEC,
    <GetRepairFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetRepairFileCollectionEndpoint as Endpoint>::SPEC,
    <GetRepairTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostRepairsIdTasksEndpoint as Endpoint>::SPEC,
    <GetSerialNumberCollectionEndpoint as Endpoint>::SPEC,
    <DeleteSerialNumberEndpoint as Endpoint>::SPEC,
    <GetSerialNumberItemEndpoint as Endpoint>::SPEC,
    <UpdateSerialNumberEndpoint as Endpoint>::SPEC,
    <GetSerialNumberActualContentCollectionEndpoint as Endpoint>::SPEC,
    <GetSerialNumberEquipmentAssignedSerialsCollectionEndpoint as Endpoint>::SPEC,
    <GetSerialNumberFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetSerialNumberFileCollectionEndpoint as Endpoint>::SPEC,
    <GetSerialNumberTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostSerialnumbersIdTasksEndpoint as Endpoint>::SPEC,
    <GetStatusCollectionEndpoint as Endpoint>::SPEC,
    <GetStatusItemEndpoint as Endpoint>::SPEC,
    <GetStockLocationCollectionEndpoint as Endpoint>::SPEC,
    <GetStockLocationItemEndpoint as Endpoint>::SPEC,
    <GetStockLocationVehicleCollectionEndpoint as Endpoint>::SPEC,
    <CreateVehicleEndpoint as Endpoint>::SPEC,
    <GetStockMovementCollectionEndpoint as Endpoint>::SPEC,
    <DeleteStockMovementEndpoint as Endpoint>::SPEC,
    <GetStockMovementItemEndpoint as Endpoint>::SPEC,
    <UpdateStockMovementEndpoint as Endpoint>::SPEC,
    <GetSubprojectCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectItemEndpoint as Endpoint>::SPEC,
    <GetSubprojectFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectProjectCrewCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectProjectEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectProjectEquipmentGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectProjectFunctionGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetSubprojectProjectVehicleCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalEquipmentItemEndpoint as Endpoint>::SPEC,
    <GetSubrentalEquipmentGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalEquipmentGroupItemEndpoint as Endpoint>::SPEC,
    <GetSubrentalEquipmentGroupSubrentalEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalItemEndpoint as Endpoint>::SPEC,
    <GetSubrentalFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalFileCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalSubrentalEquipmentCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalSubrentalEquipmentGroupCollectionEndpoint as Endpoint>::SPEC,
    <GetSubrentalTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostSubrentalsIdTasksEndpoint as Endpoint>::SPEC,
    <GetSubtaskCollectionEndpoint as Endpoint>::SPEC,
    <DeleteSubtaskEndpoint as Endpoint>::SPEC,
    <GetSubtaskItemEndpoint as Endpoint>::SPEC,
    <UpdateSubtaskEndpoint as Endpoint>::SPEC,
    <GetSupplierCollectionEndpoint as Endpoint>::SPEC,
    <DeleteSupplierEndpoint as Endpoint>::SPEC,
    <GetSupplierItemEndpoint as Endpoint>::SPEC,
    <UpdateSupplierEndpoint as Endpoint>::SPEC,
    <GetSupplierFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetSupplierFileCollectionEndpoint as Endpoint>::SPEC,
    <GetSupplierTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostSuppliersIdTasksEndpoint as Endpoint>::SPEC,
    <GetTaskAssignmentCollectionEndpoint as Endpoint>::SPEC,
    <DeleteTaskAssignmentEndpoint as Endpoint>::SPEC,
    <GetTaskAssignmentItemEndpoint as Endpoint>::SPEC,
    <UpdateTaskAssignmentEndpoint as Endpoint>::SPEC,
    <GetTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostTasksEndpoint as Endpoint>::SPEC,
    <DeleteTaskEndpoint as Endpoint>::SPEC,
    <GetTaskItemEndpoint as Endpoint>::SPEC,
    <UpdateTaskEndpoint as Endpoint>::SPEC,
    <GetTaskFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetTaskFileCollectionEndpoint as Endpoint>::SPEC,
    <GetTaskSubtaskCollectionEndpoint as Endpoint>::SPEC,
    <CreateSubtaskEndpoint as Endpoint>::SPEC,
    <GetTaskTaskAssignmentCollectionEndpoint as Endpoint>::SPEC,
    <CreateTaskAssignmentEndpoint as Endpoint>::SPEC,
    <GetTaskStatusCollectionEndpoint as Endpoint>::SPEC,
    <CreateTaskStatusEndpoint as Endpoint>::SPEC,
    <DeleteTaskStatusEndpoint as Endpoint>::SPEC,
    <GetTaskStatusItemEndpoint as Endpoint>::SPEC,
    <UpdateTaskStatusEndpoint as Endpoint>::SPEC,
    <GetTaxClassCollectionEndpoint as Endpoint>::SPEC,
    <GetTaxClassItemEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationCollectionEndpoint as Endpoint>::SPEC,
    <PostTimeregistrationEndpoint as Endpoint>::SPEC,
    <DeleteTimeRegistrationEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationItemEndpoint as Endpoint>::SPEC,
    <UpdateTimeRegistrationEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationFileCollectionEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationTimeRegistrationActivityCollectionEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationActivityCollectionEndpoint as Endpoint>::SPEC,
    <GetTimeRegistrationActivityItemEndpoint as Endpoint>::SPEC,
    <GetVehicleCollectionEndpoint as Endpoint>::SPEC,
    <PostVehiclesEndpoint as Endpoint>::SPEC,
    <DeleteVehicleEndpoint as Endpoint>::SPEC,
    <GetVehicleItemEndpoint as Endpoint>::SPEC,
    <UpdateVehicleEndpoint as Endpoint>::SPEC,
    <GetVehicleFileFolderCollectionEndpoint as Endpoint>::SPEC,
    <GetVehicleFileCollectionEndpoint as Endpoint>::SPEC,
    <GetVehicleTaskCollectionEndpoint as Endpoint>::SPEC,
    <PostVehiclesIdTasksEndpoint as Endpoint>::SPEC,
];

#[allow(dead_code)]
// Coverage is disabled because this compile-time assertion has no runtime
// behavior; contract tests cover the generated endpoint inventory.
#[cfg_attr(coverage_nightly, coverage(off))]
pub(crate) fn assert_all_endpoints_are_executable()
where
    GetAccessoryCollectionEndpoint: ExecutableEndpoint,
    DeleteAccessoryEndpoint: ExecutableEndpoint,
    GetAccessoryItemEndpoint: ExecutableEndpoint,
    UpdateAccessoryEndpoint: ExecutableEndpoint,
    GetActualContentCollectionEndpoint: ExecutableEndpoint,
    GetActualContentItemEndpoint: ExecutableEndpoint,
    GetAlternativeCollectionEndpoint: ExecutableEndpoint,
    DeleteAlternativeEndpoint: ExecutableEndpoint,
    GetAlternativeItemEndpoint: ExecutableEndpoint,
    UpdateAlternativeEndpoint: ExecutableEndpoint,
    GetAppointmentCrewCollectionEndpoint: ExecutableEndpoint,
    DeleteAppointmentCrewEndpoint: ExecutableEndpoint,
    GetAppointmentCrewItemEndpoint: ExecutableEndpoint,
    UpdateAppointmentCrewEndpoint: ExecutableEndpoint,
    GetAppointmentCollectionEndpoint: ExecutableEndpoint,
    CreateAppointmentEndpoint: ExecutableEndpoint,
    DeleteAppointmentEndpoint: ExecutableEndpoint,
    GetAppointmentItemEndpoint: ExecutableEndpoint,
    UpdateAppointmentEndpoint: ExecutableEndpoint,
    GetAppointmentAppointmentCrewCollectionEndpoint: ExecutableEndpoint,
    CreateAppointmentCrewEndpoint: ExecutableEndpoint,
    GetContactPersonCollectionEndpoint: ExecutableEndpoint,
    DeleteContactPersonEndpoint: ExecutableEndpoint,
    GetContactPersonItemEndpoint: ExecutableEndpoint,
    UpdateContactPersonEndpoint: ExecutableEndpoint,
    GetContactPersonFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetContactPersonFileCollectionEndpoint: ExecutableEndpoint,
    GetContactPersonTaskCollectionEndpoint: ExecutableEndpoint,
    CreateTaskEndpoint: ExecutableEndpoint,
    GetContactCollectionEndpoint: ExecutableEndpoint,
    CreateContactEndpoint: ExecutableEndpoint,
    DeleteContactEndpoint: ExecutableEndpoint,
    GetContactItemEndpoint: ExecutableEndpoint,
    UpdateContactEndpoint: ExecutableEndpoint,
    GetContactContactPersonCollectionEndpoint: ExecutableEndpoint,
    CreateContactPersonEndpoint: ExecutableEndpoint,
    GetContactFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetContactFileCollectionEndpoint: ExecutableEndpoint,
    GetContactTaskCollectionEndpoint: ExecutableEndpoint,
    PostContactsIdTasksEndpoint: ExecutableEndpoint,
    GetContractCollectionEndpoint: ExecutableEndpoint,
    GetContractItemEndpoint: ExecutableEndpoint,
    GetContractFileCollectionEndpoint: ExecutableEndpoint,
    GetContractInvoiceLineCollectionEndpoint: ExecutableEndpoint,
    GetContractTaskCollectionEndpoint: ExecutableEndpoint,
    PostContractsIdTasksEndpoint: ExecutableEndpoint,
    GetProjectCostCollectionEndpoint: ExecutableEndpoint,
    DeleteProjectCostEndpoint: ExecutableEndpoint,
    GetProjectCostItemEndpoint: ExecutableEndpoint,
    UpdateProjectCostEndpoint: ExecutableEndpoint,
    GetCrewCollectionEndpoint: ExecutableEndpoint,
    GetCrewItemEndpoint: ExecutableEndpoint,
    GetCrewAppointmentCollectionEndpoint: ExecutableEndpoint,
    GetCrewCrewAvailabilityCollectionEndpoint: ExecutableEndpoint,
    CreateCrewAvailabilityEndpoint: ExecutableEndpoint,
    GetCrewCrewRatesCollectionEndpoint: ExecutableEndpoint,
    GetCrewFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetCrewFileCollectionEndpoint: ExecutableEndpoint,
    GetCrewInvitationsCollectionEndpoint: ExecutableEndpoint,
    GetCrewTaskCollectionEndpoint: ExecutableEndpoint,
    PostCrewIdTasksEndpoint: ExecutableEndpoint,
    GetCrewAvailabilityCollectionEndpoint: ExecutableEndpoint,
    DeleteCrewAvailabilityEndpoint: ExecutableEndpoint,
    GetCrewAvailabilityItemEndpoint: ExecutableEndpoint,
    UpdateCrewAvailabilityEndpoint: ExecutableEndpoint,
    GetCrewRatesCollectionEndpoint: ExecutableEndpoint,
    GetCrewRatesItemEndpoint: ExecutableEndpoint,
    GetEquipmentCollectionEndpoint: ExecutableEndpoint,
    CreateEquipmentEndpoint: ExecutableEndpoint,
    GetEquipmentItemEndpoint: ExecutableEndpoint,
    UpdateEquipmentEndpoint: ExecutableEndpoint,
    GetEquipmentAccessoryCollectionEndpoint: ExecutableEndpoint,
    CreateAccessoryEndpoint: ExecutableEndpoint,
    GetEquipmentAlternativeCollectionEndpoint: ExecutableEndpoint,
    CreateAlternativeEndpoint: ExecutableEndpoint,
    GetEquipmentEquipmentSetContentCollectionEndpoint: ExecutableEndpoint,
    CreateEquipmentSetContentEndpoint: ExecutableEndpoint,
    GetEquipmentFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetEquipmentFileCollectionEndpoint: ExecutableEndpoint,
    GetEquipmentRepairCollectionEndpoint: ExecutableEndpoint,
    GetEquipmentSerialNumberCollectionEndpoint: ExecutableEndpoint,
    CreateSerialNumberEndpoint: ExecutableEndpoint,
    GetEquipmentStockMovementCollectionEndpoint: ExecutableEndpoint,
    CreateStockMovementEndpoint: ExecutableEndpoint,
    GetEquipmentSupplierCollectionEndpoint: ExecutableEndpoint,
    CreateSupplierEndpoint: ExecutableEndpoint,
    GetEquipmentTaskCollectionEndpoint: ExecutableEndpoint,
    PostEquipmentIdTasksEndpoint: ExecutableEndpoint,
    GetEquipmentAssignedSerialsCollectionEndpoint: ExecutableEndpoint,
    GetEquipmentAssignedSerialsItemEndpoint: ExecutableEndpoint,
    GetEquipmentSetContentCollectionEndpoint: ExecutableEndpoint,
    DeleteEquipmentSetContentEndpoint: ExecutableEndpoint,
    GetEquipmentSetContentItemEndpoint: ExecutableEndpoint,
    UpdateEquipmentSetContentEndpoint: ExecutableEndpoint,
    GetExtraInputFieldCollectionEndpoint: ExecutableEndpoint,
    GetExtraInputFieldItemEndpoint: ExecutableEndpoint,
    GetFactorGroupsCollectionEndpoint: ExecutableEndpoint,
    GetFactorGroupsItemEndpoint: ExecutableEndpoint,
    GetFactorGroupsFactorsCollectionEndpoint: ExecutableEndpoint,
    GetFactorsCollectionEndpoint: ExecutableEndpoint,
    GetFactorsItemEndpoint: ExecutableEndpoint,
    GetFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetFileFolderItemEndpoint: ExecutableEndpoint,
    GetFileCollectionEndpoint: ExecutableEndpoint,
    GetFileItemEndpoint: ExecutableEndpoint,
    GetFolderCollectionEndpoint: ExecutableEndpoint,
    CreateFolderEndpoint: ExecutableEndpoint,
    GetFolderItemEndpoint: ExecutableEndpoint,
    UpdateFolderEndpoint: ExecutableEndpoint,
    GetInvitationsCollectionEndpoint: ExecutableEndpoint,
    GetInvitationsItemEndpoint: ExecutableEndpoint,
    GetInvoiceLineCollectionEndpoint: ExecutableEndpoint,
    GetInvoiceLineItemEndpoint: ExecutableEndpoint,
    GetFactuurCollectionEndpoint: ExecutableEndpoint,
    GetFactuurItemEndpoint: ExecutableEndpoint,
    GetFactuurFileCollectionEndpoint: ExecutableEndpoint,
    GetFactuurInvoiceLineCollectionEndpoint: ExecutableEndpoint,
    GetFactuurPaymentCollectionEndpoint: ExecutableEndpoint,
    CreatePaymentEndpoint: ExecutableEndpoint,
    GetFactuurTaskCollectionEndpoint: ExecutableEndpoint,
    PostInvoicesIdTasksEndpoint: ExecutableEndpoint,
    GetLeaveMutationsCollectionEndpoint: ExecutableEndpoint,
    CreateLeaveMutationsEndpoint: ExecutableEndpoint,
    GetLeaveMutationsItemEndpoint: ExecutableEndpoint,
    GetLeaveRequestCollectionEndpoint: ExecutableEndpoint,
    CreateLeaveRequestEndpoint: ExecutableEndpoint,
    GetLeaveRequestItemEndpoint: ExecutableEndpoint,
    UpdateLeaveRequestEndpoint: ExecutableEndpoint,
    GetLeaveRequestTimeRegistrationCollectionEndpoint: ExecutableEndpoint,
    CreateTimeRegistrationEndpoint: ExecutableEndpoint,
    GetLeaveTypesCollectionEndpoint: ExecutableEndpoint,
    GetLeaveTypesItemEndpoint: ExecutableEndpoint,
    GetLedgerCollectionEndpoint: ExecutableEndpoint,
    GetLedgerItemEndpoint: ExecutableEndpoint,
    GetPaymentCollectionEndpoint: ExecutableEndpoint,
    GetPaymentItemEndpoint: ExecutableEndpoint,
    UpdatePaymentEndpoint: ExecutableEndpoint,
    GetProjectCrewCollectionEndpoint: ExecutableEndpoint,
    GetProjectCrewItemEndpoint: ExecutableEndpoint,
    GetProjectEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetProjectEquipmentItemEndpoint: ExecutableEndpoint,
    GetProjectEquipmentGroupCollectionEndpoint: ExecutableEndpoint,
    GetProjectEquipmentGroupItemEndpoint: ExecutableEndpoint,
    GetProjectEquipmentGroupProjectEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetProjectFunctionGroupCollectionEndpoint: ExecutableEndpoint,
    GetProjectFunctionGroupItemEndpoint: ExecutableEndpoint,
    GetProjectFunctionGroupProjectFunctionCollectionEndpoint: ExecutableEndpoint,
    GetProjectFunctionCollectionEndpoint: ExecutableEndpoint,
    GetProjectFunctionItemEndpoint: ExecutableEndpoint,
    GetProjectFunctionProjectCrewCollectionEndpoint: ExecutableEndpoint,
    GetProjectFunctionProjectVehicleCollectionEndpoint: ExecutableEndpoint,
    GetProjectRequestEquipmentCollectionEndpoint: ExecutableEndpoint,
    DeleteProjectRequestEquipmentEndpoint: ExecutableEndpoint,
    GetProjectRequestEquipmentItemEndpoint: ExecutableEndpoint,
    UpdateProjectRequestEquipmentEndpoint: ExecutableEndpoint,
    GetProjectRequestCollectionEndpoint: ExecutableEndpoint,
    CreateProjectRequestEndpoint: ExecutableEndpoint,
    DeleteProjectRequestEndpoint: ExecutableEndpoint,
    GetProjectRequestItemEndpoint: ExecutableEndpoint,
    UpdateProjectRequestEndpoint: ExecutableEndpoint,
    GetProjectRequestProjectRequestEquipmentCollectionEndpoint: ExecutableEndpoint,
    CreateProjectRequestEquipmentEndpoint: ExecutableEndpoint,
    GetProjectCollectionEndpoint: ExecutableEndpoint,
    CreateProjectEndpoint: ExecutableEndpoint,
    GetProjectItemEndpoint: ExecutableEndpoint,
    GetProjectContractCollectionEndpoint: ExecutableEndpoint,
    GetProjectProjectCostCollectionEndpoint: ExecutableEndpoint,
    CreateProjectCostEndpoint: ExecutableEndpoint,
    GetProjectFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetProjectFileCollectionEndpoint: ExecutableEndpoint,
    GetProjectProjectCrewCollectionEndpoint: ExecutableEndpoint,
    GetProjectProjectEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetProjectProjectEquipmentGroupCollectionEndpoint: ExecutableEndpoint,
    GetProjectProjectFunctionGroupCollectionEndpoint: ExecutableEndpoint,
    CreateProjectFunctionGroupEndpoint: ExecutableEndpoint,
    GetProjectProjectFunctionCollectionEndpoint: ExecutableEndpoint,
    CreateProjectFunctionEndpoint: ExecutableEndpoint,
    GetProjectProjectVehicleCollectionEndpoint: ExecutableEndpoint,
    GetProjectQuotationCollectionEndpoint: ExecutableEndpoint,
    GetProjectSubprojectCollectionEndpoint: ExecutableEndpoint,
    CreateSubprojectEndpoint: ExecutableEndpoint,
    GetProjectTaskCollectionEndpoint: ExecutableEndpoint,
    PostProjectsIdTasksEndpoint: ExecutableEndpoint,
    GetProjectTypeCollectionEndpoint: ExecutableEndpoint,
    GetProjectTypeItemEndpoint: ExecutableEndpoint,
    GetProjectVehicleCollectionEndpoint: ExecutableEndpoint,
    GetProjectVehicleItemEndpoint: ExecutableEndpoint,
    GetPurchaseOrderCostCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderCostItemEndpoint: ExecutableEndpoint,
    GetPurchaseOrderGlobalCostCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderGlobalCostItemEndpoint: ExecutableEndpoint,
    GetPurchaseOrderCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderItemEndpoint: ExecutableEndpoint,
    GetPurchaseOrderFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderFileCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderInvoiceLineCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderPurchaseOrderCostCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderPurchaseOrderGlobalCostCollectionEndpoint: ExecutableEndpoint,
    GetPurchaseOrderTaskCollectionEndpoint: ExecutableEndpoint,
    PostPurchaseordersIdTasksEndpoint: ExecutableEndpoint,
    GetQuotationCollectionEndpoint: ExecutableEndpoint,
    GetQuotationItemEndpoint: ExecutableEndpoint,
    GetQuotationFileCollectionEndpoint: ExecutableEndpoint,
    GetQuotationInvoiceLineCollectionEndpoint: ExecutableEndpoint,
    GetQuotationTaskCollectionEndpoint: ExecutableEndpoint,
    PostQuotesIdTasksEndpoint: ExecutableEndpoint,
    GetCrewRateFactorCollectionEndpoint: ExecutableEndpoint,
    GetCrewRateFactorItemEndpoint: ExecutableEndpoint,
    GetCrewRateCollectionEndpoint: ExecutableEndpoint,
    GetCrewRateItemEndpoint: ExecutableEndpoint,
    GetCrewRateCrewRateFactorCollectionEndpoint: ExecutableEndpoint,
    GetRepairCollectionEndpoint: ExecutableEndpoint,
    GetRepairItemEndpoint: ExecutableEndpoint,
    GetRepairFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetRepairFileCollectionEndpoint: ExecutableEndpoint,
    GetRepairTaskCollectionEndpoint: ExecutableEndpoint,
    PostRepairsIdTasksEndpoint: ExecutableEndpoint,
    GetSerialNumberCollectionEndpoint: ExecutableEndpoint,
    DeleteSerialNumberEndpoint: ExecutableEndpoint,
    GetSerialNumberItemEndpoint: ExecutableEndpoint,
    UpdateSerialNumberEndpoint: ExecutableEndpoint,
    GetSerialNumberActualContentCollectionEndpoint: ExecutableEndpoint,
    GetSerialNumberEquipmentAssignedSerialsCollectionEndpoint: ExecutableEndpoint,
    GetSerialNumberFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetSerialNumberFileCollectionEndpoint: ExecutableEndpoint,
    GetSerialNumberTaskCollectionEndpoint: ExecutableEndpoint,
    PostSerialnumbersIdTasksEndpoint: ExecutableEndpoint,
    GetStatusCollectionEndpoint: ExecutableEndpoint,
    GetStatusItemEndpoint: ExecutableEndpoint,
    GetStockLocationCollectionEndpoint: ExecutableEndpoint,
    GetStockLocationItemEndpoint: ExecutableEndpoint,
    GetStockLocationVehicleCollectionEndpoint: ExecutableEndpoint,
    CreateVehicleEndpoint: ExecutableEndpoint,
    GetStockMovementCollectionEndpoint: ExecutableEndpoint,
    DeleteStockMovementEndpoint: ExecutableEndpoint,
    GetStockMovementItemEndpoint: ExecutableEndpoint,
    UpdateStockMovementEndpoint: ExecutableEndpoint,
    GetSubprojectCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectItemEndpoint: ExecutableEndpoint,
    GetSubprojectFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectProjectCrewCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectProjectEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectProjectEquipmentGroupCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectProjectFunctionGroupCollectionEndpoint: ExecutableEndpoint,
    GetSubprojectProjectVehicleCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalEquipmentItemEndpoint: ExecutableEndpoint,
    GetSubrentalEquipmentGroupCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalEquipmentGroupItemEndpoint: ExecutableEndpoint,
    GetSubrentalEquipmentGroupSubrentalEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalItemEndpoint: ExecutableEndpoint,
    GetSubrentalFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalFileCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalSubrentalEquipmentCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalSubrentalEquipmentGroupCollectionEndpoint: ExecutableEndpoint,
    GetSubrentalTaskCollectionEndpoint: ExecutableEndpoint,
    PostSubrentalsIdTasksEndpoint: ExecutableEndpoint,
    GetSubtaskCollectionEndpoint: ExecutableEndpoint,
    DeleteSubtaskEndpoint: ExecutableEndpoint,
    GetSubtaskItemEndpoint: ExecutableEndpoint,
    UpdateSubtaskEndpoint: ExecutableEndpoint,
    GetSupplierCollectionEndpoint: ExecutableEndpoint,
    DeleteSupplierEndpoint: ExecutableEndpoint,
    GetSupplierItemEndpoint: ExecutableEndpoint,
    UpdateSupplierEndpoint: ExecutableEndpoint,
    GetSupplierFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetSupplierFileCollectionEndpoint: ExecutableEndpoint,
    GetSupplierTaskCollectionEndpoint: ExecutableEndpoint,
    PostSuppliersIdTasksEndpoint: ExecutableEndpoint,
    GetTaskAssignmentCollectionEndpoint: ExecutableEndpoint,
    DeleteTaskAssignmentEndpoint: ExecutableEndpoint,
    GetTaskAssignmentItemEndpoint: ExecutableEndpoint,
    UpdateTaskAssignmentEndpoint: ExecutableEndpoint,
    GetTaskCollectionEndpoint: ExecutableEndpoint,
    PostTasksEndpoint: ExecutableEndpoint,
    DeleteTaskEndpoint: ExecutableEndpoint,
    GetTaskItemEndpoint: ExecutableEndpoint,
    UpdateTaskEndpoint: ExecutableEndpoint,
    GetTaskFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetTaskFileCollectionEndpoint: ExecutableEndpoint,
    GetTaskSubtaskCollectionEndpoint: ExecutableEndpoint,
    CreateSubtaskEndpoint: ExecutableEndpoint,
    GetTaskTaskAssignmentCollectionEndpoint: ExecutableEndpoint,
    CreateTaskAssignmentEndpoint: ExecutableEndpoint,
    GetTaskStatusCollectionEndpoint: ExecutableEndpoint,
    CreateTaskStatusEndpoint: ExecutableEndpoint,
    DeleteTaskStatusEndpoint: ExecutableEndpoint,
    GetTaskStatusItemEndpoint: ExecutableEndpoint,
    UpdateTaskStatusEndpoint: ExecutableEndpoint,
    GetTaxClassCollectionEndpoint: ExecutableEndpoint,
    GetTaxClassItemEndpoint: ExecutableEndpoint,
    GetTimeRegistrationCollectionEndpoint: ExecutableEndpoint,
    PostTimeregistrationEndpoint: ExecutableEndpoint,
    DeleteTimeRegistrationEndpoint: ExecutableEndpoint,
    GetTimeRegistrationItemEndpoint: ExecutableEndpoint,
    UpdateTimeRegistrationEndpoint: ExecutableEndpoint,
    GetTimeRegistrationFileCollectionEndpoint: ExecutableEndpoint,
    GetTimeRegistrationTimeRegistrationActivityCollectionEndpoint: ExecutableEndpoint,
    GetTimeRegistrationActivityCollectionEndpoint: ExecutableEndpoint,
    GetTimeRegistrationActivityItemEndpoint: ExecutableEndpoint,
    GetVehicleCollectionEndpoint: ExecutableEndpoint,
    PostVehiclesEndpoint: ExecutableEndpoint,
    DeleteVehicleEndpoint: ExecutableEndpoint,
    GetVehicleItemEndpoint: ExecutableEndpoint,
    UpdateVehicleEndpoint: ExecutableEndpoint,
    GetVehicleFileFolderCollectionEndpoint: ExecutableEndpoint,
    GetVehicleFileCollectionEndpoint: ExecutableEndpoint,
    GetVehicleTaskCollectionEndpoint: ExecutableEndpoint,
    PostVehiclesIdTasksEndpoint: ExecutableEndpoint,
{
}
