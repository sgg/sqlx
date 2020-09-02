use futures::TryFutureExt;
use log::*;
use serde::Serialize;
use sqlx::pool::PoolConnection;
use sqlx::{Connect, Connection, Database};
use tide::{Error, IntoResponse, Request, Response, ResultExt};

use crate::api::model::*;
use crate::api::util::*;
use crate::db::model::*;
use crate::db::Db;

#[derive(Serialize)]
struct ProfileResponseBody {
    profile: Profile,
}

impl From<Profile> for ProfileResponseBody {
    fn from(profile: Profile) -> Self {
        ProfileResponseBody { profile }
    }
}

/// Retrieve a profile by username
///
/// [Get Profile](https://github.com/gothinkster/realworld/tree/master/api#get-profile)
pub async fn get_profile<DB>(
    req: Request<impl Db<Conn = PoolConnection<DB>>>,
) -> Response
    where DB: Connect + ProvideData + Database
{
    async move {
        let authenticated = optionally_auth(&req).transpose()?;

        let leader_username = req.param::<String>("username").client_err()?;
        debug!("Searching for profile {}", leader_username);

        let state = req.state();
        let mut tx = state
            .conn()
            .and_then(Connection::begin)
            .await
            .server_err()?;

        let leader = tx.get_profile_by_username(&leader_username).await?;

        debug!("Found profile for {}", leader_username);

        let is_following = if let Some((follower_id, _)) = authenticated {
            tx.is_following(leader.user_id, follower_id).await?
        } else {
            false
        };
        tx.commit().await.server_err()?;

        let resp = to_json_response(&ProfileResponseBody {
            profile: Profile::from(leader).following(is_following),
        })?;
        Ok::<_, Error>(resp)
    }
    .await
    .unwrap_or_else(IntoResponse::into_response)
}

/// Follow a user
///
/// [Follow User](https://github.com/gothinkster/realworld/tree/master/api#follow-user)
pub async fn follow_user<DB>(
    req: Request<impl Db<Conn = PoolConnection<DB>>>,
) -> Response
    where DB: Connect + ProvideData + Database
{
    should_follow(req, true)
        .await
        .unwrap_or_else(IntoResponse::into_response)
}

/// Stop following a user
///
/// [Unfollow User](https://github.com/gothinkster/realworld/tree/master/api#unfollow-user)
pub async fn unfollow_user<DB>(
    req: Request<impl Db<Conn = PoolConnection<DB>>>,
) -> Response
    where DB: Connect + ProvideData + Database
{
    should_follow(req, false)
        .await
        .unwrap_or_else(IntoResponse::into_response)
}

/// Adds or removes a following relationship
async fn should_follow<DB>(
    req: Request<impl Db<Conn = PoolConnection<DB>>>,
    should_follow: bool,
) -> tide::Result<Response>
    where DB: Connect + ProvideData + Database
{
    let (user_id, _) = extract_and_validate_token(&req)?;

    let leader_username = req.param::<String>("username").client_err()?;

    let state = req.state();
    let mut tx = state
        .conn()
        .and_then(Connection::begin)
        .await
        .server_err()?;

    let leader_ent = tx.get_profile_by_username(&leader_username).await?;

    match should_follow {
        true => {
            debug!("User {} will now follow {}", user_id, leader_username);
            tx.add_follower(&leader_username, user_id).await
        }
        false => {
            debug!("User {} will no longer follow {}", user_id, leader_username);
            tx.delete_follower(&leader_username, user_id).await
        }
    }?;

    tx.commit().await.server_err()?;

    let profile = Profile::from(leader_ent).following(should_follow);

    let resp = to_json_response(&ProfileResponseBody::from(profile))?;
    Ok(resp)
}
