mod follow;
pub use follow::*;

mod favorite;
pub use favorite::*;

mod like;
pub use like::*;

mod score;
pub use score::*;

mod comment;
pub use comment::*;

mod config;
pub use config::*;

mod user;
pub use user::*;

mod user_vip_level;
pub use user_vip_level::*;

mod user_device;
pub use user_device::*;

mod social_report;
pub use social_report::*;

mod dict;
pub use dict::*;

mod dict_item;
pub use dict_item::*;

mod user_feed_back;
pub use user_feed_back::*;

mod user_role;
pub use user_role::*;

mod user_app;
pub use user_app::*;

mod user_group;
pub use user_group::*;

mod management;
pub use management::*;

#[cfg(test)]
mod tests {
    use tracing::debug;

    use crate::{config::AppState, get_env};

    pub async fn init_state() -> AppState {
        let vars = get_env(true);
        debug!("vars:{:?}", vars);
        AppState::new(&vars).await
    }
}
