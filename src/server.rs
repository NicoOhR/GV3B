tonic::include_proto!("simulation");

use crate::bodies::SimulationState;
use bevy::prelude::{Res, ResMut};
use bevy_tokio_tasks::*;
use sim_server::{Sim, SimServer};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default, Clone)]
pub struct SimulationService {
    state: Arc<Mutex<SimulationState>>,
}

pub fn start_server(sim_state: Res<'_, SimulationState>, runtime: ResMut<'_, TokioTasksRuntime>) {
    let service = SimulationService {
        state: Arc::new(Mutex::new(SimulationState {
            body_attributes: sim_state.body_attributes.clone(),
        })),
    };

    println!("Started Server");
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    runtime.spawn_background_task(move |_ctx| async move {
        let service = service.clone();
        let addr = addr.clone();
        async move {
            Server::builder()
                .add_service(SimServer::new(service))
                .serve(addr)
                .await
                .expect("Failed to start gRPC server");
        }
    });
}

#[tonic::async_trait]
impl Sim for SimulationService {
    async fn replies(&self, _request: Request<SimReq>) -> Result<Response<SimResponse>, Status> {
        let state = self.state.lock().unwrap();
        let mut body_velocity_position: Vec<BodyAttributes> = vec![];
        let mut body_state: BodyAttributes;
        for body in &state.body_attributes {
            //TODO: Make this a little less heinous
            body_state = BodyAttributes {
                velocity: Some(Vec2D {
                    x: body.velocity.x,
                    y: body.velocity.y,
                }),
                position: Some(Vec2D {
                    x: body.position.x,
                    y: body.position.y,
                }),
            };
            body_velocity_position.push(body_state);
        }

        let response = SimResponse {
            bodies: body_velocity_position,
        };

        Ok(Response::new(response))
    }
}
