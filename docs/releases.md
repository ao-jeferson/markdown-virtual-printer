# Processo de releases

## Versões e notas

Usamos Semantic Versioning quando a compatibilidade do CLI estiver estabilizada: `MAJOR.MINOR.PATCH`. Antes de `1.0.0`, mudanças incompatíveis podem ocorrer entre versões menores e devem estar destacadas nas notas.

Cada release deve incluir resumo, mudanças, correções, limitações conhecidas, instruções de instalação/atualização e links para código e arquivos distribuídos. O `CHANGELOG.md` mantém o histórico no repositório.

## Preparação

1. Revise Issues e roadmap concluídos para a versão.
2. Atualize a versão em `crates/printer-core/Cargo.toml` e as entradas de `CHANGELOG.md`.
3. Revise dependências, documentação e sistemas suportados.
4. Crie uma tag anotada `vX.Y.Z` em `main` e publique uma GitHub Release com as notas.
5. Anexe os pacotes nativos gerados pela automação e informe hashes.

O GitHub Actions valida o núcleo nos sistemas suportados e prepara artefatos nativos ao publicar uma tag `vX.Y.Z`. Até que os instaladores específicos sejam certificados e testados em cada sistema, os artefatos são prévias de desenvolvimento e a release deve descrever suas limitações. Assinatura e publicação em lojas ficam desativadas até existir processo e responsáveis definidos.
