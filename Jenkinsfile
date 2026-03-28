
pipeline {
    agent any
    //agent {
    //    docker {
    //        image 'rust:1.93'
    //    }
    //}

    environment {
        IMAGE_NAME = "diegossg/initial_test_rust"
        TAG = "${env.GIT_COMMIT.take(7)}"
        // Ensure Jenkins finds the rustup-installed Rust
        PATH = "/var/lib/jenkins/.cargo/bin:${env.PATH}"
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Verify Rust') {
            steps {
                sh 'rustc --version'
                sh 'cargo --version'
            }
        }

        stage('Test') {
            steps {
                dir('hello-rust') {
                    sh 'cargo test --verbose'
                }
            }
        }

        stage('Build-Rust') {
            steps {
                dir('hello-rust') {
                    sh 'cargo build --verbose'
                }
            }
        }

        stage('Verify Docker') {
            steps {
                sh 'docker --version'
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -t $IMAGE_NAME:$TAG .' 
            }
        }

        stage('Push Image') {
            steps {
                withCredentials([usernamePassword(
                    credentialsId: 'docker-credentials',
                    usernameVariable: 'DOCKER_USER',
                    passwordVariable: 'DOCKER_PASS'
                )]) {
                    sh 'echo $DOCKER_PASS | docker login -u $DOCKER_USER --password-stdin'
                    sh 'docker push $IMAGE_NAME:$TAG'
                }
            }
        }

        stage('Verify helm & kubernetes & kubernetes cluster') {
            steps {
                    sh 'helm version'
                    sh 'kubectl version --client'
                    sh 'kubectl cluster-info'
            }
        }
        
        stage('Deploy to Kubernetes') {
            steps {
                    sh 'helm upgrade --install rust-app ./rust-app \
                    --set image.repository=IMAGE_NAME \
                    --set image.tag=${TAG}'
            }
        }
    }

    post {
        success {
            echo 'Build, tests, image push and deployment passed!'
        }
        failure {
            echo 'Build, tests, image push or deployment failed.'
        }
    }
}