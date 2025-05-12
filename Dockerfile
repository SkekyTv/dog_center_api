FROM rust:latest

# Installer git, docker, et docker-compose
RUN apt-get update && apt-get install -y \
  git \
  docker.io \
  docker-compose \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Configurer la version stable de Rust comme toolchain par défaut
RUN rustup default stable && cargo --version

# Créer un utilisateur non-root pour plus de sécurité
RUN useradd -m docker_user && \
  usermod -aG docker docker_user

# Donner les permissions nécessaires pour les répertoires GitHub Actions
RUN chown -R docker_user:docker_user /__w /_temp || true

# Utiliser un répertoire utilisateur pour le cache
RUN mkdir -p /home/docker_user/cache
ENV CARGO_HOME=/home/docker_user/cache/cargo
ENV RUSTUP_HOME=/home/docker_user/cache/rustup

# Définir le répertoire de travail
WORKDIR /app

# Commande par défaut (optionnelle)
ENTRYPOINT ["bash"]
