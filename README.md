# Containers: Rust

## Set up a dev container
dev container set in folder .devcontainer. 
### devcontainer.json
Cofigure things like:
- "Customizations" Installation of VS code extensions
- "RemoteUser" that is not the "root" for example "vscode" user
- "Mounts" to bind folders of the host
### DockerFile
- Define the image base and install tools like rust, helm, kubectl


## Set up a container for the application
### Dockerfile
Create a docker file and define
- Build stage: define and image and copy the application files, build it.
- Run stage: define runtime image and copy only the binary and define the start actions/commands.

Prerequesites:
- Install docker https://docs.docker.com/engine/install/ubuntu/
- Install jenkins https://www.jenkins.io/doc/book/installing/linux/#debianubuntu

### Steps to build/run manually from docker
1. Change where the Dockerfile for the application is
- `cd repos/initialTest/` 
2. Build the image with the tag "rust_basics:v1.0.0"
- `sudo docker build . -t rust_basics:v1.0.0` 
3. List the actual images
- `docker images` 
4. Run the image
- `sudo docker run -d rust_basics:v1.0.0`
5. List the running containers to copy the ID
- `docker ps`
6. Check the logs of the container using the ID
- `docker logs c50f98ded1fd`


# Jenkins
## Concepts
Access Jenkins running locally: http://localhost:8080

**Agent**: defines where the pipeline runs. 

**agent any** means run on any available executor/agent

**agent docker** --> docker pipeline plugin must be installed in Jenkins

## Prerequesities
AdminUser: Jenkinsadmin

1. Jenkins installed locally
2. Jenkins user added to docker group to be able to execute docker commands when using the docker agent
- `sudo usermod -aG docker jenkins`

3. Using **ngrok**
Github notifies to jenkings about a push to a branch through webhooks. This configuration must be done in the repo. Github can only push to a public URL, since jenkins is running locallly a workaorund was to install ngrok.

Ngrok creates secure, public URLs (tunnels) to locally hosted services, allowing developers to expose internal applications, webhooks, and APIs directly to the internet.

`sudo snap install ngrok`

Sign up and get the token from the dashboard

`ngrok config add-authtoken <YOUR_AUTHTOKEN>`

After that start the tunnel to jenkins port 

`ngrok http 8080`

The created public URL must be configure in Github
- https://vinelike-larry-coessential.ngrok-free.dev

**Start the service after booting**

add a file to start the service at

`sudo nano /etc/systemd/system/ngrok.service`

and copy the following:

```
[Unit]
Description=ngrok tunnel
After=network.target

[Service]
ExecStart=/snap/bin/ngrok http --domain=vinelike-larry-coessential.ngrok-free.dev 8080
Restart=always
User=diegosarmiento

[Install]
WantedBy=multi-user.target
```
After that, execute followings commands
```
sudo systemctl daemon-reexec  --> restart the system without rebooting the machine
sudo systemctl daemon-reload  --> reload all service definitions from disk
sudo systemctl enable ngrok  --> starts the service automatically on boot
sudo systemctl start ngrok  --> starts the service right now
```

**Hints for pipeline**
- agent any used so that jenkins user uses the install toolchain (access to docker, rust)
- rust 1.93 install for my user and for jenkins user so it can be used in the pipeline

# Github
## General

User: diegosarmiento1818-ss
e-mail: diegosarmiento1818-ss

# Docker hub / registry
## General

### Add some docker registry credentials for jenkins / kubernetes
1. Add credentials
registry name: diegossg
token: see on docker

2. Configure this credentials in jenkins
Add a username/password credential

# Deployment Kubernetes
## Kubectl
1. Install kubectl (kubernetes culter manager) on the host. In the dev container was also installed but there was some problems when talking to minikube in the host (for certificates) and it was dropped since it is not strictly necessary there.

`curl -LO "https://dl.k8s.io/release/$(curl -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"`
`sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl`

Verify
`kubectl version --client`

Commands:

`kubectl get pods`
`kubectl get deployments`
`kubectl get secret regcred`
`kubectl delete pod rust-app-69c5556b4d-hbqzz`
`kubectl delete pod --all -n default`
`kubectl delete deployment rust-app`
`kubectl logs rust-app-78d87c67c7-ld7qp` --> get logs of one pod with one container
`kubectl describe pod rust-app-5c97b9ddd9-6gfff`






