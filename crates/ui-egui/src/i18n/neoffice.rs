// //// Neoffice — added file (no upstream equivalent): Neoffice's own interface strings, kept out of upstream's catalogs.

//! The strings Neoffice added to GridCraft (the Nora assistant pane and its switch in File › Options,
//! and the messages of the hosting page's bridge, see `apps/gridcraft-web/src/host.rs`), kept outside
//! upstream's catalogs. [`super::Language::tr`] asks here when the language's own catalog has no entry
//! for a string, so an upstream merge never touches these, and a catalog that gains the same English
//! string wins over these tables.
//!
//! French is the first language here: Neoffice's customers read it, and its wording is WordCraft's
//! where the string is the same. Every other language of the interface has the same strings (all
//! Neoffice's translations are made by Haiku, the French then read against WordCraft's).

use super::Language;

/// Neoffice's strings in French (Swiss French, formal "vous"), English first. Every other table has the same
/// English strings, in the same order.
const FRENCH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Impossible d'enregistrer {name} : {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Ce classeur est ouvert depuis la page qui héberge l'éditeur."),
    ("Assistant", "Assistant"),
    ("Show the Nora assistant pane (experimental)", "Afficher le volet de l'assistant Nora (expérimental)"),
    ("Nora isn't available in this version yet", "Nora n'est pas encore disponible dans cette version"),
    ("New conversation", "Nouvelle conversation"),
    ("Undo", "Annuler"),
    ("Undo the last change Nora made to the workbook", "Annuler la dernière modification faite par Nora dans le classeur"),
    ("Ask Nora…", "Demander à Nora…"),
    ("Send", "Envoyer"),
    ("Nora is working…", "Nora travaille…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Demandez à Nora de remplir, de calculer, de trier ou de mettre en forme votre classeur.",
    ),
    ("Nora wants to:", "Nora souhaite :"),
    ("Allow", "Autoriser"),
    ("Refuse", "Refuser"),
    ("Read the workbook", "Lire le classeur"),
    ("Run {command}", "Exécuter {command}"),
    ("Run {count} commands", "Exécuter {count} commandes"),
    ("Select \"{text}\"", "Sélectionner \"{text}\""),
    ("Type \"{text}\"", "Saisir \"{text}\""),
    ("Look at page {page}", "Consulter la page {page}"),
    ("Save the workbook", "Enregistrer le classeur"),
    ("Undo the last change", "Annuler la dernière modification"),
    ("There is nothing of Nora's to undo.", "Il n'y a aucune modification de Nora à annuler."),
    ("Nora's change was made in another workbook.", "La modification de Nora a été faite dans un autre classeur."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Le classeur a changé depuis la modification de Nora : annulez-la étape par étape avec la commande Annuler.",
    ),
];

/// The same in German.
const GERMAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "{name} konnte nicht gespeichert werden: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Diese Arbeitsmappe wird über die Seite geöffnet, die den Editor enthält."),
    ("Assistant", "Assistent"),
    ("Show the Nora assistant pane (experimental)", "Nora-Assistenzbereich anzeigen (experimentell)"),
    ("Nora isn't available in this version yet", "Nora ist in dieser Version noch nicht verfügbar"),
    ("New conversation", "Neue Unterhaltung"),
    ("Undo", "Rückgängig"),
    ("Undo the last change Nora made to the workbook", "Die letzte Änderung von Nora an der Arbeitsmappe rückgängig machen"),
    ("Ask Nora…", "Nora fragen…"),
    ("Send", "Senden"),
    ("Nora is working…", "Nora arbeitet…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Bitten Sie Nora, Ihre Arbeitsmappe auszufüllen, zu berechnen, zu sortieren oder zu formatieren.",
    ),
    ("Nora wants to:", "Nora möchte:"),
    ("Allow", "Zulassen"),
    ("Refuse", "Ablehnen"),
    ("Read the workbook", "Arbeitsmappe lesen"),
    ("Run {command}", "{command} ausführen"),
    ("Run {count} commands", "{count} Befehle ausführen"),
    ("Select \"{text}\"", "„{text}“ auswählen"),
    ("Type \"{text}\"", "„{text}“ eingeben"),
    ("Look at page {page}", "Seite {page} ansehen"),
    ("Save the workbook", "Arbeitsmappe speichern"),
    ("Undo the last change", "Letzte Änderung rückgängig machen"),
    ("There is nothing of Nora's to undo.", "Es gibt keine Änderung von Nora mehr, die rückgängig gemacht werden kann."),
    ("Nora's change was made in another workbook.", "Die Änderung von Nora wurde in einer anderen Arbeitsmappe vorgenommen."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Die Arbeitsmappe hat sich seit Noras Änderung verändert: Nehmen Sie diese Schritt für Schritt mit dem Befehl Rückgängig zurück.",
    ),
];

