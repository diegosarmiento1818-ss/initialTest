# Try Out Development Containers: Rust

## Set up a dev container
dev container set in folder .devcontainer. 
### devcontainer.json
Cofigure things like:
- "Customizations" Installation of VS code extensions
- "RemoteUser" that is not the "root" for example "vscode" user
1. Using a DockerFile

### DockerFile
Definition of the container

## Set up a container for the application
Prerequesites:
- Install docker https://docs.docker.com/engine/install/ubuntu/
- Install jenkins https://www.jenkins.io/doc/book/installing/linux/#debianubuntu

### Steps to run manually from docker
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

# Github
## General

User: diegosarmiento1818-ss
e-mail: diegosarmiento1818-ss

# Docker hub / registry
## General

### Add some docker registry credentials for jenkins
1. Add credentials
registry name: diegossg
token: see on docker

2. Configure this credentials in jenkins
Add a username/password credential
