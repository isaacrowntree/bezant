# Minimal container that runs IBKR's official Client Portal Gateway.
# We skip the IBeam / IBC wrappers: user logs in once via the exposed :5000
# port and the session persists for days. bezant-server keeps it alive.
FROM eclipse-temurin:17-jre

ARG GW_VERSION=stable
WORKDIR /gw

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl unzip ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Pull IBKR's distribution zip at build time.
RUN curl -fsSL https://download2.interactivebrokers.com/portal/clientportal.gw.zip -o /tmp/gw.zip \
    && unzip -q /tmp/gw.zip -d /gw \
    && rm /tmp/gw.zip

# Gateway listens on HTTPS 5000 by default.
EXPOSE 5000

# Use the shipped run.sh; it starts the Java app in the foreground with the
# default conf.yaml (self-signed TLS, :5000, standard CPAPI routes).
CMD ["bin/run.sh", "root/conf.yaml"]
