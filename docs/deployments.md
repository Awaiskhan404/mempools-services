# Production
```
docker build -t mempools-server-prod:prod --file ./deployments/dockerfiles/prod/Dockerfile .
aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 457714833715.dkr.ecr.us-east-2.amazonaws.com
docker tag mempools-server-prod:prod 457714833715.dkr.ecr.us-east-2.amazonaws.com/mempools-server:prod
docker push 457714833715.dkr.ecr.us-east-2.amazonaws.com/mempools-server:prod
aws ecs update-service --cluster mempools-ecs-ec2-prod --service mempools-service-prod --force-new-deployment
```

# Staging
```
docker build -t mempools-server-prod:stag --file ./deployments/dockerfiles/prod/Dockerfile .
aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 457714833715.dkr.ecr.us-east-2.amazonaws.com
docker tag mempools-server-prod:stag 457714833715.dkr.ecr.us-east-2.amazonaws.com/mempools-server:stag
docker push 457714833715.dkr.ecr.us-east-2.amazonaws.com/mempools-server:stag
aws ecs update-service --cluster mempools-ecs-ec2 --service mempools-service-staging --force-new-deployment
```