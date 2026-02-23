# Try Out Development Containers: Rust

## Set up a dev container
dev container set in .devcontainer

## Set up a container for the application
In the host execute
1. Change where the Dockerfile is
- cd repos/initialTest/ 
2. Build the image with the tag "rust_basics:v1.0.0"
- sudo docker build . -t rust_basics:v1.0.0 
3. List the actual images
- docker images
4. Run the image
- sudo docker run -d rust_basics:v1.0.0
5. List the running containers to copy the ID
- docker ps
6. Check the logs of the container using the ID
- docker logs c50f98ded1fd

