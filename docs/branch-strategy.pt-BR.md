# Estratégia de branches Gitflow

## Branches permanentes

- **main** contém código pronto para release. As versões são marcadas nesta branch.
- **develop** integra mudanças para a próxima versão.

Configure rulesets do GitHub para exigir pull requests nas duas branches, invalidar aprovações antigas após novos commits, exigir conversas resolvidas e bloquear force push e exclusão. A documentação, por si só, não ativa essas regras.

## Branches temporárias

Crie branches a partir da base indicada e apague-as após o merge.

| Prefixo | Criada a partir de | Destino do PR | Uso |
|---|---|---|---|
| feature/<assunto> | develop | develop | Funcionalidade nova |
| fix/<assunto> | develop | develop | Correção normal |
| docs/<assunto> | develop | develop | Documentação |
| chore/<assunto> | develop | develop | Manutenção e automação |
| release/<versão> | develop | main | Preparar uma release |
| hotfix/<assunto> | main | main | Correção urgente em produção |

Exemplos: feature/html-tables, release/0.2.0, hotfix/pdf-crash.

Branches feature, fix, docs e chore são temporárias; não crie branches vazias só para reservar prefixos.

## Comece uma feature

1. Busque as referências e troque para develop.
2. Crie uma branch como feature/html-tables a partir de develop.
3. Faça commits focados e publique a branch.
4. Abra um pull request da feature para develop.
5. Responda à revisão; faça squash merge depois da aprovação e da resolução das conversas.
6. Apague a branch depois do merge.

## Fluxo de release

1. Crie release/X.Y.Z a partir de develop.
2. Atualize versão, changelog, documentação e artefatos nessa branch.
3. Abra pull request de release/X.Y.Z para main e faça merge após a revisão.
4. Crie a tag vX.Y.Z no commit resultante em main e publique a release.
5. Abra um pull request para levar a develop os ajustes específicos da release.

## Fluxo de hotfix

1. Crie hotfix/<assunto> a partir de main.
2. Corrija o problema e abra pull request para main.
3. Marque a versão corretiva e publique a release.
4. Abra um pull request seguinte para levar a correção a develop.

## Revisão e merge

- O autor não pode aprovar o próprio pull request. Quando houver disponibilidade, peça revisão independente.
- Novos commits devem invalidar aprovações anteriores; resolva todas as conversas antes do merge.
- Use squash merge para manter conciso o histórico das branches permanentes.
- Rulesets do GitHub protegem as branches de destino. O GitHub não impõe nativamente toda relação entre prefixo da branch de origem e destino; revise essa convenção em cada PR.

Veja [como contribuir](../CONTRIBUTING.pt-BR.md) e o [processo de releases](releases.pt-BR.md).