/// The same in Spanish.
const SPANISH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "No se pudo guardar {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Este libro se abre desde la página que aloja el editor."),
    ("Assistant", "Asistente"),
    ("Show the Nora assistant pane (experimental)", "Mostrar el panel del asistente Nora (experimental)"),
    ("Nora isn't available in this version yet", "Nora todavía no está disponible en esta versión"),
    ("New conversation", "Nueva conversación"),
    ("Undo", "Deshacer"),
    ("Undo the last change Nora made to the workbook", "Deshacer el último cambio que Nora hizo en el libro"),
    ("Ask Nora…", "Preguntar a Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "Nora está trabajando…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Pida a Nora que rellene, calcule, ordene o dé formato a su libro."),
    ("Nora wants to:", "Nora quiere:"),
    ("Allow", "Permitir"),
    ("Refuse", "Rechazar"),
    ("Read the workbook", "Leer el libro"),
    ("Run {command}", "Ejecutar {command}"),
    ("Run {count} commands", "Ejecutar {count} comandos"),
    ("Select \"{text}\"", "Seleccionar «{text}»"),
    ("Type \"{text}\"", "Escribir «{text}»"),
    ("Look at page {page}", "Ver la página {page}"),
    ("Save the workbook", "Guardar el libro"),
    ("Undo the last change", "Deshacer el último cambio"),
    ("There is nothing of Nora's to undo.", "No queda ningún cambio de Nora por deshacer."),
    ("Nora's change was made in another workbook.", "El cambio de Nora se realizó en otro libro."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "El libro ha cambiado desde el cambio de Nora: deshágalo paso a paso con el comando Deshacer.",
    ),
];

/// The same in Brazilian Portuguese.
const PORTUGUESE: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Não foi possível salvar {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Esta pasta de trabalho é aberta a partir da página que hospeda o editor."),
    ("Assistant", "Assistente"),
    ("Show the Nora assistant pane (experimental)", "Mostrar o painel do assistente Nora (experimental)"),
    ("Nora isn't available in this version yet", "A Nora ainda não está disponível nesta versão"),
    ("New conversation", "Nova conversa"),
    ("Undo", "Desfazer"),
    ("Undo the last change Nora made to the workbook", "Desfazer a última alteração feita pela Nora na pasta de trabalho"),
    ("Ask Nora…", "Pergunte à Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "A Nora está trabalhando…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Peça à Nora para preencher, calcular, ordenar ou formatar sua pasta de trabalho.",
    ),
    ("Nora wants to:", "A Nora quer:"),
    ("Allow", "Permitir"),
    ("Refuse", "Recusar"),
    ("Read the workbook", "Ler a pasta de trabalho"),
    ("Run {command}", "Executar {command}"),
    ("Run {count} commands", "Executar {count} comandos"),
    ("Select \"{text}\"", "Selecionar “{text}”"),
    ("Type \"{text}\"", "Digitar “{text}”"),
    ("Look at page {page}", "Ver a página {page}"),
    ("Save the workbook", "Salvar a pasta de trabalho"),
    ("Undo the last change", "Desfazer a última alteração"),
    ("There is nothing of Nora's to undo.", "Não há nenhuma alteração da Nora para desfazer."),
    ("Nora's change was made in another workbook.", "A alteração da Nora foi feita em outra pasta de trabalho."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "A pasta de trabalho foi alterada depois da alteração feita pela Nora: desfaça-a passo a passo com o comando Desfazer.",
    ),
];

