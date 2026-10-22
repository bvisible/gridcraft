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
    ("Stop", "Arrêter"),
    ("Stopped.", "Arrêté."),
    ("Nora did not answer in time.", "Nora n'a pas répondu à temps."),
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
    ("Stop", "Stoppen"),
    ("Stopped.", "Gestoppt."),
    ("Nora did not answer in time.", "Nora hat nicht rechtzeitig geantwortet."),
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
    ("Stop", "Detener"),
    ("Stopped.", "Detenido."),
    ("Nora did not answer in time.", "Nora no respondió a tiempo."),
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
    ("Stop", "Parar"),
    ("Stopped.", "Interrompido."),
    ("Nora did not answer in time.", "A Nora não respondeu a tempo."),
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
    ("Stop", "Остановить"),
    ("Stopped.", "Остановлено."),
    ("Nora did not answer in time.", "Nora не ответила вовремя."),
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
    ("Stop", "Zastavit"),
    ("Stopped.", "Zastaveno."),
    ("Nora did not answer in time.", "Nora neodpověděla včas."),
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
    ("Stop", "停止"),
    ("Stopped.", "停止しました。"),
    ("Nora did not answer in time.", "Nora からの応答が時間内に届きませんでした。"),
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
    ("Stop", "停止"),
    ("Stopped.", "已停止。"),
    ("Nora did not answer in time.", "Nora 未能及时回复。"),
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
    ("Stop", "중지"),
    ("Stopped.", "중지되었습니다."),
    ("Nora did not answer in time.", "Nora가 제시간에 응답하지 않았습니다."),
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
    ("Undo", "Opozovi"),
    ("Undo the last change Nora made to the workbook", "Opozovi poslednju izmenu koju je Nora napravila u radnoj svesci"),
    ("Ask Nora…", "Pitaj Noru…"),
    ("Send", "Pošalji"),
    ("Nora is working…", "Nora radi…"),
    ("Ask Nora to fill in, calculate, sort or format your workbook.", "Zamoli Noru da popuni, izračuna, sortira ili oblikuje radnu svesku."),
    ("Nora wants to:", "Nora želi da:"),
    ("Allow", "Dozvoli"),
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
    ("Stop", "Zaustavi"),
    ("Stopped.", "Zaustavljeno."),
    ("Nora did not answer in time.", "Nora nije odgovorila na vreme."),
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

/// `text` in `language`, when Neoffice has translated it. English has nothing to translate.
pub(super) fn added(language: Language, text: &str) -> Option<&'static str> {
    table(language).and_then(|table| find(table, text))
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
        let keys = super::super::JAPANESE.iter().chain(FRENCH).map(|(english, _)| *english);
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
