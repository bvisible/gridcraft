// //// Neoffice — added file (no upstream equivalent): Neoffice's own interface strings, kept out of upstream's catalogs.

//! The strings Neoffice added to GridCraft (the Nora assistant pane and its switch in File › Options,
//! the messages of the hosting page's bridge, see `apps/gridcraft-web/src/host.rs`, and the status bar's words that
//! upstream leaves in English, in four languages), kept outside
//! upstream's catalogs. [`super::Language::tr`] asks here when the language's own catalog has no entry
//! for a string, so an upstream merge never touches these, and a catalog that gains the same English
//! string wins over these tables.
//!
//! Three of the pane's buttons are asked with a context, `Nora|Allow`, `Nora|Undo` and `Nora|Stop`: upstream's catalogs
//! translate the bare words for other places (Japanese `Allow` is data validation's "type of input value"), and a
//! catalog's entry wins over these tables. The context never shows ([`super::Language::tr`] drops it).
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
    ("Nora|Undo", "Annuler"),
    ("Undo the last change Nora made to the workbook", "Annuler la dernière modification faite par Nora dans le classeur"),
    ("Ask Nora…", "Demander à Nora…"),
    ("Send", "Envoyer"),
    ("Nora is working…", "Nora travaille…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Demandez à Nora de remplir, de calculer, de trier ou de mettre en forme votre classeur.",
    ),
    ("Nora wants to:", "Nora souhaite :"),
    ("Nora|Allow", "Autoriser"),
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
    ("Nora|Stop", "Arrêter"),
    ("Stopped.", "Arrêté."),
    ("Nora did not answer in time.", "Nora n'a pas répondu à temps."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "L'onglet était en arrière-plan : Nora n'a pas pu agir dans le classeur.",
    ),
    ("Nora|Resume", "Reprendre"),
    ("Nora could not be reached. Check the connection and try again.", "Nora n'a pas pu être jointe. Vérifiez la connexion et réessayez."),
    ("Your session has expired. Sign in again.", "Votre session a expiré. Reconnectez-vous."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Le serveur n'a pas pu répondre (erreur {status}). Réessayez dans un instant.",
    ),
    ("Nora's answer could not be read.", "La réponse de Nora n'a pas pu être lue."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Ce classeur est protégé par mot de passe : GridCraft ne sait pas l'enregistrer chiffré, l'original reste intact dans le Drive.",
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
    ("Nora|Undo", "Rückgängig"),
    ("Undo the last change Nora made to the workbook", "Die letzte Änderung von Nora an der Arbeitsmappe rückgängig machen"),
    ("Ask Nora…", "Nora fragen…"),
    ("Send", "Senden"),
    ("Nora is working…", "Nora arbeitet…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Bitten Sie Nora, Ihre Arbeitsmappe auszufüllen, zu berechnen, zu sortieren oder zu formatieren.",
    ),
    ("Nora wants to:", "Nora möchte:"),
    ("Nora|Allow", "Zulassen"),
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
    ("Nora|Stop", "Stoppen"),
    ("Stopped.", "Gestoppt."),
    ("Nora did not answer in time.", "Nora hat nicht rechtzeitig geantwortet."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Dieser Tab war im Hintergrund: Nora konnte in der Arbeitsmappe nichts tun.",
    ),
    ("Nora|Resume", "Fortsetzen"),
    (
        "Nora could not be reached. Check the connection and try again.",
        "Nora war nicht erreichbar. Prüfen Sie die Verbindung und versuchen Sie es erneut.",
    ),
    ("Your session has expired. Sign in again.", "Ihre Sitzung ist abgelaufen. Bitte melden Sie sich erneut an."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Der Server konnte nicht antworten (Fehler {status}). Versuchen Sie es in einem Moment erneut.",
    ),
    ("Nora's answer could not be read.", "Die Antwort von Nora konnte nicht gelesen werden."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Diese Arbeitsmappe ist durch ein Passwort geschützt: GridCraft kann sie nicht verschlüsselt speichern, daher bleibt das Original unverändert im Drive.",
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
    ("Nora|Undo", "Deshacer"),
    ("Undo the last change Nora made to the workbook", "Deshacer el último cambio que Nora hizo en el libro"),
    ("Ask Nora…", "Preguntar a Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "Nora está trabajando…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Pida a Nora que rellene, calcule, ordene o dé formato a su libro."),
    ("Nora wants to:", "Nora quiere:"),
    ("Nora|Allow", "Permitir"),
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
    ("Nora|Stop", "Detener"),
    ("Stopped.", "Detenido."),
    ("Nora did not answer in time.", "Nora no respondió a tiempo."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Esta pestaña estaba en segundo plano: Nora no pudo actuar en el libro de cálculo.",
    ),
    ("Nora|Resume", "Reintentar"),
    ("Nora could not be reached. Check the connection and try again.", "No se pudo contactar con Nora. Compruebe la conexión y vuelva a intentarlo."),
    ("Your session has expired. Sign in again.", "Su sesión ha caducado. Inicie sesión de nuevo."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "El servidor no pudo responder (error {status}). Inténtelo de nuevo en unos instantes.",
    ),
    ("Nora's answer could not be read.", "No se pudo leer la respuesta de Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Este libro está protegido con contraseña: GridCraft no puede guardarlo cifrado, así que el original permanece intacto en el Drive.",
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
    ("Nora|Undo", "Desfazer"),
    ("Undo the last change Nora made to the workbook", "Desfazer a última alteração feita pela Nora na pasta de trabalho"),
    ("Ask Nora…", "Pergunte à Nora…"),
    ("Send", "Enviar"),
    ("Nora is working…", "A Nora está trabalhando…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Peça à Nora para preencher, calcular, ordenar ou formatar sua pasta de trabalho.",
    ),
    ("Nora wants to:", "A Nora quer:"),
    ("Nora|Allow", "Permitir"),
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
    ("Nora|Stop", "Parar"),
    ("Stopped.", "Interrompido."),
    ("Nora did not answer in time.", "A Nora não respondeu a tempo."),
    ("This tab was in the background: Nora could not act in the workbook.", "Esta aba estava em segundo plano: a Nora não pôde agir na planilha."),
    ("Nora|Resume", "Retomar"),
    ("Nora could not be reached. Check the connection and try again.", "Não foi possível conectar à Nora. Verifique a conexão e tente novamente."),
    ("Your session has expired. Sign in again.", "Sua sessão expirou. Entre novamente."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "O servidor não conseguiu responder (erro {status}). Tente novamente em instantes.",
    ),
    ("Nora's answer could not be read.", "Não foi possível ler a resposta da Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Esta pasta de trabalho está protegida por senha: o GridCraft não consegue salvá-la criptografada, então o original permanece intacto no Drive.",
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
    ("Nora|Undo", "Отменить"),
    ("Undo the last change Nora made to the workbook", "Отменить последнее изменение, внесённое Nora в книгу"),
    ("Ask Nora…", "Спросите Nora…"),
    ("Send", "Отправить"),
    ("Nora is working…", "Nora работает…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Попросите Nora заполнить, вычислить, отсортировать или отформатировать вашу книгу.",
    ),
    ("Nora wants to:", "Nora хочет:"),
    ("Nora|Allow", "Разрешить"),
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
    ("Nora|Stop", "Остановить"),
    ("Stopped.", "Остановлено."),
    ("Nora did not answer in time.", "Nora не ответила вовремя."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Эта вкладка была в фоновом режиме: Nora не могла выполнить действие в книге.",
    ),
    ("Nora|Resume", "Повторить"),
    ("Nora could not be reached. Check the connection and try again.", "Не удалось связаться с Nora. Проверьте подключение и повторите попытку."),
    ("Your session has expired. Sign in again.", "Ваш сеанс истёк. Войдите снова."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Сервер не смог ответить (ошибка {status}). Повторите попытку через некоторое время.",
    ),
    ("Nora's answer could not be read.", "Не удалось прочитать ответ Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Эта книга защищена паролем: GridCraft не может сохранить её в зашифрованном виде, поэтому оригинал в Drive остаётся без изменений.",
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
    ("Nora|Undo", "Zpět"),
    ("Undo the last change Nora made to the workbook", "Vrátit poslední změnu, kterou Nora provedla v sešitu"),
    ("Ask Nora…", "Zeptejte se Nory…"),
    ("Send", "Odeslat"),
    ("Nora is working…", "Nora pracuje…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Požádejte Noru, ať vyplní, spočítá, seřadí nebo naformátuje váš sešit."),
    ("Nora wants to:", "Nora chce:"),
    ("Nora|Allow", "Povolit"),
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
    ("Nora|Stop", "Zastavit"),
    ("Stopped.", "Zastaveno."),
    ("Nora did not answer in time.", "Nora neodpověděla včas."),
    ("This tab was in the background: Nora could not act in the workbook.", "Tato karta byla na pozadí: Nora nemohla v sešitu provést akci."),
    ("Nora|Resume", "Pokračovat"),
    ("Nora could not be reached. Check the connection and try again.", "Nora není dostupná. Zkontrolujte připojení a zkuste to znovu."),
    ("Your session has expired. Sign in again.", "Vaše relace vypršela. Přihlaste se znovu."),
    ("The server could not answer (error {status}). Try again in a moment.", "Server nemohl odpovědět (chyba {status}). Zkuste to za chvíli znovu."),
    ("Nora's answer could not be read.", "Odpověď Nory nebylo možné přečíst."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Tento sešit je chráněn heslem: GridCraft jej nedokáže uložit šifrovaně, takže originál na Drive zůstane nedotčen.",
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
    ("Nora|Undo", "元に戻す"),
    ("Undo the last change Nora made to the workbook", "Nora がブックに加えた最後の変更を元に戻す"),
    ("Ask Nora…", "Nora に質問…"),
    ("Send", "送信"),
    ("Nora is working…", "Nora が処理中…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Nora に、ブックへの入力、計算、並べ替え、書式設定を依頼してください。"),
    ("Nora wants to:", "Nora が次の操作を実行しようとしています："),
    ("Nora|Allow", "許可"),
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
    ("Nora|Stop", "停止"),
    ("Stopped.", "停止しました。"),
    ("Nora did not answer in time.", "Nora からの応答が時間内に届きませんでした。"),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "このタブはバックグラウンドにあったため、Nora はブック内で操作できませんでした。",
    ),
    ("Nora|Resume", "再開"),
    ("Nora could not be reached. Check the connection and try again.", "Nora に接続できませんでした。接続を確認して、もう一度お試しください。"),
    ("Your session has expired. Sign in again.", "セッションの有効期限が切れました。もう一度サインインしてください。"),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "サーバーから応答がありませんでした（エラー {status}）。しばらくしてから再度お試しください。",
    ),
    ("Nora's answer could not be read.", "Nora の回答を読み取れませんでした。"),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "このブックはパスワードで保護されています: GridCraft は暗号化した状態で保存できないため、元のファイルは Drive 上でそのまま残ります。",
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
    ("Nora|Undo", "撤销"),
    ("Undo the last change Nora made to the workbook", "撤销 Nora 对工作簿所做的最后一次更改"),
    ("Ask Nora…", "向 Nora 提问…"),
    ("Send", "发送"),
    ("Nora is working…", "Nora 正在处理…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "让 Nora 填充、计算、排序或设置工作簿的格式。"),
    ("Nora wants to:", "Nora 想要执行："),
    ("Nora|Allow", "允许"),
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
    ("Nora|Stop", "停止"),
    ("Stopped.", "已停止。"),
    ("Nora did not answer in time.", "Nora 未能及时回复。"),
    ("This tab was in the background: Nora could not act in the workbook.", "该标签页处于后台：Nora 无法在工作簿中执行操作。"),
    ("Nora|Resume", "恢复"),
    ("Nora could not be reached. Check the connection and try again.", "无法连接到 Nora。请检查网络连接后重试。"),
    ("Your session has expired. Sign in again.", "您的会话已过期，请重新登录。"),
    ("The server could not answer (error {status}). Try again in a moment.", "服务器无法响应（错误 {status}）。请稍后重试。"),
    ("Nora's answer could not be read.", "无法读取 Nora 的回复。"),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "此工作簿受密码保护：GridCraft 无法以加密方式保存，因此 Drive 中的原始文件保持不变。",
    ),
];

/// The same in Korean.
const KOREAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "{name}을(를) 저장할 수 없습니다: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "이 통합 문서는 편집기가 포함된 페이지에서 열려 있습니다."),
    ("Assistant", "어시스턴트"),
    ("Show the Nora assistant pane (experimental)", "Nora 어시스턴트 패널 표시 (실험적)"),
    ("Nora isn't available in this version yet", "Nora는 이 버전에서 아직 사용할 수 없습니다"),
    ("New conversation", "새 대화"),
    ("Nora|Undo", "실행 취소"),
    ("Undo the last change Nora made to the workbook", "Nora가 통합 문서에 적용한 마지막 변경을 실행 취소"),
    ("Ask Nora…", "Nora에게 질문…"),
    ("Send", "보내기"),
    ("Nora is working…", "Nora가 작업 중…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Nora에게 통합 문서 채우기, 계산, 정렬 또는 서식 지정을 요청하세요."),
    ("Nora wants to:", "Nora가 다음을 하려고 합니다:"),
    ("Nora|Allow", "허용"),
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
    ("Nora|Stop", "중지"),
    ("Stopped.", "중지되었습니다."),
    ("Nora did not answer in time.", "Nora가 제시간에 응답하지 않았습니다."),
    ("This tab was in the background: Nora could not act in the workbook.", "이 탭이 백그라운드에 있어서 Nora가 통합 문서에서 작업할 수 없었습니다."),
    ("Nora|Resume", "다시 시작"),
    ("Nora could not be reached. Check the connection and try again.", "Nora에 연결할 수 없습니다. 연결 상태를 확인하고 다시 시도해 주세요."),
    ("Your session has expired. Sign in again.", "세션이 만료되었습니다. 다시 로그인해 주세요."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "서버가 응답하지 못했습니다(오류 {status}). 잠시 후 다시 시도해 주세요.",
    ),
    ("Nora's answer could not be read.", "Nora의 응답을 읽을 수 없습니다."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "이 통합 문서는 암호로 보호되어 있습니다: GridCraft는 암호화된 상태로 저장할 수 없으므로 Drive의 원본은 그대로 유지됩니다.",
    ),
];

/// The same in Serbian (Latin script, ekavian, as upstream's `sr.rs`).
const SERBIAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Nije moguće sačuvati {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Ova radna sveska je otvorena sa stranice na kojoj se nalazi uređivač."),
    ("Assistant", "Asistent"),
    ("Show the Nora assistant pane (experimental)", "Prikaži okno asistenta Nora (eksperimentalno)"),
    ("Nora isn't available in this version yet", "Nora još nije dostupna u ovoj verziji"),
    ("New conversation", "Novi razgovor"),
    ("Nora|Undo", "Opozovi"),
    ("Undo the last change Nora made to the workbook", "Opozovi poslednju izmenu koju je Nora napravila u radnoj svesci"),
    ("Ask Nora…", "Pitaj Noru…"),
    ("Send", "Pošalji"),
    ("Nora is working…", "Nora radi…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Zamoli Noru da popuni, izračuna, sortira ili oblikuje radnu svesku."),
    ("Nora wants to:", "Nora želi da:"),
    ("Nora|Allow", "Dozvoli"),
    ("Refuse", "Odbij"),
    ("Read the workbook", "Pročitaj radnu svesku"),
    ("Run {command}", "Pokreni {command}"),
    ("Run {count} commands", "Pokreni {count} komandi"),
    ("Select \"{text}\"", "Izaberi \"{text}\""),
    ("Type \"{text}\"", "Unesi \"{text}\""),
    ("Look at page {page}", "Pogledaj stranicu {page}"),
    ("Save the workbook", "Sačuvaj radnu svesku"),
    ("Undo the last change", "Opozovi poslednju izmenu"),
    ("There is nothing of Nora's to undo.", "Nema nijedne Norine izmene za opozivanje."),
    ("Nora's change was made in another workbook.", "Norina izmena je napravljena u drugoj radnoj svesci."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Radna sveska se promenila posle Norine izmene: opozovi je korak po korak pomoću komande Opozovi.",
    ),
    ("Nora|Stop", "Zaustavi"),
    ("Stopped.", "Zaustavljeno."),
    ("Nora did not answer in time.", "Nora nije odgovorila na vreme."),
    ("This tab was in the background: Nora could not act in the workbook.", "Ovaj tab je bio u pozadini: Nora nije mogla da deluje u radnoj svesci."),
    ("Nora|Resume", "Nastavi"),
    ("Nora could not be reached. Check the connection and try again.", "Nije moguće stupiti u vezu sa Norom. Proveri vezu i pokušaj ponovo."),
    ("Your session has expired. Sign in again.", "Sesija je istekla. Prijavi se ponovo."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Server nije mogao da odgovori (greška {status}). Pokušaj ponovo za trenutak.",
    ),
    ("Nora's answer could not be read.", "Norin odgovor nije bilo moguće pročitati."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Ova radna sveska je zaštićena lozinkom: GridCraft je ne može sačuvati šifrovanu, pa original na Drive ostaje netaknut.",
    ),
];

/// The same in Greek.
const GREEK: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Αδυναμία αποθήκευσης του {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Αυτό το βιβλίο εργασίας ανοίγει από τη σελίδα που φιλοξενεί τον επεξεργαστή."),
    ("Assistant", "Βοηθός"),
    ("Show the Nora assistant pane (experimental)", "Εμφάνιση του πλαισίου βοηθού Nora (πειραματικό)"),
    ("Nora isn't available in this version yet", "Η Nora δεν είναι ακόμη διαθέσιμη σε αυτή την έκδοση"),
    ("New conversation", "Νέα συνομιλία"),
    ("Nora|Undo", "Αναίρεση"),
    ("Undo the last change Nora made to the workbook", "Αναίρεση της τελευταίας αλλαγής που έκανε η Nora στο βιβλίο εργασίας"),
    ("Ask Nora…", "Ρωτήστε τη Nora…"),
    ("Send", "Αποστολή"),
    ("Nora is working…", "Η Nora εργάζεται…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Ζητήστε από τη Nora να συμπληρώσει, να υπολογίσει, να ταξινομήσει ή να μορφοποιήσει το βιβλίο εργασίας σας.",
    ),
    ("Nora wants to:", "Η Nora θέλει να:"),
    ("Nora|Allow", "Αποδοχή"),
    ("Refuse", "Άρνηση"),
    ("Read the workbook", "Ανάγνωση του βιβλίου εργασίας"),
    ("Run {command}", "Εκτέλεση {command}"),
    ("Run {count} commands", "Εκτέλεση {count} εντολών"),
    ("Select \"{text}\"", "Επιλογή \"{text}\""),
    ("Type \"{text}\"", "Πληκτρολόγηση \"{text}\""),
    ("Look at page {page}", "Προβολή της σελίδας {page}"),
    ("Save the workbook", "Αποθήκευση του βιβλίου εργασίας"),
    ("Undo the last change", "Αναίρεση της τελευταίας αλλαγής"),
    ("There is nothing of Nora's to undo.", "Δεν υπάρχει αλλαγή της Nora για αναίρεση."),
    ("Nora's change was made in another workbook.", "Η αλλαγή της Nora έγινε σε άλλο βιβλίο εργασίας."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Το βιβλίο εργασίας έχει αλλάξει από την αλλαγή της Nora: αναιρέστε την βήμα βήμα με την εντολή Αναίρεση.",
    ),
    ("Nora|Stop", "Διακοπή"),
    ("Stopped.", "Διακόπηκε."),
    ("Nora did not answer in time.", "Η Nora δεν απάντησε εγκαίρως."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Αυτή η καρτέλα ήταν στο παρασκήνιο: η Nora δεν μπόρεσε να ενεργήσει στο βιβλίο εργασίας.",
    ),
    ("Nora|Resume", "Επανάληψη"),
    (
        "Nora could not be reached. Check the connection and try again.",
        "Δεν ήταν δυνατή η σύνδεση με τη Nora. Ελέγξτε τη σύνδεση και δοκιμάστε ξανά.",
    ),
    ("Your session has expired. Sign in again.", "Η συνεδρία σας έληξε. Συνδεθείτε ξανά."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Ο διακομιστής δεν μπόρεσε να απαντήσει (σφάλμα {status}). Δοκιμάστε ξανά σε λίγο.",
    ),
    ("Nora's answer could not be read.", "Δεν ήταν δυνατή η ανάγνωση της απάντησης της Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Αυτό το βιβλίο εργασίας προστατεύεται με κωδικό πρόσβασης: το GridCraft δεν μπορεί να το αποθηκεύσει κρυπτογραφημένο, οπότε το πρωτότυπο παραμένει αμετάβλητο στο Drive.",
    ),
];

/// The same in Turkish.
const TURKISH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "{name} kaydedilemedi: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Bu çalışma kitabı, düzenleyiciyi barındıran sayfadan açılıyor."),
    ("Assistant", "Asistan"),
    ("Show the Nora assistant pane (experimental)", "Nora asistan panelini göster (deneysel)"),
    ("Nora isn't available in this version yet", "Nora bu sürümde henüz kullanılamıyor"),
    ("New conversation", "Yeni görüşme"),
    ("Nora|Undo", "Geri al"),
    ("Undo the last change Nora made to the workbook", "Nora'nın çalışma kitabında yaptığı son değişikliği geri al"),
    ("Ask Nora…", "Nora'ya sorun…"),
    ("Send", "Gönder"),
    ("Nora is working…", "Nora çalışıyor…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Nora'dan çalışma kitabınızı doldurmasını, hesaplamasını, sıralamasını veya biçimlendirmesini isteyin.",
    ),
    ("Nora wants to:", "Nora şunu yapmak istiyor:"),
    ("Nora|Allow", "İzin ver"),
    ("Refuse", "Reddet"),
    ("Read the workbook", "Çalışma kitabını oku"),
    ("Run {command}", "{command} çalıştır"),
    ("Run {count} commands", "{count} komut çalıştır"),
    ("Select \"{text}\"", "\"{text}\" seç"),
    ("Type \"{text}\"", "\"{text}\" yaz"),
    ("Look at page {page}", "{page} sayfasına bak"),
    ("Save the workbook", "Çalışma kitabını kaydet"),
    ("Undo the last change", "Son değişikliği geri al"),
    ("There is nothing of Nora's to undo.", "Geri alınacak Nora değişikliği yok."),
    ("Nora's change was made in another workbook.", "Nora'nın değişikliği başka bir çalışma kitabında yapıldı."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Nora'nın değişikliğinden sonra çalışma kitabı değişti: Geri al komutuyla adım adım geri alın.",
    ),
    ("Nora|Stop", "Durdur"),
    ("Stopped.", "Durduruldu."),
    ("Nora did not answer in time.", "Nora zamanında yanıt vermedi."),
    ("This tab was in the background: Nora could not act in the workbook.", "Bu sekme arka plandaydı: Nora çalışma kitabında işlem yapamadı."),
    ("Nora|Resume", "Devam et"),
    ("Nora could not be reached. Check the connection and try again.", "Nora'ya ulaşılamadı. Bağlantıyı kontrol edip yeniden deneyin."),
    ("Your session has expired. Sign in again.", "Oturumunuzun süresi doldu. Yeniden oturum açın."),
    ("The server could not answer (error {status}). Try again in a moment.", "Sunucu yanıt veremedi (hata {status}). Birazdan yeniden deneyin."),
    ("Nora's answer could not be read.", "Nora'nın yanıtı okunamadı."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Bu çalışma kitabı bir parolayla korunuyor: GridCraft onu şifreli olarak kaydedemez, bu yüzden özgün dosya Drive'da olduğu gibi kalır.",
    ),
];

/// The same in Polish.
const POLISH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Nie udało się zapisać {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Ten skoroszyt jest otwierany ze strony, na której znajduje się edytor."),
    ("Assistant", "Asystent"),
    ("Show the Nora assistant pane (experimental)", "Pokaż panel asystenta Nora (eksperymentalne)"),
    ("Nora isn't available in this version yet", "Nora nie jest jeszcze dostępna w tej wersji"),
    ("New conversation", "Nowa rozmowa"),
    ("Nora|Undo", "Cofnij"),
    ("Undo the last change Nora made to the workbook", "Cofnij ostatnią zmianę, jaką Nora wprowadziła w skoroszycie"),
    ("Ask Nora…", "Zapytaj Norę…"),
    ("Send", "Wyślij"),
    ("Nora is working…", "Nora pracuje…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Poproś Norę, aby wypełniła, obliczyła, posortowała lub sformatowała Twój skoroszyt.",
    ),
    ("Nora wants to:", "Nora chce:"),
    ("Nora|Allow", "Zezwól"),
    ("Refuse", "Odrzuć"),
    ("Read the workbook", "Odczytaj skoroszyt"),
    ("Run {command}", "Uruchom {command}"),
    ("Run {count} commands", "Uruchom {count} poleceń"),
    ("Select \"{text}\"", "Zaznacz \"{text}\""),
    ("Type \"{text}\"", "Wpisz \"{text}\""),
    ("Look at page {page}", "Zobacz stronę {page}"),
    ("Save the workbook", "Zapisz skoroszyt"),
    ("Undo the last change", "Cofnij ostatnią zmianę"),
    ("There is nothing of Nora's to undo.", "Nie ma żadnej zmiany Nory do cofnięcia."),
    ("Nora's change was made in another workbook.", "Zmiana Nory została wprowadzona w innym skoroszycie."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Skoroszyt zmienił się od zmiany Nory: cofnij ją krok po kroku poleceniem Cofnij.",
    ),
    ("Nora|Stop", "Zatrzymaj"),
    ("Stopped.", "Zatrzymano."),
    ("Nora did not answer in time.", "Nora nie odpowiedziała na czas."),
    ("This tab was in the background: Nora could not act in the workbook.", "Ta karta była w tle: Nora nie mogła wykonać działania w skoroszycie."),
    ("Nora|Resume", "Wznów"),
    ("Nora could not be reached. Check the connection and try again.", "Nie udało się połączyć z Norą. Sprawdź połączenie i spróbuj ponownie."),
    ("Your session has expired. Sign in again.", "Twoja sesja wygasła. Zaloguj się ponownie."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Serwer nie mógł odpowiedzieć (błąd {status}). Spróbuj ponownie za chwilę.",
    ),
    ("Nora's answer could not be read.", "Nie udało się odczytać odpowiedzi Nory."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Ten skoroszyt jest chroniony hasłem: GridCraft nie może zapisać go zaszyfrowanego, więc oryginał pozostaje nietknięty w Drive.",
    ),
];

/// The same in Italian.
const ITALIAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Impossibile salvare {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Questa cartella di lavoro viene aperta dalla pagina che ospita l'editor."),
    ("Assistant", "Assistente"),
    ("Show the Nora assistant pane (experimental)", "Mostra il pannello dell'assistente Nora (sperimentale)"),
    ("Nora isn't available in this version yet", "Nora non è ancora disponibile in questa versione"),
    ("New conversation", "Nuova conversazione"),
    ("Nora|Undo", "Annulla"),
    ("Undo the last change Nora made to the workbook", "Annulla l'ultima modifica apportata da Nora alla cartella di lavoro"),
    ("Ask Nora…", "Chiedi a Nora…"),
    ("Send", "Invia"),
    ("Nora is working…", "Nora sta lavorando…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Chiedi a Nora di compilare, calcolare, ordinare o formattare la tua cartella di lavoro.",
    ),
    ("Nora wants to:", "Nora vuole:"),
    ("Nora|Allow", "Consenti"),
    ("Refuse", "Rifiuta"),
    ("Read the workbook", "Leggi la cartella di lavoro"),
    ("Run {command}", "Esegui {command}"),
    ("Run {count} commands", "Esegui {count} comandi"),
    ("Select \"{text}\"", "Seleziona \"{text}\""),
    ("Type \"{text}\"", "Digita \"{text}\""),
    ("Look at page {page}", "Consulta la pagina {page}"),
    ("Save the workbook", "Salva la cartella di lavoro"),
    ("Undo the last change", "Annulla l'ultima modifica"),
    ("There is nothing of Nora's to undo.", "Non c'è nessuna modifica di Nora da annullare."),
    ("Nora's change was made in another workbook.", "La modifica di Nora è stata fatta in un'altra cartella di lavoro."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "La cartella di lavoro è cambiata dopo la modifica di Nora: annullala passo passo con Annulla.",
    ),
    ("Nora|Stop", "Interrompi"),
    ("Stopped.", "Interrotto."),
    ("Nora did not answer in time.", "Nora non ha risposto in tempo."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Questa scheda era in background: Nora non ha potuto agire nella cartella di lavoro.",
    ),
    ("Nora|Resume", "Riprendi"),
    ("Nora could not be reached. Check the connection and try again.", "Impossibile raggiungere Nora. Controlla la connessione e riprova."),
    ("Your session has expired. Sign in again.", "La sessione è scaduta. Accedi di nuovo."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Il server non ha potuto rispondere (errore {status}). Riprova tra poco.",
    ),
    ("Nora's answer could not be read.", "Impossibile leggere la risposta di Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Questa cartella di lavoro è protetta da password: GridCraft non può salvarla cifrata, quindi l'originale resta intatto nel Drive.",
    ),
];

/// The same in Ukrainian.
const UKRAINIAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Не вдалося зберегти {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Цю книгу відкрито зі сторінки, на якій розміщено редактор."),
    ("Assistant", "Помічник"),
    ("Show the Nora assistant pane (experimental)", "Показати панель помічника Nora (експериментально)"),
    ("Nora isn't available in this version yet", "Nora ще недоступна в цій версії"),
    ("New conversation", "Нова розмова"),
    ("Nora|Undo", "Скасувати"),
    ("Undo the last change Nora made to the workbook", "Скасувати останню зміну, внесену Nora до книги"),
    ("Ask Nora…", "Запитайте Nora…"),
    ("Send", "Надіслати"),
    ("Nora is working…", "Nora працює…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Попросіть Nora заповнити, обчислити, відсортувати або форматувати вашу книгу.",
    ),
    ("Nora wants to:", "Nora хоче:"),
    ("Nora|Allow", "Дозволити"),
    ("Refuse", "Відмовити"),
    ("Read the workbook", "Прочитати книгу"),
    ("Run {command}", "Виконати {command}"),
    ("Run {count} commands", "Виконати {count} команд"),
    ("Select \"{text}\"", "Вибрати \"{text}\""),
    ("Type \"{text}\"", "Ввести \"{text}\""),
    ("Look at page {page}", "Переглянути сторінку {page}"),
    ("Save the workbook", "Зберегти книгу"),
    ("Undo the last change", "Скасувати останню зміну"),
    ("There is nothing of Nora's to undo.", "Немає змін Nora, які можна скасувати."),
    ("Nora's change was made in another workbook.", "Зміну Nora внесено в іншу книгу."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Книгу змінено після зміни Nora: скасуйте її крок за кроком за допомогою команди «Скасувати».",
    ),
    ("Nora|Stop", "Зупинити"),
    ("Stopped.", "Зупинено."),
    ("Nora did not answer in time.", "Nora не відповіла вчасно."),
    ("This tab was in the background: Nora could not act in the workbook.", "Ця вкладка була у фоновому режимі: Nora не могла виконати дію в книзі."),
    ("Nora|Resume", "Повторити"),
    ("Nora could not be reached. Check the connection and try again.", "Не вдалося зв'язатися з Nora. Перевірте з'єднання та спробуйте ще раз."),
    ("Your session has expired. Sign in again.", "Ваш сеанс завершився. Увійдіть знову."),
    ("The server could not answer (error {status}). Try again in a moment.", "Сервер не зміг відповісти (помилка {status}). Спробуйте за хвилину."),
    ("Nora's answer could not be read.", "Не вдалося прочитати відповідь Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Цю книгу захищено паролем: GridCraft не може зберегти її зашифрованою, тож оригінал залишається без змін у Drive.",
    ),
];

/// The same in Vietnamese.
const VIETNAMESE: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Không thể lưu {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Sổ làm việc này được mở từ trang chứa trình soạn thảo."),
    ("Assistant", "Trợ lý"),
    ("Show the Nora assistant pane (experimental)", "Hiển thị khung trợ lý Nora (thử nghiệm)"),
    ("Nora isn't available in this version yet", "Nora chưa khả dụng trong phiên bản này"),
    ("New conversation", "Cuộc trò chuyện mới"),
    ("Nora|Undo", "Hoàn tác"),
    ("Undo the last change Nora made to the workbook", "Hoàn tác thay đổi gần nhất mà Nora đã thực hiện trên sổ làm việc"),
    ("Ask Nora…", "Hỏi Nora…"),
    ("Send", "Gửi"),
    ("Nora is working…", "Nora đang làm việc…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Yêu cầu Nora điền, tính toán, sắp xếp hoặc định dạng sổ làm việc của bạn."),
    ("Nora wants to:", "Nora muốn:"),
    ("Nora|Allow", "Cho phép"),
    ("Refuse", "Từ chối"),
    ("Read the workbook", "Đọc sổ làm việc"),
    ("Run {command}", "Chạy {command}"),
    ("Run {count} commands", "Chạy {count} lệnh"),
    ("Select \"{text}\"", "Chọn \"{text}\""),
    ("Type \"{text}\"", "Nhập \"{text}\""),
    ("Look at page {page}", "Xem trang {page}"),
    ("Save the workbook", "Lưu sổ làm việc"),
    ("Undo the last change", "Hoàn tác thay đổi gần nhất"),
    ("There is nothing of Nora's to undo.", "Không có thay đổi nào của Nora để hoàn tác."),
    ("Nora's change was made in another workbook.", "Thay đổi của Nora được thực hiện trong một sổ làm việc khác."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Sổ làm việc đã thay đổi kể từ thay đổi của Nora: hãy hoàn tác từng bước bằng lệnh Hoàn tác.",
    ),
    ("Nora|Stop", "Dừng"),
    ("Stopped.", "Đã dừng."),
    ("Nora did not answer in time.", "Nora không trả lời kịp thời."),
    ("This tab was in the background: Nora could not act in the workbook.", "Thẻ này đang ở chế độ nền: Nora không thể thao tác trong sổ tính."),
    ("Nora|Resume", "Tiếp tục"),
    ("Nora could not be reached. Check the connection and try again.", "Không thể kết nối với Nora. Hãy kiểm tra kết nối và thử lại."),
    ("Your session has expired. Sign in again.", "Phiên của bạn đã hết hạn. Hãy đăng nhập lại."),
    ("The server could not answer (error {status}). Try again in a moment.", "Máy chủ không thể phản hồi (lỗi {status}). Hãy thử lại sau ít phút."),
    ("Nora's answer could not be read.", "Không thể đọc câu trả lời của Nora."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Sổ làm việc này được bảo vệ bằng mật khẩu: GridCraft không thể lưu ở dạng mã hóa, nên bản gốc vẫn còn nguyên trên Drive.",
    ),
];

/// The same in Dutch.
const DUTCH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Kon {name} niet opslaan: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Deze werkmap wordt geopend vanaf de pagina waarop de editor staat."),
    ("Assistant", "Assistent"),
    ("Show the Nora assistant pane (experimental)", "Het assistentvenster van Nora tonen (experimenteel)"),
    ("Nora isn't available in this version yet", "Nora is nog niet beschikbaar in deze versie"),
    ("New conversation", "Nieuw gesprek"),
    ("Nora|Undo", "Ongedaan maken"),
    ("Undo the last change Nora made to the workbook", "De laatste wijziging die Nora in de werkmap heeft gemaakt ongedaan maken"),
    ("Ask Nora…", "Vraag Nora…"),
    ("Send", "Verzenden"),
    ("Nora is working…", "Nora is aan het werk…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Vraag Nora om je werkmap in te vullen, te berekenen, te sorteren of op te maken.",
    ),
    ("Nora wants to:", "Nora wil:"),
    ("Nora|Allow", "Toestaan"),
    ("Refuse", "Weigeren"),
    ("Read the workbook", "De werkmap lezen"),
    ("Run {command}", "{command} uitvoeren"),
    ("Run {count} commands", "{count} opdrachten uitvoeren"),
    ("Select \"{text}\"", "\"{text}\" selecteren"),
    ("Type \"{text}\"", "\"{text}\" typen"),
    ("Look at page {page}", "Pagina {page} bekijken"),
    ("Save the workbook", "De werkmap opslaan"),
    ("Undo the last change", "De laatste wijziging ongedaan maken"),
    ("There is nothing of Nora's to undo.", "Er is geen wijziging van Nora om ongedaan te maken."),
    ("Nora's change was made in another workbook.", "De wijziging van Nora is in een andere werkmap gemaakt."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "De werkmap is veranderd sinds de wijziging van Nora: maak die stap voor stap ongedaan met Ongedaan maken.",
    ),
    ("Nora|Stop", "Stoppen"),
    ("Stopped.", "Gestopt."),
    ("Nora did not answer in time.", "Nora heeft niet op tijd geantwoord."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Dit tabblad stond op de achtergrond: Nora kon niet handelen in de werkmap.",
    ),
    ("Nora|Resume", "Hervatten"),
    (
        "Nora could not be reached. Check the connection and try again.",
        "Nora kon niet worden bereikt. Controleer de verbinding en probeer het opnieuw.",
    ),
    ("Your session has expired. Sign in again.", "Je sessie is verlopen. Meld je opnieuw aan."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "De server kon niet antwoorden (fout {status}). Probeer het zo meteen opnieuw.",
    ),
    ("Nora's answer could not be read.", "Het antwoord van Nora kon niet worden gelezen."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Deze werkmap is beveiligd met een wachtwoord: GridCraft kan hem niet versleuteld opslaan, dus het origineel blijft ongewijzigd op Drive.",
    ),
];

