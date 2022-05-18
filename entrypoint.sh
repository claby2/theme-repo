#!/bin/sh

run_backend() {
	while :; do
		echo "Running backend..."
		theme-repo-backend --themes "/usr/src/backend/themes" --templates "/usr/src/backend/templates"
		sleep 1s
	done
}

run_backend &

echo "Running frontend..."
nginx-debug -g "daemon off;"
