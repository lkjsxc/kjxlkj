import { execFileSync } from 'node:child_process';

export function resetDatabase(databaseUrl) {
    runSql(
        databaseUrl,
        "TRUNCATE app_settings, sessions, resource_daily_views, resource_snapshots, resources, admin_user RESTART IDENTITY CASCADE; " +
            "INSERT INTO app_settings " +
            "(id, home_recent_limit, home_favorite_limit, home_popular_limit, home_intro_markdown, " +
            "home_recent_visible, home_favorite_visible, home_popular_visible, " +
            "home_recent_position, home_favorite_position, home_popular_position, " +
            "search_results_per_page, session_timeout_minutes, default_new_resource_is_private, site_name, site_description, public_base_url) " +
            "VALUES (1, 5, 5, 5, $$# Home\n\nWelcome to **kjxlkj**. Use Home as the landing space for search, popular resources, and curated favorites.$$," +
            " TRUE, TRUE, TRUE, 1, 2, 3, 20, 1440, FALSE, 'kjxlkj', 'Markdown-first resource system for LLM-operated workflows.', '')"
    );
}

export function seedViewAnalytics(databaseUrl, notes) {
    runSql(
        databaseUrl,
        `INSERT INTO resource_daily_views (resource_id, view_date, view_count) VALUES
('${notes.middle.id}', CURRENT_DATE, 2),
('${notes.middle.id}', CURRENT_DATE - 10, 7),
('${notes.middle.id}', CURRENT_DATE - 50, 1),
('${notes.newest.id}', CURRENT_DATE, 9),
('${notes.newest.id}', CURRENT_DATE - 20, 1),
('${notes.oldest.id}', CURRENT_DATE - 40, 14);
UPDATE resources AS r
SET view_count_total = seeded.total, last_viewed_at = seeded.last_viewed_at
FROM (VALUES
('${notes.middle.id}', 10::BIGINT, NOW()),
('${notes.newest.id}', 10::BIGINT, NOW()),
('${notes.oldest.id}', 14::BIGINT, NOW() - INTERVAL '40 days')
) AS seeded(id, total, last_viewed_at)
WHERE r.id = seeded.id;`
    );
}

function runSql(databaseUrl, sql) {
    execFileSync('psql', [databaseUrl, '-v', 'ON_ERROR_STOP=1', '-c', sql], { stdio: 'inherit' });
}