/// The same in Romanian.
const ROMANIAN: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Nu s-a putut salva {name}: {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Acest registru este deschis din pagina care găzduiește editorul."),
    ("Assistant", "Asistent"),
    ("Show the Nora assistant pane (experimental)", "Afișează panoul asistentului Nora (experimental)"),
    ("Nora isn't available in this version yet", "Nora nu este încă disponibilă în această versiune"),
    ("New conversation", "Conversație nouă"),
    ("Nora|Undo", "Anulare acțiune"),
    ("Undo the last change Nora made to the workbook", "Anulați ultima modificare făcută de Nora în registru"),
    ("Ask Nora…", "Întrebați-o pe Nora…"),
    ("Send", "Trimitere"),
    ("Nora is working…", "Nora lucrează…"),
    (
        "Ask Nora to fill in, calculate, sort or format your workbook.",
        "Cereți Norei să completeze, să calculeze, să sorteze sau să formateze registrul dumneavoastră.",
    ),
    ("Nora wants to:", "Nora dorește să:"),
    ("Nora|Allow", "Permitere"),
    ("Refuse", "Refuzare"),
    ("Read the workbook", "Citirea registrului"),
    ("Run {command}", "Executare {command}"),
    ("Run {count} commands", "Executare {count} comenzi"),
    ("Select \"{text}\"", "Selectare \"{text}\""),
    ("Type \"{text}\"", "Tastare \"{text}\""),
    ("Look at page {page}", "Consultare pagina {page}"),
    ("Save the workbook", "Salvare registru"),
    ("Undo the last change", "Anulare ultimă modificare"),
    ("There is nothing of Nora's to undo.", "Nu există nicio modificare a Norei de anulat."),
    ("Nora's change was made in another workbook.", "Modificarea Norei a fost făcută într-un alt registru."),
    (
        "The workbook has changed since Nora's change: undo it step by step with Undo.",
        "Registrul s-a schimbat de la modificarea Norei: anulați-o pas cu pas cu Anulare acțiune.",
    ),
    ("Nora|Stop", "Oprire"),
    ("Stopped.", "Oprit."),
    ("Nora did not answer in time.", "Nora nu a răspuns la timp."),
    (
        "This tab was in the background: Nora could not act in the workbook.",
        "Această filă era în fundal: Nora nu a putut acționa în registrul de calcul.",
    ),
    ("Nora|Resume", "Reluare"),
    ("Nora could not be reached. Check the connection and try again.", "Nu s-a putut contacta Nora. Verificați conexiunea și încercați din nou."),
    ("Your session has expired. Sign in again.", "Sesiunea dumneavoastră a expirat. Conectați-vă din nou."),
    (
        "The server could not answer (error {status}). Try again in a moment.",
        "Serverul nu a putut răspunde (eroare {status}). Încercați din nou peste puțin timp.",
    ),
    ("Nora's answer could not be read.", "Răspunsul Norei nu a putut fi citit."),
    (
        "This workbook is protected by a password: GridCraft can't save it encrypted, so the original stays untouched in the Drive.",
        "Acest registru este protejat prin parolă: GridCraft nu îl poate salva criptat, așa că originalul rămâne neatins în Drive.",
    ),
];

