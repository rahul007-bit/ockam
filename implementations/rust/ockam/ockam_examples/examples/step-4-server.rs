use ockam::{async_worker, Context, RemoteMailbox, Result, Routed, Worker};
use ockam_transport_tcp::TcpTransport;

struct EchoService;

#[async_worker]
impl Worker for EchoService {
    type Message = String;
    type Context = Context;

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<String>) -> Result<()> {
        println!("echo_service: {}", msg);
        ctx.send(msg.return_route(), msg.body()).await
    }
}

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let hub = "Paste the address of the node you created on Ockam Hub here.";

    let tcp = TcpTransport::create(&ctx).await?;

    tcp.connect(hub).await?;

    ctx.start_worker("echo_service", EchoService).await?;

    let mailbox = RemoteMailbox::<String>::create(&mut ctx, hub, "echo_service").await?;

    println!(
        "echo_service forwarding address: {}",
        mailbox.remote_address()
    );

    Ok(())
}
