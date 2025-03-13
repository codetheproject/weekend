macro_rules! create_status_code {
    ($($ident:ident),*) => {
        $(
            pub const $ident: axum::http::StatusCode = axum::http::StatusCode::$ident;
        )*
    };
}

create_status_code!(CONTINUE, SWITCHING_PROTOCOLS, PROCESSING);
create_status_code!(
    OK, CREATED, ACCEPTED, NON_AUTHORITATIVE_INFORMATION, NO_CONTENT, RESET_CONTENT, PARTIAL_CONTENT, MULTI_STATUS,
    ALREADY_REPORTED, IM_USED
);
create_status_code!(
    MULTIPLE_CHOICES, MOVED_PERMANENTLY, FOUND, SEE_OTHER, NOT_MODIFIED, USE_PROXY, TEMPORARY_REDIRECT,
    PERMANENT_REDIRECT
);
create_status_code!(
    BAD_REQUEST, UNAUTHORIZED, PAYMENT_REQUIRED, FORBIDDEN, NOT_FOUND, METHOD_NOT_ALLOWED, NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED, REQUEST_TIMEOUT, CONFLICT, GONE, LENGTH_REQUIRED, PRECONDITION_FAILED,
    PAYLOAD_TOO_LARGE, URI_TOO_LONG, UNSUPPORTED_MEDIA_TYPE, RANGE_NOT_SATISFIABLE, EXPECTATION_FAILED, IM_A_TEAPOT,
    MISDIRECTED_REQUEST, UNPROCESSABLE_ENTITY, LOCKED, FAILED_DEPENDENCY, UPGRADE_REQUIRED, PRECONDITION_REQUIRED,
    TOO_MANY_REQUESTS, REQUEST_HEADER_FIELDS_TOO_LARGE, UNAVAILABLE_FOR_LEGAL_REASONS
);
create_status_code!(
    INTERNAL_SERVER_ERROR, NOT_IMPLEMENTED, BAD_GATEWAY, SERVICE_UNAVAILABLE, GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED, VARIANT_ALSO_NEGOTIATES, INSUFFICIENT_STORAGE, LOOP_DETECTED, NOT_EXTENDED,
    NETWORK_AUTHENTICATION_REQUIRED
);
/*
TODO

Improve body wrapper by recursively adding error!

*/
// Represents a request body wrapper
// #[derive(Deserialize)]
// pub struct Body<T>(pub T);

// /// Type alias for storing validation errors
// type Store = HashMap<Cow<'static, str>, Vec<(Cow<'static, str>, Cow<'static, str>)>>;

// /// Represents an error response with optional validation messages
// #[derive(new, Debug, Serialize)]
// pub struct Error {
//     reason: Cow<'static, str>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     messages: Option<Store>,
// }

// impl<S, T> FromRequest<S> for Body<T>
// where
//     S: Send + Sync,
//     T: Send + Sync + DeserializeOwned + Validate,
// {
//     type Rejection = (StatusCode, Json<Error>);

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         // Parse JSON body
//         let Json(Body(body)) = Json::<Body<T>>::from_request(req, state)
//             .await
//             .map_err(|_| {
//                 (BAD_REQUEST, Json(Error::new(Cow::Borrowed("Failed to parse the body into valid JSON"), None)))
//             })?;

//         // Validate the body
//         if let Err(err) = body.validate() {
//             let mut store = HashMap::with_capacity(err.field_errors().len());
//             make_error(None, &err, &mut store);

//             return Err((
//                 BAD_REQUEST,
//                 Json(Error::new(Cow::Borrowed("Invalid payload data"), (!store.is_empty()).then_some(store))),
//             ));
//         }

//         Ok(Body(body))
//     }
// }

// Process validation errors recursively and store them in the error store
/*

i want something like this
{
    'field': {
        'field_name': [errors]
    },

}
*/
// pub fn make_error(key: Option<&str>, err: &ValidationErrors, store: &mut Store) {
//     if err.is_empty() {
//         return;
//     };

//     use validator::ValidationErrorsKind::*;

//     fn compute_err(err: &ValidationError, column_key: &str, store_key: &str, store: &mut Store) {
//         if let Some(ref message) = err.message {
//             if let Some(message_store) = store.get_mut(store_key) {
//                 message_store.push((Cow::Owned(column_key.to_string()), message.clone()));
//                 return;
//             }
//             // Key didn't exist create the vec and populate it
//             store
//                 .insert(Cow::Owned(store_key.to_string()), vec![(Cow::Owned(column_key.to_string()), message.clone())]);
//         }
//     }

//     for (column_key, value) in err.0.iter() {
//         match value {
//             Field(fields) => {
//                 for field in fields {
//                     compute_err(field, column_key, key.unwrap_or(&column_key), store)
//                 }
//             }
//             Struct(errors) => make_error(Some(&column_key), &**errors, store),
//             List(errors) => {
//                 for (_, error) in errors {
//                     make_error(Some(&column_key), &**error, store);
//                 }
//             }
//         }
//     }
// }
