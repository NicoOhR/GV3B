//use tonic::{transport::Server, Request, Response, Status};
//tonic::include_proto!("bridge");
//
//#[derive(Debug, Default)]
//pub struct Bridge {}
//
//#[tonic::async_trait]
//impl bridge for Bridge {
//    async fn serve_values(
//        &self,
//        request: Request<BodyRequest>,
//    ) -> Result<Response<BodyResponse>, Status> {
//        println!("test service");
//    }
//}
