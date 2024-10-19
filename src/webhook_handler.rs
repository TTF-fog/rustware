use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read};
use discord_webhook2::message::Message;
use discord_webhook2::webhook::DiscordWebhook;
pub async fn send_files(image_path:&str) {
    let mut files:BTreeMap<String,Vec<u8>> = BTreeMap::new();
    let webhook = DiscordWebhook::new("https://discord.com/api/webhooks/1297187641636159538/58LmoxoSJRxLzByNcGmEQ8v1ZpuIMb_RXku5WynFoI-R6IpNGm_qV_y4TOXUmCn3KKxF").unwrap();
    let file =File::open(image_path);
    let mut bytes:Vec<u8> = Vec::new();
    file.unwrap().read_to_end(&mut bytes);
    files.insert(
        String::from(image_path),
        Vec::from(bytes),
    );


    webhook
        .send_with_files(
            &Message::new(|message| {
                message.embed(|embed| embed.title("hi!"))
            }),
            files,
        )
        .await
        .unwrap();

}