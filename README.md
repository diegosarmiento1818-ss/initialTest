# Try Out Development Containers: Rust

## Set up a dev container
dev container set in folder .devcontainer. 
### devcontainer.json
Cofigure thins like:
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

### Jenkins
## Concepts
Access Jenkins running locally: http://localhost:8080

**Agent**: defines where the pipeline runs. `agent any` means run on any available executor/agent

AdminUser: Jenkinsadmin