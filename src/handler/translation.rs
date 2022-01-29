use serde::Deserialize;
use url::Url;

pub struct Translator {
    secret: String,
}

impl Translator {
    pub fn new(auth_key: &str) -> Translator {
        Self {
            secret: auth_key.to_string(),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String, String> {
        let authorization_header_value = format!("DeepL-Auth-Key {}", self.secret);
        let end_point = Url::parse_with_params(
            "https://api-free.deepl.com/v2/translate",
            &[("text", text), ("target_lang", "JA")],
        )
        .map_err(|e| e.to_string())?;

        Ok("".into())
        // let mut init = RequestInit::new();
        // init.with_method(Method::Get);
        // let mut request =
        //     Request::new_with_init(end_point.as_str(), &init).map_err(|e| e.to_string())?;

        // // let mut request =
        // //     Request::new(end_point.as_str(), Method::Get).map_err(|e| e.to_string())?;
        // let headers = request.headers_mut().unwrap();
        // headers
        //     .set("Authorization", &authorization_header_value)
        //     .map_err(|e| e.to_string())?;
        // headers
        //     .set("host", "ranwei.dev")
        //     .map_err(|e| e.to_string())?;

        // let mut response = Fetch::Request(request)
        //     .send()
        //     .await
        //     .map_err(|e| e.to_string())?;

        // let byte = response.bytes().await.map_err(|e| e.to_string())?;

        // let translation_response = serde_json::from_slice::<TranslationResponse>(byte.as_ref())
        //     .map_err(|e| e.to_string())?;

        // Ok(translation_response.translations[0].text.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TranslationResponse {
    pub translations: Vec<Translation>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}
