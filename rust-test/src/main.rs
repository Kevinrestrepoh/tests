use std::time::Instant;

use rand::{random_bool, random_range};
use tokio::{
    sync::mpsc,
    time::{Duration, sleep},
};

use axum::Router;
use db::create_pool;
use dotenv::dotenv;
use routes::{products::product_routes, users::user_routes};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn server() {
    dotenv().ok();

    let pool = create_pool().await.expect("Failed to connect to database");

    let app = Router::new()
        .merge(user_routes())
        .merge(product_routes())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on port :8000");
    axum::serve(listener, app).await.unwrap();
}

struct DownloadJob {
    id: usize,
    url: String,
}

struct DownloadResult {
    id: usize,
    success: bool,
    message: String,
}

struct DownloaderPool {
    workers: usize,
}

impl DownloaderPool {
    fn new(n: usize) -> Self {
        DownloaderPool { workers: n }
    }

    async fn run(&self, jobs: Vec<DownloadJob>) {
        let (tx, mut rx) = mpsc::channel::<DownloadResult>(self.workers);

        for job in jobs {
            println!("downloading {}", job.url);
            let tx = tx.clone();
            tokio::spawn(async move {
                let result: DownloadResult = worker(job).await;
                tx.send(result).await.unwrap();
            });
        }

        drop(tx);

        while let Some(msg) = rx.recv().await {
            println!(
                "Download {}, success: {}, msg: {}",
                msg.id, msg.success, msg.message
            );
        }
    }
}

#[tokio::main]
async fn main() {
    server();
    let start = Instant::now();

    let pool = DownloaderPool::new(3);
    let jobs: Vec<DownloadJob> = (0..10)
        .map(|i| DownloadJob {
            id: i,
            url: String::from("url {id}"),
        })
        .collect();

    pool.run(jobs).await;

    println!("Total execution {:?}", start.elapsed());
}

async fn worker(job: DownloadJob) -> DownloadResult {
    let delay = random_range(500..1000);
    sleep(Duration::from_millis(delay)).await;
    let succeeded: bool = random_bool(0.80);
    let msg = if succeeded {
        "download succeeded"
    } else {
        "download failed"
    };

    DownloadResult {
        id: job.id,
        success: succeeded,
        message: msg.to_string(),
    }
}
