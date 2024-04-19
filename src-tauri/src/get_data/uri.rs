use audiotags::MimeType;
use base64::{engine::general_purpose::URL_SAFE, Engine};

pub fn build_image_uri(mime: MimeType, data: Vec<u8>) -> String {
    let img = URL_SAFE.encode(data);

	format!("data:{};base64,{}", String::from(mime), img)
}