/// The same in Russian.
const RUSSIAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Не удалось сохранить {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Эта книга открыта со страницы, на которой размещён редактор."),
    ("Assistant", "Помощник"),
    ("Show the Nora assistant pane (experimental)", "Показывать панель помощника Nora (экспериментально)"),
    ("Nora isn't available in this version yet", "Nora пока недоступна в этой версии"),
    ("New conversation", "Новый диалог"),
    ("Undo", "Отменить"),
    ("Undo the last change Nora made to the workbook", "Отменить последнее изменение, внесённое Nora в книгу"),
    ("Ask Nora…", "Спросите Nora…"),
    ("Send", "Отправить"),
    ("Nora is working…", "Nora работает…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Попросите Nora заполнить, вычислить, отсортировать или отформатировать вашу книгу.",
    ),
    ("Nora wants to:", "Nora хочет:"),
    ("Allow", "Разрешить"),
    ("Refuse", "Отклонить"),
    ("Read the workbook", "Прочитать книгу"),
    ("Run {command}", "Выполнить {command}"),
    ("Run {count} commands", "Выполнить команды ({count})"),
    ("Select \"{text}\"", "Выбрать «{text}»"),
    ("Type \"{text}\"", "Ввести «{text}»"),
    ("Look at page {page}", "Посмотреть страницу {page}"),
    ("Save the workbook", "Сохранить книгу"),
    ("Undo the last change", "Отменить последнее изменение"),
    ("There is nothing of Nora's to undo.", "Нет изменений Nora, которые можно отменить."),
    ("Nora's change was made in another workbook.", "Изменение Nora было сделано в другой книге."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Книга была изменена после изменения, сделанного Nora: отменяйте его шаг за шагом с помощью команды «Отменить».",
    ),
];

/// The same in Czech.
const CZECH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Nepodařilo se uložit {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Tento sešit je otevřen ze stránky, na které je editor vložen."),
    ("Assistant", "Asistent"),
    ("Show the Nora assistant pane (experimental)", "Zobrazit panel asistenta Nory (experimentální)"),
    ("Nora isn't available in this version yet", "Nora zatím není v této verzi k dispozici"),
    ("New conversation", "Nová konverzace"),
    ("Undo", "Zpět"),
    ("Undo the last change Nora made to the workbook", "Vrátit poslední změnu, kterou Nora provedla v sešitu"),
    ("Ask Nora…", "Zeptejte se Nory…"),
    ("Send", "Odeslat"),
    ("Nora is working…", "Nora pracuje…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Požádejte Noru, ať vyplní, spočítá, seřadí nebo naformátuje váš sešit."),
    ("Nora wants to:", "Nora chce:"),
    ("Allow", "Povolit"),
    ("Refuse", "Odmítnout"),
    ("Read the workbook", "Číst sešit"),
    ("Run {command}", "Spustit {command}"),
    ("Run {count} commands", "Spustit příkazy ({count})"),
    ("Select \"{text}\"", "Vybrat „{text}“"),
    ("Type \"{text}\"", "Napsat „{text}“"),
    ("Look at page {page}", "Podívat se na stránku {page}"),
    ("Save the workbook", "Uložit sešit"),
    ("Undo the last change", "Vrátit poslední změnu"),
    ("There is nothing of Nora's to undo.", "Není žádná změna Nory, kterou by bylo možné vrátit."),
    ("Nora's change was made in another workbook.", "Změna Nory byla provedena v jiném sešitu."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Sešit se změnil po změně, kterou provedla Nora: vraťte ji krok za krokem pomocí příkazu Zpět.",
    ),
];

/// The same in Japanese.
const JAPANESE: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "{name} を保存できませんでした：{error}"),
    ("This workbook is opened from the page that hosts the editor.", "このブックは、エディターを埋め込んだページから開かれています。"),
    ("Assistant", "アシスタント"),
    ("Show the Nora assistant pane (experimental)", "Nora アシスタントパネルを表示する（実験的）"),
    ("Nora isn't available in this version yet", "Nora はこのバージョンではまだ利用できません"),
    ("New conversation", "新しい会話"),
    ("Undo", "元に戻す"),
    ("Undo the last change Nora made to the workbook", "Nora がブックに加えた最後の変更を元に戻す"),
    ("Ask Nora…", "Nora に質問…"),
    ("Send", "送信"),
    ("Nora is working…", "Nora が処理中…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Nora に、ブックへの入力、計算、並べ替え、書式設定を依頼してください。"),
    ("Nora wants to:", "Nora が次の操作を実行しようとしています："),
    ("Allow", "許可"),
    ("Refuse", "拒否"),
    ("Read the workbook", "ブックを読み取る"),
    ("Run {command}", "{command} を実行"),
    ("Run {count} commands", "{count} 件のコマンドを実行"),
    ("Select \"{text}\"", "「{text}」を選択"),
    ("Type \"{text}\"", "「{text}」を入力"),
    ("Look at page {page}", "ページ {page} を表示"),
    ("Save the workbook", "ブックを保存"),
    ("Undo the last change", "最後の変更を元に戻す"),
    ("There is nothing of Nora's to undo.", "取り消せる Nora の変更はありません。"),
    ("Nora's change was made in another workbook.", "Nora の変更は別のブックで行われました。"),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Nora の変更後にブックが変更されています：元に戻すコマンドで一つずつ取り消してください。",
    ),
];

