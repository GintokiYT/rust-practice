use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Post {
  #[serde(rename = "userId")]
  user_id: u32,
  id: u32,
  title: String,
  body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Hacemos la solicitud a la API
  let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
    .await?;

  // Verificamos si la respuesta es exitosa antes de deserializar
  if response.status().is_success() {
    let post: Post = response.json().await?;
    println!("{:#?}", post);
  } else {
    eprintln!("Error en la solicitud: {}", response.status());
  }

  Ok(())
}
