#!/usr/bin/env bash
cd $(dirname "$0")
sqlite3 ../quotes.db < quotes.db
sqlite3 ../quotes.db ".mode csv" ".separator | \n" ".import --skip 1 ../submodules/AvatarApi/Quotes.csv Quotes"
