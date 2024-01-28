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
