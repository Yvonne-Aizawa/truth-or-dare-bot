use crate::helper;
use crate::helper::ApplicationContext;
use crate::helper::Error;
use crate::helper::create_embed;
use crate::helper::error_happened;
use crate::helper::get_username_from_id;
use database::model::UpdateDare;
use database::service::DbService;
use database::*;
use model::{NewDare, NewTruth, Rating, Status, UpdateTruth};
use poise::serenity_prelude::{self as serenity};
#[poise::command(slash_command)]
pub async fn register(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(poise::Context::Application(ctx))
        .await?;
    Ok(())
}
//Add a truth
#[poise::command(slash_command)]
pub async fn add_truth(
    ctx: ApplicationContext<'_>,
    #[description = "Truth content"] content: String,
    #[description = "Truth rating"] rating: Rating,
) -> Result<(), Error> {
    // Check if the user is in a safe channel and the rating is not safe
    let is_safe_channel = !helper::is_age_gated(ctx).await?;
    // if we are in a safe channel and the rating is not safe, return error
    if is_safe_channel && rating == Rating::NSFW {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "This channel is not safe for this content",
            "Please use a different channel",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    if helper::check_and_update_cooldown(&ctx).await? {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "Cooldown active",
            "Please wait before submitting another truth.",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
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
            db.new_moderation(
                "Truth Added".to_string(),
                "Truth added by user".to_string(),
                truth.id(),
                ctx.author().id.get().to_string(),
                None,
            )?;
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
pub async fn add_dare(
    ctx: ApplicationContext<'_>,
    #[description = "Dare content"] content: String,
    #[description = "Dare rating"] rating: Rating,
) -> Result<(), Error> {
    // Check if the user is in a safe channel and the rating is not safe
    let is_safe_channel = !helper::is_age_gated(ctx).await?;
    // if we are in a safe channel and the rating is not safe, return error
    if is_safe_channel && rating == Rating::NSFW {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "This channel is not safe for this content",
            "Please use a different channel",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }
    if helper::check_and_update_cooldown(&ctx).await? {
        ctx.send(poise::CreateReply::default().embed(create_embed(
            "Cooldown active",
            "Please wait before submitting another dare.",
            "",
            serenity::Colour::RED,
        )))
        .await?;
        return Ok(());
    }

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
            db.new_moderation(
                "Dare Added".to_string(),
                "Dare added by user".to_string(),
                dare.id(),
                ctx.author().id.get().to_string(),
                None,
            )?;
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
pub async fn get_dare(
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
pub async fn get_truth(
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
pub async fn accept(
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
                            Ok(t) => {
                                db.new_moderation(
                                    "Truth Accepted".to_string(),
                                    "truth accepted by admin".to_string(),
                                    t.id(),
                                    ctx.author().id.get().to_string(),
                                    None,
                                )?;
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
                            Ok(d) => {
                                db.new_moderation(
                                    "Dare Accepted".to_string(),
                                    "Dare accepted by admin".to_string(),
                                    d.id(),
                                    ctx.author().id.get().to_string(),
                                    None,
                                )?;
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

// Reject a truth/dare
#[poise::command(slash_command)]
pub async fn reject(
    ctx: ApplicationContext<'_>,
    kind: database::model::DbType,
    id: i32,
    #[description = "Optional reason for rejection"] reason: Option<String>,
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
    let res = db.reject(kind, id);
    match res {
        Ok(_) => {
            db.new_moderation(
                "Suggestion Rejected".to_string(),
                format!("{kind} rejected by admin").to_string(),
                id,
                ctx.author().id.get().to_string(),
                reason,
            )?;
            ctx.send(
                poise::CreateReply::default().embed(create_embed(
                    format!(
                        "{} rejected",
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

//delete a truth/dare
#[poise::command(slash_command)]
pub async fn delete(
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
            db.new_moderation(
                "Suggestion Deleted".to_string(),
                format!("{kind} deleted by admin").to_string(),
                id,
                ctx.author().id.get().to_string(),
                None,
            )?;
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
pub async fn list_dares(
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
pub async fn list_truths(
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
pub async fn help(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let help = "**Commands**\n\
    `/add_truth` - Add a truth\n\
    `/add_dare` - Add a dare\n\
    `/get_truth` - Get a random truth\n\
    `/get_dare` - Get a random dare\n\
     admin only \n\
    `/accept` - Accept a truth or dare\n\
    `/reject` - Reject a truth or dare\n\
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