## Minikube
1. Install minikube in the host

`curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube`

```
Host machine
├── Minikube (Kubernetes cluster) ✅
├── Docker ✅
└── Jenkins ✅
└── Helm (for Jenkins) ✅

Devcontainer
├── Helm (for testing) ✅
├── kubectl ✅
└── connects to → Minikube
```
**Start minike after boot**
add a file to start the service at

`sudo nano /etc/systemd/system/minikube.service`

and copy the following:

```
[Unit]
Description=Minikube Kubernetes Cluster
After=docker.service network-online.target
Requires=docker.service network-online.target

[Service]
Type=oneshot
User=diegosarmiento
Group=diegosarmiento
Enviroment=HOME=/home/diegosarmiento
ExecStart=/usr/local/bin/minikube start --driver=docker --profile=minikube
ExecStop=/usr/local/bin/minikube stop --profile=minikube
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target

```
After that, execute followings commands
```
sudo systemctl daemon-reexec  --> restart the system without rebooting the machine
sudo systemctl daemon-reload  --> reload all service definitions from disk
sudo systemctl enable minikube  --> starts the service automatically on boot
sudo systemctl start minikube  --> starts the service right now
```

**help script to update minikube**
Unfortunaetly the minikube cluster does not run automatically this way and a update of the context is needed. Therefore a workaorund was to create a script and run it with the bash. 

`nano ~/start_minikube.sh`

add this content

```
#!/bin/bash

# Ensure Docker is ready
while ! docker info >/dev/null 2>&1; do
    echo "Waiting for Docker..."
    sleep 10
done

# Start Minikube if it's not running
#if ! minikube status | grep -q "apiserver: Running"; then
#    echo "Starting Minikube..."
#    minikube start --driver=docker
#fi

# Fix kubeconfig
minikube update-context
echo "Minikube context updated from the script, the apiserver should be ready soon"

# Optional: Wait until API server is fully ready
#until minikube status | grep -q "apiserver: Running"; do
#    echo "Waiting for Minikube apiserver..."
#    sleep 10
#done

#echo "Minikube is ready!"
```
 make it executable
 `chmod +x ~/start_minikube.sh`

 run the script on the .bashrc (add at the end of the .bashrc)
 ```
 # Start Minikube automatically if needed
~/start_minikube.sh
```
Verifiy that minikube is running

`minikube status`

## Using helm
1. Install helm in the host (jenkis user for pipeline) and the dev container for testing commands
`curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash`

2. Check with
`helm version`

3. Ensure Jenkins user can access it
`sudo -u jenkins helm version`

**In the dev container**

- Create a package in the root of the repo using 

`helm create rust-app`

This generates templates
```
rust-app/
  Chart.yaml
  values.yaml
  templates/
    deployment.yaml
    service.yaml
    ingress.yaml
```    
- Adjust all the specifics for the files like image, tag, port, service

## Manual test of deployment using kubectl from the terminal
Create a secret to access to the registry and update with the right credentials
```
kubectl create secret docker-registry regcred --docker-server=https://index.docker.io/v1/ --docker-username=dummy --docker-password=dummy --docker-email=dummy
```

install the application
`kubectl logs rust-app-78d87c67c7-ld7qp`

upgrade with install
`helm upgrade --install rust-app ./rust-app`

By doing it one should see something like this. the logs can also be showed.
```
helm upgrade --install rust-app ./rust-app
Release "rust-app" has been upgraded. Happy Helming!
NAME: rust-app
LAST DEPLOYED: Sat Mar 21 13:02:30 2026
NAMESPACE: default
STATUS: deployed
REVISION: 5
NOTES:
1. Get the application URL by running these commands:
  export NODE_PORT=$(kubectl get --namespace default -o jsonpath="{.spec.ports[0].nodePort}" services rust-app)
  export NODE_IP=$(kubectl get nodes --namespace default -o jsonpath="{.items[0].status.addresses[0].address}")
  echo http://$NODE_IP:$NODE_PORT
diegosarmiento@diegosarmiento-ubuntu:~/repos/initialTest$ kubectl get pods
NAME                        READY   STATUS    RESTARTS        AGE
rust-app-78d87c67c7-ld7qp   0/1     Running   14 (6m9s ago)   39m
```


