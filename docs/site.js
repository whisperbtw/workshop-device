const translations = {
  en: {
    language: 'Language', eyebrow: 'STEAM WORKSHOP · WINDOWS', title: 'Download Workshop files to your folder.',
    intro: 'Download mods, maps and other Steam Workshop items. Paste the item link, choose where to save it and press Download. The app detects the game automatically.',
    download: 'Download for Windows', version: 'Installer + portable · Windows 10/11 x64', readme: 'Read the guide', source: 'Source code',
    one: 'Paste a Workshop link.', oneText: 'The game is identified from Steam metadata. Official SteamCMD is prepared automatically; no terminal commands needed.',
    two: 'Choose the destination.', twoText: 'Save files where you want them. Each completed item gets a new folder, without overwriting previous downloads.',
    three: 'Follow the download.', threeText: 'See the current stage, elapsed time and errors in the display. Cancel while downloading or open the finished folder.',
    limits: 'Before you download', limitsText: 'Supports individual Workshop items across games that permit anonymous SteamCMD access. Collections and login-required items are unsupported. Downloading saves files; installation and activation in the game are up to you. The release is unsigned.',
    art: 'The illustration is application artwork, not a screenshot.', contribute: 'Contributions welcome.',
    contributeText: 'Help improve translations, accessibility, reliability or the native interface. Read the contribution guide, open an issue or send a focused pull request.',
    contributionGuide: 'Contribution guide', issues: 'Issues and ideas', community: 'Community project. Not affiliated with Valve or any game developer.'
  },
  pt: {
    language: 'Idioma', eyebrow: 'STEAM WORKSHOP · WINDOWS', title: 'Baixe itens do Workshop para sua pasta.',
    intro: 'Baixe mods, mapas e outros itens do Steam Workshop. Cole o link do item, escolha onde salvar e aperte Baixar. O app identifica o jogo automaticamente.',
    download: 'Baixar para Windows', version: 'Instalador + portátil · Windows 10/11 x64', readme: 'Ler o guia', source: 'Código-fonte',
    one: 'Cole o link do Workshop.', oneText: 'O jogo é identificado pelos metadados da Steam. O SteamCMD oficial é preparado automaticamente; você não precisa digitar comandos.',
    two: 'Escolha a pasta de destino.', twoText: 'Salve os arquivos onde quiser. Cada item concluído ganha uma pasta nova, sem sobrescrever downloads anteriores.',
    three: 'Acompanhe o download.', threeText: 'Veja etapa, tempo decorrido e erros no visor. Cancele durante o download ou abra a pasta ao concluir.',
    limits: 'Antes de baixar', limitsText: 'Suporta itens individuais do Workshop de diferentes jogos com acesso anônimo pelo SteamCMD. Coleções e itens que exigem login não são suportados. O download salva arquivos; você instala e ativa o mod no jogo. A release não tem assinatura digital.',
    art: 'A ilustração é a arte do aplicativo, não uma captura de tela.', contribute: 'Contribuições são bem-vindas.',
    contributeText: 'Ajude com traduções, acessibilidade, confiabilidade ou a interface nativa. Leia o guia, abra uma issue ou envie um pull request focado.',
    contributionGuide: 'Guia de contribuição', issues: 'Problemas e ideias', community: 'Projeto da comunidade. Sem vínculo com Valve ou desenvolvedores dos jogos.'
  },
  ja: {
    language: '言語', eyebrow: 'STEAM WORKSHOP · WINDOWS', title: 'Workshop のファイルを、指定のフォルダーへ。',
    intro: 'Steam Workshop の MOD、マップなどをダウンロード。リンクを貼り付け、保存先を選んでダウンロードすると、ゲームを自動判別します。',
    download: 'Windows 版をダウンロード', version: 'インストーラー＋ポータブル版 · Windows 10/11 x64', readme: 'ガイドを読む', source: 'ソースコード',
    one: 'Workshop リンクを貼り付け。', oneText: 'Steam のメタデータからゲームを判別し、公式 SteamCMD を自動で準備します。コマンド入力は不要です。',
    two: '保存先を選択。', twoText: '好きな場所にファイルを保存。各アイテムに新しいフォルダーを作成し、以前のダウンロードを上書きしません。',
    three: '進行状況を確認。', threeText: '処理段階、経過時間、エラーを画面に表示。途中でキャンセルしたり、完了後にフォルダーを開いたりできます。',
    limits: 'ダウンロードの前に', limitsText: '匿名 SteamCMD アクセスが許可された 各ゲームの個別 Workshop アイテムに対応します。コレクションとログインが必要なアイテムには対応しません。ファイルを保存するアプリのため、ゲームへのインストールと有効化はご自身で行ってください。リリースにコード署名はありません。',
    art: '画像はアプリのアートワークであり、スクリーンショットではありません。', contribute: '貢献を歓迎します。',
    contributeText: '翻訳、アクセシビリティ、信頼性、ネイティブ UI の改善にご協力ください。ガイドを読み、Issue または目的を絞った Pull Request を送ってください。',
    contributionGuide: '貢献ガイド', issues: '問題とアイデア', community: 'Valve および各ゲームの開発元 とは関係のないコミュニティプロジェクトです。'
  }
};
const root = 'https://github.com/whisperbtw/workshop-device';
const language = document.getElementById('language');
function setLanguage(value) {
  value = Object.hasOwn(translations, value) ? value : 'en';
  language.value = value;
  const strings = translations[value];
  document.documentElement.lang = value === 'pt' ? 'pt-BR' : value;
  document.querySelectorAll('[data-t]').forEach(element => {
    element.textContent = strings[element.dataset.t];
  });
  document.getElementById('readme').href = value === 'en' ? root + '#readme' : root + '/blob/main/docs/README.' + (value === 'pt' ? 'pt-BR' : 'ja') + '.md';
  document.getElementById('contributing').href = root + '/blob/main/' + (value === 'en' ? 'CONTRIBUTING.md' : 'docs/CONTRIBUTING.' + (value === 'pt' ? 'pt-BR' : 'ja') + '.md');
}
function initialLanguage() {
  try {
    const saved = localStorage.getItem('workshop-device-language');
    if (Object.hasOwn(translations, saved)) return saved;
  } catch { /* Automatic detection still works when browser storage is unavailable. */ }
  const locale = (navigator.languages?.[0] || navigator.language || 'en').split('-')[0].toLowerCase();
  return Object.hasOwn(translations, locale) ? locale : 'en';
}
language.addEventListener('change', () => {
  setLanguage(language.value);
  try { localStorage.setItem('workshop-device-language', language.value); } catch { /* Keep the selection for this visit. */ }
});
setLanguage(initialLanguage());
