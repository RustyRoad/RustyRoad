use rustyroad::Project;

#[tokio::main]
pub async fn main() {
    Project::run().await;
}
