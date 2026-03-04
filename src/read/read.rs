use super::{
    prelude::*,
    ReadServer,
};

use grpcrow::{
    pb::shelter::{
        read_service_server::{ReadService},
        AnimalRequest, AnimalResponse,
    },
    components::db::args,
    queries,
};

#[derive(Debug, FromRow)]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub breed: Option<String>,
}

#[tonic::async_trait]
impl ReadService for ReadServer {
    async fn get_animal(
        &self,
        request: Request<AnimalRequest>,
    ) -> Result<Response<AnimalResponse>, Status> {
        let req = request.into_inner();

        let result = self.db.fetch_one::<Animal>(
            &queries().unwrap().read.get_animal_by_id,
            args![req.id]
        ).await;

        match result {
            Ok(Some(animal)) => {
                let reply = AnimalResponse {
                    id: animal.id,
                    name: animal.name,
                    species: animal.species,
                    breed: animal.breed.unwrap_or_else(|| "Unknown".to_string()),
                };
                Ok(Response::new(reply))
            }
            Ok(None) => Err(Status::not_found("Animal not found in shelter database")),
            Err(e) => Err(Status::internal(format!("DB error: {}", e))),
        }
    }
}