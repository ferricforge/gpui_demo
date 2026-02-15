// models/file_form_model.rs

use std::{fmt, path::PathBuf};

/// Represents the collected values from the file selection form.
#[derive(Clone, Debug, Default)]
pub struct FileFormModel {
    pub source_file: PathBuf,
    pub database_file: PathBuf,
    pub log_directory: PathBuf,
    pub db_backend: String,
    pub log_level: String,
    pub selected_sheet: String,
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
        writeln!(f, "Sheet:         {}", self.selected_sheet)?;
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
        assert!(!model.log_stdout);
        assert!(!model.has_headers);
    }

    #[test]
    fn test_display_populated() {
        let model = FileFormModel {
            source_file: PathBuf::from("data.xlsx"),
            database_file: PathBuf::from("app.db"),
            log_directory: PathBuf::from("output.log"),
            db_backend: "MySQL".to_string(),
            log_level: "INFO".to_string(),
            selected_sheet: "Sheet1".to_string(),
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
