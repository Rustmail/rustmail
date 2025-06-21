use crate::errors::dictionary::ErrorDictionary;
use crate::errors::dictionary::ErrorMessage;

pub fn load_french_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert("database.connection_failed".to_string(),
        ErrorMessage::new("Échec de connexion à la base de données")
            .with_description("Le bot n'a pas pu établir une connexion à la base de données")
            .with_help("Vérifiez la configuration de la base de données et assurez-vous que le serveur est en marche"));
    dict.messages.insert(
        "database.query_failed".to_string(),
        ErrorMessage::new("Échec de la requête de base de données : {error}")
            .with_description("Une opération de base de données a échoué"),
    );
    dict.messages.insert(
        "database.not_found".to_string(),
        ErrorMessage::new("Enregistrement non trouvé dans la base de données")
            .with_description("Les données demandées n'ont pas pu être trouvées"),
    );
    dict.messages.insert(
        "discord.channel_not_found".to_string(),
        ErrorMessage::new("Canal non trouvé")
            .with_description("Le canal spécifié n'existe pas ou le bot n'y a pas accès"),
    );
    dict.messages.insert(
        "discord.user_not_found".to_string(),
        ErrorMessage::new("Utilisateur non trouvé")
            .with_description("L'utilisateur spécifié n'existe pas ou n'est pas accessible"),
    );
    dict.messages.insert(
        "discord.permission_denied".to_string(),
        ErrorMessage::new("Permission refusée")
            .with_description("Le bot n'a pas les permissions requises pour effectuer cette action"),
    );
    dict.messages.insert(
        "discord.dm_creation_failed".to_string(),
        ErrorMessage::new("Échec de création du canal DM")
            .with_description("Impossible de créer un canal de message privé avec l'utilisateur"),
    );
    dict.messages.insert(
        "discord.api_error".to_string(),
        ErrorMessage::new("Erreur de l'API Discord")
            .with_description("Une erreur s'est produite lors de la communication avec Discord"),
    );
    dict.messages.insert(
        "command.invalid_format".to_string(),
        ErrorMessage::new("Format de commande invalide")
            .with_description("La syntaxe de la commande est incorrecte")
            .with_help("Utilisez `{prefix}help` pour voir le format correct de la commande"),
    );
    dict.messages.insert(
        "command.missing_arguments".to_string(),
        ErrorMessage::new("Arguments requis manquants")
            .with_description("Cette commande nécessite des paramètres supplémentaires"),
    );
    dict.messages.insert(
        "command.invalid_arguments".to_string(),
        ErrorMessage::new("Arguments invalides : {arguments}")
            .with_description("Un ou plusieurs arguments sont invalides"),
    );
    dict.messages.insert(
        "command.unknown_command".to_string(),
        ErrorMessage::new("Commande inconnue : {command}")
            .with_description("La commande spécifiée n'existe pas")
            .with_help("Utilisez `{prefix}help` pour voir les commandes disponibles"),
    );
    dict.messages.insert(
        "command.insufficient_permissions".to_string(),
        ErrorMessage::new("Permissions insuffisantes").with_description(
            "Vous n'avez pas les permissions requises pour utiliser cette commande",
        ),
    );
    dict.messages.insert(
        "thread.not_found".to_string(),
        ErrorMessage::new("Thread non trouvé")
            .with_description("Aucun thread actif trouvé pour cet utilisateur ou ce canal"),
    );
    dict.messages.insert(
        "thread.already_exists".to_string(),
        ErrorMessage::new("Thread existe déjà")
            .with_description("Vous avez déjà un thread de support actif"),
    );
    dict.messages.insert(
        "thread.creation_failed".to_string(),
        ErrorMessage::new("Échec de création du thread").with_description(
            "Une erreur s'est produite lors de la création du thread de support",
        ),
    );
    dict.messages.insert(
        "message.not_found".to_string(),
        ErrorMessage::new("Message non trouvé")
            .with_description("Le message spécifié n'a pas pu être trouvé"),
    );
    dict.messages.insert(
        "message.number_not_found".to_string(),
        ErrorMessage::new("Message #{number} non trouvé")
            .with_description("Aucun message avec ce numéro n'existe"),
    );
    dict.messages.insert(
        "message.edit_failed".to_string(),
        ErrorMessage::new("Échec de modification du message")
            .with_description("Une erreur s'est produite lors de la modification du message"),
    );
    dict.messages.insert(
        "message.send_failed".to_string(),
        ErrorMessage::new("Échec d'envoi du message")
            .with_description("Une erreur s'est produite lors de l'envoi du message"),
    );
    dict.messages.insert(
        "message.too_long".to_string(),
        ErrorMessage::new("Message trop long")
            .with_description("Les messages Discord ne peuvent pas dépasser 2000 caractères"),
    );
    dict.messages.insert(
        "message.empty".to_string(),
        ErrorMessage::new("Le message ne peut pas être vide")
            .with_description("Veuillez fournir un message à envoyer"),
    );
    dict.messages.insert(
        "validation.invalid_input".to_string(),
        ErrorMessage::new("Entrée invalide : {input}")
            .with_description("L'entrée fournie n'est pas valide"),
    );
    dict.messages.insert(
        "validation.out_of_range".to_string(),
        ErrorMessage::new("Valeur hors limites : {range}")
            .with_description("La valeur doit être dans la plage spécifiée"),
    );
    dict.messages.insert(
        "validation.required_field_missing".to_string(),
        ErrorMessage::new("Champ requis manquant : {field}")
            .with_description("Ce champ est requis et ne peut pas être vide"),
    );
    dict.messages.insert(
        "success.message_sent".to_string(),
        ErrorMessage::new("Message envoyé avec succès ! (Message #{number})")
            .with_description("Votre message a été livré")
            .with_help("Utilisez `{prefix}edit {number}` pour modifier ce message"),
    );
    dict.messages.insert(
        "success.message_edited".to_string(),
        ErrorMessage::new("Message modifié avec succès")
            .with_description("Le message a été mis à jour dans le thread et en DM"),
    );
    dict.messages.insert(
        "success.thread_created".to_string(),
        ErrorMessage::new("Thread de support créé")
            .with_description("Un nouveau thread de support a été créé pour vous"),
    );
    dict.messages.insert(
        "thread.closed".to_string(),
        ErrorMessage::new("Merci d'avoir contacté le support ! Ton ticket est désormais clos.")
            .with_description("Le ticket de support a été fermé et la conversation terminée."),
    );
    dict.messages.insert(
        "reply.missing_content".to_string(),
        ErrorMessage::new("Veuillez fournir un message à envoyer à l'utilisateur.")
            .with_description("Vous devez fournir un message pour répondre à l'utilisateur."),
    );
    dict.messages.insert(
        "reply.send_failed_thread".to_string(),
        ErrorMessage::new("Échec de l'envoi du message dans le salon.")
            .with_description("Le bot n'a pas pu envoyer le message dans le salon du thread."),
    );
    dict.messages.insert(
        "reply.send_failed_dm".to_string(),
        ErrorMessage::new("Échec de l'envoi du message en DM à l'utilisateur.")
            .with_description("Le bot n'a pas pu envoyer le message en message privé à l'utilisateur."),
    );
    dict.messages.insert(
        "edit.validation.invalid_format".to_string(),
        ErrorMessage::new("❌ Format de commande invalide. Utilisation : `edit <numéro> <nouveau message>`")
            .with_description("Le format de la commande edit est invalide."),
    );
    dict.messages.insert(
        "edit.validation.missing_number".to_string(),
        ErrorMessage::new("❌ Format invalide. Il manque le numéro du message. Exemple : `edit 3 Nouveau message`")
            .with_description("Le numéro du message est manquant dans la commande edit."),
    );
    dict.messages.insert(
        "edit.validation.missing_content".to_string(),
        ErrorMessage::new("❌ Format invalide. Il manque le contenu. Exemple : `edit 3 Nouveau message`")
            .with_description("Le contenu du nouveau message est manquant dans la commande edit."),
    );
    dict.messages.insert(
        "edit.validation.invalid_number".to_string(),
        ErrorMessage::new("❌ Le numéro du message est invalide. Il doit être un nombre positif.")
            .with_description("Le numéro du message doit être positif."),
    );
    dict.messages.insert(
        "edit.validation.empty_content".to_string(),
        ErrorMessage::new("❌ Le nouveau message ne peut pas être vide.")
            .with_description("Le contenu du nouveau message ne peut pas être vide."),
    );
    dict.messages.insert(
        "reply_numbering.confirmation".to_string(),
        ErrorMessage::new("✅ Message envoyé ! (Message #{number}) - Utilisez `{prefix}edit {number}` pour modifier ce message.")
            .with_description("Confirmation après l'envoi d'un message avec son numéro."),
    );
    dict.messages.insert(
        "reply_numbering.preview".to_string(),
        ErrorMessage::new("(Message #{number} - Utilisez `{prefix}edit {number}` pour modifier)")
            .with_description("Aperçu du numéro de message pour modification."),
    );
    dict.messages.insert(
        "reply_numbering.footer".to_string(),
        ErrorMessage::new("Message #{number} • {prefix}edit {number} pour modifier")
            .with_description("Footer pour les embeds avec numéro de message et commande edit."),
    );
    dict.messages.insert(
        "reply_numbering.text_footer".to_string(),
        ErrorMessage::new("*Message #{number} - `{prefix}edit {number}` pour modifier*")
            .with_description("Footer pour les messages texte avec numéro de message et commande edit."),
    );
    dict.messages.insert(
        "edit.not_found".to_string(),
        ErrorMessage::new("❌ Message à modifier non trouvé.")
            .with_description("Impossible de trouver le message original à modifier. Assurez-vous que le numéro est correct et que vous êtes l'auteur du message."),
    );
    dict.messages.insert(
        "edit.invalid_id_thread".to_string(),
        ErrorMessage::new("❌ ID de message invalide pour le thread.")
            .with_description("L'ID du message dans le salon est invalide ou corrompu."),
    );
    dict.messages.insert(
        "edit.edit_failed_thread".to_string(),
        ErrorMessage::new("❌ Échec de la modification du message dans le thread.")
            .with_description("Le bot n'a pas pu modifier le message dans le salon du thread."),
    );
    dict.messages.insert(
        "edit.invalid_id_dm".to_string(),
        ErrorMessage::new("❌ ID de message invalide pour le DM.")
            .with_description("L'ID du message en message privé est invalide ou corrompu."),
    );
    dict.messages.insert(
        "edit.dm_access_failed".to_string(),
        ErrorMessage::new("❌ Impossible d'accéder aux DMs de l'utilisateur.")
            .with_description("Le bot n'a pas pu envoyer de message privé à l'utilisateur. Il a peut-être bloqué le bot ou désactivé ses DMs."),
    );
    dict.messages.insert(
        "edit.edit_failed_dm".to_string(),
        ErrorMessage::new("❌ Échec de la modification du message en DM.")
            .with_description("Le bot n'a pas pu modifier le message en message privé."),
    );
    dict.messages.insert(
        "permission.insufficient_permissions".to_string(),
        ErrorMessage::new("Permissions insuffisantes")
            .with_description("Vous n'avez pas les permissions nécessaires pour cette action"),
    );
    dict.messages.insert(
        "server.wrong_guild_single".to_string(),
        ErrorMessage::new("Serveur incorrect")
            .with_description("Vous devez être dans le serveur principal pour ouvrir un ticket")
            .with_help("Rejoignez le serveur principal pour pouvoir contacter le support"),
    );
    dict.messages.insert(
        "server.wrong_guild_dual".to_string(),
        ErrorMessage::new("Serveur incorrect")
            .with_description("Vous devez être dans le serveur communautaire pour ouvrir un ticket")
            .with_help("Rejoignez le serveur communautaire pour pouvoir contacter le support"),
    );
    dict.messages.insert(
        "server.not_in_community".to_string(),
        ErrorMessage::new("Utilisateur non trouvé dans le serveur communautaire")
            .with_description("L'utilisateur doit être membre du serveur communautaire"),
    );
    dict.messages.insert(
        "user.left_server".to_string(),
        ErrorMessage::new("❌ **ERREUR** : Impossible d'envoyer le message car l'utilisateur **{username}** n'est plus membre du serveur communautaire.")
            .with_description("L'utilisateur a quitté le serveur communautaire"),
    );
    dict.messages.insert(
        "user.left_server_close".to_string(),
        ErrorMessage::new("ℹ️ **INFORMATION** : Le ticket a été fermé. L'utilisateur **{username}** n'est plus membre du serveur communautaire, donc aucun message de fermeture ne lui a été envoyé.")
            .with_description("Information lors de la fermeture d'un ticket d'un utilisateur qui a quitté"),
    );
    dict.messages.insert(
        "user.left_server_notification".to_string(),
        ErrorMessage::new("⚠️ **ALERTE** : L'utilisateur **{username}** (ID: {user_id}) a quitté le serveur.\n\nLe thread reste ouvert mais vous ne pouvez plus envoyer de messages à cet utilisateur.")
            .with_description("Notification quand un utilisateur quitte le serveur"),
    );
    dict.messages.insert(
        "reply.user_not_found".to_string(),
        ErrorMessage::new("Utilisateur non trouvé")
            .with_description("L'utilisateur n'existe pas ou n'est pas accessible"),
    );
    dict.messages.insert(
        "config.invalid_configuration".to_string(),
        ErrorMessage::new("Configuration invalide")
            .with_description("La configuration du bot est incorrecte"),
    );
    dict.messages.insert(
        "general.unknown_error".to_string(),
        ErrorMessage::new("Erreur inconnue : {message}")
            .with_description("Une erreur inattendue s'est produite"),
    );
} 