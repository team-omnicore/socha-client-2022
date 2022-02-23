#! /bin/sh
echo Client started! > output.txt
DIR="$(cd "$(dirname "$0")" && pwd)"
chmod +x client
"$DIR"/client "$@" >> output.txt