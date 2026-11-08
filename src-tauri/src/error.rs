use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The supplied setting is not supported.")]
    Validation,
    #[error("Ascend could not prepare its local data directory.")]
    DataDirectory,
    #[error("Ascend could not save settings locally. Please try again.")]
    Storage,
}

impl AppError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Validation => "validation_error",
            Self::DataDirectory => "data_directory_error",
            Self::Storage => "storage_error",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct ErrorDto<'a> {
            code: &'a str,
            message: String,
        }

        ErrorDto {
            code: self.code(),
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}
