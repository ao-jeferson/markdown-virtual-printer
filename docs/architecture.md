# Arquitetura e formatos

## Princípio

O mdprint separa conversão de documento, apresentação e integração com a fila de impressão. O núcleo recebe um formato identificável, produz Markdown UTF-8 e mantém o processamento local.

```text
HTML / texto / PDF -> adaptador de formato -> Markdown -> modelo opcional -> arquivo .md
                                            ^
             no futuro: fila de impressão -> auxiliar de desktop -> diálogo salvar
```

## Conversão

- **HTML:** usa `markdownify` como dependência opcional. Elementos sem equivalência em Markdown podem virar texto ou ser simplificados.
- **TXT/MD:** normaliza quebras de linha e espaços; conteúdo Markdown existente é preservado.
- **PDF:** usa `pdftotext -layout` para extrair texto na ordem/layout aproximado. O PDF raramente contém semântica de títulos, listas e tabelas que possa ser recuperada com certeza. PDF digitalizado exige OCR, fora do escopo inicial.
- **Relatório:** modelo inicial coloca título e data antes do conteúdo. Modelos futuros devem ser configuráveis e não presumir que texto de chat tenha metadados ausentes.

## Segurança e privacidade

Conversão local por padrão. Arquivos não devem ser enviados a terceiros. Adaptadores devem limitar uso de memória/CPU e tratar entrada malformada. Nunca execute conteúdo extraído como código.

## Integração de impressão

Uma fila de impressão costuma transmitir uma representação destinada à aparência, não a árvore semântica do documento. A impressora virtual não pode prometer recuperar formatação que não está presente no trabalho recebido. Em macOS/Linux, a integração provavelmente usará CUPS e um aplicativo auxiliar para abrir a caixa de salvamento no contexto da sessão do usuário; no Windows, será necessário usar a arquitetura de impressora virtual suportada pela plataforma.
