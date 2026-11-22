# Estratégia de branches

## Branch permanente

- `main`: linha principal, deve permanecer instalável e pronta para release.
- Não enviar mudanças diretamente para `main`; toda alteração chega por pull request e revisão.

## Branches de trabalho

Criar cada branch a partir de `main` e usar prefixos curtos:

- `feat/<assunto>` para funcionalidade.
- `fix/<assunto>` para correção.
- `docs/<assunto>` para documentação.
- `chore/<assunto>` para manutenção e automação.

Exemplos: `feat/html-tables`, `fix/pdf-encoding`, `docs/linux-setup`.

## Revisão e merge

1. O autor abre PR para `main` e descreve objetivo, impacto e limitações.
2. O mantenedor revisa o comportamento e a compatibilidade; alterações podem ser solicitadas.
3. Para mudanças de código, o mantenedor deve exigir uma aprovação antes do merge quando houver outro revisor disponível. O autor não aprova o próprio PR.
4. Preferir squash merge para manter `main` com um commit claro por mudança.
5. Se um PR não puder ser revisado, fica aberto; não é incorporado automaticamente.

## Releases

- Releases partem de `main` usando tags anotadas `vMAJOR.MINOR.PATCH`.
- Versões seguem Semantic Versioning quando a compatibilidade estiver estabilizada.
- Atualizar `CHANGELOG.md`, publicar notas da versão no GitHub e anexar artefatos gerados.
- Correção urgente de uma release estável pode usar `fix/<assunto>` a partir da tag afetada; depois, integrar a correção também em `main`.

A política descreve o processo desejado. Regras técnicas de proteção de branch devem ser configuradas nas regras do repositório quando disponíveis.
