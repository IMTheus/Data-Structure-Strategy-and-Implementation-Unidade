# Data-Structure-Strategy-and-Implementation-Unidade


# Data-Structure-Strategy-and-Implementation-Unidade


Sistema de Busca em Rust

Visão Geral

Este projeto implementa um sistema de busca simples em Rust, permitindo indexar e pesquisar produtos com base em seus atributos como nome, categoria e marca. O objetivo é estruturar uma base de dados de produtos e fornecer uma busca eficiente.

Tecnologias Utilizadas

Rust: Linguagem de programação utilizada para desenvolver a aplicação.

Cargo: Gerenciador de pacotes e build system do Rust.

Estrutura do Projeto

/sistema_de_busca
│── src
│   ├── main.rs       # Arquivo principal que executa a aplicação
    ├── libs.rs       # //modulos publicos
│   ├── product.rs    # Definição da estrutura Product
│   ├── indexer.rs    # Mecanismo de indexação dos produtos
│   ├── search.rs     # Função de busca de produtos
│── Cargo.toml        # Configuração do projeto e dependências
│── README.md         # Documentação do projeto

Como Rodar o Projeto

Certifique-se de ter o Rust instalado. Caso não tenha, instale via Rustup.

Clone este repositório:

git clone https://github.com/seu-usuario/sistema_de_busca.git

Acesse o diretório do projeto:

cd sistema_de_busca

Compile e execute o projeto com o Cargo:

cargo run

Funcionamento

O main.rs cria uma lista de produtos e os indexa usando a estrutura Indexer. Em seguida, ele executa buscas por diferentes termos e exibe os resultados no terminal.

Exemplo de Saída

Produtos encontrados para 'eletrônicos':
 - Smartphone X (Categoria: eletrônicos, Marca: TechBrand, Preço: R$ 1999.99)
 - Notebook Pro (Categoria: eletrônicos, Marca: PowerComp, Preço: R$ 4999.99)

Produtos encontrados para 'Smartphone X':
 - Smartphone X (Categoria: eletrônicos, Marca: TechBrand, Preço: R$ 1999.99)

Nenhum produto encontrado para 'Camiseta'

Funcionalidades

Indexação de produtos: Adiciona produtos a um índice para facilitar buscas futuras.

Busca por palavras-chave: Permite pesquisar produtos pelo nome ou categoria.

Filtragem: Exibe apenas os produtos relevantes com base na busca.

