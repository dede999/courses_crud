# Courses CRUD - Sistema de Gerenciamento de Cursos

Este é um projeto Rust usando Rocket para criar um sistema de gerenciamento de cursos com PostgreSQL e RabbitMQ.

## 🚀 Configuração do Ambiente de Desenvolvimento

### Pré-requisitos
- Docker e Docker Compose instalados
- Rust (versão 1.70+)

### 1. Iniciar os Serviços

Para iniciar o PostgreSQL e RabbitMQ:

```bash
docker-compose -f compose.dev.yaml up -d
```

### 2. Verificar Status dos Serviços

```bash
docker-compose -f compose.dev.yaml ps
```

### 3. Acessar as Interfaces

- **PostgreSQL**: `localhost:5432`
  - Database: `courses_crud`
  - Usuário: `postgres`
  - Senha: `postgres123`

- **RabbitMQ Management**: `http://localhost:15672`
  - Usuário: `admin`
  - Senha: `admin123`

### 4. Configurar Variáveis de Ambiente

Copie o arquivo de configuração:

```bash
cp config.example .env
```

### 5. Executar a Aplicação

```bash
cargo run
```

A aplicação estará disponível em: `http://localhost:8000`

## 📋 Endpoints Disponíveis

- `GET /api/v1/hello` - Endpoint de teste
- `POST /api/v1/auth/register` - Registro de usuário

## 🛠️ Comandos Úteis

### Parar os serviços
```bash
docker-compose -f compose.dev.yaml down
```

### Parar e remover volumes (dados serão perdidos)
```bash
docker-compose -f compose.dev.yaml down -v
```

### Ver logs dos serviços
```bash
docker-compose -f compose.dev.yaml logs -f postgres
docker-compose -f compose.dev.yaml logs -f rabbitmq
```

### Conectar ao PostgreSQL
```bash
docker exec -it courses_crud_postgres psql -U postgres -d courses_crud
```

## 📊 Estrutura do Banco de Dados

O arquivo `init.sql` cria as seguintes tabelas:

- **users**: Armazena informações dos usuários
- **courses**: Armazena informações dos cursos
- **enrollments**: Gerencia matrículas de usuários em cursos

## 🔧 Próximos Passos

1. Implementar autenticação JWT
2. Adicionar endpoints CRUD para cursos
3. Implementar sistema de matrículas
4. Adicionar integração com RabbitMQ para notificações
5. Implementar testes automatizados 