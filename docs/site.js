const translations = {
  en: {
    language: 'Language', eyebrow: 'WINDOWS · RUST · GPUI', title: 'Your mods. In your folder.',
    intro: 'Paste a Project Zomboid Workshop link, choose a destination and press download. A small native app that feels like a device.',
    download: 'Download for Windows', version: 'Installer + portable · Windows 10/11 x64', readme: 'Read the guide', source: 'Source code',
    one: 'One link. One folder.', oneText: 'Official SteamCMD, prepared automatically. Each completed mod gets its own folder.',
    two: 'A native little device.', twoText: 'Rounded edges, physical button animation and status inside the display. Built in Rust with GPUI.',
    three: 'In your language.', threeText: 'English, Portuguese and Japanese. The first launch detects your Windows language.',
    limits: 'Before you download', limitsText: 'Supports individual Project Zomboid items that permit anonymous SteamCMD access. Collections and login-required items are unsupported. Downloading saves files; installation and activation in the game are up to you. The release is unsigned.',
    art: 'The illustration is application artwork, not a screenshot.', contribute: 'Contributions welcome.',
    contributeText: 'Help improve translations, accessibility, reliability or the native interface. Read the contribution guide, open an issue or send a focused pull request.',
    contributionGuide: 'Contribution guide', issues: 'Issues and ideas', community: 'Community project. Not affiliated with Valve or The Indie Stone.'
  },
  pt: {
    language: 'Idioma', eyebrow: 'WINDOWS · RUST · GPUI', title: 'Seus mods. Na sua pasta.',
    intro: 'Cole o link de um mod do Workshop de Project Zomboid, escolha o destino e baixe. Um aplicativo nativo e pequeno, com cara de dispositivo.',
    download: 'Baixar para Windows', version: 'Instalador + portátil · Windows 10/11 x64', readme: 'Ler o guia', source: 'Código-fonte',
    one: 'Um link. Uma pasta.', oneText: 'SteamCMD oficial, preparado automaticamente. Cada mod concluído fica na própria pasta.',
    two: 'Um pequeno dispositivo.', twoText: 'Cantos arredondados, animação física nos botões e status no visor. Feito em Rust com GPUI.',
    three: 'No seu idioma.', threeText: 'Inglês, português e japonês. A primeira abertura detecta o idioma do Windows.',
    limits: 'Antes de baixar', limitsText: 'Suporta itens individuais de Project Zomboid com acesso anônimo pelo SteamCMD. Coleções e itens que exigem login não são suportados. O download salva arquivos; você instala e ativa o mod no jogo. A release não tem assinatura digital.',
    art: 'A ilustração é a arte do aplicativo, não uma captura de tela.', contribute: 'Contribuições são bem-vindas.',
    contributeText: 'Ajude com traduções, acessibilidade, confiabilidade ou a interface nativa. Leia o guia, abra uma issue ou envie um pull request focado.',
    contributionGuide: 'Guia de contribuição', issues: 'Problemas e ideias', community: 'Projeto da comunidade. Sem vínculo com Valve ou The Indie Stone.'
  },
  ja: {
    language: '言語', eyebrow: 'WINDOWS · RUST · GPUI', title: 'MOD を、あなたのフォルダーへ。',
    intro: 'Project Zomboid の Workshop リンクを貼り付け、保存先を選んでダウンロード。実機のような、小さなネイティブアプリです。',
    download: 'Windows 版をダウンロード', version: 'インストーラー＋ポータブル版 · Windows 10/11 x64', readme: 'ガイドを読む', source: 'ソースコード',
    one: 'リンクと保存先だけ。', oneText: '公式 SteamCMD を自動で準備。完了した MOD はそれぞれ専用フォルダーに保存します。',
    two: '小さなネイティブ機器。', twoText: '丸い輪郭、押下アニメーション、画面内の状態表示。Rust と GPUI で作られています。',
    three: 'あなたの言語で。', threeText: '英語、ポルトガル語、日本語に対応。初回起動時に Windows の表示言語を検出します。',
    limits: 'ダウンロードの前に', limitsText: '匿名 SteamCMD アクセスが許可された Project Zomboid の個別アイテムに対応します。コレクションとログインが必要なアイテムには対応しません。ファイルを保存するアプリのため、ゲームへのインストールと有効化はご自身で行ってください。リリースにコード署名はありません。',
    art: '画像はアプリのアートワークであり、スクリーンショットではありません。', contribute: '貢献を歓迎します。',
    contributeText: '翻訳、アクセシビリティ、信頼性、ネイティブ UI の改善にご協力ください。ガイドを読み、Issue または目的を絞った Pull Request を送ってください。',
    contributionGuide: '貢献ガイド', issues: '問題とアイデア', community: 'Valve および The Indie Stone とは関係のないコミュニティプロジェクトです。'
  }
};
const root = 'https://github.com/whisperbtw/workshop-device';
const language = document.getElementById('language');
function setLanguage(value) {
  const strings = translations[value] || translations.en;
  document.documentElement.lang = value === 'pt' ? 'pt-BR' : value;
  document.querySelectorAll('[data-t]').forEach(element => {
    element.textContent = strings[element.dataset.t];
  });
  document.getElementById('readme').href = value === 'en' ? root + '#readme' : root + '/blob/main/docs/README.' + (value === 'pt' ? 'pt-BR' : 'ja') + '.md';
  document.getElementById('contributing').href = root + '/blob/main/' + (value === 'en' ? 'CONTRIBUTING.md' : 'docs/CONTRIBUTING.' + (value === 'pt' ? 'pt-BR' : 'ja') + '.md');
}
language.addEventListener('change', () => setLanguage(language.value));
setLanguage('en');
