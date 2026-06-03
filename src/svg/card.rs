// fire-symbol paths (original viewBox 0 0 32 32):
// translate(26.9,13.4) scale(4.57) maps them to x:40-160, y:18-155 in our 200x200 card
const FIRE_PATHS: &str = "M16 1c-3.282 15.79-13.125 17.366 0 28.418 13.126-11.053 3.282-12.628 0-28.418z\
M7.797 10.463c-1.641 4.736-4.922 7.919-4.922 12.656s6.562 7.881 11.485 7.881c-9.844-7.896-6.563-12.643-6.563-20.537z\
M24.203 10.463c0 7.895 3.282 12.642-6.562 20.537 4.922 0 11.485-3.144 11.485-7.881s-3.282-7.92-4.922-12.656z";

// center tongue only, scaled slightly smaller for the hot-core highlight
const CORE_PATH: &str = "M16 1c-3.282 15.79-13.125 17.366 0 28.418 13.126-11.053 3.282-12.628 0-28.418z";

pub fn render_card(username: &str, streak: i64, longest_streak: i64, rank: i64) -> String {
    let (outer_color, inner_color) = shades(streak);
    let num_color = if streak == 0 { "#484f58" } else { "#ffffff" };
    let anim = anim_css(streak);

    let flame = if streak > 0 {
        format!(
            r##"<g class="flame">
    <path transform="translate(26.9,13.4) scale(4.57)" d="{fire}" fill="{oc}"/>
    <path transform="translate(36,26) scale(4.0)" d="{core}" fill="{ic}"/>
  </g>"##,
            fire = FIRE_PATHS,
            core = CORE_PATH,
            oc = outer_color,
            ic = inner_color,
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

fn shades(streak: i64) -> (&'static str, &'static str) {
    match streak {
        1..=6     => ("#e6a800", "#ffd700"),
        7..=13    => ("#e67300", "#ffaa00"),
        14..=29   => ("#cc4400", "#ff6600"),
        30..=59   => ("#cc1a00", "#ff3300"),
        60..=99   => ("#aa0000", "#ee1111"),
        100..=149 => ("#880044", "#cc0066"),
        150..=199 => ("#660088", "#9900cc"),
        200..=364 => ("#3300aa", "#5500ff"),
        _         => ("#0033bb", "#0055ff"),
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
