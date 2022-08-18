use super::super::command_parser::InteractionHandleError;
use config::Config;
use serenity::builder::CreateInteractionResponse;
use serenity::http::Http;
use serenity::model::application::component::ActionRowComponent;
use serenity::model::application::interaction::modal::ModalSubmitInteraction;
use serenity::model::id::ChannelId;
use serenity::utils::Color;

pub async fn apply(
    cmd: ModalSubmitInteraction,
    client: Http,
) -> Result<CreateInteractionResponse<'static>, InteractionHandleError> {
    // probably dont want to load a config object every time we run a command, but meh.
    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();
    let notification_chan_id: u64 = settings.get("apply.notification_chan_id").unwrap();
    let target_channel = ChannelId(notification_chan_id);

    let submitted_name = match cmd.data.components[0].components[0].clone() {
        ActionRowComponent::InputText(input_text) => input_text.value,
        _ => "".to_string(),
    };

    let _ = target_channel
        .send_message(client, |m| {
            m.embed(|embed| {
                embed.title("Guild Application");
                embed.description(format!(
                    "<@{}> `{}#{}` [`{}`] - {}",
                    cmd.user.id, cmd.user.name, cmd.user.discriminator, cmd.user.id, submitted_name
                ));
                embed.color(Color::DARK_GREEN)
            })
        })
        .await;

    let mut resp = CreateInteractionResponse::default();

    resp.interaction_response_data(|rdata| {
        rdata.content("Thank you for submitting your application, someone will reach out to you soon about joining the guild.");
        rdata.ephemeral(true);
        rdata
    });

    Ok(resp)
}
