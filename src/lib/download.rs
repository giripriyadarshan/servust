use std::io::Write;

pub async fn download_file(source: &str, destination: &str) {
    let response = reqwest::get(source).await.unwrap();

    let path = std::path::Path::new(&destination);

    let mut file = match std::fs::File::create(&path) {
        Ok(f) => f,
        Err(e) => panic!("couldn't create {}", e)
    };

    let content = response.bytes().await.unwrap();
    file.write_all(&content).unwrap();
}