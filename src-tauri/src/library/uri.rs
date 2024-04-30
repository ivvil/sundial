use audiotags::Picture;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub fn img_base64(picture: &Picture) -> String { // Magic
	let img_data = URL_SAFE.encode(picture.data);

	let img_type = match picture.mime_type {
        audiotags::MimeType::Jpeg => "jpeg",
		audiotags::MimeType::Png => "png",
        audiotags::MimeType::Gif => "gif",
		audiotags::MimeType::Bmp => "bmp",
		audiotags::MimeType::Tiff => "tiff",
    };

	format!("data:{};base64,{}", img_type, img_data)
}
