SELECT
  channelId,
  count(videoPublished) total,
  avg(`totalviews/channelelapsedtime`) avg_view,
  sum(elapsedtime) total_view_time
FROM
  youtube
GROUP BY
  channelId
ORDER BY
  total DESC
LIMIT 5000;

SELECT
  league_name,
  club_name,
  count(*) AS total_players
FROM
  players_new
WHERE
  fifa_version = 21
  AND fifa_update = 64
  AND league_id IS NOT NULL
  AND (overall >= 90
    OR potential >= 85)
GROUP BY
  1,
  2
ORDER BY
  total_players DESC
LIMIT 100;

-- show top 10 channels which have the most viewership
