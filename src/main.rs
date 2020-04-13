
mod shardconnection;
mod rest;

fn main() {
    //shardconnection::random_function();
    async {
        println!("In async");
        rest::RestSender::new().get("https://discordapp.com/api/gateway/bot").await;
    };
}
