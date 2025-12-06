use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

pub const DEFAULT_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub trait DateTimeFormatter {
    fn to_formatted_string(&self)->String;
    fn to_formatted_string_with(&self, format: &[FormatItem<'_>])->String;
    fn to_formatted_string_custom(&self, format_str: &str) -> Result<String, time::error::Format>;

}

impl DateTimeFormatter for OffsetDateTime {
    fn to_formatted_string(&self) -> String {
        self.format(&DEFAULT_FORMAT).unwrap_or_else(|_| "Invalid date".to_string())
    }

    fn to_formatted_string_with(&self, format: &[FormatItem]) -> String {
        self.format(format).unwrap_or_else(|_| "Invalid date".to_string())
    }

    fn to_formatted_string_custom(&self, format_str: &str) -> Result<String, time::error::Format> {
        let format = time::format_description::parse(format_str)
            .map_err(|_| time::error::Format::InvalidComponent("Invalid format string"))?;
        self.format(&format)
    }
}

impl DateTimeFormatter for Option<OffsetDateTime> {
    fn to_formatted_string(&self) -> String {
        match self {
            Some(dt) => dt.to_formatted_string(),
            None => "N/A".to_string(),
        }
    }

    fn to_formatted_string_with(&self, format: &[FormatItem]) -> String{
        match self {
            Some(dt) => dt.to_formatted_string_with(format),
            None => "N/A".to_string(),
        }
    }

    fn to_formatted_string_custom(&self, format_str: &str) -> Result<String, time::error::Format> {
        match self {
            Some(dt) => dt.to_formatted_string_custom(format_str),
            None => Ok("N/A".to_string()),
        }
    }
}

// Дополнительные удобные форматы
pub const ISO_DATE_FORMAT: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");
pub const TIME_FORMAT: &[FormatItem<'static>] = format_description!("[hour]:[minute]:[second]");
pub const DATETIME_WITH_MILLISECONDS: &[FormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");