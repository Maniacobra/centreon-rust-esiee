pub mod common {
    tonic::include_proto!("com.centreon.common");
}

pub mod broker {
    tonic::include_proto!("com.centreon.broker");
}

use broker::broker_client::BrokerClient;
use broker::Version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    todo!();
}
