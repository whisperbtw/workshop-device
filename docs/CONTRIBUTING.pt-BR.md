# Como contribuir

[English](../CONTRIBUTING.md) · **Português** · [日本語](CONTRIBUTING.ja.md)

**Contribuições são bem-vindas.** Você pode ajudar com correções, traduções, acessibilidade, testes, documentação e melhorias pontuais da interface nativa.

## Antes de começar

Pesquise issues e pull requests existentes. Para uma funcionalidade grande, abra uma issue com o problema e o comportamento proposto antes de implementar. Correções pequenas podem ir direto para um PR. Discussões e relatos podem estar em inglês, português ou japonês.

Não envie credenciais Steam, caminhos pessoais, logs privados, mods baixados ou conteúdo de terceiros protegido. Respeite os autores e as restrições de acesso da Steam.

## Fluxo de desenvolvimento

1. Faça um fork e crie uma branch a partir de `main`.
2. Instale Rust 1.96.1+ com MSVC, C++ Build Tools e Windows SDK.
3. Faça uma mudança focada. Preserve GPUI nativo, controles compactos, transparência arredondada e animação física.
4. Adicione teste de regressão quando houver mudança de comportamento. Para mudanças visuais, inclua captura ou vídeo real no Windows e informe a escala de tela.
5. Execute:

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

6. Abra um PR explicando problema, comportamento final, validação e limitações. Vincule a issue correspondente.

Textos da interface ficam em `src/i18n.rs`. Inclua as três traduções ou peça ajuda explicitamente no PR. Atualize todos os READMEs quando o comportamento mudar.

## Qualidade e segurança

Mantenha módulos focados. Preserve validação estrita de links, acesso anônimo, cancelamento, encerramento dos subprocessos e cópia sem sobrescrever. Telemetria e login exigem discussão de projeto prévia. Ao alterar dependências, inclua Cargo.lock, revise licenças e regenere os avisos de terceiros. Nunca execute comandos de shell fornecidos pelo usuário.

Execute `cargo audit` com cargo-audit antes de propor mudanças de dependências. Os avisos de manutenção existentes estão em [DEPENDENCIES.md](DEPENDENCIES.md); novos avisos precisam de correção ou avaliação documentada. O empacotamento usa `scripts/package.ps1` e Inno Setup 6.

Ao contribuir, você concorda com a distribuição da contribuição sob MIT. Seja respeitoso, forneça relatos reproduzíveis e revise o código, não as pessoas.
