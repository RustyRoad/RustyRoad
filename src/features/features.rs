use crate::features::GrapesJS;
use color_eyre::eyre::Result;
use eyre::Error;
pub async fn add_feature(feature_name: String) -> Result<(), Error> {
     match feature_name.as_str() {
        "grapesjs" => {
            let mut grapesjs = GrapesJS::new();

            match grapesjs.add_grapesjs().await {
                Ok(_) => {
                    println!("Successfully added grapesjs");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        _ => {
            println!("Couldn't find feature: {}", feature_name);
        }
    }

    Ok(())


}
