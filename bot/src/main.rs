use crate::helper::ApplicationContext;
use crate::helper::Data;
use crate::helper::Error;
use database::model::UpdateDare;
use database::service::DbService;
use database::*;
use model::{NewDare, NewTruth, Rating, Status, UpdateTruth};
mod helper;
use poise::serenity_prelude::{self as serenity, CreateEmbed};

#[poise::command(slash_command)]
pub async fn register(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(poise::Context::Application(ctx))
        .await?;
    Ok(())
}
//Add a truth
#[poise::command(slash_command)]
async fn add_truth(
    ctx: ApplicationContext<'_>,
    #[description = "Truth content"] content: String,
    #[description = "Truth rating"] rating: Rating,
) -> Result<(), Error> {
    let mut db = DbService::new();
    let new_truth = NewTruth {
        content: &content,
        author: ctx.author().id.get().to_string(),
        rating,
        status: if helper::is_admin(ctx).await? {
            Status::ACCEPTED
        } else {
            Status::PENDING
        },
        submit_date: chrono::Utc::now().naive_utc(),
    };
    let res = db.create_truth(new_truth);
    match res {
        Ok(truth) => {
            let status_string = truth.status().to_string();
            let content_string = truth.content().to_string();
            println!("Truth added successfully!");
            ctx.send(poise::CreateReply::default().embed(create_embed(
                &format!("Truth added with status: {}", status_string),
                &content_string,
                "",
                serenity::Colour::BLITZ_BLUE,
            )))
            .await?;
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }

    Ok(())
}
//Add a dare
#[poise::command(slash_command)]
async fn add_dare(
    ctx: ApplicationContext<'_>,
    #[description = "Dare content"] content: String,
    #[description = "Dare rating"] rating: Rating,
) -> Result<(), Error> {
    let mut db = DbService::new();
    let new_dare = NewDare {
        content: &content,
        author: ctx.author().id.get().to_string(),
        rating,
        status: if helper::is_admin(ctx).await? {
            Status::ACCEPTED
        } else {
            Status::PENDING
        },
        submit_date: chrono::Utc::now().naive_utc(),
    };
    let res = db.create_dare(new_dare);
    match res {
        Ok(dare) => {
            let status_string = dare.status().to_string();
            let content_string = dare.content().to_string();
            println!("Dare added successfully!");
            ctx.send(poise::CreateReply::default().embed(create_embed(
                &format!("Dare added with status: {}", status_string),
                &content_string,
                format!("id: {}", dare.id()).as_str(),
                serenity::Colour::BLITZ_BLUE,
            )))
            .await?;
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }

    Ok(())
}

//Get a random dare
#[poise::command(slash_command)]
async fn get_dare(
    ctx: ApplicationContext<'_>,
    #[description = "Dare rating"] mut rating: Option<Rating>,
) -> Result<(), Error> {
    let is_safe_channel = !helper::is_age_gated(ctx).await?;
    //if we are in a safe channel and the rating is not safe, return error
    if let Some(r) = rating {
        if is_safe_channel && r == Rating::NSFW {
            ctx.send(poise::CreateReply::default().embed(create_embed(
                "This channel is not safe for this content",
                "Please use a different channel",
                "",
                serenity::Colour::RED,
            )))
            .await?;
            return Ok(());
        }
    }
    //if we are in an non safe channel and rating is not set, set it to adult
    if !is_safe_channel && rating.is_none() {
        rating = Some(Rating::NSFW);
    }
    let mut db = DbService::new();
    let res = db.get_random_dare(rating);
    match res {
        Ok(dare) => {
            let _status_string = dare.status().to_string();
            let rating_string = dare.rating().to_string();
            let content_string = dare.content().to_string();
            ctx.send(
                poise::CreateReply::default().embed(create_embed(
                    &format!("Here is your Dare"),
                    &content_string,
                    format!(
                        "Rating: {}, ID: {}, Suggested By {}",
                        rating_string,
                        dare.id(),
                        get_username_from_id(&dare.author(), Some(ctx)).await
                    )
                    .as_str(),
                    serenity::Colour::BLITZ_BLUE,
                )),
            )
            .await?;
        }
        Err(e) => {
            println!("{e}");
            match e {
                diesel::result::Error::NotFound => {
                    ctx.send(poise::CreateReply::default().embed(create_embed(
                        &format!("No dares found"),
                        "Add a new one. or ask admins to approve some",
                        "",
                        serenity::Colour::RED,
                    )))
                    .await?;
                }

                e => {
                    error_happened(Box::new(e), Some(ctx)).await?;
                }
            }
        }
    }

    Ok(())
}

//Get a random truth
#[poise::command(slash_command)]
async fn get_truth(
    ctx: ApplicationContext<'_>,
    #[description = "Truth rating"] mut rating: Option<Rating>,
) -> Result<(), Error> {
    let is_safe_channel = !helper::is_age_gated(ctx).await?;
    //if we are in a safe channel and the rating is not safe, return error
    if let Some(r) = rating {
        if is_safe_channel && r == Rating::NSFW {
            ctx.send(poise::CreateReply::default().embed(create_embed(
                "This channel is not safe for this content",
                "Please use a different channel",
                "",
                serenity::Colour::RED,
            )))
            .await?;
            return Ok(());
        }
    }
    //if we are in an non safe channel and rating is not set, set it to adult
    if !is_safe_channel && rating.is_none() {
        rating = Some(Rating::NSFW);
    }

    let mut db = DbService::new();
    let res = db.get_random_truth(rating);
    match res {
        Ok(truth) => {
            let _status_string = truth.status().to_string();
            let rating_string = truth.rating().to_string();
            let content_string = truth.content().to_string();
            ctx.send(
                poise::CreateReply::default().embed(create_embed(
                    &format!("Here is your Truth"),
                    &content_string,
                    format!(
                        "Rating: {}, ID: {}, Suggested By {}",
                        rating_string,
                        truth.id(),
                        get_username_from_id(&truth.author(), Some(ctx)).await
                    )
                    .as_str(),
                    serenity::Colour::BLITZ_BLUE,
                )),
            )
            .await?;
        }
        Err(e) => {
            println!("{e}");
            match e {
                diesel::result::Error::NotFound => {
                    ctx.send(poise::CreateReply::default().embed(create_embed(
                        &format!("No truths found"),
                        "Add a new one. or ask admins to approve some",
                        "",
                        serenity::Colour::RED,
                    )))
                    .await?;
                }

                e => {
                    error_happened(Box::new(e), Some(ctx)).await?;
                }
            }
        }
    }

    Ok(())
}
//Accept a truth/dare
#[poise::command(slash_command)]
async fn accept(
    ctx: ApplicationContext<'_>,
    kind: database::model::DbType,
    id: i32,
    rating_overide: Option<Rating>,
) -> Result<(), Error> {
    let is_admin = helper::is_admin(ctx).await?;
    if !is_admin {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "You are not allowed to do this",
            "Ask an admin to do it for you",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    let mut db = DbService::new();
    let res = db.accept(kind, id);
    match res {
        Ok(_) => {
            if let Some(rating) = rating_overide {
                match kind {
                    database::model::DbType::Truth => {
                        let res = db.update_truth(
                            id,
                            UpdateTruth {
                                content: None,
                                rating: Some(rating),
                                status: None,
                            },
                        );
                        match res {
                            Ok(_) => {
                                ctx.send(poise::CreateReply::default().embed(create_embed(
                                    "Truth accepted",
                                    "",
                                    "",
                                    serenity::Colour::BLITZ_BLUE,
                                )))
                                .await?;
                            }
                            Err(e) => {
                                println!("{e}");
                                error_happened(Box::new(e), Some(ctx)).await?;
                            }
                        }
                    }
                    database::model::DbType::Dare => {
                        let res = db.update_dare(
                            id,
                            UpdateDare {
                                content: None,
                                rating: Some(rating),
                                status: None,
                            },
                        );
                        match res {
                            Ok(_) => {
                                ctx.send(poise::CreateReply::default().embed(create_embed(
                                    "Dare accepted",
                                    "",
                                    "",
                                    serenity::Colour::BLITZ_BLUE,
                                )))
                                .await?;
                            }
                            Err(e) => {
                                println!("{e}");
                                error_happened(Box::new(e), Some(ctx)).await?;
                            }
                        }
                    }
                }
            } else {
                ctx.send(
                    poise::CreateReply::default().embed(create_embed(
                        format!(
                            "{} accepted",
                            match kind {
                                model::DbType::Dare => "Dare",
                                model::DbType::Truth => "Truth",
                            }
                        )
                        .as_str(),
                        "",
                        "",
                        serenity::Colour::BLITZ_BLUE,
                    )),
                )
                .await?;
            }
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }
    Ok(())
}

//delete a truth/dare
#[poise::command(slash_command)]
async fn delete(
    ctx: ApplicationContext<'_>,
    kind: database::model::DbType,
    id: i32,
) -> Result<(), Error> {
    let is_admin = helper::is_admin(ctx).await?;
    if !is_admin {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "You are not allowed to do this",
            "Ask an admin to do it for you",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    let mut db = DbService::new();
    let res = db.delete(kind, id);
    match res {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default().embed(create_embed(
                    format!(
                        "{} deleted",
                        match kind {
                            model::DbType::Dare => "Dare",
                            model::DbType::Truth => "Truth",
                        }
                    )
                    .as_str(),
                    "",
                    "",
                    serenity::Colour::BLITZ_BLUE,
                )),
            )
            .await?;
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }
    Ok(())
}
//list dares
#[poise::command(slash_command)]
async fn list_dares(
    ctx: ApplicationContext<'_>,
    #[description = "Page number"] page: Option<i32>,
    #[description = "Rating filter"] rating: Option<Rating>,
    #[description = "Status filter"] status: Option<Status>,
) -> Result<(), Error> {
    let is_admin = helper::is_admin(ctx).await?;
    if !is_admin {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "You are not allowed to do this",
            "Ask an admin to do it for you",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    let mut db = DbService::new();
    let res = db.list_dares_filtered(rating, status);
    let page = page.unwrap_or(0);
    if page < 0 {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "Page number must be greater than 0",
            "",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }

    let page: usize = page.try_into()?;

    match res {
        Ok(dares) => {
            //if dares is empty, return error
            if dares.is_empty() {
                ctx.send(poise::CreateReply::default().embed(create_embed(
                    "No dares found",
                    "Add a new one. or ask admins to approve some",
                    "",
                    serenity::Colour::RED,
                )))
                .await?;
                return Ok(());
            }
            let max_len = (dares.len() + 5 - 1) / 5;
            if page > max_len {
                ctx.send(poise::CreateReply::default().embed(create_embed(
                    "Page number out of range",
                    format!("Max page number is {}", max_len).as_str(),
                    "",
                    serenity::Colour::RED,
                )))
                .await?;
                return Ok(());
            }
            let mut content = String::new();
            for dare in helper::paginate(&dares, page, 5) {
                content.push_str(&format!(
                    "ID:{}, Content: {},{}{}\n",
                    dare.id(),
                    dare.content(),
                    if dare.status() == Status::ACCEPTED {
                        "✅"
                    } else {
                        "❌"
                    },
                    if dare.rating() == Rating::NSFW {
                        "🔥"
                    } else {
                        "💧"
                    }
                ));
            }
            ctx.send(poise::CreateReply::default().embed(create_embed(
                "Dares List",
                format!("{content}").as_str(),
                "",
                serenity::Colour::ORANGE,
            )))
            .await?;
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }
    Ok(())
}
// list truths
#[poise::command(slash_command)]
async fn list_truths(
    ctx: ApplicationContext<'_>,
    #[description = "Page number"] page: Option<i32>,
    #[description = "Rating filter"] rating: Option<Rating>,
    #[description = "Status filter"] status: Option<Status>,
) -> Result<(), Error> {
    let is_admin = helper::is_admin(ctx).await?;
    if !is_admin {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "You are not allowed to do this",
            "Ask an admin to do it for you",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    let mut db = DbService::new();
    let res = db.list_truths_filtered(rating, status);
    let page = page.unwrap_or(0);
    if page < 0 {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "Page number must be greater than 0",
            "",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }

    let page: usize = page.try_into()?;

    match res {
        Ok(truths) => {
            //if truths is empty, return error
            if truths.is_empty() {
                ctx.send(poise::CreateReply::default().embed(create_embed(
                    "No truths found",
                    "Add a new one. or ask admins to approve some.",
                    "",
                    serenity::Colour::RED,
                )))
                .await?;
                return Ok(());
            }
            let max_len = (truths.len() + 5 - 1) / 5;
            if page > max_len {
                ctx.send(poise::CreateReply::default().embed(create_embed(
                    "Page number out of range",
                    format!("Max page number is {}", max_len).as_str(),
                    "",
                    serenity::Colour::RED,
                )))
                .await?;
                return Ok(());
            }
            let mut content = String::new();
            for truth in helper::paginate(&truths, page, 5) {
                content.push_str(&format!(
                    "ID:{}, Content: {},{}{}\n",
                    truth.id(),
                    truth.content(),
                    if truth.status() == Status::ACCEPTED {
                        "✅"
                    } else {
                        "❌"
                    },
                    if truth.rating() == Rating::NSFW {
                        "🔥"
                    } else {
                        "💧"
                    }
                ));
            }
            ctx.send(poise::CreateReply::default().embed(create_embed(
                "Truths List",
                format!("{content}").as_str(),
                "",
                serenity::Colour::ORANGE,
            )))
            .await?;
        }
        Err(e) => {
            println!("{e}");
            error_happened(Box::new(e), Some(ctx)).await?;
        }
    }
    Ok(())
}

// Show Help
#[poise::command(slash_command)]
async fn help(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let help = "**Commands**\n\
    `/add_truth` - Add a truth\n\
    `/add_dare` - Add a dare\n\
    `/get_truth` - Get a random truth\n\
    `/get_dare` - Get a random dare\n\
     admin only \n\
    `/accept` - Accept a truth or dare\n\
    `/delete` - Delete a truth or dare\n\
    `/list_truths` - List all truths\n\
    `/list_dares` - List all dares";
    ctx.send(poise::CreateReply::default().embed(create_embed(
        "Help",
        help,
        "",
        serenity::Colour::ORANGE,
    )))
    .await?;
    Ok(())
}
// This is the main function that runs the bot
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let mut db = DbService::new();
    // Create the database tables if they don't exist
    db.run_migrations();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // register(),
                add_truth(),
                add_dare(),
                get_dare(),
                get_truth(),
                accept(),
                delete(),
                list_dares(),
                list_truths(),
                help(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

//error function
async fn error_happened(e: Error, ctx: Option<ApplicationContext<'_>>) -> Result<(), Error> {
    println!("some kind of error happened: {}", e);
    match ctx {
        None => {}
        Some(ctx) => {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .embed(create_embed(
                        "Some kind of error happened",
                        "Ask a server admin to look at the logs",
                        "",
                        serenity::Colour::RED,
                    )),
            )
            .await?;
        }
    };
    Ok(())
}
//creates an embed
fn create_embed(
    title: &str,
    description: &str,
    footer: &str,
    color: serenity::Color,
) -> CreateEmbed {
    serenity::CreateEmbed::new()
        .title(title)
        .description(description)
        .footer(serenity::CreateEmbedFooter::new(footer))
        .color(color)
}
//discord user id string to username
async fn get_username_from_id(id: &str, ctx: Option<ApplicationContext<'_>>) -> String {
    match ctx {
        None => id.to_string(),
        Some(ctx) => {
            let user = ctx
                .serenity_context()
                .http
                .get_user(serenity::UserId::new(id.parse::<u64>().unwrap()))
                .await;
            match user {
                Ok(user) => user.name,
                Err(_) => id.to_string(),
            }
        }
    }
}
