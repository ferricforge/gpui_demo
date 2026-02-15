// models/file_form_model.rs

use std::fmt;

/// Represents the collected values from the file selection form.
#[derive(Clone, Debug, Default)]
pub struct FileFormModel {
    pub source_file: String,
    pub database_file: String,
    pub log_directory: String,
    pub log_stdout: bool,
    pub has_headers: bool,
}

impl fmt::Display for FileFormModel {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(f, "Source file:   {}", self.source_file)?;
        writeln!(f, "Database:      {}", self.database_file)?;
        writeln!(f, "Log folder:    {}", self.log_directory)?;
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
        assert!(model.source_file.is_empty());
        assert!(model.database_file.is_empty());
        assert!(model.log_directory.is_empty());
        assert!(!model.log_stdout);
        assert!(!model.has_headers);
    }

    #[test]
    fn test_display_populated() {
        let model = FileFormModel {
            source_file: "data.xlsx".to_string(),
            database_file: "app.db".to_string(),
            log_directory: "output.log".to_string(),
            log_stdout: true,
            has_headers: true,
        };
        let output = model.to_string();
        assert!(output.contains("data.xlsx"));
        assert!(output.contains("app.db"));
        assert!(output.contains("output.log"));
        assert!(output.contains("true"));
    }

    #[test]
    fn test_debug_derives() {
        let model = FileFormModel::default();
        let debug = format!("{:?}", model);
        assert!(debug.contains("FileFormModel"));
    }
}