/// The same in Simplified Chinese.
const CHINESE: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "无法保存 {name}：{error}"),
    ("This workbook is opened from the page that hosts the editor.", "此工作簿是从承载编辑器的页面打开的。"),
    ("Assistant", "助手"),
    ("Show the Nora assistant pane (experimental)", "显示 Nora 助手窗格（实验性）"),
    ("Nora isn't available in this version yet", "Nora 在此版本中暂不可用"),
    ("New conversation", "新对话"),
    ("Undo", "撤销"),
    ("Undo the last change Nora made to the workbook", "撤销 Nora 对工作簿所做的最后一次更改"),
    ("Ask Nora…", "向 Nora 提问…"),
    ("Send", "发送"),
    ("Nora is working…", "Nora 正在处理…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "让 Nora 填充、计算、排序或设置工作簿的格式。"),
    ("Nora wants to:", "Nora 想要执行："),
    ("Allow", "允许"),
    ("Refuse", "拒绝"),
    ("Read the workbook", "读取工作簿"),
    ("Run {command}", "执行 {command}"),
    ("Run {count} commands", "执行 {count} 个命令"),
    ("Select \"{text}\"", "选择“{text}”"),
    ("Type \"{text}\"", "输入“{text}”"),
    ("Look at page {page}", "查看第 {page} 页"),
    ("Save the workbook", "保存工作簿"),
    ("Undo the last change", "撤销上一次更改"),
    ("There is nothing of Nora's to undo.", "没有可以撤销的 Nora 更改。"),
    ("Nora's change was made in another workbook.", "Nora 的更改是在另一个工作簿中进行的。"),
    ("The workbook has changed since Nora's change: undo it step by step with Undo.", "Nora 更改后工作簿已发生变化：请使用“撤销”命令逐步撤销。"),
];

/// The same in Korean.
const KOREAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "{name}을(를) 저장할 수 없습니다: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "이 통합 문서는 편집기가 포함된 페이지에서 열려 있습니다."),
    ("Assistant", "어시스턴트"),
    ("Show the Nora assistant pane (experimental)", "Nora 어시스턴트 패널 표시 (실험적)"),
    ("Nora isn't available in this version yet", "Nora는 이 버전에서 아직 사용할 수 없습니다"),
    ("New conversation", "새 대화"),
    ("Undo", "실행 취소"),
    ("Undo the last change Nora made to the workbook", "Nora가 통합 문서에 적용한 마지막 변경을 실행 취소"),
    ("Ask Nora…", "Nora에게 질문…"),
    ("Send", "보내기"),
    ("Nora is working…", "Nora가 작업 중…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Nora에게 통합 문서 채우기, 계산, 정렬 또는 서식 지정을 요청하세요."),
    ("Nora wants to:", "Nora가 다음을 하려고 합니다:"),
    ("Allow", "허용"),
    ("Refuse", "거부"),
    ("Read the workbook", "통합 문서 읽기"),
    ("Run {command}", "{command} 실행"),
    ("Run {count} commands", "{count}개 명령 실행"),
    ("Select \"{text}\"", "“{text}” 선택"),
    ("Type \"{text}\"", "“{text}” 입력"),
    ("Look at page {page}", "{page} 페이지 보기"),
    ("Save the workbook", "통합 문서 저장"),
    ("Undo the last change", "마지막 변경 실행 취소"),
    ("There is nothing of Nora's to undo.", "실행 취소할 Nora의 변경 사항이 없습니다."),
    ("Nora's change was made in another workbook.", "Nora의 변경 사항은 다른 통합 문서에서 이루어졌습니다."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Nora의 변경 이후 통합 문서가 변경되었습니다: 실행 취소 명령으로 한 단계씩 되돌리세요.",
    ),
];

