#!/bin/bash

# Example script to fetch education board results

# Configuration
API_URL="http://localhost:3000/fetch"
EXAM="ssc"
YEAR="2024"
BOARD="dhaka"
ROLL="123456"
REG="1234567890"

# Fetch result
echo "Fetching result for:"
echo "  Exam: $EXAM"
echo "  Year: $YEAR"
echo "  Board: $BOARD"
echo "  Roll: $ROLL"
echo "  Registration: $REG"
echo ""

curl -s "$API_URL?exam=$EXAM&year=$YEAR&board=$BOARD&roll=$ROLL&reg=$REG" | jq '.'

# Note: Install jq for pretty JSON output: sudo apt-get install jq
