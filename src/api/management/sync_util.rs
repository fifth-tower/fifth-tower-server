use tokio::task::JoinSet;
use tower::{
    common::{dict::Dict, empty_req, ApiError},
    reqwest::async_post_and,
};
use tracing::error;

use crate::StateEx;

pub async fn sync_dict(state: &StateEx, token: Option<String>) -> Result<(), ApiError> {
    sync(state, Dict::SyncDict, token)
        .await
        .inspect_err(|err| error!("sync_dict error:{:?}", err))
}
pub async fn sync_config(state: &StateEx, token: Option<String>) -> Result<(), ApiError> {
    sync(state, Dict::SyncConfig, token)
        .await
        .inspect_err(|err| error!("sync_config error:{:?}", err))
}

async fn sync(state: &StateEx, dict: Dict, token: Option<String>) -> Result<(), ApiError> {
    match dict {
        Dict::SyncDict => {
            state
                .refresh_dict()
                .await
                .map_err(|err| ApiError::from(err))?;
        }
        Dict::SyncConfig => {
            state
                .refresh_config()
                .await
                .map_err(|err| ApiError::from(err))?;
        }
        _ => todo!(),
    }

    let clients = {
        let dicts = state.dicts.lock().unwrap();
        dicts.get_dict(dict)
    };
    let mut set = JoinSet::new();
    clients.data.iter().for_each(|item| {
        let url = item.value.clone();
        let token = token.clone();
        set.spawn(async move {
            let ret: Result<(), ApiError> =
                async_post_and(&url.clone(), &empty_req(), token.clone()).await;
            ret
        });
    });
    while let Some(res) = set.join_next().await {
        match res {
            Ok(res) => res?,
            Err(err) => return Err(ApiError::from(err)),
        };
    }
    Ok(())
}
