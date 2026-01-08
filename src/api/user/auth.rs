use std::time::Duration;

use actix_web::{dev::ConnectionInfo, post, web, Error, HttpResponse};
use sea_orm::DbErr;
use tokio::task::{self, JoinHandle};
use tower::{
    common::{
        bincode_decode, bincode_encode, user::*, App, ConfigGetter, TowerResource, Urlable, YesNo,
    },
    config_model::ConfigKey,
    jwt::{JwtString, Principal, Token},
    server_lib::prelude::*,
};
use tracing::warn;

use crate::{
    entity::{user_app, user_device, user_group, user_role},
    service::{
        AddDeviceParam, UserAppService, UserDeviceService, UserGroupService, UserRoleService,
        UserService, UserVipLevelService,
    },
    StateEx,
};

/// 注册
#[post("/register")]
pub async fn register(config: StateEx, body: web::Payload) -> Result<HttpResponse, Error> {
    let mut req: RegisterReq = bin_req(body).await?;

    let password: JwtString = req.password.try_into().unwrap();
    req.password = password.0;

    UserService::add_user(&config.conn, req.into())
        .await
        .into_resp_and(|_| ())
}

/// 登陆
#[post("/login")]
pub async fn login(
    config: StateEx,
    body: web::Payload,
    conn: ConnectionInfo,
) -> Result<HttpResponse, Error> {
    let req: LoginReq = bin_req(body).await?;

    let exist = UserService::get_user_by_username(&config.conn, &req.username)
        .await
        .into_error()?;
    if exist.is_none() {
        return tip_resp("账号不存在。");
    }
    let exist = exist.unwrap();
    let password: JwtString = req.password.try_into().unwrap();
    if password.0 != exist.password {
        return tip_resp("密码错误。");
    }
    if YesNo::No.is(exist.status) {
        return tip_resp("账户被禁用。");
    }

    let device_code: JwtString = req.device_code.try_into().unwrap();
    let add_device = spawn_add_device(
        config.clone(),
        exist.user_id.clone(),
        device_code.0.clone(),
        conn.realip_remote_addr().map(|f| f.to_string()),
    );
    let vip_level = spawn_vip_level(config.clone(), exist.user_id.clone());
    let roles = spawn_roles(config.clone(), exist.user_id.clone());
    let groups = spawn_groups(config.clone(), exist.user_id.clone());
    let apps = spawn_apps(config.clone(), exist.user_id.clone());

    let (add_device, vip_level, roles, groups, apps) =
        tokio::join!(add_device, vip_level, roles, groups, apps);
    join_result(add_device).into_error()?;
    let (roles, groups, apps): (Vec<String>, Option<String>, Vec<String>) = {
        let roles = join_result(roles).into_error()?;
        let groups = join_result(groups).into_error()?;
        let apps = join_result(apps).into_error()?;
        (
            roles.iter().map(|m| m.role_id.clone()).collect(),
            groups.map(|m| m.group_id.clone()),
            apps.iter().map(|m| m.app_id.clone()).collect(),
        )
    };

    let principal = Principal {
        user_id: exist.user_id,
        nickname: exist.nickname.clone(),
        avatar: exist.avatar,
        vip_level: join_result(vip_level).into_error()?,
        roles,
        groups,
        apps,
    };
    let token: Token = principal.into();
    to_resp(Ok(LoginResp {
        access: token.to_string(),
        refresh: get_refresh_token(
            exist.nickname,
            device_code.0,
            config.get_value_or(ConfigKey::RefreshExpiredDays, 7),
        ),
    }))
}

fn spawn_add_device(
    config: StateEx,
    user_id: String,
    device_code: String,
    ip: Option<String>,
) -> JoinHandle<Result<i32, DbErr>> {
    task::spawn(async move {
        UserDeviceService::add_device(
            &config.conn,
            AddDeviceParam {
                user_id: user_id.clone(),
                device_code,
                ip,
            },
        )
        .await
    })
}
fn spawn_vip_level(config: StateEx, user_id: String) -> JoinHandle<Result<String, DbErr>> {
    task::spawn(
        async move { UserVipLevelService::get_vip_level_by_user(&config.conn, &user_id).await },
    )
}
fn spawn_roles(
    config: StateEx,
    user_id: String,
) -> JoinHandle<Result<Vec<user_role::Model>, DbErr>> {
    task::spawn(async move { UserRoleService::get_roles(&config.conn, &user_id).await })
}
fn spawn_apps(config: StateEx, user_id: String) -> JoinHandle<Result<Vec<user_app::Model>, DbErr>> {
    task::spawn(async move { UserAppService::get_apps(&config.conn, &user_id).await })
}
fn spawn_groups(
    config: StateEx,
    user_id: String,
) -> JoinHandle<Result<Option<user_group::Model>, DbErr>> {
    task::spawn(async move { UserGroupService::get_group(&config.conn, &user_id).await })
}

