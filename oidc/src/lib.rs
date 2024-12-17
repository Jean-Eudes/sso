use std::cmp::Reverse;

enum ResponseType {
    Code,
    IdToken,
    Token,
}

enum Display {
    Page,
    Popup,
    Touch,
    Wap,
}

enum Prompt {
    None,
    Login,
    Consent,
    SelectAccount,
}

struct AuthenticationRequest {
    scope: Vec<String>,
    client_id: String,
    response_type: Vec<ResponseType>,
    redirect_uri: String,
    state: Option<String>,
    response_mode: Option<String>,
    nonce: Option<String>,
    display: Display,
    prompt: Prompt,
    max_age: Option<u64>,
    ui_locales: Vec<String>,
    id_token_hint: Option<String>,
    login_hint: Option<String>,
    acr_values: Vec<String>,
}

enum ErrorCode {
    InteractionRequired,
    LoginRequired,
    AccountSelectionRequired,
    ConsentRequired,
    InvalidRequestUri,
    InvalidRequestObject,
    RequestNotSupported,
    RequestUriNotSupported,
    RegistrationNotSupported,

}
struct AuthenticationErrorResponse {
    error: ErrorCode,
    error_description: Option<String>,
    error_uri: Option<String>,
    state: Option<String>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
