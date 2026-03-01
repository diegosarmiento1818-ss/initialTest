
pipeline {

    agent {
        docker {
            image 'rust:latest'
        }
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

        stage('Build') {
            steps {
                dir('hello-rust') {
                    sh 'cargo build --verbose'
                }
            }
        }

        stage('Test') {
            steps {
                dir('hello-rust') {
                    sh 'cargo test --verbose'
                }
            }
        }
    }

    post {
        success {
            echo 'Build and tests passed!'
        }
        failure {
            echo 'Build or tests failed.'
        }
    }
}