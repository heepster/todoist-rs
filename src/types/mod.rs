mod date;
mod intbool;
mod error;
mod color;

pub use self::date::{Date, TimeZoneInfo};
pub use self::intbool::IntBool;
pub use self::error::Error;
pub use self::color::Color;

/// A 2 character language ID
/// valid ids: `en`, `da`, `pl`, `zh`, `ko`, `de`, `pt`, `ja`, `it`, `fr`, `sv`, `ru`, `es`, `nl`
pub type Language = String;

/// A number between 1-4, specifies how important an item is
pub type Priority = u8;

/// A todoist object ID
pub type ID = usize;
