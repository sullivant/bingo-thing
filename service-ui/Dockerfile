FROM node:21-alpine
WORKDIR /usr/src/app
COPY package.json .
RUN npm install
COPY . ./
EXPOSE 80
CMD [ "npm", "run", "dev" ]

# # production stage
# FROM nginx:stable-alpine as production-stage
# COPY ./dist /usr/share/nginx/html
# EXPOSE 80
# CMD ["nginx", "-g", "daemon off;"]
