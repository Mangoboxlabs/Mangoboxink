## Mangobox Docker

#### RUN

```
docker run -p8080:80 mangobox/ui1:v1
```

### config

###### Dockerfile

```
FROM nginx:1.15.2-alpine
COPY ./dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
```

