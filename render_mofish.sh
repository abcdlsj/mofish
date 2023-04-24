douban=$(jq -r '.douban | map("- \(.title) \u001b[34m\(.url)\u001b[0m") | join("\n") | "douban:\n\(.)"' mofish.json)
hackernews=$(jq -r '.hackernews | map("- \(.title) \u001b[34m\(.url)\u001b[0m") | join("\n") | "hackernews:\n\(.)"' mofish.json)
hupu=$(jq -r '.hupu | map("- \(.title) \u001b[34m\(.url)\u001b[0m") | join("\n") | "hupu:\n\(.)"' mofish.json)
echo "$douban\n"
echo "$hackernews\n"
echo "$hupu"