/// Corrections of upstream's French catalog as it stands in v0.6.1: about 50 of its entries read as the
/// French of ANOTHER English string ("File" is "Nom du fichier", "Hide Detail" is "Insérer les cellules
/// copiées"), the rest are wrappings and wordings. Upstream fixed them in storytold/gridcraft `c59e942`
/// ("correct scattered wrong French translations"); these are its values, asked BEFORE the catalog.
/// Drop this table at the merge that brings that commit. Three of them keep their key's context
/// ("Options|Général"): [`super::Language::tr`] shows the label alone, and keeps doing so after that merge.
const FRENCH_FIXES: &[(&str, &str)] = &[
    ("Insert Copied Cells", "Insérer les cellules copiées"),
    ("Insert Copied Cells…", "Insérer les cellules copiées…"),
    ("Insert Cut Cells", "Insérer les cellules coupées"),
    ("Insert Paste", "Insérer en collant"),
    ("Double Underline", "Double soulignement"),
    ("Increase Font Size", "Augmenter la taille de police"),
    ("Decrease Font Size", "Réduire la taille de police"),
    ("Wrap Text", "Renvoyer à la ligne automatiquement"),
    ("Unmerge Cells", "Annuler la fusion des cellules"),
    ("Top\nAlign", "Aligner en\nhaut"),
    ("Middle\nAlign", "Aligner au\nmilieu"),
    ("Bottom\nAlign", "Aligner en\nbas"),
    ("Comma Style", "Style de milliers"),
    ("Conditional\nFormatting", "Mise en forme\nconditionnelle"),
    ("Format\nas Table", "Mettre sous forme\nde tableau"),
    ("Cell\nStyles", "Styles de\ncellules"),
    ("Format Cell Alignment…", "Format d'alignement de cellule…"),
    ("Format\nPane", "Volet\nFormat"),
    ("Flash\nFill", "Remplissage\ninstantané"),
    ("Clear Print Area", "Effacer la zone d'impression"),
    ("Find &\nSelect", "Rechercher et\nsélectionner"),
    ("Recommended\nCharts", "Graphiques\nrecommandés"),
    ("Text\nBox", "Zone de\ntexte"),
    ("New\nComment", "Nouveau\ncommentaire"),
    ("Header &\nFooter", "En-tête et\npied de page"),
    ("Text to\nColumns", "Convertir le texte\nen colonnes"),
    ("Sparklines", "Courbes de tendance"),
    ("Checkbox", "Case à cocher"),
    ("Insert checkboxes in the selected cells", "Insérer des cases à cocher dans les cellules sélectionnées"),
    ("Draw with ink", "Dessiner à l'encre"),
    ("Ink to\nShape", "Encre en\nforme"),
    ("Select\nObjects", "Sélectionner les\nobjets"),
    ("Print\nArea", "Zone\nd'impression"),
    ("Set Print Area", "Définir la zone d'impression"),
    ("Print\nTitles", "Titres à\nimprimer"),
    ("Sheet\nRight-to-Left", "Feuille de\ndroite à gauche"),
    ("Sheet Right-to-Left", "Feuille de droite à gauche"),
    ("Narrow", "Étroit"),
    ("Wide", "Large"),
    ("Normal", "Normal"),
    ("Insert\nFunction", "Insérer une\nfonction"),
    ("Name\nManager", "Gestionnaire\nde noms"),
    ("Calculation\nOptions", "Options de\ncalcul"),
    ("Sort &\nFilter", "Trier et\nfiltrer"),
    ("Get Data\n(Text/CSV)", "Obtenir des données\n(Texte/CSV)"),
    ("Remove\nDuplicates", "Supprimer les\ndoublons"),
    ("Data\nValidation", "Validation des\ndonnées"),
    ("Show Detail", "Afficher le détail"),
    ("Hide Detail", "Masquer le détail"),
    ("Data\nAnalysis", "Analyse de\ndonnées"),
    ("Data Analysis", "Analyse de données"),
    ("Solver", "Solveur"),
    ("What-If\nAnalysis", "Analyse\nde scénarios"),
    ("Forecast\nSheet", "Feuille de\nprévision"),
    ("Check\nAccessibility", "Vérifier\nl'accessibilité"),
    ("Threaded Comments", "Commentaires en fil de discussion"),
    ("Show\nComments", "Afficher les\ncommentaires"),
    ("Previous", "Précédent"),
    ("Next", "Suivant"),
    ("Previous Comment", "Commentaire précédent"),
    ("Next Comment", "Commentaire suivant"),
    ("Show All Notes", "Afficher toutes les notes"),
    ("Convert to Comments", "Convertir en commentaires"),
    ("Protect\nWorkbook", "Protéger le\nclasseur"),
    ("Allow Edit\nRanges", "Autoriser la modification\ndes plages"),
    ("Allow Edit Ranges", "Autoriser la modification des plages"),
    ("Unprotect\nSheet", "Ôter la protection\nde la feuille"),
    ("Page Break\nPreview", "Aperçu des sauts\nde page"),
    ("Page\nLayout", "Mise en\npage"),
    ("100%", "100 %"),
    ("Zoom to\nSelection", "Zoom sur la\nsélection"),
    ("Freeze\nPanes", "Figer les\nvolets"),
    ("Display theme", "Thème d'affichage"),
    ("Animations", "Animations"),
    ("Interface language", "Langue de l'interface"),
    ("Interface size", "Taille de l'interface"),
    ("Auto", "Automatique"),
    ("Drag to freeze columns", "Faites glisser pour figer les colonnes"),
    ("Drag to freeze rows", "Faites glisser pour figer les lignes"),
    ("Command\nPalette", "Palette de\ncommandes"),
    ("Agent\nControl", "Contrôle de\nl'agent"),
    ("Action\nJournal", "Journal des\nactions"),
    ("About\nGridCraft", "À propos de\nGridCraft"),
    ("Add Chart\nElement", "Ajouter un élément\nde graphique"),
    ("Change\nChart Type", "Modifier le type\nde graphique"),
    ("Switch\nRow/Column", "Permuter\nligne/colonne"),
    ("Select\nData", "Sélectionner les\ndonnées"),
    ("Chart\nTitle", "Titre du\ngraphique"),
    ("Delete\nChart", "Supprimer le\ngraphique"),
    ("Convert\nto Range", "Convertir\nen plage"),
    ("Table\nStyles", "Styles de\ntableau"),
    ("Icon Sets", "Jeux d'icônes"),
    ("Date &\nTime", "Date et\nheure"),
    ("Lookup &\nReference", "Recherche et\nréférence"),
    ("Math &\nTrig", "Maths et\ntrigonométrie"),
    ("More\nFunctions", "Autres\nfonctions"),
    ("Sheet Options|View", "Options de la feuille|Affichage"),
    ("Click to add data", "Cliquez pour ajouter des données"),
    ("Add header", "Ajouter un en-tête"),
    ("Add footer", "Ajouter un pied de page"),
    ("Header", "En-tête"),
    ("Footer", "Pied de page"),
    ("Page\nNumber", "Numéro de\npage"),
    ("Number\nof Pages", "Nombre\nde pages"),
    ("Current\nDate", "Date\nactuelle"),
    ("Current\nTime", "Heure\nactuelle"),
    ("File\nPath", "Chemin\ndu fichier"),
    ("File\nName", "Nom du\nfichier"),
    ("Sheet\nName", "Nom de la\nfeuille"),
    ("Go to\nHeader", "Aller à\nl'en-tête"),
    ("Go to\nFooter", "Aller au\npied de page"),
    ("Page Number", "Numéro de page"),
    ("Number of Pages", "Nombre de pages"),
    ("Current Date", "Date actuelle"),
    ("Current Time", "Heure actuelle"),
    ("File Path", "Chemin du fichier"),
    ("File Name", "Nom du fichier"),
    ("Sheet Name", "Nom de la feuille"),
    ("Go to Header", "Aller à l'en-tête"),
    ("Go to Footer", "Aller au pied de page"),
    ("Ruler", "Règle"),
    ("File", "Fichier"),
    ("Back", "Retour"),
    ("New", "Nouveau"),
    ("Open", "Ouvrir"),
    ("Info", "Informations"),
    ("Save", "Enregistrer"),
    ("Save As", "Enregistrer sous"),
    ("Export", "Exporter"),
    ("Close", "Fermer"),
    ("Options", "Options"),
    ("Blank workbook", "Classeur vide"),
    ("Recent", "Récent"),
    ("No recent workbooks yet.", "Aucun classeur récent pour le moment."),
    ("More workbooks", "Plus de classeurs"),
    ("Browse…", "Parcourir…"),
    ("Not saved yet", "Pas encore enregistré"),
    ("Location", "Emplacement"),
    ("Sheets", "Feuilles"),
    ("Protect Workbook", "Protéger le classeur"),
    ("Properties", "Propriétés"),
    ("Subject", "Sujet"),
    ("Author", "Auteur"),
    ("Company", "Société"),
    ("Keywords", "Mots-clés"),
    ("Description", "Description"),
    ("No workbook is open.", "Aucun classeur n'est ouvert."),
    ("Save as type", "Type de fichier"),
    ("Excel Workbook (*.xlsx)", "Classeur Excel (*.xlsx)"),
    ("CSV UTF-8 (Comma delimited) (*.csv)", "CSV UTF-8 (séparateur point-virgule) (*.csv)"),
    ("PDF (*.pdf)", "PDF (*.pdf)"),
    ("Download", "Télécharger"),
    ("Save the workbook under another name or in another folder.", "Enregistrer le classeur sous un autre nom ou dans un autre dossier."),
    ("Print Active Sheets", "Imprimer les feuilles actives"),
    ("Print Entire Workbook", "Imprimer tout le classeur"),
    ("Fit Sheet on One Page", "Ajuster la feuille sur une page"),
    ("Page Setup…", "Mise en page…"),
    ("Page {page} of {pages}", "Page {page} sur {pages}"),
    ("Nothing to print.", "Rien à imprimer."),
    ("Previous page", "Page précédente"),
    ("Next page", "Page suivante"),
    ("Create PDF", "Créer un PDF"),
    ("Change File Type", "Changer le type de fichier"),
    ("CSV (Comma delimited)", "CSV (séparateur point-virgule)"),
    ("Save the active sheet as comma-separated text.", "Enregistrer la feuille active sous forme de texte séparé par des points-virgules."),
    ("Save the workbook as a web page.", "Enregistrer le classeur en tant que page web."),
    ("Options|General", "Options|Général"),
    ("Options|Save", "Options|Enregistrer"),
    ("Show formula bar", "Afficher la barre de formule"),
    ("Save .xlsx workbooks automatically after every change", "Enregistrer automatiquement les classeurs .xlsx après chaque modification"),
    ("Enable iterative calculation", "Activer le calcul itératif"),
    ("Calculation settings are saved with the workbook.", "Les paramètres de calcul sont enregistrés avec le classeur."),
];

