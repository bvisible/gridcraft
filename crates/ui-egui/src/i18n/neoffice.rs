// //// Neoffice — added file (no upstream equivalent): Neoffice's own interface strings, kept out of upstream's catalogs.

//! The strings Neoffice added to GridCraft (the message the hosting page's bridge shows, see
//! `apps/gridcraft-web/src/host.rs`), kept outside upstream's catalogs. [`super::Language::tr`] asks
//! here when the language's own catalog has no entry for a string, so an upstream merge never
//! touches these, and a catalog that gains the same English string wins over this table.
//!
//! French is the first language here: Neoffice's customers read it. The other languages show these
//! strings in English until someone adds them.

/// Neoffice's strings in French, English first.
const FRENCH: &[(&str, &str)] = &[
    ("Couldn't save {name}: {error}", "Impossible d'enregistrer {name} : {error}"),
    ("This workbook is opened from the page that hosts the editor.", "Ce classeur est ouvert depuis la page qui héberge l'éditeur."),
];

/// Corrections of upstream's French catalog as it stands in v0.6.1: about 50 of its entries read as the
/// French of ANOTHER English string ("File" is "Nom du fichier", "Hide Detail" is "Insérer les cellules
/// copiées"), the rest are wrappings and wordings. Upstream fixed them in storytold/gridcraft `c59e942`
/// ("correct scattered wrong French translations"); these are its values, asked BEFORE the catalog.
/// Drop this table at the merge that brings that commit.
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
    FRENCH.iter().find(|(english, _)| *english == text).map(|(_, french)| *french)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    /// The `{placeholders}` of a string, in order.
    fn placeholders(text: &str) -> Vec<&str> {
        text.split('{').skip(1).filter_map(|rest| rest.split_once('}').map(|(name, _)| name)).collect()
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

    #[test]
    fn only_french_has_them_for_now() {
        let refusal = "Couldn't save {name}: {error}";
        assert_eq!(Language::Fr.tr(refusal), "Impossible d'enregistrer {name} : {error}");
        for language in Language::ALL.into_iter().filter(|l| *l != Language::Fr) {
            assert_eq!(language.tr(refusal), refusal, "{language:?}");
        }
        assert_eq!(french("A string upstream has"), None);
    }
}
