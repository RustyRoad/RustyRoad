


    use actix_web::guard::Host;
use actix_web::web::Buf;
use actix_web::{post, web::Data, HttpRequest, HttpResponse};
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde_json::json;
use std::{fs, io::Write, path::PathBuf};

use actix_multipart::Multipart;
use image::imageops::FilterType;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
#[post("/image")]
async fn upload_image(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];
    let image_directory = PathBuf::from("static/images");

    // ensure image directory exists
    if !image_directory.exists() {
        if let Err(_) = fs::create_dir_all(&image_directory) {
            return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to create image directory.",
                    "status": 500
                })
            });
        }
    }

    let mut files_info = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_type().clone();

        // ensure file is an image
        if content_type.is_none() || !legal_filetypes.contains(&content_type.unwrap()) {
            return HttpResponse::BadRequest().json({
                json!({
                    "error": "File must be an image.",
                    "status": 400
                })
            });
        }

        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap_or_default()
            .to_string();

        // Replace spaces in the filename with underscores
        let sanitized_filename = sanitize_filename::sanitize(&filename);

        let filepath = image_directory.join(&sanitized_filename);

        // create a path to save the file
        let mut file = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to create file.",
                    "status": 500
                })
            })
        };

        // copy the content of file into the file variable
        while let Ok(Some(chunk)) = field.try_next().await {
            if let Err(_) = file.write_all(&chunk) {
                return HttpResponse::InternalServerError().json({
                    json!({
                        "error": "Failed to write file.",
                        "status": 500
                    })
                });
            }
        }

        // create a thumbnail of the image
        match image::open(&filepath) {
            Ok(image) => {
                let thumbnail = image.thumbnail(100, 100);
                if let Err(_) = thumbnail.save(filepath.with_extension("thumb.jpg")) {
                    return HttpResponse::InternalServerError().json({
                        json!({
                            "error": "Failed to save thumbnail.",
                            "status": 500
                        })
                    });
                }
            }
            Err(_) => return HttpResponse::InternalServerError().json({
                json!({
                    "error": "Failed to open image.",
                    "status": 500
                })
            }),
        };

        files_info.push(json!({
            "filename": sanitized_filename.as_str(),
            "thumbnail": format!("{}://{}/images/{}",
                req.connection_info().scheme(),
                req.connection_info().host(),
                sanitized_filename.as_str()
            ),
            "filepath": format!("{}://{}/images/{}",
                req.connection_info().scheme(),
                req.connection_info().host(),
                sanitized_filename.as_str()
            )
        }));
    }

    // Check if any files were uploaded
    if files_info.is_empty() {
        return HttpResponse::BadRequest().json({
            json!({
                "error": "No file uploaded.",
                "status": 400
            })
        });
    }

    // On successful upload of multiple files, return JSON response
    HttpResponse::Ok().json({
        json!({
            "status": 200,
            "message": "Files uploaded successfully.",
            "files": files_info
        })
    })
}

