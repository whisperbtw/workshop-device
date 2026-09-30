<p align="center"><img src="../assets/app-icon.png" width="180" alt="Arte do Workshop Device"></p>

# Workshop Device

**Baixe itens do Steam Workshop para a pasta que você escolher.** Cole o link de um mod, mapa ou outro item do Workshop, escolha onde salvar e aperte **Baixar**. O Workshop Device identifica o jogo automaticamente e salva os arquivos em uma pasta separada. É um aplicativo nativo para Windows; você não precisa digitar comandos do SteamCMD.

[English](../README.md) · **Português** · [日本語](README.ja.md)

[Baixar a versão mais recente](https://github.com/whisperbtw/workshop-device/releases/latest) · [Página do projeto](https://whisperbtw.github.io/workshop-device/) · [Contribuir](CONTRIBUTING.pt-BR.md) · [Relatar problema](https://github.com/whisperbtw/workshop-device/issues)

A imagem acima é a arte do aplicativo, não uma captura de tela.

## Instalação

Requer Windows 10/11 x64, internet e um driver gráfico compatível com o renderizador do GPUI.

- **Instalador:** baixe `Workshop-Device-Setup-1.2.0.exe` na seção Releases. Instala para o seu usuário, cria um atalho no menu Iniciar, oferece um atalho opcional na área de trabalho e inclui desinstalador. Não exige administrador.
- **Portátil:** extraia `Workshop-Device-1.2.0-windows-x64.zip` e abra `Workshop-Device.exe`. Mantenha os avisos de licença incluídos.
- `SHA256SUMS.txt` contém os hashes dos arquivos. Confira com `Get-FileHash .\Workshop-Device-Setup-1.2.0.exe -Algorithm SHA256`.

O aplicativo é distribuído sem assinatura digital; não há previsão de contratar um certificado. O Windows pode mostrar um aviso de publicador desconhecido. Baixe apenas das Releases deste repositório.

## Como usar

1. Abra o Workshop Device.
2. Cole o link HTTPS de um item individual no campo **LINK DO WORKSHOP**. Copie o link da página do item no Steam Workshop. Também é possível usar o ID numérico.
3. Escolha o destino pelo pequeno botão de pasta.
4. Aperte o controle grande **BAIXAR**. O visor mostra etapa, tempo decorrido e erros.
5. Ao concluir, use **SALVO · ABRIR PASTA** para acessar os arquivos.

O SteamCMD oficial é baixado e preparado automaticamente no primeiro uso. O destino inicial é a pasta Downloads do Windows com a subpasta `Workshop`; você pode escolher qualquer pasta onde tenha permissão para salvar. Cada download concluído fica em uma nova subpasta `<id-do-workshop>-<timestamp>`. Downloads anteriores não são sobrescritos.

Arraste a alça superior para mover a janela. Os controles gravados **−** e **×** minimizam e fecham o app. Durante o download, o controle principal vira **PARAR**. Fechar o app encerra seu processo SteamCMD.

Depois de identificar o item, o visor mostra o **nome do jogo e do item**. O link **Dependências não incluídas** abre a página no Workshop para você conferir os itens exigidos e as instruções de instalação; eles não são baixados automaticamente.

Os erros diferenciam falta de chave de acesso, acesso negado, problemas de conexão, item indisponível e falha ao salvar quando há evidência. Falhas sem causa informada pelo SteamCMD continuam identificadas como desconhecidas. **Copiar diagnóstico** copia a versão do app, a categoria do erro e os IDs disponíveis do item e do jogo para relatar problemas. Não inclui caminhos pessoais nem logs completos da Steam.

## O que pode ser baixado

- Itens individuais do Workshop de diferentes jogos que permitem download anônimo pelo SteamCMD. O app identifica o jogo pelos metadados da Steam, sem restringir a um jogo ou exigir um seletor.
- Coleções, links inválidos, itens sem um ID de jogo válido e itens indisponíveis são recusados.
- Inscrever-se na Steam não inicia um download neste app: cole o link aqui.
- Alguns itens exigem autenticação ou acesso pela conta Steam. Esta versão não tem login na Steam e não baixa esses itens.
- O download não instala nem ativa o item no jogo. Siga as instruções do autor e confira a compatibilidade com a sua versão.
- O app usa o acesso permitido pelo SteamCMD e não contorna restrições.

Consulte [os itens testados e os resultados](COMPATIBILITY.md). A compatibilidade depende de cada item e do acesso pela Steam.

## Arquivos, privacidade e problemas

Preferências, SteamCMD, cache dos itens, recibos e logs ficam em `%LOCALAPPDATA%\PZWorkshopDownloader`. Esse nome antigo do cache é mantido para preservar preferências e downloads nas atualizações. A pasta de destino recebe os arquivos copiados do item. Uma cópia interrompida pode deixar uma pasta provisória `.partial`; ela não representa um download concluído.

O app não pede senha Steam nem chave de API. Ele acessa a API de metadados da Steam e o serviço oficial de download do SteamCMD; o SteamCMD acessa os servidores Steam. Não há serviço de análise de uso no app.

Os erros aparecem no visor. Se o download falhar, confira se o item é público e o jogo permite acesso anônimo pelo SteamCMD. Para erros ao salvar, verifique permissões e espaço disponível. Os logs do próprio SteamCMD ficam no cache.

A desinstalação remove o aplicativo e atalhos; preserva arquivos baixados, cache e preferências. Para reiniciar as preferências, feche o app e remova a pasta de cache manualmente.

## Configurações e idiomas

A engrenagem à esquerda muda o mesmo visor para configurações. Escolha **English**, **Português** ou **日本語**. Na primeira abertura, o idioma da interface do Windows é detectado; idiomas não suportados usam inglês. Sua escolha fica salva.

**Abrir pasta ao concluir** é opcional e começa desativado. As configurações são salvas automaticamente. Volte pela seta ou pelo controle central. Abrir as configurações não cancela o download em andamento.

## Compilar e empacotar

Instale Rust **1.96.1 ou posterior**, a toolchain MSVC, Visual Studio C++ Build Tools e Windows SDK. Em Windows x64:

```powershell
git clone https://github.com/whisperbtw/workshop-device.git
cd workshop-device
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

O executável fica em `target\release\workshop-device.exe`. Ícone e metadados são incorporados na compilação.

Instale [Inno Setup 6](https://jrsoftware.org/isinfo.php) e execute:

```powershell
pwsh -File scripts/package.ps1
```

Os pacotes e hashes ficam em `dist\`. O CI verifica formatação, Clippy, testes e avisos de dependências. Tags `v*` geram os pacotes; a versão deve corresponder ao Cargo.toml. Uma compilação de tag bem-sucedida publica a Release.

## Estrutura

| Caminho | Função |
| --- | --- |
| `src/ui.rs`, `buttons.rs`, `device.rs`, `native.rs` | Interface GPUI, animação e janela sem moldura |
| `src/backend.rs`, `steamcmd.rs`, `install.rs` | Validação, ciclo do SteamCMD e cópia segura |
| `src/session.rs`, `model.rs`, `i18n.rs` | Estado do download, preferências e traduções |
| `assets/` | Ícones incorporados e arte original |
| `installer/`, `scripts/`, `.github/workflows/` | Empacotamento, validação e releases |
| `docs/` | Documentação traduzida e página pública |

## Contribuições são bem-vindas

**Este projeto aceita contribuições.** Correções, traduções, acessibilidade, testes e melhorias pontuais da interface nativa são bem-vindos. Leia [o guia de contribuição](CONTRIBUTING.pt-BR.md) antes de abrir um pull request. O guia também existe em inglês e japonês.

## Licença e créditos

Código e arte estão sob [MIT](../LICENSE). As dependências mantêm suas licenças; [THIRD-PARTY-NOTICES.html](../THIRD-PARTY-NOTICES.html) acompanha os pacotes. O ícone foi gerado por IA; o prompt está em [assets/IMAGE.md](../assets/IMAGE.md).

Feito com [GPUI](https://www.gpui.rs/), [gpui-component](https://github.com/longbridge/gpui-component) e [SteamCMD](https://developer.valvesoftware.com/wiki/SteamCMD). O SteamCMD é baixado durante o uso e não é incluído no pacote. Projeto da comunidade, sem vínculo com a Valve ou desenvolvedores dos jogos. O uso do SteamCMD não significa aprovação da Valve; respeite os termos da Steam e a licença de cada item.
