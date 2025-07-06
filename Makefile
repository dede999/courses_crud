# Makefile para Courses CRUD
# Comandos disponíveis: make help

.PHONY: help run build test clean docker-up docker-down docker-logs setup

# Variáveis
APP_NAME = courses_crud
COMPOSE_FILE = compose.dev.yaml

# Cores para output
GREEN = \033[0;32m
YELLOW = \033[1;33m
BLUE = \033[0;34m
RED = \033[0;31m
NC = \033[0m # No Color

# Comando padrão
.DEFAULT_GOAL := help

help: ## Mostra esta ajuda
	@echo "$(BLUE)🚀 Courses CRUD - Comandos Disponíveis$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""

setup: ## Configura o ambiente inicial
	@echo "$(YELLOW)🔧 Configurando ambiente...$(NC)"
	@if [ ! -f .env ]; then \
		echo "$(BLUE)📝 Copiando env.example para .env...$(NC)"; \
		cp env.example .env; \
		echo "$(GREEN)✅ Arquivo .env criado!$(NC)"; \
	else \
		echo "$(GREEN)✅ Arquivo .env já existe$(NC)"; \
	fi
	@echo "$(GREEN)✅ Ambiente configurado!$(NC)"

run: setup ## Executa a aplicação
	@echo "$(BLUE)🚀 Iniciando Courses CRUD...$(NC)"
	@echo "$(YELLOW)📊 Database URL: $(shell grep DATABASE_URL .env | cut -d '=' -f2)$(NC)"
	@echo "$(YELLOW)🐰 RabbitMQ URL: $(shell grep RABBITMQ_URL .env | cut -d '=' -f2)$(NC)"
	@echo "$(YELLOW)🌐 Rocket Address: $(shell grep ROCKET_ADDRESS .env | cut -d '=' -f2):$(shell grep ROCKET_PORT .env | cut -d '=' -f2)$(NC)"
	@echo ""
	watchexec -e rs -r cargo run

build: ## Compila o projeto
	@echo "$(BLUE)🔨 Compilando projeto...$(NC)"
	cargo build

build-release: ## Compila o projeto em modo release
	@echo "$(BLUE)🔨 Compilando projeto em modo release...$(NC)"
	cargo build --release

test: ## Executa os testes
	@echo "$(BLUE)🧪 Executando testes...$(NC)"
	cargo test

check: ## Verifica o código sem compilar
	@echo "$(BLUE)🔍 Verificando código...$(NC)"
	cargo check

clean: ## Limpa arquivos de build
	@echo "$(BLUE)🧹 Limpando arquivos de build...$(NC)"
	cargo clean

docker-up: ## Inicia os serviços Docker
	@echo "$(BLUE)🐳 Iniciando serviços Docker...$(NC)"
	docker-compose -f $(COMPOSE_FILE) up -d
	@echo "$(GREEN)✅ Serviços iniciados!$(NC)"
	@echo "$(YELLOW)📊 PostgreSQL: localhost:5434$(NC)"
	@echo "$(YELLOW)🐰 RabbitMQ: localhost:5673 (Management: http://localhost:15673)$(NC)"

docker-down: ## Para os serviços Docker
	@echo "$(BLUE)🐳 Parando serviços Docker...$(NC)"
	docker-compose -f $(COMPOSE_FILE) down

docker-down-volumes: ## Para os serviços Docker e remove volumes
	@echo "$(BLUE)🐳 Parando serviços Docker e removendo volumes...$(NC)"
	docker-compose -f $(COMPOSE_FILE) down -v

docker-logs: ## Mostra logs dos serviços Docker
	@echo "$(BLUE)📋 Logs dos serviços Docker:$(NC)"
	docker-compose -f $(COMPOSE_FILE) logs -f

docker-logs-postgres: ## Mostra logs do PostgreSQL
	@echo "$(BLUE)📋 Logs do PostgreSQL:$(NC)"
	docker-compose -f $(COMPOSE_FILE) logs -f postgres

docker-logs-rabbitmq: ## Mostra logs do RabbitMQ
	@echo "$(BLUE)📋 Logs do RabbitMQ:$(NC)"
	docker-compose -f $(COMPOSE_FILE) logs -f rabbitmq

docker-status: ## Mostra status dos serviços Docker
	@echo "$(BLUE)📊 Status dos serviços Docker:$(NC)"
	docker-compose -f $(COMPOSE_FILE) ps

dev: docker-up run ## Inicia tudo: Docker + aplicação

stop: docker-down ## Para tudo

restart: stop dev ## Reinicia tudo

# Comandos de desenvolvimento
fmt: ## Formata o código
	@echo "$(BLUE)🎨 Formatando código...$(NC)"
	cargo fmt

clippy: ## Executa clippy (linter)
	@echo "$(BLUE)🔍 Executando clippy...$(NC)"
	cargo clippy

# Comandos de banco de dados
db-connect: ## Conecta ao PostgreSQL
	@echo "$(BLUE)🗄️ Conectando ao PostgreSQL...$(NC)"
	docker exec -it courses_crud_postgres psql -U postgres -d courses_crud

db-reset: ## Reseta o banco de dados
	@echo "$(RED)⚠️ Resetando banco de dados...$(NC)"
	@read -p "Tem certeza? Isso apagará todos os dados! (y/N): " confirm; \
	if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
		docker-compose -f $(COMPOSE_FILE) down -v; \
		docker-compose -f $(COMPOSE_FILE) up -d; \
		echo "$(GREEN)✅ Banco resetado!$(NC)"; \
	else \
		echo "$(YELLOW)❌ Operação cancelada$(NC)"; \
	fi 