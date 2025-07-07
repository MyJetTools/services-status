use std::os::unix::fs::FileTypeExt;

use crate::app_ctx::AppContext;

pub async fn get_sockets_list(app: &AppContext) -> Vec<String> {
    let mut result = Vec::new();
    let path = app
        .settings_reader
        .use_settings(|itm| rust_extensions::file_utils::FilePath::from_str(&itm.unix_sockets_path))
        .await;
    let mut read_dir = match tokio::fs::read_dir(path.as_str()).await {
        Ok(result) => result,
        Err(err) => {
            panic!("Can not scan directory {}. Err: {:?}", path.as_str(), err);
        }
    };

    while let Some(dir) = read_dir.next_entry().await.unwrap() {
        let name = dir.file_name();
        let file_type = dir.file_type().await.unwrap();

        if file_type.is_socket() {
            let mut path = path.clone();
            if let Some(name) = name.to_str() {
                path.append_segment(name);
                result.push(path.to_string());
            }
        }
    }

    println!("{:#?}", result);

    result
}
