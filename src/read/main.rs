mod prelude;
mod read;

use prelude::*;

use grpcrow::{
    pb::shelter::read_service_server::ReadServiceServer,
    components::db::DbComponent,
    settings,
};

#[derive(Debug)]
pub struct ReadServer {
    db: DbComponent,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let settings = settings()?;

    let addr = format!("{}:{}", settings.read.host, settings.read.port).parse()?;

    println!("Connecting to Shelter Database...");

    let db_component = DbComponent::new(&settings.db.url()).await?;

    let read_service = ReadServer { db: db_component };

    println!("gRPCrow read service listening on {}", addr);

    Server::builder()
        .add_service(ReadServiceServer::new(read_service))
        .serve(addr)
        .await?;

    Ok(())
}