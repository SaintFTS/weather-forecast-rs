use super::structs::ErrorCodes;

pub async fn get_class_from_response<T: serde::de::DeserializeOwned>(link: &str) -> Result<T, ErrorCodes> {
    let body = reqwest::get(link)
        .await
        .map_err(|e| ErrorCodes::NoResponse(e.to_string()))?
        .text()
        .await
        .map_err(|e| ErrorCodes::FailedRead(e.to_string()))?;

    serde_json::from_str::<T>(&body)
        .map_err(|e| ErrorCodes::FailedRead(e.to_string()))
}

