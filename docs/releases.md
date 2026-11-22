# Processo de releases

## Versões e notas

Usamos Semantic Versioning quando a compatibilidade do CLI estiver estabilizada: `MAJOR.MINOR.PATCH`. Antes de `1.0.0`, mudanças incompatíveis podem ocorrer entre versões menores e devem estar destacadas nas notas.

Cada release deve incluir resumo, mudanças, correções, limitações conhecidas, instruções de instalação/atualização e links para código e arquivos distribuídos. O `CHANGELOG.md` mantém o histórico no repositório.

## Preparação

1. Revise Issues e roadmap concluídos para a versão.
2. Atualize a versão em `pyproject.toml` e as entradas de `CHANGELOG.md`.
3. Revise dependências, documentação e sistemas suportados.
4. Crie uma tag anotada `vX.Y.Z` em `main` e publique uma GitHub Release com as notas.
5. Anexe wheel e sdist; informe hashes quando disponíveis.

A automação do GitHub Actions pode construir wheel/sdist quando uma tag de release for publicada. Sem Actions, o mesmo passo pode ser executado localmente com `python -m build` e os artefatos anexados manualmente. Publicar em registries ficará desativado até haver responsáveis, credenciais e processo de assinatura definidos.
