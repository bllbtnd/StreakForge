pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let streak_color = streak_color(streak);
    let flame_color = flame_color(streak);
    let anim_speed = anim_speed(streak);
    let anim_style = if streak > 0 {
        format!(
            r#"@keyframes flicker {{0%,100%{{transform:scaleY(1);opacity:1}}50%{{transform:scaleY(.9);opacity:.8}}}}
      .fl{{transform-origin:7px 14px;animation:flicker {anim_speed}s ease-in-out infinite;}}"#
        )
    } else {
        String::new()
    };
    let flame_class = if streak > 0 { r#" class="fl""# } else { "" };

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <defs><style>{anim_style}</style></defs>
  <rect width="200" height="200" rx="10" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <g transform="translate(14,14)"{flame_class}>
    <path d="M7 0C7 0 3 4 3 7.5C3 10 4.8 12 7 12.5C5.8 11.8 5 10.5 5.4 9C6 10 7 10.5 8.6 9.8C10.6 8.8 11 6 9.5 4C9 5.5 8 5.5 7.3 5C7.8 3.5 8 1.5 7 0Z" fill="{flame_color}"/>
  </g>
  <text x="30" y="24" style="font:600 11px system-ui,monospace;fill:#8b949e;">@{username}</text>
  <text x="100" y="116" text-anchor="middle" style="font:bold 60px system-ui,monospace;fill:{streak_color};">{streak}</text>
  <text x="100" y="134" text-anchor="middle" style="font:11px system-ui,monospace;fill:#8b949e;">day streak</text>
  <text x="14" y="182" style="font:10px system-ui,monospace;fill:#484f58;">Longest: {longest_streak}d</text>
  <text x="186" y="182" text-anchor="end" style="font:10px system-ui,monospace;fill:#484f58;">Rank #{rank}</text>
</svg>"##,
        anim_style = anim_style,
        flame_class = flame_class,
        flame_color = flame_color,
        username = escape_xml(username),
        streak_color = streak_color,
        streak = streak,
        longest_streak = longest_streak,
        rank = rank,
    )
}

pub fn render_error_card(message: &str) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <rect width="200" height="200" rx="10" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <rect x="0" y="0" width="3" height="200" rx="1" fill="#f85149"/>
  <text x="100" y="92" text-anchor="middle" style="font:bold 13px system-ui,monospace;fill:#f85149;">&#x26A0; Error</text>
  <text x="100" y="114" text-anchor="middle" style="font:10px system-ui,monospace;fill:#8b949e;">{message}</text>
</svg>"##,
        message = escape_xml(message),
    )
}

fn streak_color(streak: i64) -> &'static str {
    match streak {
        0 => "#484f58",
        1..=6 => "#e3b341",
        7..=29 => "#f0883e",
        30..=99 => "#ff6b35",
        _ => "#ff4444",
    }
}

fn flame_color(streak: i64) -> &'static str {
    match streak {
        0..=6 => "#e3b341",
        7..=29 => "#f0883e",
        30..=99 => "#ff6b35",
        _ => "#ff4444",
    }
}

fn anim_speed(streak: i64) -> f32 {
    match streak {
        7..=29 => 2.0,
        30..=99 => 1.2,
        _ => 0.7,
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
