# FROM node:21-alpine
# WORKDIR /app
# COPY package.json .
# RUN npm install
# COPY . .
# RUN npm run build
# EXPOSE 80
# CMD [ "npm", "run", "preview" ]

# production stage
FROM nginx:stable-alpine as production-stage
COPY ./dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
