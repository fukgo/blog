# Dockerfile
FROM node:lts-alpine3.20
WORKDIR /app
COPY package*.json ./
RUN npm install
EXPOSE 3000
CMD ["npm", "start"]