// The status bar (tabs.rs `status_bar`): upstream writes its Sum, Count and Average, its macro and circular-reference
// notices and its zoom buttons' tooltips in English. Neoffice has them in French, German, Italian and Spanish only, the
// languages its customers read; every other language shows them as upstream does. A language's own catalog still wins
// (Italian has `Zoom In` and `Zoom Out`). Each table has the same English strings, in the same order.

/// The status bar in French.
const STATUS_BAR_FRENCH: &[(&str, &str)] = &[
    ("Sum: {value}", "Somme : {value}"),
    ("Count: {value}", "Nb (non vides) : {value}"),
    ("Average: {value}", "Moyenne : {value}"),
    ("Macros preserved (not executed)", "Macros conservées (non exécutées)"),
    ("Circular References: {cell}", "Références circulaires : {cell}"),
    ("Zoom In", "Zoom avant"),
    ("Zoom Out", "Zoom arrière"),
];

/// The status bar in German.
const STATUS_BAR_GERMAN: &[(&str, &str)] = &[
    ("Sum: {value}", "Summe: {value}"),
    ("Count: {value}", "Anzahl: {value}"),
    ("Average: {value}", "Mittelwert: {value}"),
    ("Macros preserved (not executed)", "Makros erhalten (nicht ausgeführt)"),
    ("Circular References: {cell}", "Zirkelbezug: {cell}"),
    ("Zoom In", "Vergrößern"),
    ("Zoom Out", "Verkleinern"),
];

