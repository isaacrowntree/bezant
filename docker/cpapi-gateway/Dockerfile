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

# The Gateway ships with an `ips.allow` list in root/conf.yaml limited to
# `127.0.0.1`, `192.*`, `131.216.*` — connections from anywhere else
# (Railway TCP proxy, Cloud Run, k8s ingress) get an "Access Denied" page
# before the login UI can even render. Replace the two limiting glob
# patterns with `*` (any IP). Port 5000 still shouldn't be exposed to the
# open internet without auth in front — the Gateway itself does no
# request-level auth. See README.
# `ips.allow` entries are matched by an IPv4Validator that expects
# octet-glob patterns like `192.*` — it *also* tries each entry as a
# regex somewhere in the pipeline, which is why `*` alone blows up with
# `Dangling meta character`, but `.*` blows up with `invalidate ip
# address`. Safe answer: keep the glob form, just widen it to cover the
# Railway internal ranges we've observed (CGNAT `100.64/10` for proxy
# ingress, `10.*` for Railway private networking) plus the LAN defaults
# already allowed upstream.
RUN sed -i 's/- 192\.\*/- 10.*\n        - 100.*\n        - 172.*\n        - 192.*/' /gw/root/conf.yaml \
    && cat /gw/root/conf.yaml

# Gateway listens on HTTPS 5000 by default.
EXPOSE 5000

# Use the shipped run.sh; it starts the Java app in the foreground with the
# default conf.yaml (self-signed TLS, :5000, standard CPAPI routes).
CMD ["bin/run.sh", "root/conf.yaml"]
