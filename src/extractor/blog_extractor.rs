use axum::{
    Json,
    extract::{FromRequest, Request},
};

use crate::{
    core::image_core::base64_image_uploader_core, error::api_error::ApiErrors, fields::Text,
    payload_description::blog_payload_description::UpdateBlogRequest,
    payload_handler::blog_payload_handler::BlogCreateRequest, state::AppState,
};

pub struct BlogCreateInput {
    pub title: String,
    pub description: String,
    pub content: String,
    pub image: String,
    pub image_id: String,
}

impl FromRequest<AppState> for BlogCreateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<BlogCreateRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let payload_data = payload.validate()?;

        let image = base64_image_uploader_core(
            payload_data.image,
            payload_data.image_name,
            state.image_service.clone(),
        )
        .await?;

        Ok(BlogCreateInput {
            title: payload_data.title,
            description: payload_data.description,
            content: payload_data.content,
            image: image.secure_url,
            image_id: image.public_id,
        })
    }
}

pub struct BlogUpateInput {
    pub description: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
}

impl FromRequest<AppState> for BlogUpateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<UpdateBlogRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let description = payload.description.as_deref().map(Text::new).transpose()?;

        let image_data = if let (Some(base64), Some(name)) =
            (payload.image.as_ref(), payload.image_name.as_ref())
        {
            Some(
                base64_image_uploader_core(
                    base64.clone(),
                    name.clone(),
                    state.image_service.clone(),
                )
                .await?,
            )
        } else {
            None
        };

        let (image, image_id) = match image_data {
            Some(upload) => (Some(upload.secure_url), Some(upload.public_id)),
            None => (None, None),
        };

        Ok(BlogUpateInput {
            description,
            content: payload.content,
            image,
            image_id,
        })
    }
}
