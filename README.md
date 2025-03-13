# Vito - A Bookkeeping System (Rust + Axum)

**Vito** is a **bookkeeping system** written in **Rust** using the **Axum framework**.  

This is the Rust rewrite of the old Django project:   
**[9-8-7-6/vito_](https://github.com/9-8-7-6/vito_.git)**.

---

## Features
 **Rust-based**: High-performance backend using Axum.  
 **Docker support**: Easily deploy using `docker-compose`.  
 **RESTful API**: Designed for financial tracking & bookkeeping.  

---

## Getting Started

### **1️⃣ Clone the repository**
```sh
git clone https://github.com/9-8-7-6/vito.git
cd vito
docker compose up --build -d
cargo watch -x run
