// models/file_form_model.rs

use std::{fmt, path::PathBuf};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DbBackend {
    #[default]
    Sqlite,
    MySql,
    Db2,
    PostgreSql,
    MariaDb,
    MsSql,
    Redis,
    Aws,
    Azure,
    GoogleCloud,
    Apache,
}

impl DbBackend {
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "SQLite" => Some(Self::Sqlite),
            "MySQL" => Some(Self::MySql),
            "DB2" => Some(Self::Db2),
            "Postgresql" => Some(Self::PostgreSql),
            "MariaDB" => Some(Self::MariaDb),
            "MSSQL" => Some(Self::MsSql),
            "Redis" => Some(Self::Redis),
            "AWS" => Some(Self::Aws),
            "Azure" => Some(Self::Azure),
            "Google Cloud" => Some(Self::GoogleCloud),
            "Apache" => Some(Self::Apache),
            _ => None,
        }
    }
}

impl fmt::Display for DbBackend {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let label = match self {
            Self::Sqlite => "SQLite",
            Self::MySql => "MySQL",
            Self::Db2 => "DB2",
            Self::PostgreSql => "Postgresql",
            Self::MariaDb => "MariaDB",
            Self::MsSql => "MSSQL",
            Self::Redis => "Redis",
            Self::Aws => "AWS",
            Self::Azure => "Azure",
            Self::GoogleCloud => "Google Cloud",
            Self::Apache => "Apache",
        };
        write!(f, "{label}")
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "ERROR" => Some(Self::Error),
            "WARN" => Some(Self::Warn),
            "INFO" => Some(Self::Info),
            "DEBUG" => Some(Self::Debug),
            "TRACE" => Some(Self::Trace),
            _ => None,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let label = match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        };
        write!(f, "{label}")
    }
}

/// Represents the collected values from the file selection form.
#[derive(Clone, Debug, Default)]
pub struct FileFormModel {
    pub source_file: PathBuf,
    pub database_file: PathBuf,
    pub log_directory: PathBuf,
    pub db_backend: DbBackend,
    pub log_level: LogLevel,
    pub selected_sheet: Option<String>,
    pub log_stdout: bool,
    pub has_headers: bool,
}

