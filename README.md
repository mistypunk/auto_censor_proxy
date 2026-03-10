# 🛡️ Auto Censor Proxy

A self-hosted proxy to automatically censor inappropriate content in images, in real-time, as you browse.

---

### ✨ Features

- **Real-time Censorship:** Analyzes and blurs images with sensitive content directly in your browser.
- **Privacy First:** Uses a local AI model. Your browsing data never leaves your machine.
- **HTTP/HTTPS Proxy:** Acts as a proxy for all web traffic, allowing image analysis on any website.
- **Easy to Use:** Simplified setup with Docker.

---

### 🚀 Getting Started

Follow the steps below to get the proxy running in minutes.

#### 1. Prerequisites

The only prerequisite is having **Docker & Docker Compose** installed. They manage all the application's services in an isolated and efficient way.

- [Download Docker Desktop here](https://www.docker.com/products/docker-desktop/)

#### 2. Start the Application

With Docker running, open your terminal in the project's root directory. Run the following command. It will build the services (this may take a few minutes the first time) and start them in the background.

```bash
docker-compose up --build -d
```

---

### 🌐 Using the Proxy in Browsers

Once the application is running, you need to tell your browser to use it.

#### Brave / Google Chrome (and derivatives)

You can launch the browser directly from the terminal with the proxy already configured.

```bash
# For Brave
brave-browser --proxy-server="http://127.0.0.1:8082" --ignore-certificate-errors

# For Google Chrome
google-chrome-stable --proxy-server="http://127.0.0.1:8082" --ignore-certificate-errors
```
> **Note:** The `--ignore-certificate-errors` flag is a convenient shortcut, but the recommended and more secure method is to install a certificate.

---

### 🛑 Stopping the Application

To stop all services, run the following command in your terminal:

```bash
docker-compose down
```
