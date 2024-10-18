# Dockerfile
FROM node:18-alpine AS build
WORKDIR /app
COPY package*.json ./
RUN npm install
RUN npm run build

WORKDIR /app
COPY . .
EXPOSE 3000
CMD ["npm", "start"]

