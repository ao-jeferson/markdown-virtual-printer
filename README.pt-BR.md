# Impressora Virtual Markdown

**Uma impressora virtual nativa que salva trabalhos de impressão como Markdown.**

O projeto pretende instalar uma impressora na caixa de impressão do sistema. Selecione-a no comando normal **Imprimir** de um aplicativo; o trabalho será convertido em arquivo Markdown e uma janela nativa permitirá escolher onde salvá-lo. Não é um conversor de Markdown para PDF nem um editor de documentos.

[Read in English](README.md)

## Direção do produto

O aplicativo terá um núcleo Rust compartilhado que recebe um trabalho de impressão, extrai o conteúdo e a estrutura disponíveis e gera Markdown. Adaptadores específicos de cada sistema registram a impressora virtual, recebem trabalhos, mostram a interface nativa para salvar e instalam ou removem a integração.

A primeira plataforma será o macOS, seguido por Windows e Linux. Android e iOS ficam como possibilidades no roadmap, sem previsão.

A conversão não consegue reconstruir semântica ausente no trabalho recebido. A aparência pode ser preservada enquanto títulos, links, tabelas ou outras estruturas do documento original se perdem.

## Protótipo atual

O conversor de linha de comando existente é um protótipo experimental em Python. Ele aceita HTML, texto/Markdown e PDF; a extração de PDF usa pdftotext. O protótipo serve para avaliar a qualidade da conversão. Ele não é o produto autocontido planejado e não instala uma impressora.

O aplicativo final usará um núcleo Rust compilado e pacotes nativos por plataforma, sem exigir Python instalado. Cada pacote incluirá os componentes de runtime do projeto e explicará as permissões do sistema necessárias para instalar ou remover a impressora.

## Arquitetura

A proposta usa **arquitetura hexagonal (ports and adapters)**:

- **Núcleo Rust:** modelo canônico de documento, conversão de formatos, saída Markdown, modelos de relatório e avisos de conversão.
- **Adaptadores de formato:** interpretam HTML, texto simples, PDF e trabalhos de impressão e os transformam no modelo canônico.
- **Adaptadores nativos:** registro da impressora, recebimento de trabalhos, janelas para salvar e integração com o sistema.
- **Instaladores:** instalam, atualizam e removem o aplicativo e a impressora virtual pelo mecanismo suportado pelo sistema.

O núcleo não dependerá das APIs de interface do macOS, Windows ou Linux. Consulte a [arquitetura](docs/architecture.pt-BR.md) e o [roadmap](ROADMAP.pt-BR.md).

## Protótipo de desenvolvimento

O protótipo Python atual requer Python 3.10 ou mais recente. A conversão de PDF requer o pdftotext do Poppler; HTML usa a dependência opcional markdownify.


    python3 -m venv .venv
    source .venv/bin/activate
    python -m pip install -e '.[html]'
    mdprint documento.html -o documento.md

Esses comandos executam somente o protótipo. Eles não instalam uma impressora virtual.

## Contribuição

Registre ideias como Issues usando o modelo **Idea** ou **Ideia** e aplique as labels em inglês, como type: idea, platform: macos ou area: virtual-printer. Branches de funcionalidade têm como destino develop; releases têm como destino main. Veja [como contribuir](CONTRIBUTING.pt-BR.md) e a [estratégia Gitflow](docs/branch-strategy.pt-BR.md).

## Licença

MIT. A licença permite uso, modificação e redistribuição de acordo com seus avisos. Consulte [LICENSE](LICENSE).