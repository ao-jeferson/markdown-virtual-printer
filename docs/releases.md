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

O GitHub Actions valida o núcleo e, para uma tag `vX.Y.Z`, produz instaladores Linux `.deb`, macOS `.pkg`, Windows MSIX autoassinado e APK Android de desenvolvimento, além dos arquivos compactados. O pacote macOS não é assinado nem notarizado e não consegue registrar a fila na configuração SIP testada; o pacote Windows usa certificado de desenvolvimento autoassinado incluído na mesma release; o APK Android é assinado para depuração. O registro da impressora e o fluxo completo ainda não foram validados ponta a ponta em todos os sistemas. Trate os pacotes como experimentais. Nunca use um certificado Windows de outra release. Assinatura/notarização macOS e assinatura Windows confiável para produção exigem identidades próprias e secrets no GitHub.
