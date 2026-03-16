# Organa Backend

Backend da aplicação **Organa**, um sistema de gestão de finanças pessoais.

## Objetivo
Este projeto tem como objetivo:
- Servir como backend para aplicações web e mobile
- Praticar arquitetura de software e boas práticas
- Explorar o uso de Rust no desenvolvimento backend

## Estado atual
🚧 Projeto em estágio inicial.

Neste momento, o repositório contém apenas a estrutura básica do projeto.

O projeto utiliza a Rust Edition 2021 por ser o padrão moderno e estável da linguagem, oferecendo melhor ergonomia, compatibilidade com o ecossistema async e previsibilidade no código, sem abrir mão de estabilidade.

## Arquitetura (visão inicial)

O projeto segue uma arquitetura em camadas:

- **HTTP**: handlers e middlewares
- **Domain**: regras de negócio
- **Repository**: acesso a dados
- **DB**: configuração e migrações
- **Shared**: tipos e utilidades compartilhadas

## Funcionalidades planejadas (MVP)
- Autenticação de usuários
- Registro de transações financeiras
- Listagem mensal de transações
- Cálculo de saldo por conta

## Tecnologias planejadas
- Rust
- Axum (API HTTP)
- PostgreSQL
- SQLx
- Docker

## Como executar
Assumindo que você já têm docker, sqlx-cli e cargo instalados.

Suba o banco com:
```bash
docker compose up -d
```
Após subir o banco, execute as migrations:
```bash
sqlx migrate run
```
Depois rode a aplicação com:
```bash
cargo run
```
A API estará disponĩvel em http://localhost:3000.

## Endpoints disponíveis

### Health check
```http
GET /health

Novos endpoints serão documentados aqui ou devo mover tudo pra um API docs.
```


> Este projeto está sendo desenvolvido publicamente, com commits pequenos e bem documentados.
