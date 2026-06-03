pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let (outer, inner) = flame_shades(streak);
    let num_color = if streak == 0 { "#484f58" } else { "#ffffff" };
    let anim = animation_block(streak);

    let flame = if streak > 0 {
        format!(
            r##"<g class="flame">
    <path d="M48,155 C30,153 21,138 27,122 C31,110 43,104 56,113 C51,93 53,72 64,55 C58,42 55,26 67,18 C74,13 82,20 85,38 C88,22 94,10 100,16 C106,10 112,22 115,38 C118,20 126,13 133,18 C145,26 142,42 136,55 C147,72 149,93 144,113 C157,104 169,110 173,122 C179,138 170,153 152,155 Z" fill="{outer}"/>
    <path d="M62,150 C50,148 44,136 50,122 C53,113 62,107 72,115 C68,97 70,78 79,62 C74,50 72,36 82,30 C87,26 93,32 95,44 C97,30 100,22 103,30 C105,22 113,26 118,30 C128,36 126,50 121,62 C130,78 132,97 128,115 C138,107 147,113 150,122 C156,136 150,148 138,150 Z" fill="{inner}"/>
  </g>"##,
            outer = outer,
            inner = inner,
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
  <text x="100" y="122" text-anchor="middle" style="font:bold 62px system-ui,monospace;fill:{num_color};paint-order:stroke fill;stroke:#0d1117;stroke-width:6;">{streak}</text>
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

fn flame_shades(streak: i64) -> (&'static str, &'static str) {
    match streak {
        1..=6   => ("#cc9900", "#ffd700"),
        7..=13  => ("#cc6600", "#ff9900"),
        14..=29 => ("#cc4400", "#ff6600"),
        30..=59 => ("#cc1a00", "#ff3300"),
        60..=99 => ("#aa0000", "#ee0000"),
        100..=149 => ("#880044", "#cc0066"),
        150..=199 => ("#660088", "#9900cc"),
        200..=364 => ("#440099", "#6600ff"),
        _          => ("#002299", "#0055ff"),
    }
}

fn animation_block(streak: i64) -> String {
    if streak == 0 {
        return String::new();
    }
    let speed: f32 = match streak {
        1..=6   => 2.4,
        7..=29  => 1.8,
        30..=99 => 1.2,
        _       => 0.7,
    };
    format!(
        r#"@keyframes flicker {{
    0%,100%{{transform:scaleX(1) scaleY(1) rotate(0deg);opacity:1}}
    25%{{transform:scaleX(.97) scaleY(1.03) rotate(-1.5deg);opacity:.93}}
    50%{{transform:scaleX(1.02) scaleY(.97) rotate(1deg);opacity:.88}}
    75%{{transform:scaleX(.98) scaleY(1.02) rotate(-.5deg);opacity:.95}}
  }}
  .flame{{transform-origin:100px 155px;animation:flicker {speed}s ease-in-out infinite;}}"#
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
