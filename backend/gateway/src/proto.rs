#[allow(clippy::all)]
pub mod auth {
    tonic::include_proto!("auth.v1");
}

#[allow(clippy::all)]
pub mod statistics {
    tonic::include_proto!("statistics.v1");
}

#[allow(clippy::all)]
pub mod notification {
    tonic::include_proto!("notification.v1");
}

#[allow(clippy::all)]
pub mod engagement {
    tonic::include_proto!("engagement.v1");
}

#[allow(clippy::all)]
pub mod content {
    tonic::include_proto!("content.v1");
}
