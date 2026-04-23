#!/bin/sh
set -e

envsubst < /etc/coturn/turnserver.conf.template > /etc/coturn/turnserver.conf
exec turnserver -c /etc/coturn/turnserver.conf
