# ipsecret-sync(imagePullSecret-sync)


### Create secret
```bash
k -n default create secret generic docker-registry --from-file=registry_secrets=registry_secrets.yaml --dry-run -o yaml | kubectl apply -f -
```