fn get_refresh_token(id: String, device_code: String, days: u64) -> String {
    let source = format!(
        "({},{},{},{})",
        App::TowerServer.to_string(),
        id,
        device_code,
        utc_secs()
    );

    JwtString(bincode_encode(source)).signature(Duration::from_secs(days * 24 * 60 * 60))
}
fn from_refresh_token(val: &str) -> (String, String) {
    let val: String = bincode_decode(val);
    let val: Vec<&str> = val.split(",").collect();
    (
        val.get(1).map(|v| v.to_string()).unwrap_or_default(),
        val.get(2).map(|v| v.to_string()).unwrap_or_default(),
    )
}
/// 刷新token
#[post("/refresh")]
pub async fn refresh(config: StateEx, body: web::Payload) -> Result<HttpResponse, Error> {
    let req: RefreshReq = bin_req(body).await?;

    let Ok(refresh) = JwtString::try_from(req.refresh) else {
        return Err(unauthorized_error());
    };
    let (_, device_code) = from_refresh_token(&refresh);
    let token = Token(req.access);
    let mut principal = token
        .get_attrs()
        .map(|attr| Principal::from(attr))
        .map_err(|_| unauthorized_error())?;

    let latest_device = spawn_latest_device(config.clone(), principal.user_id.clone());
    let vip_level = spawn_vip_level(config.clone(), principal.user_id.clone());
    let roles = spawn_roles(config.clone(), principal.user_id.clone());
    let groups = spawn_groups(config.clone(), principal.user_id.clone());

    let (latest_device, vip_level, roles, groups) =
        tokio::join!(latest_device, vip_level, roles, groups);
    let (roles, groups): (Vec<String>, Option<String>) = {
        let roles = join_result(roles).into_error()?;
        let groups = join_result(groups).into_error()?;
        (
            roles.iter().map(|m| m.role_id.clone()).collect(),
            groups.map(|m| m.group_id.clone()),
        )
    };
    principal.vip_level = join_result(vip_level).into_error()?;
    principal.roles = roles;
    principal.groups = groups;

    join_result(latest_device)
        .into_error()?
        .ok_or(unauthorized_error())
        .and_then(|device| {
            if YesNo::No.is(device.status) {
                return Err(unauthorized_error());
            }
            if device.device_code != device_code {
                warn!(
                    "refresh时失败,latest={}, device={}",
                    device.device_code, device_code
                );
                Err(unauthorized_error())
            } else {
                Ok(())
            }
        })?;
    let token = Token::from(principal);

    success_resp(token.0)
}

fn spawn_latest_device(
    config: StateEx,
    user_id: String,
) -> JoinHandle<Result<Option<user_device::Model>, DbErr>> {
    task::spawn(async move { UserDeviceService::get_latest_device(&config.conn, &user_id).await })
}
/// 修改密码
#[post("/change_password")]
pub async fn change_password(
    principal: TowerPrincipal,
    config: StateEx,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut req: ChangePasswordReq = bin_req(body).await?;

    let password: JwtString = req.password.try_into().unwrap();
    req.password = password.0;

    let password: JwtString = req.new_password.try_into().unwrap();
    req.new_password = password.0;

    //check password
    let exist = UserService::get_user_by_id(&config.conn, &principal.user_id)
        .await
        .into_error()?;
    if exist.is_none() {
        return tip_resp("账号不存在。");
    }
    let exist = exist.unwrap();
    if req.password != exist.password {
        return tip_resp("密码错误。");
    }
    //save change
    let result = UserService::change_password(&config.conn, exist.user_id, req.new_password).await;
    to_resp(result)
}

/// 获取用户信息
#[post("/user_info")]
pub async fn get_user_info(
    principal: TowerPrincipal,
    config: StateEx,
) -> Result<HttpResponse, Error> {
    let exist = UserService::get_user_by_id(&config.conn, &principal.user_id)
        .await
        .into_error()?;

    if exist.is_none() {
        return tip_resp("用户不存在。");
    }
    let exist = exist.unwrap();
    if YesNo::No.is(exist.status) {
        return tip_resp("账户被禁用。");
    }
    to_resp(Ok(UserInfoResp {
        user_id: exist.user_id,
        nickname: exist.nickname,
        avatar: exist.avatar,
    }))
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(TowerResource::Auth.scope())
            .service(register)
            .service(login)
            .service(get_user_info)
            .service(change_password)
            .service(refresh),
    );
}
