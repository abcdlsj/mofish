jq -r '.douban | map("- \(.title) \(.url)") | join("\n") | "douban:\n\(.)"' mofish.json
echo
jq -r '.hackernews | map("- \(.title) \(.url)") | join("\n") | "hackernews:\n\(.)"' mofish.json
echo 
jq -r '.hupu | map("- \(.title) \(.url)") | join("\n") | "hupu:\n\(.)"' mofish.json
