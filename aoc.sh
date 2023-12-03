#!/usr/bin/env bash

set -eu

function get_cookie {
    if [ ! -f aoc_cookie ]; then
        echo "aoc_cookie file not found, cannot download" >&2
        exit 1
    fi
    cat aoc_cookie
}

: "${1:?usage: ./aoc.sh new}"

case "$1" in
    new)
        : "${2:?usage: ./aoc.sh new DAY}"
        day="$(printf %02d "$2")"
        cp -r "2023/day00" "2023/day$day"
        find "2023/day$day" -type f -exec sed -i -e "s/00/$day/g" {} \;

        curl -H "Cookie: $(get_cookie)" "https://adventofcode.com/2023/day/$2/input" -o "2023/day$day/input.txt"
        ;;
    *)
        echo "usage: ./aoc.sh new" >&2
        ;;
esac
