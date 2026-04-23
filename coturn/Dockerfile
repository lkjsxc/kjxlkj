FROM coturn/coturn:alpine

USER root
RUN apk add --no-cache gettext

COPY turnserver.conf.template /etc/coturn/turnserver.conf.template
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
