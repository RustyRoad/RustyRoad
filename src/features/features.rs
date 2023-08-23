use crate::features::GrapesJS;

pub async fn add_feature(feature_name: String)  -> String {
    return match feature_name.as_str() {
        "grapesjs" => {
            let mut grapesjs = GrapesJS::new();
            grapesjs.add_grapesjs().await;

            String::from("GrapesJS added")
        }
        _ => {
            println!("Invalid feature name");

            String::from("Invalid feature name")
        }
    }
}
