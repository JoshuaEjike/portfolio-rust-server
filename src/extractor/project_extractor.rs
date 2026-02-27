use axum::{
    Json,
    extract::{FromRequest, Request},
};

use crate::{
    core::{image_core::base64_image_uploader_core, stack_identifier_core::ensure_stack_exists},
    error::api_error::ApiErrors,
    fields::Text,
    payload_description::project_payload_description::UpdateProjectRequest,
    payload_handler::project_payload_handler::ProjectCreateRequest,
    state::AppState,
};

pub struct ProjectCreateInput {
    pub title: String,
    pub description: String,
    pub stack: String,
    pub content: String,
    pub image: String,
    pub image_id: String,
}

impl FromRequest<AppState> for ProjectCreateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<ProjectCreateRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let payload_data = payload.validate()?;

        ensure_stack_exists(payload_data.stack.clone(), state.stack_services.clone()).await?;

        let image = base64_image_uploader_core(
            payload_data.image,
            payload_data.image_name,
            state.image_service.clone(),
        )
        .await?;

        Ok(ProjectCreateInput {
            title: payload_data.title,
            description: payload_data.description,
            stack: payload_data.stack,
            content: payload_data.content,
            image: image.secure_url,
            image_id: image.public_id,
        })
    }
}

pub struct ProjectUpateInput {
    pub description: Option<Text>,
    pub stack: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
}

impl FromRequest<AppState> for ProjectUpateInput {
    type Rejection = ApiErrors;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<UpdateProjectRequest>::from_request(req, state)
            .await
            .map_err(|_| ApiErrors::BadRequest("Invalid request body".into()))?;

        let description = payload.description.as_deref().map(Text::new).transpose()?;

        let stack = payload.stack.as_deref().map(Text::new).transpose()?;

        if let Some(title) = payload.stack {
            ensure_stack_exists(title, state.stack_services.clone()).await?;
        }

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

        Ok(ProjectUpateInput {
            description,
            stack,
            content: payload.content,
            image,
            image_id,
        })
    }
}