impl FileFormModel {
    /// Returns `true` if the source file has an Excel extension.
    pub fn is_excel(&self) -> bool {
        matches!(
            self.source_file
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_lowercase())
                .as_deref(),
            Some("xlsx" | "xlsm" | "xlsb" | "xls")
        )
    }

    /// Returns `true` if the source file has an CSV extension.
    pub fn is_csv(&self) -> bool {
        matches!(
            self.source_file
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_lowercase())
                .as_deref(),
            Some("csv")
        )
    }

    /// Returns `true` if the database file has a SQLite extension.
    pub fn is_sqlite(&self) -> bool {
        matches!(
            self.database_file
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_lowercase())
                .as_deref(),
            Some("db" | "db3" | "sqlite")
        )
    }

    /// Validates that the model has all required values for submission.
    ///
    /// Rules:
    /// - source file is required
    /// - database file is required
    /// - selected sheet is required only for Excel source files
    pub fn validate_for_submit(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.source_file.as_os_str().is_empty() {
            errors.push("Source file is required.".to_string());
        }

        if self.database_file.as_os_str().is_empty() {
            errors.push("Database file is required.".to_string());
        }

        if self.is_excel()
            && self
                .selected_sheet
                .as_deref()
                .map(str::trim)
                .filter(|sheet| !sheet.is_empty())
                .is_none()
        {
            errors.push("Sheet selection is required for Excel sources.".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl fmt::Display for FileFormModel {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(f, "Source file:   {}", self.source_file.to_string_lossy())?;
        writeln!(f, "Database:      {}", self.database_file.to_string_lossy())?;
        writeln!(f, "Log folder:    {}", self.log_directory.to_string_lossy())?;
        writeln!(f, "DB Backend:    {}", self.db_backend)?;
        writeln!(f, "Log Level:     {}", self.log_level)?;
        writeln!(
            f,
            "Sheet:         {}",
            self.selected_sheet.as_deref().unwrap_or("(none)")
        )?;
        writeln!(f, "Log to stdout: {}", self.log_stdout)?;
        write!(f, "Has headers:   {}", self.has_headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let model = FileFormModel::default();
        assert!(model.source_file.as_os_str().is_empty());
        assert!(model.database_file.as_os_str().is_empty());
        assert!(model.log_directory.as_os_str().is_empty());
        assert!(model.selected_sheet.is_none());
        assert!(!model.log_stdout);
        assert!(!model.has_headers);
    }

    #[test]
    fn test_display_populated() {
        let model = FileFormModel {
            source_file: PathBuf::from("data.xlsx"),
            database_file: PathBuf::from("app.db"),
            log_directory: PathBuf::from("output.log"),
            db_backend: DbBackend::MySql,
            log_level: LogLevel::Info,
            selected_sheet: Some("Sheet1".to_string()),
            log_stdout: true,
            has_headers: true,
        };
        let output = model.to_string();
        assert!(output.contains("data.xlsx"));
        assert!(output.contains("app.db"));
        assert!(output.contains("output.log"));
        assert!(output.contains("MySQL"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Sheet1"));
        assert!(output.contains("true"));
    }

    #[test]
    fn test_validate_for_submit_excel_requires_sheet() {
        let model = FileFormModel {
            source_file: PathBuf::from("input.xlsx"),
            database_file: PathBuf::from("app.db"),
            db_backend: DbBackend::Sqlite,
            log_level: LogLevel::Info,
            selected_sheet: None,
            ..FileFormModel::default()
        };

        let errors = model
            .validate_for_submit()
            .expect_err("expected validation error");
        assert!(
            errors
                .iter()
                .any(|err| err.contains("Sheet selection is required")),
            "expected sheet validation error, got: {errors:?}"
        );
    }

    #[test]
    fn test_validate_for_submit_excel_with_sheet_is_valid() {
        let model = FileFormModel {
            source_file: PathBuf::from("input.xlsx"),
            database_file: PathBuf::from("app.db"),
            db_backend: DbBackend::Sqlite,
            log_level: LogLevel::Info,
            selected_sheet: Some("Sheet1".to_string()),
            ..FileFormModel::default()
        };

        assert!(model.validate_for_submit().is_ok());
    }

    #[test]
    fn test_validate_for_submit_csv_without_sheet_is_valid() {
        let model = FileFormModel {
            source_file: PathBuf::from("input.csv"),
            database_file: PathBuf::from("app.db"),
            db_backend: DbBackend::Sqlite,
            log_level: LogLevel::Info,
            selected_sheet: None,
            ..FileFormModel::default()
        };

        assert!(model.validate_for_submit().is_ok());
    }

    #[test]
    fn test_validate_for_submit_requires_source_and_database() {
        let model = FileFormModel {
            source_file: PathBuf::new(),
            database_file: PathBuf::new(),
            ..FileFormModel::default()
        };

        let errors = model
            .validate_for_submit()
            .expect_err("expected validation errors");
        assert!(
            errors
                .iter()
                .any(|err| err.contains("Source file is required")),
            "expected source file validation error, got: {errors:?}"
        );
        assert!(
            errors
                .iter()
                .any(|err| err.contains("Database file is required")),
            "expected database file validation error, got: {errors:?}"
        );
    }

    #[test]
    fn test_db_backend_from_label() {
        assert_eq!(DbBackend::from_label("MySQL"), Some(DbBackend::MySql));
        assert_eq!(DbBackend::from_label("not-a-backend"), None);
    }

    #[test]
    fn test_log_level_from_label() {
        assert_eq!(LogLevel::from_label("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_label("not-a-level"), None);
    }

    #[test]
    fn test_debug_derives() {
        let model = FileFormModel::default();
        let debug = format!("{:?}", model);
        assert!(debug.contains("FileFormModel"));
    }

    #[test]
    fn test_is_sqlite_positive() {
        let sqlite_extensions = [
            "main.db",
            "main.db3",
            "main.sqlite",
            "main.DB",
        ];
        for file_name in sqlite_extensions {
            let model = FileFormModel {
                database_file: PathBuf::from(file_name),
                ..FileFormModel::default()
            };
            assert!(
                model.is_sqlite(),
                "expected {file_name} to be recognized as sqlite"
            );
        }
    }

    #[test]
    fn test_is_sqlite_negative() {
        let non_sqlite = FileFormModel {
            database_file: PathBuf::from("main.sqlite3"),
            ..FileFormModel::default()
        };
        assert!(!non_sqlite.is_sqlite());
    }

    #[test]
    fn test_is_csv_positive() {
        let csv_sources = [
            "input.csv",
            "INPUT.CSV",
        ];
        for file_name in csv_sources {
            let model = FileFormModel {
                source_file: PathBuf::from(file_name),
                ..FileFormModel::default()
            };
            assert!(
                model.is_csv(),
                "expected {file_name} to be recognized as csv"
            );
        }
    }

    #[test]
    fn test_is_csv_negative() {
        let non_csv = FileFormModel {
            source_file: PathBuf::from("input.tsv"),
            ..FileFormModel::default()
        };
        assert!(!non_csv.is_csv());
    }
}
