// fire-1-svgrepo outer contour only (viewBox 0 0 24 24)
// inner sub-paths omitted — they create holes due to winding direction
// transform: translate(-5,-19) scale(8.75) → maps to x:47-152, y:15-155 in our 200×200 card
const FIRE: &str = "M14.4527 8.48679L12.1842 3.93896L11.4471 4.74309\
C7.30945 9.25693 6 11.9609 6 14.2499C6 17.422 8.73452 19.909 12 19.909\
C15.2655 19.909 18 17.422 18 14.2499C18 13.3179 17.6746 12.3124 17.2381 11.3658\
C16.796 10.4069 16.2091 9.44335 15.62 8.57788L15.1065 7.82342L14.4527 8.48679Z";

pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let color = flame_color(streak);
    let num_color = if streak == 0 { "#484f58" } else { "#ffffff" };
    let anim = anim_css(streak);

    let flame = if streak > 0 {
        format!(
            r##"<g class="flame">
    <path transform="translate(-5,-19) scale(8.75)" d="{fire}" fill="{color}"/>
  </g>"##,
            fire = FIRE,
            color = color,
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

fn flame_color(streak: i64) -> &'static str {
    match streak {
        1..=6     => "#ffd700",
        7..=13    => "#ffaa00",
        14..=29   => "#ff6600",
        30..=59   => "#ff3300",
        60..=99   => "#ee1111",
        100..=149 => "#cc0066",
        150..=199 => "#9900cc",
        200..=364 => "#5500ff",
        _         => "#0055ff",
    }
}

fn anim_css(streak: i64) -> String {
    if streak == 0 { return String::new(); }
    let speed: f32 = match streak {
        1..=6   => 2.4,
        7..=29  => 1.8,
        30..=99 => 1.2,
        _       => 0.7,
    };
    format!(
        r#"@keyframes f{{
    0%,100%{{transform:scaleX(1) scaleY(1) rotate(0deg);opacity:1}}
    25%{{transform:scaleX(.97) scaleY(1.03) rotate(-1.5deg);opacity:.93}}
    50%{{transform:scaleX(1.02) scaleY(.97) rotate(1deg);opacity:.88}}
    75%{{transform:scaleX(.98) scaleY(1.02) rotate(-.5deg);opacity:.95}}
  }}
  .flame{{transform-origin:100px 155px;animation:f {speed}s ease-in-out infinite;}}"#
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
