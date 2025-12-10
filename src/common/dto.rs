use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            status: 200,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn with_status(status: u16, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            status,
            message: message.into(),
            data,
        }
    }
}

pub struct RestApiResponse<T>(pub ApiResponse<T>);

impl<T: Serialize> IntoResponse for RestApiResponse<T> {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.0.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, Json(self.0)).into_response()
    }
}

impl<T> From<ApiResponse<T>> for RestApiResponse<T> {
    fn from(response: ApiResponse<T>) -> Self {
        RestApiResponse(response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        #[derive(Serialize)]
        struct TestData {
            id: i32,
            name: String,
        }

        let data = TestData {
            id: 1,
            name: "test".to_string(),
        };

        let response = ApiResponse::success(data);

        assert_eq!(response.status, 200);
        assert_eq!(response.message, "success");
        assert!(response.data.is_some());
    }

    #[test]
    fn test_response_serialization() {
        let response = ApiResponse::success("test data");
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"status\":200"));
        assert!(json.contains("\"message\":\"success\""));
        assert!(json.contains("\"data\":\"test data\""));
    }
}
