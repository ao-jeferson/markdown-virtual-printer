# Arquitetura

O produto é uma **impressora virtual nativa que exporta trabalhos de impressão como Markdown**. O sistema a apresenta no fluxo normal de impressão; o usuário a seleciona, o aplicativo converte o trabalho recebido e abre a janela nativa para salvar. Isso é diferente de converter um arquivo Markdown em PDF.

[Read in English](architecture.md)

## Padrão recomendado: arquitetura hexagonal

Use ports and adapters ao redor de um núcleo Rust independente de plataforma. O núcleo é responsável pela semântica do documento e pela conversão. Os adaptadores cuidam das filas de impressão, interface nativa, ciclo de vida dos trabalhos e instalação. Assim, o código específico do sistema pode mudar sem acoplar a conversão a uma plataforma.


    Caixa de impressão
        -> adaptador de impressora nativo
        -> entrada normalizada do trabalho
        -> pipeline de conversão Rust
        -> documento Markdown + avisos
        -> janela nativa para salvar
        -> arquivo .md

## Limites entre componentes

- **Modelo de documento:** representação canônica UTF-8 para títulos, parágrafos, listas, tabelas, links, código, imagens e metadados disponíveis.
- **Adaptadores de entrada:** convertem HTML, texto simples, PDF e dados de impressão do sistema para o modelo canônico; cada adaptador informa o que não conseguiu preservar.
- **Pipeline de conversão:** normaliza o conteúdo, aplica modelos de relatório opcionais e gera Markdown com avisos estruturados.
- **Porta de saída:** grava o Markdown escolhido com segurança e de forma atômica.
- **Adaptadores de plataforma:** registram e removem a impressora virtual, recebem ou armazenam trabalhos temporários, comunicam-se com o app da sessão do usuário e abrem a janela nativa.
- **Instalador:** empacota e instala o aplicativo e o adaptador pelo mecanismo suportado pelo sistema.

Mantenha os crates centrais independentes de frameworks gráficos e SDKs de impressão. Prefira um workspace Rust explícito com um crate pequeno para o núcleo e crates separados para adaptadores; crie um crate por plataforma quando sua implementação começar.

## Integração por sistema

### macOS

Use uma integração CUPS suportada e um aplicativo auxiliar nativo para a janela de salvamento. Filtros de impressão podem rodar em contexto restrito sem acesso à área de trabalho; o componente de impressão deve encaminhar o trabalho concluído ao app da sessão por uma interface IPC local e limitada. Não dependa apenas de drivers PPD legados.

### Windows

Use a arquitetura suportada de impressora virtual e Print Support App. Mantenha as APIs de impressão e o empacotamento do Windows em um adaptador; conecte-o ao núcleo Rust por uma fronteira estável de biblioteca ou processo local.

### Linux

Use CUPS para integrar a fila e um auxiliar de desktop para salvar. Mantenha detalhes de empacotamento separados para selecionar explicitamente as distribuições suportadas.

### Android e iOS

São possibilidades futuras no roadmap, sem datas. Reavalie as APIs de impressão e requisitos de distribuição depois que o núcleo e os fluxos de desktop estiverem estáveis.

## Limites de conversão

Um trabalho de impressão pode conter aparência visual sem títulos semânticos, links, tabelas ou metadados de origem. O conversor deve preservar o possível, avisar sobre transformações incertas ou com perdas e não alegar reconstruir informação ausente. PDFs digitalizados podem exigir OCR e ficam fora do primeiro marco de desktop.

## Segurança e privacidade

Processe trabalhos localmente por padrão. Trate cada carga de impressão como entrada não confiável; limite memória, CPU, recursão e armazenamento temporário; nunca execute conteúdo extraído. Use IPC local restrito e remova arquivos temporários após conclusão ou cancelamento.

## Estado atual da implementação

O repositório contém um protótipo experimental de CLI em Python, que não é o runtime-alvo. Os lançamentos para desktop planejados usarão o núcleo Rust e empacotamento nativo; usuários não deverão precisar instalar Python.