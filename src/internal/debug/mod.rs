#[macro_use]
mod macros;
mod position;
mod checked;
mod status;
mod error;

pub use self::position::Position;
pub use self::checked::Checked;
pub use self::status::Status;
pub use self::error::Error;

use internal::*;

fn comma_seperated_list(list: &Vector<Data>) -> AsciiString {
    let mut string = AsciiString::new();
    for (index, item) in list.iter().enumerate() {
        if index == 0 {
            string.push_str(&item.serialize());
        } else if index == list.len() - 1 {
            string.push_str(&format_ascii!(" or {}", item.serialize()));
        } else {
            string.push_str(&format_ascii!(", {}", item.serialize()));
        }
    }
    return string;
}

fn expanded_list(_errors: Vector<Error>) -> AsciiString {
    return AsciiString::from("<expanded list>");
}