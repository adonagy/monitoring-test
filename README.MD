# Test app for monitoring measurement tests

Add various node load simulations and tests to check the correctness of the monitoring app

The app is structured to be used only in docker containers

## Building 

```
docker build -t tezedge/monitoring-test:latest . && docker push tezedge/monitoring-test:latest
```