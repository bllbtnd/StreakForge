pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let streak_color = streak_color(streak);
    let flame_svg = flame_svg(streak);
    let streak_label = streak.to_string();
    let longest_label = format!("Longest: {} days", longest_streak);
    let rank_label = format!("Rank: #{}", rank);

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="200" viewBox="0 0 400 200">
  <defs>
    <style>
      .username {{ font: bold 16px system-ui, monospace; fill: #c9d1d9; }}
      .streak-num {{ font: bold 64px system-ui, monospace; fill: {streak_color}; }}
      .streak-label {{ font: 14px system-ui, monospace; fill: #8b949e; }}
      .meta {{ font: 12px system-ui, monospace; fill: #8b949e; }}
      {animation}
    </style>
    {defs}
  </defs>
  <rect width="400" height="200" rx="12" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <g transform="translate(16, 20)">
    {flame_svg}
    <text x="44" y="20" class="username">@{username}</text>
  </g>
  <text x="200" y="120" text-anchor="middle" class="streak-num">{streak_label}</text>
  <text x="200" y="142" text-anchor="middle" class="streak-label">day streak</text>
  <text x="16" y="185" class="meta">{longest_label}</text>
  <text x="384" y="185" text-anchor="end" class="meta">{rank_label}</text>
</svg>"##,
        streak_color = streak_color,
        animation = animation_css(streak),
        defs = gradient_defs(streak),
        flame_svg = flame_svg,
        username = escape_xml(username),
        streak_label = streak_label,
        longest_label = longest_label,
        rank_label = rank_label,
    )
}

pub fn render_error_card(message: &str) -> String {
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="200" viewBox="0 0 400 200">
  <rect width="400" height="200" rx="12" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <rect x="0" y="0" width="4" height="200" rx="2" fill="#f85149"/>
  <text x="200" y="90" text-anchor="middle" style="font: bold 18px system-ui, monospace; fill: #f85149;">&#x26A0; Error</text>
  <text x="200" y="118" text-anchor="middle" style="font: 13px system-ui, monospace; fill: #8b949e;">{message}</text>
</svg>"##,
        message = escape_xml(message),
    )
}

fn streak_color(streak: i64) -> &'static str {
    match streak {
        0..=6 => "#e3b341",
        7..=29 => "#f0883e",
        30..=99 => "#ff6b35",
        _ => "#ff4444",
    }
}

fn flame_svg(streak: i64) -> String {
    let (height, color, anim_class) = match streak {
        0..=6 => (18i32, "#e3b341", ""),
        7..=29 => (22i32, "#f0883e", r#" class="flicker-gentle""#),
        30..=99 => (26i32, "#ff6b35", r#" class="flicker-active""#),
        _ => (30i32, "#ff4444", r#" class="flicker-intense""#),
    };
    let y = 24 - height;
    let mid = height - 2;
    let quarter = height / 2;
    let low = height / 3;
    let low2 = height / 4;
    let mid2 = height - 4;
    format!(
        r#"<g transform="translate(14,{y})"{anim_class}>
    <path d="M8 {height}C8 {height} 2 {mid} 4 {quarter}C5 {low} 7 {low2} 8 {mid2}C9 {low2} 11 {low} 12 {quarter}C14 {mid} 8 {height} 8 {height}Z" fill="{color}"/>
  </g>"#,
    )
}

fn animation_css(streak: i64) -> &'static str {
    match streak {
        0..=6 => "",
        7..=29 => r#"
      @keyframes flicker-gentle { 0%,100%{opacity:1;transform:scaleY(1)} 50%{opacity:.85;transform:scaleY(.95)} }
      .flicker-gentle { animation: flicker-gentle 2s ease-in-out infinite; transform-origin: bottom center; }"#,
        30..=99 => r#"
      @keyframes flicker-active { 0%,100%{opacity:1;transform:scaleY(1)} 30%{opacity:.8;transform:scaleY(.9)} 70%{opacity:.9;transform:scaleY(1.05)} }
      .flicker-active { animation: flicker-active 1.2s ease-in-out infinite; transform-origin: bottom center; }"#,
        _ => r#"
      @keyframes flicker-intense { 0%,100%{opacity:1;transform:scaleY(1) rotate(-1deg)} 25%{opacity:.7;transform:scaleY(.85) rotate(1deg)} 75%{opacity:.9;transform:scaleY(1.1) rotate(-1deg)} }
      .flicker-intense { animation: flicker-intense 0.8s ease-in-out infinite; transform-origin: bottom center; }"#,
    }
}

fn gradient_defs(streak: i64) -> &'static str {
    if streak >= 100 {
        r##"<linearGradient id="flame-grad" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="#ff0000"/>
      <stop offset="100%" stop-color="#ff6b35"/>
    </linearGradient>"##
    } else {
        ""
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
