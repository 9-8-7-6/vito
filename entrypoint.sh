#!/bin/sh

echo "Applying database migrations..."
python manage.py migrate --noinput

echo "Collecting static files..."
python manage.py collectstatic --noinput

echo "Starting uWSGI server..."
exec uwsgi --ini /app/uwsgi.ini
