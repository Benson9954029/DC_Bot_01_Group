use dotenvy::dotenv;
use poise::serenity_prelude as serenity_pre;
use serenity_pre::async_trait;
use serenity_pre::model::channel::Message;
use serenity_pre::prelude::*;
use serenity::builder::{CreateForumPost, CreateMessage};
use serenity::model::id::ChannelId;

struct Handler;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type PoiseContext<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    // Limit slash in some channel
    ctx: PoiseContext<'_>,
    #[description = "Selected user"] user: Option<serenity_pre::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    let channel_id = std::env::var("CHANNEL_ID").expect("missing CHANNEL_ID").parse::<u64>().expect("format not u4");
    let message = CreateMessage::new().content("First message content");
    let post = CreateForumPost::new("Forum Post Title", message);
    let channel_id = ChannelId::new(channel_id); // Replace with your forum channel ID
    channel_id.create_forum_post(&ctx.http(), post).await?;
    Ok(())
}

// Serenity Event Handler
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity_pre::Context, msg: Message) {
        // Handle regular text-based commands
        if msg.author.bot {
            return;
        }
        println!("{0}",msg.channel_id);

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        } else {
            if let Err(why) = msg.channel_id.say(&ctx.http, "51哥最帥").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from `.env`
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    

    // Setup intents required for both poise and serenity
    let intents = serenity_pre::GatewayIntents::GUILD_MESSAGES
        | serenity_pre::GatewayIntents::MESSAGE_CONTENT;

    // Build the poise framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()], // Register poise commands here
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Create the serenity client with both the poise framework and the event handler
    let mut client = serenity_pre::ClientBuilder::new(&token, intents)
        .event_handler(Handler) // Register the serenity event handler
        .framework(framework)   // Register the poise command framework
        .await
        .expect("Error creating client");

    // Start the client
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

