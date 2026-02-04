#![allow(warnings)]

use crc32c::crc32c;
capnp::generated_code!(pub mod addressbook_capnp);

pub mod addressbook {
    use crate::addressbook_capnp::{address_book, person, my_data};
    use capnp::{serialize, serialize_packed};
    use crc32c::crc32c;

    pub fn generate_checksum() {
        let mut message = ::capnp::message::Builder::new_default();
        let mut my_data = message.init_root::<my_data::Builder>();
        my_data.set_a(64);
        my_data.set_b(128);
        my_data.set_c(123.321);
        my_data.set_d(123.123);
        my_data.set_e(true);

        let mut buffer = Vec::new();
        serialize::write_message(&mut buffer, &message);

        let checksum = crc32c(&buffer);
        println!("Checksum: {:08x}", checksum);

        println!("Number of segments: {}", message.get_segments_for_output().len());
    }
}

pub fn main() {
    addressbook::generate_checksum();
}
