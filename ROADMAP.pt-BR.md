# Roadmap

Este roadmap descreve resultados do produto, não um diário de desenvolvimento. A ordem é intencional; não há datas prometidas.

[Read in English](ROADMAP.md)

## 1. Núcleo Rust compartilhado

- [ ] Definir o modelo canônico de documento e as interfaces de conversão.
- [ ] Converter HTML e texto simples em Markdown estruturado.
- [ ] Adicionar adaptadores para PDF e trabalhos de impressão com avisos explícitos sobre perdas.
- [ ] Criar modelos de relatório para respostas de chat e outros conteúdos estruturados.
- [ ] Empacotar o núcleo sem exigir instalação ou runtime separado de Python.

## 2. macOS

- [ ] Criar o aplicativo auxiliar nativo e a janela para salvar.
- [ ] Registrar uma impressora virtual por um caminho de impressão suportado pelo macOS.
- [ ] Encaminhar trabalhos com segurança do serviço de impressão ao aplicativo da sessão do usuário.
- [ ] Fornecer instalador assinado e remoção limpa.
- [ ] Validar trabalhos de aplicativos representativos e documentar limites de conversão.

## 3. Windows

- [ ] Implementar a impressora virtual com a arquitetura Print Support App suportada pela Microsoft.
- [ ] Conectar o fluxo de impressão do Windows ao núcleo Rust compartilhado.
- [ ] Empacotar e instalar pelo mecanismo de aplicativos suportado no Windows.
- [ ] Validar caixa de impressão nativa, salvamento, atualização e remoção.

## 4. Linux

- [ ] Implementar integração com CUPS e um auxiliar de desktop para salvar.
- [ ] Selecionar as primeiras distribuições e ambientes de desktop suportados.
- [ ] Empacotar instalação e remoção sem deixar uma fila de impressão quebrada.

## 5. Android e iOS — sem previsão

- [ ] Reavaliar viabilidade quando os fluxos de desktop e o núcleo compartilhado estiverem estáveis.
- [ ] Escolher integrações específicas somente após confirmar capacidades do sistema e demanda.

## 6. Preparação de releases

- [ ] Publicar instaladores assinados e autocontidos por plataforma.
- [ ] Documentar versões de sistema suportadas, permissões, atualizações e remoção.
- [ ] Automatizar builds de release e fornecer checksums.