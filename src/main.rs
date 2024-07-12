use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut etcd = etcd_client::Client::connect(["localhost:2379"], None).await?;

    let _ = etcd
    .put(
        "/playground",
        "",
        Some(etcd_client::PutOptions::new().with_ignore_value()),
    )
    .await?;

    let _ = etcd
    .put(
        "/playground/loona",
        "favourite",
        None
    )
    .await?;

    let _ = etcd
    .put(
        "/playground/gyubin",
        "reallylikeyou",
        None,
    )
    .await?;

    let resulting_values = etcd.get("/playground/", None).await.unwrap().take_kvs();

    println!("{:#?}", resulting_values);

    Ok(())
}

