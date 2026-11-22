# Roadmap

As etapas avançam conforme os critérios de qualidade e suporte forem atendidos. Não há datas prometidas enquanto os formatos e a experiência ainda estão em validação.

## 1. Núcleo de conversão — em andamento

- [x] CLI para HTML, texto/Markdown e PDF via `pdftotext`.
- [x] Modelo simples de relatório.
- [ ] Melhorar tabelas, imagens, links e blocos de código em HTML.
- [ ] Relatar perdas e confiança por conversão.
- [ ] Reunir amostras anonimizadas para avaliar diferentes documentos.

## 2. Aplicativo macOS

- [ ] Interface para selecionar arquivo, formato, destino e nome.
- [ ] Pré-visualização do Markdown antes de salvar.
- [ ] Empacotamento e instalação sem alterar a fila de impressão.

## 3. Imprimir para Markdown no macOS

- [ ] Provar o fluxo com CUPS e aplicativos de origem diferentes.
- [ ] Encaminhar o conteúdo recebido ao auxiliar de desktop para escolha do destino.
- [ ] Documentar limites da semântica recebida por aplicativo.

## 4. Linux

- [ ] Compatibilidade CUPS em distribuições selecionadas.
- [ ] Empacotamento e configuração reversível da fila.

## 5. Windows

- [ ] Avaliar a arquitetura atual de impressora virtual.
- [ ] Implementar integração após estabilizar o formato intermediário.

## 6. Releases e comunidade

- [ ] Automatizar builds assináveis para tags de release.
- [ ] Definir critérios de suporte e compatibilidade.
