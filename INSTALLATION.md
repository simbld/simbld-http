# Guide de création et d'organisation perso

### Étape 1 : Initialisation du projet de bibliothèque HTTP avec Cargo

```bash
cargo new simbld-http --lib && cd simbld-http
```

```bash
cargo init --lib . (rootDir)
```

### Cargo.toml

[package]
name = "simbld-http"
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["Simon Bullado <s_bullado51@hotmail.com>"]
repository = "<https://github.com/simbld/simbld-http>"
description = "A comprehensive HTTP response library for Rust."
exclude = ["tests/", "examples/", "*.bak"]
keywords = [
"http", "response", "status", "code", "status-codes", "httpstatus", "httpresponse",
"headers", "body", "json", "xml", "text", "library", "rust", "crawler", "custom", "informational",
"success", "redirection", "client-error", "server-error", "error", "redirect",
"client", "server", "header", "custom-code", "informational-code", "success-code",
"redirection-code", "client-error-code", "server-error-code", "crawler-code", "custom-response"
]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
actix-web = "4.0"

### Héberger avec GitHub Pages

Génère la documentation de ta bibliothèque avec la commande suivante :

```bash
cargo doc --no-deps
```

### Ouvre la documentation dans ton navigateur web

```bash
cargo doc --open
```

Publie ces fichiers sur une branche gh-pages pour les héberger :
Cp target/doc dans une branche gh-pages:

```bash
git subtree push --prefix target/doc origin gh-pages

```

### Consulter la documentation en local

```bash
  cargo doc --no-deps --open
```

### Héberger avec docs.rs

Pour héberger ta documentation sur [docs.rs](https://docs.rs/), il te suffit de publier ta bibliothèque sur [crates.io](https://crates.io/).

### Créer une bibliothèque

Pour créer une bibliothèque, utilise la commande suivante :

```bash
cargo new nom-de-la-bibliotheque --lib
```

### Créer un compte sur [crates.io](https://crates.io/)

```bash
cargo login
```

### Cargo.toml includes

```toml
documentation = "https://docs.rs/simbld-http"
repository = "https://github.com/simbld/simbld-http.git"
```

### Publier sur crates.io

```bash
cargo publish
```

### Utiliser des dépendances

Pour ajouter une dépendance à ton projet, ajoute-la à ton fichier `Cargo.toml` :

```toml
[dependencies]
nom-de-la-dependance = "0.1.0"
```

### Utiliser des dépendances de développement

Pour ajouter une dépendance de développement à ton projet, ajoute-la à ton fichier `Cargo.toml` :

```toml
[dev-dependencies]
nom-de-la-dependance = "0.1.0"
```
