# Processo de releases

[Read in English](releases.md)

## Versionamento e notas

Use Semantic Versioning conforme a compatibilidade se estabilizar: MAJOR.MINOR.PATCH. Antes de 1.0.0, mudanças incompatíveis podem ocorrer em versões menores e devem aparecer claramente nas notas.

Cada release deve resumir funcionalidades, correções, limitações conhecidas, instruções de instalação e atualização, sistemas suportados e artefatos disponíveis. Mantenha CHANGELOG.md focado em mudanças do produto já lançadas; não use o changelog como diário de desenvolvimento.

## Etapas Gitflow

1. Revise Issues e itens do roadmap incluídos na versão.
2. Crie release/X.Y.Z a partir de develop e prepare versão, changelog, traduções e documentação.
3. Abra pull request da branch de release para main e faça merge após a revisão.
4. Crie a tag anotada vX.Y.Z no commit integrado em main.
5. Publique a GitHub Release com notas geradas e artefatos.
6. Abra um pull request para levar ajustes específicos da release de volta a develop.

## Automação atual

O workflow atual do GitHub Actions compila o pacote Python experimental quando uma tag com prefixo v é enviada. Ele ainda não compila os instaladores nativos planejados. O workspace Rust e os pacotes de cada plataforma substituirão esse fluxo de protótipo conforme forem implementados.

As releases nativas devem fornecer instaladores assinados e autocontidos, além de checksums. Não anuncie suporte a uma plataforma antes de validar sua integração de impressão e os fluxos de instalação e remoção.