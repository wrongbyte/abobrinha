use tokio::fs::OpenOptions; 
use tokio::fs;

async fn open_file() -> Result<tokio::fs::File, std::io::Error> {
    OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open("todo.txt")
    .await
}