/// The status bar in Italian.
const STATUS_BAR_ITALIAN: &[(&str, &str)] = &[
    ("Sum: {value}", "Somma: {value}"),
    ("Count: {value}", "Conteggio: {value}"),
    ("Average: {value}", "Media: {value}"),
    ("Macros preserved (not executed)", "Macro conservate (non eseguite)"),
    ("Circular References: {cell}", "Riferimenti circolari: {cell}"),
    ("Zoom In", "Ingrandisci"),
    ("Zoom Out", "Riduci"),
];

/// The status bar in Spanish.
const STATUS_BAR_SPANISH: &[(&str, &str)] = &[
    ("Sum: {value}", "Suma: {value}"),
    ("Count: {value}", "Recuento: {value}"),
    ("Average: {value}", "Promedio: {value}"),
    ("Macros preserved (not executed)", "Macros conservadas (no ejecutadas)"),
    ("Circular References: {cell}", "Referencias circulares: {cell}"),
    ("Zoom In", "Acercar"),
    ("Zoom Out", "Alejar"),
];

/// `text` in `language`, when Neoffice has translated it. English has nothing to translate.
pub(super) fn added(language: Language, text: &str) -> Option<&'static str> {
    table(language).and_then(|table| find(table, text)).or_else(|| status_bar(language).and_then(|table| find(table, text)))
}

