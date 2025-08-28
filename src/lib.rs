use std::{sync::Arc, vec};

async fn books_list(
    state: Arc<AppState>,
    input: proto::Cursor,
    headers: proto::Authorization,
) -> Result<proto::Items<model::Book>, proto::BooksListError> {
    if headers.authorization != "password" {
        return Err(proto::BooksListError::Unauthorized);
    }

    let max_limit_allowed = 100;
    if input.limit.unwrap_or(0) > max_limit_allowed {
        return Err(proto::BooksListError::LimitExceeded {
            requested: input.limit.unwrap_or(0),
            allowed: max_limit_allowed,
        });
    }

    Ok(proto::Items {
        items: state.books.clone(),
        pagination: proto::Pagination {
            next_cursor: None,
            prev_cursor: None,
        },
    })
}

pub fn builder() -> reflectapi::Builder<Arc<AppState>> {
    reflectapi::Builder::new()
        .route(books_list, |b| {
            b.name("books.list").description("List all books")
        })
        .rename_types("reflectapi_demo::", "booksapi::")
}

#[derive(Debug)]
pub struct AppState {
    pub books: Vec<model::Book>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            books: vec![model::Book {
                id: uuid::Uuid::new_v4(),
                isbn: "978-3-16-148410-0".into(),
                title: "Default Book".into(),
                author: "John Doe".into(),
                genre: model::BookGenre::Fiction,
                release_year: Some(2020),
                tags: vec!["default".into()],
            }],
        }
    }
}

mod model {
    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, reflectapi::Input, reflectapi::Output,
    )]
    pub struct Book {
        /// Database identity
        pub id: uuid::Uuid,
        /// ISBN - identity
        pub isbn: String,
        /// Title
        pub title: String,
        /// Book author, full name
        pub author: String,
        /// Genre
        pub genre: BookGenre,
        /// age of the pet
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[deprecated(note = "demo deprecation")]
        pub release_year: Option<u16>,
        /// behaviors of the pet
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
    }

    #[derive(
        Debug, Clone, serde::Serialize, serde::Deserialize, reflectapi::Input, reflectapi::Output,
    )]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum BookGenre {
        Fiction,
        Science { subject: String },
        Biography,
        Other { description: String },
    }
}

pub mod proto {
    #[derive(serde::Deserialize, reflectapi::Input)]
    pub struct Authorization {
        pub authorization: String,
    }

    #[derive(serde::Deserialize, reflectapi::Input)]
    pub struct Cursor {
        #[serde(default)]
        pub cursor: Option<String>,
        #[serde(default)]
        pub limit: Option<u32>,
    }

    #[derive(serde::Serialize, reflectapi::Output)]
    pub struct Items<T> {
        pub items: Vec<T>,
        pub pagination: Pagination,
    }

    #[derive(serde::Serialize, reflectapi::Output)]
    pub struct Pagination {
        pub next_cursor: Option<String>,
        pub prev_cursor: Option<String>,
    }

    #[derive(serde::Serialize, reflectapi::Output)]
    pub enum BooksListError {
        Unauthorized,
        LimitExceeded { requested: u32, allowed: u32 },
    }

    impl reflectapi::StatusCode for BooksListError {
        fn status_code(&self) -> http::StatusCode {
            match self {
                BooksListError::Unauthorized => http::StatusCode::UNAUTHORIZED,
                BooksListError::LimitExceeded { .. } => http::StatusCode::UNPROCESSABLE_ENTITY,
            }
        }
    }
}
