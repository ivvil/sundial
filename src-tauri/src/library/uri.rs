use audiotags::Picture;
use base64::Engine;

pub fn img_base64(picture: &Picture) -> String { // Magic
    let base64_engine = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::general_purpose::NO_PAD);
	let img_data = base64_engine.encode(picture.data);	

	let img_type = match picture.mime_type {
        audiotags::MimeType::Jpeg => "jpeg",
		audiotags::MimeType::Png => "png",
        audiotags::MimeType::Gif => "gif",
		audiotags::MimeType::Bmp => "bmp",
		audiotags::MimeType::Tiff => "tiff",
    };


	format!("data:{};base64,{}", img_type, img_data)
}
