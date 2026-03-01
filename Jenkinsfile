
pipeline {

    agent any
    // Set the PATH environment of rust to make it visible to Jenkins
    environment {
        PATH = "/home/diegosarmiento/.cargo/bin:${env.PATH}"
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
                sh 'cargo build --verbose'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose'
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