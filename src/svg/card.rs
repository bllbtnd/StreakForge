pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let (outer_color, inner_color) = flame_colors(streak);
    let num_color = if streak == 0 { "#484f58" } else { "#ffffff" };
    let anim = animation_block(streak);

    let flame = if streak > 0 {
        format!(
            r##"<g class="flame">
    <path d="M100,158 C68,144 50,116 58,91 C62,74 76,66 88,80 C86,54 90,30 100,18 C110,30 114,54 112,80 C124,66 138,74 142,91 C150,116 132,144 100,158Z" fill="{o}" opacity="0.75"/>
    <path d="M100,150 C76,137 64,112 70,93 C74,80 83,74 92,84 C90,66 94,46 100,34 C106,46 110,66 108,84 C117,74 126,80 130,93 C136,112 124,137 100,150Z" fill="{i}" opacity="0.9"/>
  </g>"##,
            o = outer_color,
            i = inner_color,
        )
    } else {
        String::new()
    };

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <defs><style>{anim}</style></defs>
  <rect width="200" height="200" rx="10" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <text x="14" y="20" style="font:500 11px system-ui,monospace;fill:#484f58;">@{username}</text>
  {flame}
  <text x="100" y="122" text-anchor="middle" style="font:bold 62px system-ui,monospace;fill:{num_color};paint-order:stroke fill;stroke:#0d1117;stroke-width:4;">{streak}</text>
  <text x="100" y="142" text-anchor="middle" style="font:11px system-ui,monospace;fill:#8b949e;">day streak</text>
  <text x="14" y="184" style="font:10px system-ui,monospace;fill:#484f58;">Longest: {longest_streak}d</text>
  <text x="186" y="184" text-anchor="end" style="font:10px system-ui,monospace;fill:#484f58;">Rank #{rank}</text>
</svg>"##,
        anim = anim,
        username = escape_xml(username),
        flame = flame,
        num_color = num_color,
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

fn flame_colors(streak: i64) -> (&'static str, &'static str) {
    match streak {
        1..=6 => ("#b86e00", "#e3b341"),
        7..=29 => ("#c45200", "#f0883e"),
        30..=99 => ("#b83800", "#ff6b35"),
        _ => ("#991111", "#ff4444"),
    }
}

fn animation_block(streak: i64) -> String {
    if streak == 0 {
        return String::new();
    }
    let speed: f32 = match streak {
        1..=6 => 2.5,
        7..=29 => 2.0,
        30..=99 => 1.3,
        _ => 0.8,
    };
    format!(
        r#"@keyframes flicker {{
    0%,100%{{transform:scaleX(1) scaleY(1);opacity:1}}
    33%{{transform:scaleX(.96) scaleY(1.04);opacity:.92}}
    66%{{transform:scaleX(1.03) scaleY(.97);opacity:.88}}
  }}
  .flame{{transform-origin:100px 158px;animation:flicker {speed}s ease-in-out infinite;}}"#
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