/// What the corrected French says for `text`, when upstream's v0.6.1 catalog has it wrong.
pub(super) fn french_fix(text: &str) -> Option<&'static str> {
    FRENCH_FIXES.iter().find(|(english, _)| *english == text).map(|(_, french)| *french)
}

/// `text` in French, when Neoffice has translated it.
pub(super) fn french(text: &str) -> Option<&'static str> {
    find(FRENCH, text)
}

/// `text` in `language`, when Neoffice has translated it. English has nothing to translate, and French
/// is asked in its own chain, after its fixes and its catalog (see [`french`]).
pub(super) fn added(language: Language, text: &str) -> Option<&'static str> {
    table(language).and_then(|table| find(table, text))
}

/// Neoffice's table for a language other than English and French.
fn table(language: Language) -> Option<&'static [(&'static str, &'static str)]> {
    match language {
        Language::En | Language::Fr => None,
        Language::De => Some(GERMAN),
        Language::Es => Some(SPANISH),
        Language::PtBr => Some(PORTUGUESE),
        Language::Ru => Some(RUSSIAN),
        Language::Cs => Some(CZECH),
        Language::Ja => Some(JAPANESE),
        Language::Zh => Some(CHINESE),
        Language::Ko => Some(KOREAN),
    }
}

fn find(table: &'static [(&'static str, &'static str)], text: &str) -> Option<&'static str> {
    table.iter().find(|(english, _)| *english == text).map(|(_, translated)| *translated)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `{placeholders}` of a string, in order.
    fn placeholders(text: &str) -> Vec<&str> {
        text.split('{').skip(1).filter_map(|rest| rest.split_once('}').map(|(name, _)| name)).collect()
    }

    /// Every table of ours, French first.
    fn tables() -> Vec<(Language, &'static [(&'static str, &'static str)])> {
        let mut all = vec![(Language::Fr, FRENCH)];
        all.extend(Language::ALL.into_iter().filter_map(|language| table(language).map(|table| (language, table))));
        all
    }

    #[test]
    fn every_french_string_keeps_the_placeholders_of_its_english() {
        for (english, french) in FRENCH {
            assert_eq!(placeholders(english), placeholders(french), "{english:?}");
            assert_eq!(french.trim(), *french, "stray whitespace in {french:?}");
            assert!(!french.contains("..."), "use … rather than three dots: {french:?}");
        }
    }

    #[test]
    fn every_language_has_every_string_with_its_placeholders() {
        assert_eq!(tables().len(), Language::ALL.len() - 1, "every language but English");
        let english: Vec<&str> = FRENCH.iter().map(|(english, _)| *english).collect();
        for (language, table) in tables() {
            assert_eq!(table.iter().map(|(english, _)| *english).collect::<Vec<_>>(), english, "{language:?}: the same strings, in the same order");
            for (english, translated) in table {
                assert!(!translated.is_empty(), "{language:?}: {english:?}");
                assert_eq!(placeholders(english), placeholders(translated), "{language:?}: {english:?}");
                assert_eq!(translated.trim(), *translated, "{language:?}: stray whitespace in {translated:?}");
                assert!(!translated.contains("..."), "{language:?}: use … rather than three dots: {translated:?}");
                assert!(!english.contains('…') || translated.contains('…'), "{language:?}: {english:?} keeps its …");
            }
        }
    }

    #[test]
    fn the_english_strings_are_listed_once() {
        let mut seen = std::collections::HashSet::new();
        for (english, _) in FRENCH {
            assert!(seen.insert(*english), "{english:?} is listed twice");
        }
    }

    #[test]
    fn the_fixes_replace_what_the_catalog_gets_wrong() {
        assert_eq!(Language::Fr.tr("Hide Detail"), "Masquer le détail");
        assert_eq!(Language::Fr.tr("Previous Comment"), "Commentaire précédent");
        // A label that was right stays what upstream says.
        assert_eq!(french_fix("A string upstream has"), None);
    }

    #[test]
    fn a_fix_only_replaces_an_entry_the_catalog_has() {
        for (english, french) in FRENCH_FIXES {
            assert!(super::super::fr::translate(english).is_some(), "{english:?} is not in upstream's catalog: it belongs in FRENCH");
            assert_eq!(placeholders(english), placeholders(french), "{english:?}");
            assert_eq!(french.trim(), *french, "stray whitespace in {french:?}");
        }
    }

    #[test]
    fn the_fixes_are_listed_once() {
        let mut seen = std::collections::HashSet::new();
        for (english, _) in FRENCH_FIXES {
            assert!(seen.insert(*english), "{english:?} is listed twice");
        }
    }

    /// A `context|label` key shows its label alone, translated or not. Upstream's French of `c59e942` (the
    /// fixes above) writes three with their context, and File › Options read "Options|Général".
    #[test]
    fn a_key_s_context_never_shows() {
        assert_eq!(Language::Fr.tr("Options|General"), "Général");
        assert_eq!(Language::Fr.tr("Options|Save"), "Enregistrer");
        assert_eq!(Language::Fr.tr("Sheet Options|View"), "Affichage");
        let keys = super::super::JAPANESE.iter().chain(FRENCH_FIXES).chain(FRENCH).map(|(english, _)| *english);
        for key in keys.filter(|key| key.contains('|')) {
            for language in Language::ALL {
                assert!(!language.tr(key).contains('|'), "{language:?}: {key:?} reads {:?}", language.tr(key));
            }
        }
    }

    #[test]
    fn every_language_says_them_and_english_keeps_them() {
        let refusal = "Couldn't save {name}: {error}";
        assert_eq!(Language::Fr.tr(refusal), "Impossible d'enregistrer {name} : {error}");
        assert_eq!(Language::En.tr(refusal), refusal);
        for (language, table) in tables() {
            for (english, translated) in table {
                assert_eq!(language.tr(english), *translated, "{language:?}: {english:?}");
            }
        }
        assert_eq!(Language::De.tr("New conversation"), find(GERMAN, "New conversation").unwrap_or_default());
        assert_eq!(french("A string upstream has"), None);
        assert_eq!(added(Language::Ja, "A string upstream has"), None);
        // A string of upstream's catalogs is theirs: "Close" is not in our tables.
        assert_eq!(added(Language::De, "Close"), None);
        assert_eq!(Language::De.tr("Close"), "Schließen");
    }
}
