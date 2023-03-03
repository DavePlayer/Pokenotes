use actix_web::{HttpRequest, get};
use actix_files::NamedFile;

#[get("/image/{filename:.*}")]
#[allow(unused)]
pub async fn get_image_by_name(req: HttpRequest) -> NamedFile {
    todo!();
}