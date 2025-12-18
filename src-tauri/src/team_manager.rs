use serde::{Deserialize, Serialize};
use crate::http_client::create_proxy_client;

/// 团队用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamUser {
    pub id: String,
    pub email: String,
    pub role: String,
    pub joined_at: String,
}

/// 团队邀请信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamInvitation {
    pub id: String,
    pub email: String,
    pub invited_at: String,
}

/// 团队设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSettings {
    pub discoverable: bool,
}

/// 团队信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub users: Vec<TeamUser>,
    pub seats: i32,
    pub invitations: Vec<TeamInvitation>,
    pub settings: TeamSettings,
    pub join_requests: Vec<serde_json::Value>,
}

/// 团队响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<Team>,
}

/// 邀请请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteRequest {
    pub emails: Vec<String>,
}

/// 获取团队信息
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
pub async fn get_team_info(auth_session: &str) -> Result<TeamResponse, String> {
    println!("🔍 [Team Manager] 获取团队信息...");
    println!("📝 [Team Manager] Auth Session 长度: {}", auth_session.len());

    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    println!("✅ [Team Manager] App Session 长度: {}", app_session.len());

    // 使用 app_session 作为 _session Cookie
    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    println!("📡 [Team Manager] 发送请求到: https://app.augmentcode.com/api/team");

    let response = client
        .get("https://app.augmentcode.com/api/team")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch team info: {}", e))?;

    let status = response.status();
    println!("✅ [Team Manager] 响应状态码: {}", status);

    if !status.is_success() {
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("❌ [Team Manager] API 错误: {} - {}", status, body);
        return Err(format!("API returned error {}: {}", status, body));
    }

    // 先获取原始文本,用于调试
    let response_text = response.text().await
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    println!("📦 [Team Manager] API 响应: {}", response_text);

    // 尝试解析为 JSON
    let json_value: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // 检查响应结构
    if let Some(status) = json_value.get("status").and_then(|s| s.as_str()) {
        if status == "none" {
            // 没有团队信息
            return Ok(TeamResponse {
                status: status.to_string(),
                team: None,
            });
        }
    }

    // 尝试解析完整的团队响应
    serde_json::from_str::<TeamResponse>(&response_text)
        .map_err(|e| format!("Failed to parse team info: {}", e))
}

/// 邀请团队成员
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
pub async fn invite_team_members(auth_session: &str, emails: Vec<String>) -> Result<(), String> {
    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    // 使用 app_session 作为 _session Cookie
    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    let invite_request = InviteRequest { emails };

    let response = client
        .post("https://app.augmentcode.com/api/team/invite")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Content-Type", "application/json")
        .header("Cookie", cookie_str)
        .json(&invite_request)
        .send()
        .await
        .map_err(|e| format!("Failed to invite team members: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API returned error {}: {}", status, body));
    }

    Ok(())
}

/// 移除团队成员
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
pub async fn remove_team_member(auth_session: &str, user_id: &str) -> Result<(), String> {
    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    // 使用 app_session 作为 _session Cookie
    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    let url = format!("https://app.augmentcode.com/api/team/user/{}", user_id);

    let response = client
        .delete(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to remove team member: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API returned error {}: {}", status, body));
    }

    Ok(())
}

/// 取消团队邀请
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
pub async fn cancel_team_invitation(auth_session: &str, invitation_id: &str) -> Result<(), String> {
    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    // 使用 app_session 作为 _session Cookie
    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    let url = format!("https://app.augmentcode.com/api/team/invite/{}", invitation_id);

    let response = client
        .delete(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to cancel team invitation: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API returned error {}: {}", status, body));
    }

    Ok(())
}

