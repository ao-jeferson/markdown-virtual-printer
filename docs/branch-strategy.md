# Estratégia Gitflow

## Branches permanentes

- `main`: código pronto para release. Atualizações somente por pull request.
- `develop`: integração da próxima versão. Atualizações somente por pull request.

Ambas devem exigir PR e revisão nas regras do repositório. Não faça push direto, mesmo como mantenedor.

## Branches temporárias

Crie branches a partir da base indicada e apague-as após o merge:

| Prefixo | Criada a partir de | PR de destino | Uso |
|---|---|---|---|
| `feature/<assunto>` | `develop` | `develop` | Funcionalidade |
| `fix/<assunto>` | `develop` | `develop` | Correção comum |
| `docs/<assunto>` | `develop` | `develop` | Documentação |
| `chore/<assunto>` | `develop` | `develop` | Manutenção |
| `release/<versão>` | `develop` | `main` | Preparar uma versão |
| `hotfix/<assunto>` | `main` | `main` | Correção urgente em produção |

Exemplos: `feature/html-tables`, `release/0.2.0`, `hotfix/pdf-crash`.

Branches `feature/*`, `fix/*`, `docs/*` e `chore/*` não são permanentes; elas entram em `develop`. Não crie branches vazias apenas para representar prefixos.

## Fluxos

### Funcionalidade ou correção normal

1. Atualize sua cópia de `develop`.
2. Crie uma branch temporária a partir de `develop`.
3. Abra PR para `develop), aguarde revisão e resolva as conversas.
4. Faça squash merge e apague a branch temporária.

### Release

1. Crie `release/X.Y.Z` a partir de `develop`.
2. Atualize versão, changelog, documentação e artefatos nessa branch.
3. Abra PR de `release/X.Y.Z` para `main`; após revisão, faça squash merge.
4. Marque a release com a tag anotada `vX.Y.Z) em `main`.
5. Abra PR para integrar os ajustes finais da release de volta em `develop`.

### Hotfix

1. Crie `hotfix/<assunto>` a partir de `main`.
2. Corrija e abra PR para `main).
3. Após o merge e a release corretiva, abra PR para levar a mesma correção a `develop`.

## Revisão e proteção

- Um PR não pode ser aprovado pelo próprio autor. Quando houver outro revisor disponível, exija uma aprovação.
- Novos commits invalidam aprovações anteriores; resolva todas as conversas antes do merge.
- Use squash merge para manter o histórico das branches permanentes legível.
- `main` e `develop` não aceitam commits ou force pushes diretos.
- Proteção de branch é aplicada por rulesets do GitHub. Os prefixos e bases dos PRs são convenções revisadas pelos mantenedores; regras nativas protegem as branches de destino, mas não impõem sozinhas qual prefixo de origem cada PR deve usar.

## Releases

Versões usam tags anotadas `vMAJOR.MINOR.PATCH`. Consulte [processo de releases](releases.md) para changelog, notas e artefatos.
