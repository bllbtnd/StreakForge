use crate::types::LeaderboardEntry;

pub fn render_leaderboard(entries: &[LeaderboardEntry]) -> String {
    let rows = entries.iter().map(render_row).collect::<Vec<_>>().join("\n");
    let height = 420usize;

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="{height}" viewBox="0 0 400 {height}">
  <defs>
    <style>
      .title {{ font: 14px system-ui, monospace; fill: #8b949e; }}
      .rank {{ font: bold 13px system-ui, monospace; }}
      .uname {{ font: 13px system-ui, monospace; fill: #c9d1d9; }}
      .streak {{ font: bold 13px system-ui, monospace; fill: #c9d1d9; }}
      @keyframes shimmer {{ 0%,100%{{opacity:1}} 50%{{opacity:.6}} }}
      .shimmer {{ animation: shimmer 2s ease-in-out infinite; }}
    </style>
  </defs>
  <rect width="400" height="{height}" rx="12" fill="#0d1117" stroke="#30363d" stroke-width="1"/>
  <text x="200" y="30" text-anchor="middle" class="title">StreakForge Leaderboard</text>
  <line x1="16" y1="42" x2="384" y2="42" stroke="#30363d" stroke-width="1"/>
{rows}
</svg>"#,
        height = height,
        rows = rows,
    )
}

fn render_row(entry: &LeaderboardEntry) -> String {
    let y_base = 42 + (entry.rank - 1) * 36;
    let text_y = y_base + 23;
    let rank_color = rank_color(entry.rank);
    let shimmer = if entry.rank == 1 { r#" class="shimmer""# } else { "" };

    let flame = small_flame(entry.streak);

    format!(
        r#"  <g{shimmer}>
    <text x="24" y="{text_y}" class="rank" fill="{rank_color}">#{rank}</text>
    <text x="60" y="{text_y}" class="uname">{username}</text>
    <g transform="translate(348,{flame_y})">
      {flame}
    </g>
    <text x="372" y="{text_y}" text-anchor="end" class="streak">{streak}</text>
  </g>"#,
        shimmer = shimmer,
        text_y = text_y,
        rank_color = rank_color,
        rank = entry.rank,
        username = escape_xml(&entry.username),
        flame_y = y_base + 10,
        flame = flame,
        streak = entry.streak,
    )
}

fn rank_color(rank: usize) -> &'static str {
    match rank {
        1 => "#ffd700",
        2 => "#c0c0c0",
        3 => "#cd7f32",
        _ => "#8b949e",
    }
}

fn small_flame(streak: i64) -> &'static str {
    match streak {
        0..=6 => r#"<path d="M4 12C4 12 1 9 2 6C2.5 4 3.5 4 4 6C4.5 4 5.5 4 6 6C7 9 4 12 4 12Z" fill="#e3b341"/>"#,
        7..=29 => r#"<path d="M4 12C4 12 1 9 2 6C2.5 4 3.5 4 4 6C4.5 4 5.5 4 6 6C7 9 4 12 4 12Z" fill="#f0883e"/>"#,
        30..=99 => r#"<path d="M4 12C4 12 1 9 2 6C2.5 4 3.5 4 4 6C4.5 4 5.5 4 6 6C7 9 4 12 4 12Z" fill="#ff6b35"/>"#,
        _ => r#"<path d="M4 12C4 12 1 9 2 6C2.5 4 3.5 4 4 6C4.5 4 5.5 4 6 6C7 9 4 12 4 12Z" fill="#ff4444"/>"#,
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