/// 刷新 session (通过调用 API 获取新的 session)
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
/// 返回: 新的 app_session (可以直接作为 _session Cookie 使用)
pub async fn refresh_session(auth_session: &str) -> Result<String, String> {
    println!("🔄 [Session Refresh] 开始刷新 session...");
    println!("📝 [Session Refresh] Auth Session 长度: {}", auth_session.len());

    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    println!("✅ [Session Refresh] App Session 长度: {}", app_session.len());

    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    println!("📡 [Session Refresh] 调用 API 刷新 session...");

    // 调用 /api/team 来刷新 session (轻量级接口)
    let response = client
        .get("https://app.augmentcode.com/api/team")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to refresh session: {}", e))?;

    let status = response.status();
    println!("✅ [Session Refresh] 响应状态码: {}", status);

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("❌ [Session Refresh] 刷新失败: {}", error_text);
        return Err(format!("Failed to refresh session: {} - {}", status, error_text));
    }

    // 从响应头中提取新的 _session
    if let Some(set_cookie) = response.headers().get("set-cookie") {
        if let Ok(cookie_value) = set_cookie.to_str() {
            println!("🍪 [Session Refresh] Set-Cookie: {}", cookie_value);

            // 解析 _session=xxx; 格式
            if let Some(session_start) = cookie_value.find("_session=") {
                let session_part = &cookie_value[session_start + 9..];
                if let Some(session_end) = session_part.find(';') {
                    let new_session = &session_part[..session_end];
                    println!("✅ [Session Refresh] 获得新 session,长度: {}", new_session.len());
                    return Ok(new_session.to_string());
                } else {
                    // 没有分号,说明 session 到字符串末尾
                    let new_session = session_part.trim();
                    println!("✅ [Session Refresh] 获得新 session,长度: {}", new_session.len());
                    return Ok(new_session.to_string());
                }
            }
        }
    }

    println!("⚠️ [Session Refresh] 响应中没有新 session,返回原 app_session");
    // 如果没有新 session,返回原 app_session
    Ok(app_session)
}

/// 使用现有 session 刷新 (不需要 auth_session)
/// 参数 current_session: 当前的 _session Cookie
/// 返回: 新的 _session Cookie
pub async fn refresh_session_with_current(current_session: &str) -> Result<String, String> {
    println!("🔄 [Session Refresh] 使用现有 session 刷新...");
    println!("📝 [Session Refresh] Current Session 长度: {}", current_session.len());

    let cookie_str = format!("_session={}", current_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    println!("📡 [Session Refresh] 调用 API 刷新 session...");

    // 调用 /api/team 来刷新 session (轻量级接口)
    let response = client
        .get("https://app.augmentcode.com/api/team")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to refresh session: {}", e))?;

    let status = response.status();
    println!("✅ [Session Refresh] 响应状态码: {}", status);

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        println!("❌ [Session Refresh] 刷新失败: {}", error_text);
        return Err(format!("Failed to refresh session: {} - {}", status, error_text));
    }

    // 从响应头中提取新的 _session
    if let Some(set_cookie) = response.headers().get("set-cookie") {
        if let Ok(cookie_value) = set_cookie.to_str() {
            println!("🍪 [Session Refresh] Set-Cookie: {}", cookie_value);

            // 解析 _session=xxx; 格式
            if let Some(session_start) = cookie_value.find("_session=") {
                let session_part = &cookie_value[session_start + 9..];
                if let Some(session_end) = session_part.find(';') {
                    let new_session = &session_part[..session_end];
                    println!("✅ [Session Refresh] 获得新 session,长度: {}", new_session.len());
                    return Ok(new_session.to_string());
                } else {
                    // 没有分号,说明 session 到字符串末尾
                    let new_session = session_part.trim();
                    println!("✅ [Session Refresh] 获得新 session,长度: {}", new_session.len());
                    return Ok(new_session.to_string());
                }
            }
        }
    }

    println!("⚠️ [Session Refresh] 响应中没有新 session,返回原 session");
    // 如果没有新 session,返回原 session
    Ok(current_session.to_string())
}

/// 检查 session 是否有效
/// 参数 auth_session: 用户输入的原始 session (需要转换为 app_session)
/// 返回: true 表示有效, false 表示已过期
pub async fn check_session_validity(auth_session: &str) -> Result<bool, String> {
    println!("🔍 [Session Check] 检查 session 有效性...");

    // 先交换 auth_session 为 app_session
    let app_session = crate::augment_user_info::exchange_auth_session_for_app_session(auth_session)
        .await
        .map_err(|e| format!("Failed to exchange auth_session: {}", e))?;

    let cookie_str = format!("_session={}", app_session);

    // 使用 ProxyClient
    let client = create_proxy_client()?;

    let response = client
        .get("https://app.augmentcode.com/api/team")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", cookie_str)
        .send()
        .await
        .map_err(|e| format!("Failed to check session: {}", e))?;

    let status = response.status();
    let is_valid = status.as_u16() != 401;

    if is_valid {
        println!("✅ [Session Check] Session 有效");
    } else {
        println!("❌ [Session Check] Session 已过期 (401)");
    }

    Ok(is_valid)
}
