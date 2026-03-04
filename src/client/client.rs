use grpcrow::{
    settings,
    pb::shelter::{
        read_service_client::ReadServiceClient,
        AnimalRequest,
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Shelter Network...");

    let settings = settings()?;

    let mut client = ReadServiceClient::connect(settings.read.url()).await?;

    let request = tonic::Request::new(AnimalRequest { id: 1 });

    println!("Fetching animal data for ID 1...");

    let response = client.get_animal(request).await?;

    println!("Success OwO!!! Received: {:#?}", response.into_inner());

    Ok(())
}