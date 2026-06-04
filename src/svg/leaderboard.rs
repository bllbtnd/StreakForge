use crate::types::LeaderboardEntry;

pub fn render_leaderboard_html(entries: &[LeaderboardEntry]) -> String {
    let rows = entries.iter().map(html_row).collect::<Vec<_>>().join("\n");

    let empty = if entries.is_empty() {
        r#"<div class="empty">No streaks recorded yet. Visit /card/:username to get started.</div>"#
    } else {
        ""
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width,initial-scale=1"/>
  <title>StreakForge Leaderboard</title>
  <style>
    *{{margin:0;padding:0;box-sizing:border-box}}
    body{{background:#0d1117;color:#c9d1d9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',system-ui,sans-serif;min-height:100vh;display:flex;flex-direction:column;align-items:center;padding:52px 16px}}
    header{{text-align:center;margin-bottom:32px}}
    h1{{font-size:22px;font-weight:700;color:#e6edf3;letter-spacing:.04em}}
    .sub{{font-size:13px;color:#484f58;margin-top:6px}}
    .board{{width:100%;max-width:460px;border:1px solid #30363d;border-radius:10px;overflow:hidden}}
    .row{{display:flex;align-items:center;padding:14px 20px;border-bottom:1px solid #21262d;gap:14px}}
    .row:last-child{{border-bottom:none}}
    .row:hover{{background:#161b22}}
    .rank{{font-size:13px;font-weight:700;width:30px;text-align:right;flex-shrink:0}}
    .name{{flex:1;font-size:14px;color:#c9d1d9}}
    .score{{display:flex;align-items:center;gap:7px;font-size:15px;font-weight:700}}
    .dot{{width:8px;height:8px;border-radius:50%;flex-shrink:0}}
    .empty{{color:#484f58;font-size:14px;padding:32px;text-align:center}}
  </style>
</head>
<body>
  <header>
    <h1>StreakForge</h1>
    <p class="sub">Top contribution streaks</p>
  </header>
  <div class="board">
{rows}{empty}
  </div>
</body>
</html>"##,
        rows = rows,
        empty = empty,
    )
}

fn html_row(entry: &LeaderboardEntry) -> String {
    let rank_color = rank_color(entry.rank);
    let streak_color = streak_color(entry.streak);
    let name = escape_html(&entry.username);

    format!(
        r##"    <div class="row">
      <span class="rank" style="color:{rank_color}">#{rank}</span>
      <span class="name">{name}</span>
      <span class="score">
        <span class="dot" style="background:{streak_color}"></span>
        <span style="color:{streak_color}">{streak}</span>
      </span>
    </div>"##,
        rank_color = rank_color,
        rank = entry.rank,
        name = name,
        streak_color = streak_color,
        streak = entry.streak,
    )
}

fn rank_color(rank: usize) -> &'static str {
    match rank {
        1 => "#ffd700",
        2 => "#c0c0c0",
        3 => "#cd7f32",
        _ => "#484f58",
    }
}

fn streak_color(streak: i64) -> &'static str {
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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