/// The status bar's words for a language that has them (see `STATUS_BAR_FRENCH`).
fn status_bar(language: Language) -> Option<&'static [(&'static str, &'static str)]> {
    match language {
        Language::Fr => Some(STATUS_BAR_FRENCH),
        Language::De => Some(STATUS_BAR_GERMAN),
        Language::It => Some(STATUS_BAR_ITALIAN),
        Language::Es => Some(STATUS_BAR_SPANISH),
        _ => None,
    }
}

/// Neoffice's table for a language other than English. The match is exhaustive: a language upstream adds
/// does not compile until it has its table.
fn table(language: Language) -> Option<&'static [(&'static str, &'static str)]> {
    match language {
        Language::En => None,
        Language::Fr => Some(FRENCH),
        Language::De => Some(GERMAN),
        Language::Es => Some(SPANISH),
        Language::PtBr => Some(PORTUGUESE),
        Language::Ru => Some(RUSSIAN),
        Language::Cs => Some(CZECH),
        Language::Ja => Some(JAPANESE),
        Language::Zh => Some(CHINESE),
        Language::Ko => Some(KOREAN),
        Language::Sr => Some(SERBIAN),
        Language::El => Some(GREEK),
        Language::Tr => Some(TURKISH),
        Language::Pl => Some(POLISH),
        Language::It => Some(ITALIAN),
        Language::Uk => Some(UKRAINIAN),
        Language::Vi => Some(VIETNAMESE),
        Language::Nl => Some(DUTCH),
        Language::Ro => Some(ROMANIAN),
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

    /// Every table of ours, in the order of `Language::ALL`.
    fn tables() -> Vec<(Language, &'static [(&'static str, &'static str)])> {
        Language::ALL.into_iter().filter_map(|language| table(language).map(|table| (language, table))).collect()
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

    /// The four status bar tables: the same strings in the same order, placeholders kept, no English left, and each shown
    /// in its language unless the language's own catalog has the string (it wins, as for every string of ours).
    #[test]
    fn the_status_bar_says_its_words_in_french_german_italian_and_spanish() {
        let english: Vec<&str> = STATUS_BAR_FRENCH.iter().map(|(english, _)| *english).collect();
        let catalogs: [(Language, fn(&str) -> Option<&'static str>); 4] = [
            (Language::Fr, super::super::fr::translate),
            (Language::De, super::super::de::translate),
            (Language::It, super::super::it::translate),
            (Language::Es, super::super::es::translate),
        ];
        for (language, catalog) in catalogs {
            let table = status_bar(language).unwrap_or_default();
            assert_eq!(table.iter().map(|(english, _)| *english).collect::<Vec<_>>(), english, "{language:?}");
            for (english, translated) in table {
                assert_eq!(placeholders(english), placeholders(translated), "{language:?}: {english:?}");
                assert_eq!(translated.trim(), *translated, "{language:?}: stray whitespace in {translated:?}");
                assert_ne!(translated, english, "{language:?}: {english:?} is not translated");
                assert_eq!(language.tr(english), catalog(english).unwrap_or(*translated), "{language:?}: {english:?}");
            }
        }
        assert_eq!(Language::Fr.tr("Sum: {value}"), "Somme : {value}");
        assert_eq!(Language::En.tr("Sum: {value}"), "Sum: {value}");
        assert_eq!(status_bar(Language::Ja), None, "other languages show upstream's English");
        assert_eq!(Language::Ja.tr("Count: {value}"), "Count: {value}");
    }

    #[test]
    fn the_english_strings_are_listed_once() {
        let mut seen = std::collections::HashSet::new();
        for (english, _) in FRENCH {
            assert!(seen.insert(*english), "{english:?} is listed twice");
        }
    }

    /// Upstream fixed the French of v0.6.1 in c59e942 (in v0.7.0): the entries our table of fixes replaced until the
    /// v0.10.0 merge read right in its catalog now.
    #[test]
    fn upstream_s_french_has_the_values_our_fixes_gave() {
        assert_eq!(Language::Fr.tr("Hide Detail"), "Masquer le détail");
        assert_eq!(Language::Fr.tr("Previous Comment"), "Commentaire précédent");
        assert_eq!(Language::Fr.tr("Insert Copied Cells"), "Insérer les cellules copiées");
    }

    /// A `context|label` key shows its label alone, translated or not. Our table of fixes of v0.6.1's French
    /// (dropped at the v0.10.0 merge) wrote three with their context, and File › Options read "Options|Général".
    #[test]
    fn a_key_s_context_never_shows() {
        assert_eq!(Language::Fr.tr("Options|General"), "Général");
        assert_eq!(Language::Fr.tr("Options|Save"), "Enregistrer");
        assert_eq!(Language::Fr.tr("Sheet Options|View"), "Affichage");
        // Upstream's Japanese catalog is the reference every language translates (`ja::CORE` and `ja::DIALOGS` since v0.11).
        let keys = super::super::ja::CORE.iter().chain(super::super::ja::DIALOGS).chain(FRENCH).map(|(english, _)| *english);
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
        assert_eq!(added(Language::Fr, "A string upstream has"), None);
        assert_eq!(added(Language::Ja, "A string upstream has"), None);
        // A string of upstream's catalogs is theirs: "Close" is not in our tables.
        assert_eq!(added(Language::De, "Close"), None);
        assert_eq!(Language::De.tr("Close"), "Schließen");
    }
}
