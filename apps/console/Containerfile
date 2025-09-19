# syntax = docker/dockerfile:1

ARG NODE_VERSION=23.11.0
FROM node:${NODE_VERSION}-slim AS base

LABEL launch_runtime="SvelteKit"

WORKDIR /app

ENV NODE_ENV="production"

#-------------------------------------
FROM base AS build
# Install packages needed to build node modules
RUN apt-get update -qq && \
	apt-get install --no-install-recommends -y build-essential node-gyp pkg-config python-is-python3

# Install node modules
COPY package-lock.json package.json ./
RUN npm ci --include=dev

# Copy application code
COPY . .

# Build application
RUN npm run build

# Remove development dependencies
RUN npm prune --omit=dev


#-------------------------------------
FROM base

# Copy built application
COPY --from=build /app/build /app/build
COPY --from=build /app/node_modules /app/node_modules
COPY --from=build /app/package.json /app

# Setup sqlite3 on a separate volume
RUN mkdir -p /data
VOLUME /data

# Start the server by default, this can be overwritten at runtime
EXPOSE 3000
ENV DATABASE_URL="file:///data/sqlite.db"
CMD [ "node", "./build/index.js" ]
