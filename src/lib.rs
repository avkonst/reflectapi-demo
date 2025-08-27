
use std::sync::Arc;

async fn pets_create(
    _state: Arc<AppState>,
    _request: proto::PetsCreateRequest,
    _headers: proto::Headers,
) -> Result<reflectapi::Empty, proto::PetsCreateError> {
    todo!("not implemented")
}

pub fn builder() -> reflectapi::Builder<Arc<AppState>> {
    reflectapi::Builder::new()
        .route(pets_create, |b| {
            b.name("pets.create").description("Create a new pet")
        })
        .rename_types("reflectapi_demo::", "myapi::")
}

#[derive(Debug)]
pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

mod model {
    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, reflectapi::Input, reflectapi::Output,
    )]
    pub struct Pet {
        /// identity
        pub name: String,
        /// kind of pet
        pub kind: Kind,
        /// age of the pet
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[deprecated(note = "test deprecation")]
        pub age: Option<u8>,
        /// behaviors of the pet
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub behaviors: Vec<Behavior>,
    }

    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, reflectapi::Input, reflectapi::Output,
    )]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum Kind {
        /// A dog
        Dog {
            /// breed of the dog
            breed: String,
        },
        /// A cat
        Cat {
            /// lives left
            lives: u8,
        },
    }

    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, reflectapi::Input, reflectapi::Output,
    )]
    pub enum Behavior {
        Calm,
        Aggressive(/** aggressiveness level */ f64, /** some notes */ String),
        Other {
            /// Custom provided description of a behavior
            description: String,
            /// Additional notes
            /// Up to a user to put free text here
            #[serde(default, skip_serializing_if = "String::is_empty")]
            notes: String,
        },
    }
}

pub mod proto {
    #[derive(serde::Deserialize, reflectapi::Input)]
    pub struct Headers {
        pub authorization: String,
    }

    #[derive(serde::Deserialize, reflectapi::Input)]
    pub struct PetsCreateRequest(pub crate::model::Pet);

    #[derive(serde::Serialize, reflectapi::Output)]
    pub enum PetsCreateError {
        Conflict,
        NotAuthorized,
        InvalidIdentityCharacter { char: char },
    }

    impl reflectapi::StatusCode for PetsCreateError {
        fn status_code(&self) -> http::StatusCode {
            match self {
                PetsCreateError::Conflict => http::StatusCode::CONFLICT,
                PetsCreateError::NotAuthorized => http::StatusCode::UNAUTHORIZED,
                PetsCreateError::InvalidIdentityCharacter { .. } => http::StatusCode::UNPROCESSABLE_ENTITY,
            }
        }
    }
}
