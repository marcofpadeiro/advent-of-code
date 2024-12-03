#!/bin/bash

COOKIE_FILE="session_cookie.txt"

if [[ ! -f "$COOKIE_FILE" || ! -s "$COOKIE_FILE" ]]; then
    echo "Error: Session cookie file '$COOKIE_FILE' is missing or empty."
    exit 1
fi

YEAR=2024

START_DAY=1
END_DAY=25

BASE_URL="https://adventofcode.com/$YEAR/day"

fetch_input_for_day() {
    local day=$1
    url="$BASE_URL/$day/input"

    response=$(curl -s -b "session=$SESSION_COOKIE" "$url")

    if [[ $? -eq 0 && -n "$response" ]]; then
        echo "$response"
    else
        echo "Failed to fetch input for Day $day. Status: $?"
        return 1
    fi
}

save_input_to_file() {
    local day=$1
    local input_data=$2
    output_dir="inputs"

    mkdir -p "$output_dir"

    echo "$input_data" > "$output_dir/day$(printf "%02d" $day).txt"
    echo "Saved input for Day $day to $output_dir/day_$(printf "%02d" $day).txt"
}

for (( day=$START_DAY; day<=$END_DAY; day++ ))
do
    echo "Fetching input for Day $day..."
    
    input_data=$(fetch_input_for_day $day)
    
    if [[ $? -eq 0 ]]; then
        save_input_to_file $day "$input_data"
    fi
done
