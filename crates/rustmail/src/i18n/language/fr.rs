use crate::prelude::errors::*;

pub fn load_french_messages(dict: &mut ErrorDictionary) {
    dict.messages.insert("database.connection_failed".to_string(),
                         DictionaryMessage::new("Échec de connexion à la base de données")
            .with_description("Le rustmail n'a pas pu établir une connexion à la base de données")
            .with_help("Vérifiez la configuration de la base de données et assurez-vous que le serveur est en marche"));
    dict.messages.insert(
        "database.query_failed".to_string(),
        DictionaryMessage::new("Échec de la requête de base de données : {error}")
            .with_description("Une opération de base de données a échoué"),
    );
    dict.messages.insert(
        "database.not_found".to_string(),
        DictionaryMessage::new("Enregistrement non trouvé dans la base de données")
            .with_description("Les données demandées n'ont pas pu être trouvées"),
    );
    dict.messages.insert(
        "discord.channel_not_found".to_string(),
        DictionaryMessage::new("Canal non trouvé")
            .with_description("Le canal spécifié n'existe pas ou le rustmail n'y a pas accès"),
    );
    dict.messages.insert(
        "discord.user_not_found".to_string(),
        DictionaryMessage::new("Utilisateur non trouvé")
            .with_description("L'utilisateur spécifié n'existe pas ou n'est pas accessible"),
    );
    dict.messages.insert(
        "discord.permission_denied".to_string(),
        DictionaryMessage::new("Permission refusée").with_description(
            "Le rustmail n'a pas les permissions requises pour effectuer cette action",
        ),
    );
    dict.messages.insert(
        "discord.dm_creation_failed".to_string(),
        DictionaryMessage::new("Échec de création du canal DM")
            .with_description("Impossible de créer un canal de message privé avec l'utilisateur"),
    );
    dict.messages.insert(
        "discord.api_error".to_string(),
        DictionaryMessage::new("Erreur de l'API Discord")
            .with_description("Une erreur s'est produite lors de la communication avec Discord"),
    );
    dict.messages.insert(
        "discord.attachment_too_large".to_string(),
        DictionaryMessage::new("Votre pièce jointe est trop volumineuse ! Discord a une limite de taille de fichier de 8 Mo pour les pièces jointes. Veuillez réduire la taille du fichier ou envoyer un lien."),
    );
    dict.messages.insert(
        "discord.user_is_a_bot".to_string(),
        DictionaryMessage::new("L'utilisateur spécifié est un rustmail"),
    );
    dict.messages.insert(
        "discord.shard_manager_not_found".to_string(),
        DictionaryMessage::new("Shard manager non trouvé."),
    );
    dict.messages.insert(
        "command.invalid_format".to_string(),
        DictionaryMessage::new("Format de commande invalide")
            .with_description("La syntaxe de la commande est incorrecte")
            .with_help("Utilisez `{prefix}help` pour voir le format correct de la commande"),
    );
    dict.messages.insert(
        "command.missing_arguments".to_string(),
        DictionaryMessage::new("Arguments requis manquants")
            .with_description("Cette commande nécessite des paramètres supplémentaires"),
    );
    dict.messages.insert(
        "command.invalid_arguments".to_string(),
        DictionaryMessage::new("Arguments invalides : {arguments}")
            .with_description("Un ou plusieurs arguments sont invalides"),
    );
    dict.messages.insert(
        "command.unknown_command".to_string(),
        DictionaryMessage::new("Commande inconnue : {command}")
            .with_description("La commande spécifiée n'existe pas")
            .with_help("Utilisez `{prefix}help` pour voir les commandes disponibles"),
    );
    dict.messages.insert(
        "command.unknown_slash_command".to_string(),
        DictionaryMessage::new("Slash Commande inconnue : {command}"),
    );
    dict.messages.insert(
        "command.insufficient_permissions".to_string(),
        DictionaryMessage::new("Permissions insuffisantes").with_description(
            "Vous n'avez pas les permissions requises pour utiliser cette commande",
        ),
    );
    dict.messages.insert(
        "thread.not_found".to_string(),
        DictionaryMessage::new("Thread non trouvé")
            .with_description("Aucun thread actif trouvé pour cet utilisateur ou ce canal"),
    );
    dict.messages.insert(
        "thread.already_exists".to_string(),
        DictionaryMessage::new("Thread existe déjà")
            .with_description("Vous avez déjà un thread de support actif"),
    );
    dict.messages.insert(
        "thread.creation_failed".to_string(),
        DictionaryMessage::new("Échec de création du thread")
            .with_description("Une erreur s'est produite lors de la création du thread de support"),
    );
    dict.messages.insert(
        "thread.user_still_in_server".to_string(),
        DictionaryMessage::new("L'utilisateur est toujours sur le serveur.")
            .with_description("Utilisez la commande « close » pour fermer ce ticket."),
    );
    dict.messages.insert(
        "thread.not_a_thread_channel".to_string(),
        DictionaryMessage::new("Ce channel n'est pas issu d'un ticket de support."),
    );
    dict.messages.insert(
        "thread.modal_invalid_user_id".to_string(),
        DictionaryMessage::new("User Id invalide"),
    );
    dict.messages.insert(
        "message.not_found".to_string(),
        DictionaryMessage::new("Message non trouvé")
            .with_description("Le message spécifié n'a pas pu être trouvé"),
    );
    dict.messages.insert(
        "message.number_not_found".to_string(),
        DictionaryMessage::new("Message #{number} non trouvé")
            .with_description("Aucun message avec ce numéro n'existe"),
    );
    dict.messages.insert(
        "message.edit_failed".to_string(),
        DictionaryMessage::new("Échec de modification du message")
            .with_description("Une erreur s'est produite lors de la modification du message"),
    );
    dict.messages.insert(
        "message.send_failed".to_string(),
        DictionaryMessage::new("Échec d'envoi du message")
            .with_description("Une erreur s'est produite lors de l'envoi du message"),
    );
    dict.messages.insert(
        "message.too_long".to_string(),
        DictionaryMessage::new("Message trop long")
            .with_description("Les messages Discord ne peuvent pas dépasser 2000 caractères"),
    );
    dict.messages.insert(
        "message.empty".to_string(),
        DictionaryMessage::new("Le message ne peut pas être vide")
            .with_description("Veuillez fournir un message à envoyer"),
    );
    dict.messages.insert(
        "validation.invalid_input".to_string(),
        DictionaryMessage::new("Entrée invalide : {input}")
            .with_description("L'entrée fournie n'est pas valide"),
    );
    dict.messages.insert(
        "validation.out_of_range".to_string(),
        DictionaryMessage::new("Valeur hors limites : {range}")
            .with_description("La valeur doit être dans la plage spécifiée"),
    );
    dict.messages.insert(
        "validation.required_field_missing".to_string(),
        DictionaryMessage::new("Champ requis manquant : {field}")
            .with_description("Ce champ est requis et ne peut pas être vide"),
    );
    dict.messages.insert(
        "success.message_sent".to_string(),
        DictionaryMessage::new("Message envoyé avec succès ! (Message #{number})")
            .with_description("Votre message a été livré")
            .with_help("Utilisez `{prefix}edit {number}` pour modifier ce message"),
    );
    dict.messages.insert(
        "success.message_edited".to_string(),
        DictionaryMessage::new("Message modifié avec succès")
            .with_description("Le message a été mis à jour dans le thread et en DM"),
    );
    dict.messages.insert(
        "success.thread_created".to_string(),
        DictionaryMessage::new("Thread de support créé")
            .with_description("Un nouveau thread de support a été créé pour vous"),
    );
    dict.messages.insert(
        "thread.closed".to_string(),
        DictionaryMessage::new(
            "Merci d'avoir contacté le support ! Ton ticket est désormais clos.",
        )
        .with_description("Le ticket de support a été fermé et la conversation terminée."),
    );
    dict.messages.insert(
        "thread.ask_to_close".to_string(),
        DictionaryMessage::new("Fermer"),
    );
    dict.messages.insert(
        "thread.ask_to_keep_open".to_string(),
        DictionaryMessage::new("Laisser ouvert"),
    );
    dict.messages.insert(
        "thread.thread_closing".to_string(),
        DictionaryMessage::new(
            "Le ticket se fermera dans {seconds} secondes à la demande de {user}.",
        ),
    );
    dict.messages.insert(
        "thread.action_in_progress".to_string(),
        DictionaryMessage::new("Une action est déjà en cours, merci de patienter."),
    );
    dict.messages.insert(
        "thread.will_remain_open".to_string(),
        DictionaryMessage::new("Le thread restera ouvert."),
    );
    dict.messages.insert(
        "thread.ask_create_ticket".to_string(),
        DictionaryMessage::new("Ce channel à été crée dans la catégorie des tickets de support. Voulez vous en créer un ?")
    );
    dict.messages.insert(
        "thread.modal_to_create_ticket".to_string(),
        DictionaryMessage::new("Créer un ticket"),
    );
    dict.messages.insert(
        "thread.modal_bot_user".to_string(),
        DictionaryMessage::new(
            "L'utilisateur spécifié est un rustmail, veuillez en choisir un autre.",
        ),
    );
    dict.messages.insert(
        "thread.created".to_string(),
        DictionaryMessage::new("Ticket créé: {channel}")
            .with_description("Un nouveau ticket de support a été ouvert ou récupéré"),
    );
    dict.messages.insert(
        "thread.unknown_action".to_string(),
        DictionaryMessage::new("Action inconnue")
            .with_description("L'action demandée pour le ticket est inconnue"),
    );
    dict.messages.insert(
        "thread.modal_user_not_found".to_string(),
        DictionaryMessage::new(
            "L'utilisateur spécifié est introuvable, veuillez en choisir un autre.",
        ),
    );
    dict.messages.insert(
        "thread.category_not_found".to_string(),
        DictionaryMessage::new(
            "La catégorie spécifiée pour les tickets n'existe pas sur le serveur.",
        ),
    );
    dict.messages.insert(
        "reply.missing_content".to_string(),
        DictionaryMessage::new("Veuillez fournir un message à envoyer à l'utilisateur.")
            .with_description("Vous devez fournir un message pour répondre à l'utilisateur."),
    );
    dict.messages.insert(
        "reply.send_failed_thread".to_string(),
        DictionaryMessage::new("Échec de l'envoi du message dans le salon.")
            .with_description("Le rustmail n'a pas pu envoyer le message dans le salon du thread."),
    );
    dict.messages.insert(
        "reply.send_failed_dm".to_string(),
        DictionaryMessage::new("Échec de l'envoi du message en DM à l'utilisateur.")
            .with_description(
                "Le rustmail n'a pas pu envoyer le message en message privé à l'utilisateur.",
            ),
    );
    dict.messages.insert(
        "edit.validation.invalid_format".to_string(),
        DictionaryMessage::new(
            "❌ Format de commande invalide. Utilisation : `edit <numéro> <nouveau message>`",
        )
        .with_description("Le format de la commande edit est invalide."),
    );
    dict.messages.insert(
        "edit.validation.missing_number".to_string(),
        DictionaryMessage::new("❌ Format invalide. Il manque le numéro du message. Exemple : `edit 3 Nouveau message`")
            .with_description("Le numéro du message est manquant dans la commande edit."),
    );
    dict.messages.insert(
        "edit.validation.missing_content".to_string(),
        DictionaryMessage::new(
            "❌ Format invalide. Il manque le contenu. Exemple : `edit 3 Nouveau message`",
        )
        .with_description("Le contenu du nouveau message est manquant dans la commande edit."),
    );
    dict.messages.insert(
        "edit.validation.invalid_number".to_string(),
        DictionaryMessage::new(
            "❌ Le numéro du message est invalide. Il doit être un nombre positif.",
        )
        .with_description("Le numéro du message doit être positif."),
    );
    dict.messages.insert(
        "edit.validation.empty_content".to_string(),
        DictionaryMessage::new("❌ Le nouveau message ne peut pas être vide.")
            .with_description("Le contenu du nouveau message ne peut pas être vide."),
    );
    dict.messages.insert(
        "edit.modification_from_user".to_string(),
        DictionaryMessage::new("L'utilisateur a modifié son message.\n\nAvant:\n{before}\n\nAprès:\n{after}\n\nLien: {link}")
    );
    dict.messages.insert(
        "edit.modification_from_staff".to_string(),
        DictionaryMessage::new(
            "Un staff a modifié son message.\n\nAvant:\n{before}\n\nAprès:\n{after}\n\nLien: {link}",
        ),
    );
    dict.messages.insert(
        "reply_numbering.confirmation".to_string(),
        DictionaryMessage::new("✅ Message envoyé ! (Message #{number}) - Utilisez `{prefix}edit {number}` pour modifier ce message.")
            .with_description("Confirmation après l'envoi d'un message avec son numéro."),
    );
    dict.messages.insert(
        "reply_numbering.preview".to_string(),
        DictionaryMessage::new(
            "(Message #{number} - Utilisez `{prefix}edit {number}` pour modifier)",
        )
        .with_description("Aperçu du numéro de message pour modification."),
    );
    dict.messages.insert(
        "reply_numbering.footer".to_string(),
        DictionaryMessage::new("Message #{number} • {prefix}edit {number} pour modifier")
            .with_description("Footer pour les embeds avec numéro de message et commande edit."),
    );
    dict.messages.insert(
        "reply_numbering.text_footer".to_string(),
        DictionaryMessage::new("*Message #{number} - `{prefix}edit {number}` pour modifier*")
            .with_description(
                "Footer pour les messages texte avec numéro de message et commande edit.",
            ),
    );
    dict.messages.insert(
        "edit.not_found".to_string(),
        DictionaryMessage::new("❌ Message à modifier non trouvé.")
            .with_description("Impossible de trouver le message original à modifier. Assurez-vous que le numéro est correct et que vous êtes l'auteur du message."),
    );
    dict.messages.insert(
        "edit.invalid_id_thread".to_string(),
        DictionaryMessage::new("❌ ID de message invalide pour le thread.")
            .with_description("L'ID du message dans le salon est invalide ou corrompu."),
    );
    dict.messages.insert(
        "edit.edit_failed_thread".to_string(),
        DictionaryMessage::new("❌ Échec de la modification du message dans le thread.")
            .with_description(
                "Le rustmail n'a pas pu modifier le message dans le salon du thread.",
            ),
    );
    dict.messages.insert(
        "edit.invalid_id_dm".to_string(),
        DictionaryMessage::new("❌ ID de message invalide pour le DM.")
            .with_description("L'ID du message en message privé est invalide ou corrompu."),
    );
    dict.messages.insert(
        "edit.dm_access_failed".to_string(),
        DictionaryMessage::new("❌ Impossible d'accéder aux DMs de l'utilisateur.")
            .with_description("Le rustmail n'a pas pu envoyer de message privé à l'utilisateur. Il a peut-être bloqué le rustmail ou désactivé ses DMs."),
    );
    dict.messages.insert(
        "edit.edit_failed_dm".to_string(),
        DictionaryMessage::new("❌ Échec de la modification du message en DM.")
            .with_description("Le rustmail n'a pas pu modifier le message en message privé."),
    );
    dict.messages.insert(
        "permission.insufficient_permissions".to_string(),
        DictionaryMessage::new("Permissions insuffisantes")
            .with_description("Vous n'avez pas les permissions nécessaires pour cette action"),
    );
    dict.messages.insert(
        "server.wrong_guild_single".to_string(),
        DictionaryMessage::new("Serveur incorrect")
            .with_description("Vous devez être dans le serveur principal pour ouvrir un ticket")
            .with_help("Rejoignez le serveur principal pour pouvoir contacter le support"),
    );
    dict.messages.insert(
        "server.wrong_guild_dual".to_string(),
        DictionaryMessage::new("Serveur incorrect")
            .with_description("Vous devez être dans le serveur communautaire pour ouvrir un ticket")
            .with_help("Rejoignez le serveur communautaire pour pouvoir contacter le support"),
    );
    dict.messages.insert(
        "server.not_in_community".to_string(),
        DictionaryMessage::new("Utilisateur non trouvé dans le serveur communautaire")
            .with_description("L'utilisateur doit être membre du serveur communautaire"),
    );
    dict.messages.insert(
        "user.left_server".to_string(),
        DictionaryMessage::new("❌ **ERREUR** : Impossible d'envoyer le message car l'utilisateur **{username}** n'est plus membre du serveur communautaire.")
            .with_description("L'utilisateur a quitté le serveur communautaire"),
    );
    dict.messages.insert(
        "user.left_server_close".to_string(),
        DictionaryMessage::new("ℹ️ **INFORMATION** : Le ticket a été fermé. L'utilisateur **{username}** n'est plus membre du serveur communautaire, donc aucun message de fermeture ne lui a été envoyé.")
            .with_description("Information lors de la fermeture d'un ticket d'un utilisateur qui a quitté"),
    );
    dict.messages.insert(
        "user.left_server_notification".to_string(),
        DictionaryMessage::new("⚠️ **ALERTE** : L'utilisateur **{username}** (ID: {user_id}) a quitté le serveur.\n\nLe thread reste ouvert mais vous ne pouvez plus envoyer de messages à cet utilisateur.")
            .with_description("Notification quand un utilisateur quitte le serveur"),
    );
    dict.messages.insert(
        "reply.user_not_found".to_string(),
        DictionaryMessage::new("Utilisateur non trouvé")
            .with_description("L'utilisateur n'existe pas ou n'est pas accessible"),
    );
    dict.messages.insert(
        "config.invalid_configuration".to_string(),
        DictionaryMessage::new("Configuration invalide")
            .with_description("La configuration du rustmail est incorrecte"),
    );
    dict.messages.insert(
        "general.unknown_error".to_string(),
        DictionaryMessage::new("Erreur inconnue : {message}")
            .with_description("Une erreur inattendue s'est produite"),
    );
    dict.messages
        .insert("general.yes".to_string(), DictionaryMessage::new("Oui"));
    dict.messages
        .insert("general.no".to_string(), DictionaryMessage::new("Non"));
    dict.messages.insert(
        "recovery.messages_recovered".to_string(),
        DictionaryMessage::new(
            "📥 **{count} message(s) récupéré(s)** pendant la période d'indisponibilité du rustmail",
        )
        .with_description("Notification de récupération de messages manqués"),
    );
    dict.messages.insert(
        "recovery.summary".to_string(),
        DictionaryMessage::new("Récupération terminée : {total} messages récupérés dans {threads} threads ({failed} échecs)")
            .with_description("Résumé de la récupération des messages"),
    );
    dict.messages.insert(
        "recovery.started".to_string(),
        DictionaryMessage::new("🔄 Début de la récupération des messages manqués...")
            .with_description("Notification de début de récupération"),
    );
    dict.messages.insert(
        "recovery.completed".to_string(),
        DictionaryMessage::new("✅ Récupération des messages terminée")
            .with_description("Notification de fin de récupération"),
    );
    dict.messages.insert(
        "alert.not_in_thread".to_string(),
        DictionaryMessage::new(
            "❌ Cette commande ne peut être utilisée que dans un thread de support",
        )
        .with_description("La commande alert doit être utilisée dans un canal de thread"),
    );
    dict.messages.insert(
        "alert.alert_not_found".to_string(),
        DictionaryMessage::new("Aucune alerte définie pour ce ticket"),
    );
    dict.messages.insert(
        "command.not_in_thread".to_string(),
        DictionaryMessage::new(
            "Cette commande ne peut être utilisée que dans un thread de support",
        ),
    );
    dict.messages.insert(
        "alert.set_failed".to_string(),
        DictionaryMessage::new("❌ Vous avez déjà définie une alerte pour ce ticket !"),
    );
    dict.messages.insert(
        "alert.confirmation".to_string(),
        DictionaryMessage::new(
            "🔔 Alerte définie ! Vous serez notifié quand {user} enverra son prochain message",
        )
        .with_description("Confirmation que l'alerte a été définie"),
    );
    dict.messages.insert(
        "alert.ping_message".to_string(),
        DictionaryMessage::new("**Nouveau message reçu de {user} !**").with_description(
            "Ping du staff quand l'utilisateur envoie un nouveau message après la commande alert",
        ),
    );
    dict.messages.insert(
        "alert.cancel_failed".to_string(),
        DictionaryMessage::new("❌ Échec de l'annulation de l'alerte")
            .with_description("Une erreur s'est produite lors de l'annulation de l'alerte"),
    );
    dict.messages.insert(
        "alert.cancel_confirmation".to_string(),
        DictionaryMessage::new("🔕 Alerte annulée ! Vous ne serez plus notifié quand {user} enverra un nouveau message")
            .with_description("Confirmation que l'alerte a été annulée"),
    );
    dict.messages.insert(
        "move_thread.not_in_thread".to_string(),
        DictionaryMessage::new(
            "❌ Cette commande ne peut être utilisée que dans un thread de support",
        )
        .with_description("La commande move_thread doit être utilisée dans un canal de thread"),
    );
    dict.messages.insert(
        "move_thread.missing_category".to_string(),
        DictionaryMessage::new("❌ Veuillez spécifier un nom de catégorie. Utilisation : `{prefix}move_thread <nom_catégorie>`")
            .with_description("Le nom de la catégorie est manquant dans la commande move_thread"),
    );
    dict.messages.insert(
        "move_thread.failed_to_fetch_categories".to_string(),
        DictionaryMessage::new("❌ Échec de récupération des catégories du serveur")
            .with_description(
                "Le rustmail n'a pas pu récupérer la liste des catégories du serveur",
            ),
    );
    dict.messages.insert(
        "move_thread.category_not_found".to_string(),
        DictionaryMessage::new("❌ Catégorie '{category}' non trouvée")
            .with_description("Aucune catégorie avec ce nom n'existe sur le serveur"),
    );
    dict.messages.insert(
        "move_thread.failed_to_move".to_string(),
        DictionaryMessage::new("❌ Échec du déplacement du thread vers la catégorie spécifiée")
            .with_description("Une erreur s'est produite lors du déplacement du thread"),
    );
    dict.messages.insert(
        "move_thread.success".to_string(),
        DictionaryMessage::new("✅ Thread déplacé vers la catégorie **{category}** par <@{staff}>")
            .with_description("Le thread a été déplacé avec succès vers la nouvelle catégorie"),
    );
    dict.messages.insert(
        "new_thread.missing_user".to_string(),
        DictionaryMessage::new("❌ Veuillez spécifier un utilisateur. Utilisation : `{prefix}new <id_utilisateur_ou_mention>`")
            .with_description("L'ID utilisateur ou la mention est manquant dans la commande new_thread"),
    );
    dict.messages.insert(
        "new_thread.user_has_thread".to_string(),
        DictionaryMessage::new("❌ Cet utilisateur a déjà un thread de support actif"),
    );
    dict.messages.insert(
        "new_thread.user_has_thread_with_link".to_string(),
        DictionaryMessage::new("❌ {user} a déjà un thread de support actif\n\n📎 **Lien du thread :** <#{channel_id}>")
    );
    dict.messages.insert(
        "new_thread.user_not_found".to_string(),
        DictionaryMessage::new("❌ Utilisateur non trouvé")
            .with_description("L'utilisateur spécifié n'existe pas ou n'est pas accessible"),
    );
    dict.messages.insert(
        "new_thread.user_not_in_community".to_string(),
        DictionaryMessage::new("❌ L'utilisateur n'est pas membre du serveur communautaire")
            .with_description(
                "L'utilisateur doit être dans le serveur communautaire pour créer un thread",
            ),
    );
    dict.messages.insert(
        "new_thread.user_is_a_bot".to_string(),
        DictionaryMessage::new("❌ Vous ne pouvez pas créer un ticket pour un rustmail."),
    );
    dict.messages.insert(
        "new_thread.channel_creation_failed".to_string(),
        DictionaryMessage::new("❌ Échec de création du canal de thread de support")
            .with_description("Une erreur s'est produite lors de la création du canal de thread"),
    );
    dict.messages.insert(
        "new_thread.database_error".to_string(),
        DictionaryMessage::new("❌ Échec de création du thread dans la base de données")
            .with_description(
                "Une erreur s'est produite lors de la sauvegarde du thread dans la base de données",
            ),
    );
    dict.messages.insert(
        "new_thread.welcome_message".to_string(),
        DictionaryMessage::new("🎫 **Thread de support créé pour {user}**\n\nCe thread a été créé par le staff. Vous pouvez maintenant communiquer avec l'équipe de support.")
            .with_description("Message de bienvenue dans le thread nouvellement créé"),
    );
    dict.messages.insert(
        "new_thread.dm_notification".to_string(),
        DictionaryMessage::new("🎫 **Thread de support ouvert**\n\nUn membre du staff a initié une conversation de support avec vous. Vous pouvez maintenant communiquer avec l'équipe de support.")
            .with_description("Notification DM envoyée à l'utilisateur quand un thread est créé"),
    );
    dict.messages.insert(
        "new_thread.success_with_dm".to_string(),
        DictionaryMessage::new("✅ Thread de support créé pour {user} dans {channel_id} par {staff}\n\nNotification DM envoyée avec succès.")
            .with_description("Message de succès quand le thread est créé et le DM envoyé"),
    );
    dict.messages.insert(
        "new_thread.success_without_dm".to_string(),
        DictionaryMessage::new("✅ Thread de support créé pour {user} dans <#{channel_id}> par {staff}\n\n⚠️ Impossible d'envoyer la notification DM (l'utilisateur peut avoir désactivé les DMs).")
            .with_description("Message de succès quand le thread est créé mais le DM échoue"),
    );
    dict.messages.insert(
        "delete.not_in_thread".to_string(),
        DictionaryMessage::new(
            "❌ Cette commande ne peut être utilisée que dans un thread de support",
        )
        .with_description("La commande delete doit être utilisée dans un canal de thread"),
    );
    dict.messages.insert(
        "delete.missing_number".to_string(),
        DictionaryMessage::new(
            "❌ Veuillez spécifier un numéro de message. Utilisation : `{prefix}delete <numéro>`",
        )
        .with_description("Le numéro de message est manquant dans la commande delete"),
    );
    dict.messages.insert(
        "delete.message_not_found".to_string(),
        DictionaryMessage::new("❌ Message #{number} non trouvé")
            .with_description("Aucun message avec ce numéro n'existe dans ce thread"),
    );
    dict.messages.insert(
        "command.discord_delete_failed".to_string(),
        DictionaryMessage::new("❌ Échec de suppression du message depuis Discord")
            .with_description(
                "Une erreur s'est produite lors de la suppression du message depuis Discord",
            ),
    );
    dict.messages.insert(
        "delete.database_delete_failed".to_string(),
        DictionaryMessage::new("❌ Échec de suppression du message depuis la base de données")
            .with_description("Une erreur s'est produite lors de la suppression du message depuis la base de données"),
    );
    dict.messages.insert(
        "delete.success".to_string(),
        DictionaryMessage::new("✅ Message #{number} a été supprimé avec succès")
            .with_description("Confirmation que le message a été supprimé"),
    );
    dict.messages.insert(
        "delete.removed_by_user".to_string(),
        DictionaryMessage::new("L'utilisateur {userid} a supprimé son message : \n\n{content}")
            .with_description("Entrée de log lorsque l'utilisateur supprime son message DM (répercuté dans le thread)")
            .with_help("Paramètres: content, number (optionnel si message staff)"),
    );
    dict.messages.insert(
        "delete.removed_by_staff".to_string(),
        DictionaryMessage::new("Le staff {userid} a supprimé un message : \n\n{content}")
            .with_description("Entrée de log lorsqu'un membre du staff supprime un message dans le thread ou son miroir DM")
            .with_help("Paramètres: content, number (optionnel), link (optionnel)"),
    );
    dict.messages.insert(
        "add_staff.add_success".to_string(),
        DictionaryMessage::new("L'utilisateur {user} a été ajouté au ticket avec succès."),
    );
    dict.messages.insert(
        "add_staff.remove_success".to_string(),
        DictionaryMessage::new("L'utilisateur {user} a été retiré du ticket avec succès."),
    );
    dict.messages.insert(
        "id.show_id".to_string(),
        DictionaryMessage::new("ID de {user} : {id}"),
    );
    dict.messages.insert(
        "close.closure_canceled".to_string(),
        DictionaryMessage::new("Fermeture programmée annulée."),
    );
    dict.messages.insert(
        "close.auto_canceled_on_message".to_string(),
        DictionaryMessage::new(
            "La fermeture programmée a été automatiquement annulée car un message a été reçu.",
        ),
    );
    dict.messages.insert(
        "close.replacing_existing_closure".to_string(),
        DictionaryMessage::new("⚠️ Attention : Une fermeture était déjà programmée dans {old_time}. Elle sera remplacée par la nouvelle."),
    );
    dict.messages.insert(
        "close.no_scheduled_closures_to_cancel".to_string(),
        DictionaryMessage::new("Aucune fermeture programmée à annuler."),
    );
    dict.messages.insert(
        "close.closure_already_scheduled".to_string(),
        DictionaryMessage::new("Une fermeture est déjà programmée dans {seconds} secondes. Utilisez !close cancel pour l'annuler."),
    );
    dict.messages.insert(
        "close.closing".to_string(),
        DictionaryMessage::new("Ce ticket sera fermé dans {time}."),
    );
    dict.messages.insert(
        "close.silent_closing".to_string(),
        DictionaryMessage::new("Ce ticket sera fermé silencieusement dans {time}."),
    );
    dict.messages.insert(
        "logs.ticket_closed".to_string(),
        DictionaryMessage::new("Ticket fermé par <@{staff}> pour l'utilisateur **{username}** (ID: {user_id})\n[Voir le log sur le panel]({panel_url})"),
    );
    dict.messages.insert(
        "feature.not_implemented".to_string(),
        DictionaryMessage::new("Cette feature n'est pas encore implémentée."),
    );
    dict.messages.insert(
        "slash_command.id_command_description".to_string(),
        DictionaryMessage::new("Afficher l'ID d'un utilisateur du thread de support"),
    );
    dict.messages.insert(
        "slash_command.move_command_description".to_string(),
        DictionaryMessage::new("Déplacer le thread de support vers une autre catégorie"),
    );
    dict.messages.insert(
        "slash_command.move_command_name_argument".to_string(),
        DictionaryMessage::new("La catégorie vers laquelle déplacer le thread"),
    );
    dict.messages.insert(
        "slash_command.new_thread_command_description".to_string(),
        DictionaryMessage::new("Créer un nouveau thread de support pour un utilisateur"),
    );
    dict.messages.insert(
        "slash_command.new_thread_user_id_argument".to_string(),
        DictionaryMessage::new("L'ID de l'utilisateur pour lequel créer le thread"),
    );
    dict.messages.insert(
        "slash_command.close_command_description".to_string(),
        DictionaryMessage::new("Fermer un ticket de support"),
    );
    dict.messages.insert(
        "slash_command.close_time_before_close_argument".to_string(),
        DictionaryMessage::new("Le temps avant la fermeture du ticket (ex: 1s, 1m, 1h, 1d)"),
    );
    dict.messages.insert(
        "slash_command.close_silent_argument".to_string(),
        DictionaryMessage::new(
            "Fermer le ticket silencieusement sans envoyer de message à l'utilisateur",
        ),
    );
    dict.messages.insert(
        "slash_command.close_cancel_argument".to_string(),
        DictionaryMessage::new("Annuler la fermeture programmée du ticket"),
    );
    dict.messages.insert(
        "slash_command.edit_command_description".to_string(),
        DictionaryMessage::new("Editer un message envoyé dans un ticket de support"),
    );
    dict.messages.insert(
        "slash_command.edit_message_id_argument".to_string(),
        DictionaryMessage::new("Le numéro du message à éditer. Vous pouvez trouver le numéro en regardant le footer du message."),
    );
    dict.messages.insert(
        "slash_command.edit_message_argument".to_string(),
        DictionaryMessage::new("Le nouveau contenu du message"),
    );
    dict.messages.insert(
        "slash_command.add_staff_command_description".to_string(),
        DictionaryMessage::new(
            "Ajouter un membre du staff à un ticket de support auquel il n'a pas accès",
        ),
    );
    dict.messages.insert(
        "slash_command.add_staff_user_id_argument".to_string(),
        DictionaryMessage::new("L'ID du staff à ajouter au ticket"),
    );
    dict.messages.insert(
        "slash_command.remove_staff_command_description".to_string(),
        DictionaryMessage::new("Retirer un membre du staff d'un ticket de support"),
    );
    dict.messages.insert(
        "slash_command.remove_staff_user_id_argument".to_string(),
        DictionaryMessage::new("L'ID du staff à retirer du ticket"),
    );
    dict.messages.insert(
        "slash_command.alert_command_description".to_string(),
        DictionaryMessage::new("Définir ou annuler une alerte pour être notifié quand l'utilisateur enverra un nouveau message"),
    );
    dict.messages.insert(
        "slash_command.alert_cancel_argument".to_string(),
        DictionaryMessage::new("Annuler l'alerte"),
    );
    dict.messages.insert(
        "slash_command.force_close_command_description".to_string(),
        DictionaryMessage::new("Forcer la fermeture d'un ticket de support dont l'utilisateur n'est plus membre du serveur"),
    );
    dict.messages.insert(
        "slash_command.reply_command_description".to_string(),
        DictionaryMessage::new("Répondre à un utilisateur dans son ticket de support"),
    );
    dict.messages.insert(
        "slash_command.reply_message_argument_description".to_string(),
        DictionaryMessage::new("Le message à envoyer à l'utilisateur"),
    );
    dict.messages.insert(
        "slash_command.reply_snippet_argument_description".to_string(),
        DictionaryMessage::new("Utiliser un snippet au lieu d'écrire un message"),
    );
    dict.messages.insert(
        "slash_command.reply_attachment_argument_description".to_string(),
        DictionaryMessage::new("Une pièce jointe à envoyer avec le message"),
    );
    dict.messages.insert(
        "slash_command.reply_anonymous_argument_description".to_string(),
        DictionaryMessage::new("Envoyer la réponse anonymement"),
    );
    dict.messages.insert(
        "slash_command.delete_command_description".to_string(),
        DictionaryMessage::new("Supprimer un message envoyé dans un ticket de support"),
    );
    dict.messages.insert(
        "slash_command.delete_message_id_argument_description".to_string(),
        DictionaryMessage::new("Le numéro du message à supprimer. Vous pouvez trouver le numéro en regardant le footer du message."),
    );
    dict.messages.insert(
        "slash_command.recover_command_description".to_string(),
        DictionaryMessage::new("Récupérer les messages manqués pendant la période d'indisponibilité du rustmail (automatique)"),
    );
    dict.messages.insert(
        "slash_command.help_command_description".to_string(),
        DictionaryMessage::new("Afficher le message d'aide"),
    );
    dict.messages.insert(
        "reminder.registered_without_content".to_string(),
        DictionaryMessage::new("⏰ Rappel programmé pour **{time}** ({remaining_time})"),
    );
    dict.messages.insert(
        "reminder.registered_with_content".to_string(),
        DictionaryMessage::new(
            "⏰ Rappel programmé pour **{time}** ({remaining_time})\n\n{content}",
        ),
    );
    dict.messages.insert(
        "reminder.registered_without_content_roles".to_string(),
        DictionaryMessage::new(
            "⏰ Rappel pour {roles} programmé pour **{time}** ({remaining_time})",
        ),
    );
    dict.messages.insert(
        "reminder.registered_with_content_roles".to_string(),
        DictionaryMessage::new(
            "⏰ Rappel pour {roles} programmé pour **{time}** ({remaining_time})\n\n{content}",
        ),
    );
    dict.messages.insert(
        "reminder.show_with_content".to_string(),
        DictionaryMessage::new("⏰ Rappel <@{user}> : \n\n{content} !"),
    );
    dict.messages.insert(
        "reminder.show_without_content".to_string(),
        DictionaryMessage::new("⏰ Rappel <@{user}> !"),
    );
    dict.messages.insert(
        "reminder.show_with_content_roles".to_string(),
        DictionaryMessage::new("⏰ Rappel pour {roles} : \n\n{content} !"),
    );
    dict.messages.insert(
        "reminder.show_without_content_roles".to_string(),
        DictionaryMessage::new("⏰ Rappel pour {roles} !"),
    );
    dict.messages.insert(
        "reminder.already_complete".to_string(),
        DictionaryMessage::new("Le rappel **#{reminder_id}** a déjà été complété."),
    );
    dict.messages.insert(
        "slash_command.add_reminder_command_description".to_string(),
        DictionaryMessage::new("Ajouter un rappel pour vous-même"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_time_argument_description".to_string(),
        DictionaryMessage::new("L'heure à laquelle vous souhaitez être rappelé (format HH:MM)"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_content_argument_description".to_string(),
        DictionaryMessage::new("Le contenu du rappel (optionnel)"),
    );
    dict.messages.insert(
        "remove_reminder.confirmation".to_string(),
        DictionaryMessage::new("Le rappel **#{id}** a été supprimé avec succès."),
    );
    dict.messages.insert(
        "slash_command.remove_reminder_command_description".to_string(),
        DictionaryMessage::new("Supprimer un rappel que vous avez créé."),
    );
    dict.messages.insert(
        "slash_command.remove_reminder_id_argument".to_string(),
        DictionaryMessage::new("L'ID du rappel à supprimer."),
    );
    dict.messages.insert(
        "reminder_subscription.subscribed".to_string(),
        DictionaryMessage::new("Vous êtes maintenant inscrit aux rappels du rôle **{role}**."),
    );
    dict.messages.insert(
        "reminder_subscription.unsubscribed".to_string(),
        DictionaryMessage::new("Vous êtes maintenant désinscrit des rappels du rôle **{role}**."),
    );
    dict.messages.insert(
        "reminder_subscription.already_subscribed".to_string(),
        DictionaryMessage::new("Vous êtes déjà inscrit aux rappels du rôle **{role}**."),
    );
    dict.messages.insert(
        "reminder_subscription.already_unsubscribed".to_string(),
        DictionaryMessage::new("Vous êtes déjà désinscrit des rappels du rôle **{role}**."),
    );
    dict.messages.insert(
        "reminder_subscription.role_required".to_string(),
        DictionaryMessage::new("Vous devez avoir le rôle **{role}** pour effectuer cette action."),
    );
    dict.messages.insert(
        "reminder_subscription.role_not_found".to_string(),
        DictionaryMessage::new("Le rôle **{role}** n'existe pas sur ce serveur."),
    );
    dict.messages.insert(
        "reminder_subscription.missing_role".to_string(),
        DictionaryMessage::new("Veuillez spécifier un rôle. Usage : `{prefix}rem subscribe <role>` ou `{prefix}rem unsubscribe <role>`"),
    );
    dict.messages.insert(
        "slash_command.reminder_subscribe_description".to_string(),
        DictionaryMessage::new("S'inscrire ou se désinscrire des rappels d'un rôle"),
    );
    dict.messages.insert(
        "slash_command.reminder_action_argument".to_string(),
        DictionaryMessage::new("Action à effectuer (subscribe/unsubscribe)"),
    );
    dict.messages.insert(
        "slash_command.reminder_role_argument".to_string(),
        DictionaryMessage::new("Le rôle pour lequel modifier l'inscription"),
    );
    dict.messages.insert(
        "slash_command.add_reminder_roles_argument_description".to_string(),
        DictionaryMessage::new("Rôles à cibler (séparés par des virgules, ex: dev,mod)"),
    );
    dict.messages.insert(
        "help.reminder_subscription".to_string(),
        DictionaryMessage::new("Gérer vos inscriptions aux rappels par rôle. `!rem subscribe <role>` pour s'inscrire, `!rem unsubscribe <role>` pour se désinscrire. Vous devez avoir le rôle pour modifier votre inscription."),
    );
    dict.messages.insert(
        "logs_command.next".to_string(),
        DictionaryMessage::new("Suivant"),
    );
    dict.messages.insert(
        "logs_command.prev".to_string(),
        DictionaryMessage::new("Précédent"),
    );
    dict.messages.insert(
        "slash_commands.logs_command_description".to_string(),
        DictionaryMessage::new("Afficher les logs d'un utilisateur"),
    );
    dict.messages.insert(
        "slash_commands.logs_id_argument_description".to_string(),
        DictionaryMessage::new("L'ID de l'utilisateur dont vous souhaitez voir les logs"),
    );
    dict.messages.insert(
        "slash_commands.no_logs_found".to_string(),
        DictionaryMessage::new("Aucun log trouvé pour cet utilisateur."),
    );
    dict.messages.insert(
        "new_thread.show_logs".to_string(),
        DictionaryMessage::new("Cet utilisateur a **{logs_count}** ancien(s) ticket(s) rustmail. Utilisez `{prefix}logs` pour les voir."),
    );
    dict.messages.insert(
        "reminder.reminder_already_exists".to_string(),
        DictionaryMessage::new("Vous avez déjà un rappel programmé à cette heure."),
    );
    dict.messages.insert(
        "help.add_reminder".to_string(),
        DictionaryMessage::new("Configure un rappel à une heure spécifique. Usage : `!rem <HH:MM> [contenu]` pour un rappel personnel, ou `!rem <HH:MM> @role1,@role2 [contenu]` pour cibler des rôles (ex: `!rem 14:30 @dev,@mod Réunion`). Vous pouvez aussi utiliser les mentions Discord. Si l'heure est déjà passée, le rappel sera programmé pour demain. Utilisez `!rem subscribe <rôle>` ou `!rem unsubscribe <rôle>` pour gérer vos notifications."),
    );
    dict.messages.insert(
        "help.add_staff".to_string(),
        DictionaryMessage::new("Ajoute un membre du staff à un ticket. Pour ce faire, faites `!addmod <id du staff>` ou `!am <id du staff>` dans un ticket."),
    );
    dict.messages.insert(
        "help.alert".to_string(),
        DictionaryMessage::new("Configure une alerte pour un utilisateur lorsqu'il envoie un nouveau message. Pour programmer une alerte, faites `!alert` dans un ticket. Pour annuler une alerte programmée, faites `!alert cancel` ou `!alert c`."),
    );
    dict.messages.insert(
        "help.close".to_string(),
        DictionaryMessage::new("Ferme le ticket actuel. Vous pouvez spécifier un délai avant la fermeture en faisant : `!close <durée (d, h, m ou s)>` ou `!c <durée (d, h, m ou s)>`. Vous pouvez ajouter l'option `--silent` ou `-s` pour ne pas avertir l'utilisateur que son ticket a été fermé. Vous pouvez également annuler une fermeture programmée en faisant `!close --cancel`, `!close -c` ou `!close cancel`."),
    );
    dict.messages.insert(
        "help.delete".to_string(),
        DictionaryMessage::new("Supprime un message spécifique dans un fil de discussion. Pour ce faire, faites `!delete <id du message>` dans un ticket."),
    );
    dict.messages.insert(
        "help.edit".to_string(),
        DictionaryMessage::new("Modifie le contenu d'un message précédemment envoyé dans un ticket. Pour modifier un message, faites `!edit <id du message> <nouveau contenu>` ou `!e <id du message> <nouveau contenu>` dans un ticket."),
    );
    dict.messages.insert(
        "help.force_close".to_string(),
        DictionaryMessage::new("Ferme un ticket lorsqu'une erreur empêche la fermeture normale. Cette commande disparaîtra dans les prochaines versions. Pour forcer la fermeture d'un ticket, faites `!force_close` ou `!fc` dans un ticket."),
    );
    dict.messages.insert(
        "help.category".to_string(),
        DictionaryMessage::new(
            "Gère les catégories de tickets que les utilisateurs peuvent sélectionner pour diriger leurs demandes.\n\n\
            **Sous-commandes :**\n\
            `create <discord_category_id> <nom> [| description] [| emoji]` - Crée une nouvelle catégorie.\n\
            `list` - Affiche la liste des catégories configurées.\n\
            `rename <ancien_nom> <nouveau_nom>` - Renomme une catégorie existante.\n\
            `move <nom> <position>` - Change la position d'une catégorie.\n\
            `delete <nom>` ou `remove <nom>` - Supprime une catégorie.\n\
            `enable <nom>` - Active une catégorie spécifique.\n\
            `disable <nom>` - Désactive une catégorie spécifique.\n\
            `on` - Active globalement la fonctionnalité de sélection de catégorie.\n\
            `off` - Désactive globalement la fonctionnalité de sélection de catégorie.\n\
            `timeout <secondes>` - Définit le délai (en secondes) dont disposent les utilisateurs pour choisir une catégorie avant le choix par défaut."
        ),
    );
    dict.messages.insert(
        "help.help".to_string(),
        DictionaryMessage::new("Affiche une liste de toutes les commandes disponibles avec une brève description. Pour afficher le message d'aide, faites `!help`. Si vous souhaitez obtenir de l'aide sur une commande spécifique, faites `!help <nom_de_la_commande>`."),
    );
    dict.messages.insert(
        "help.id".to_string(),
        DictionaryMessage::new("Affiche l'identifiant Discord de l'utilisateur associé au ticket. Pour afficher l'ID de l'utilisateur, faites `!id` dans un ticket."),
    );
    dict.messages.insert(
        "help.logs".to_string(),
        DictionaryMessage::new("Récupère les logs de tous les anciens tickets d'un utilisateur. Vous pouvez soit spécifier un identifiant Discord (`!logs <discord_id>`), soit exécuter la commande dans un ticket pour obtenir les logs de ce ticket."),
    );
    dict.messages.insert(
        "help.move".to_string(),
        DictionaryMessage::new("Déplace le ticket actuel vers une autre catégorie. Pour déplacer un ticket, faites `!move <catégorie>` ou `!mv <catégorie>` dans le ticket."),
    );
    dict.messages.insert(
        "help.new_thread".to_string(),
        DictionaryMessage::new("Crée un nouveau ticket pour un utilisateur spécifié. Pour créer un ticket, faites `!new_thread <utilisateur>` ou `!nt <utilisateur>`."),
    );
    dict.messages.insert(
        "help.recover".to_string(),
        DictionaryMessage::new("Lance le processus de récupération des messages manquants dans les tickets Modmail. Ce processus est automatique, mais cette commande permet de le relancer manuellement si nécessaire. Pour cela, faites `!recover`."),
    );
    dict.messages.insert(
        "help.remove_reminder".to_string(),
        DictionaryMessage::new("Supprime un rappel que vous avez précédemment configuré. Pour supprimer un rappel, faites `!unremind <id>` ou `!urem <id>`."),
    );
    dict.messages.insert(
        "help.remove_staff".to_string(),
        DictionaryMessage::new("Retire un membre du staff du ticket actuel. Pour retirer un staff, faites `!delmod <utilisateur>` ou `!dm <utilisateur>` dans le ticket."),
    );
    dict.messages.insert(
        "help.reply".to_string(),
        DictionaryMessage::new("Répond dans un ticket. Pour répondre, faites `!reply <message> [attachment]` ou `!r <message> [attachment]` dans le ticket. Si vous souhaitez répondre anonymement, utilisez la commande `!anonreply`, `!ar`, ou spécifiez l'option dans la commande slash `reply`."),
    );
    dict.messages.insert(
        "help.message".to_string(),
        DictionaryMessage::new("## Commandes :\n\n**Toutes les commandes** disponibles sont également utilisables via des **__commandes slash__** portant le __même nom__.\n\nSi vous souhaitez obtenir de l'aide sur une commande spécifique, faites `!help <nom_de_la_commande>`.\n\n")
    );
    dict.messages.insert(
        "help.take".to_string(),
        DictionaryMessage::new("Permet de prendre en charge un ticket en remplaçant le nom de celui-ci par le vôtre. Pour prendre en charge un ticket, faites `!take` dans le ticket."),
    );
    dict.messages.insert(
        "help.release".to_string(),
        DictionaryMessage::new("Permet de ne plus prendre en charge un ticket pris en charge via la commande `take`. Pour libérer un ticket, faites `!release` dans le ticket."),
    );
    dict.messages.insert(
        "help.ping".to_string(),
        DictionaryMessage::new("Permet d'afficher la latence actuelle du bot."),
    );
    dict.messages.insert(
        "add_reminder.helper".to_string(),
        DictionaryMessage::new("Format incorrect. Utilisation : `{prefix}remind ou {prefix}rem <HH:MM> [contenu du rappel]`"),
    );
    dict.messages.insert(
        "take.ticket_already_taken".to_string(),
        DictionaryMessage::new("Vous avez déjà pris en charge ce ticket."),
    );
    dict.messages.insert(
        "take.confirmation".to_string(),
        DictionaryMessage::new("Le ticket est maintenant pris en charge par {staff}.\nA cause de **l’API de Discord**, le changement de nom du salon peut prendre jusqu’à **10 minutes**."),
    );
    dict.messages.insert(
        "take.confirmation_rate_limited".to_string(),
        DictionaryMessage::new("Le ticket est maintenant pris en charge par {staff}.\n⚠️ **L’API de Discord** impose une limite de **2** modifications de nom de salon toutes les **10 minutes**. Le changement sera **__automatiquement__** appliqué une fois le délai écoulé."),
    );
    dict.messages.insert(
        "take.timeout".to_string(),
        DictionaryMessage::new("⚠️ **L’API de Discord** impose une limite de **2** changements de salon toutes les **10 minutes**. L’action sera appliquée **__automatiquement__** dès que le délai sera écoulé."),
    );
    dict.messages.insert(
        "slash_command.take_command_description".to_string(),
        DictionaryMessage::new("Prendre en charge le ticket actuel"),
    );
    dict.messages.insert(
        "slash_command.release_command_description".to_string(),
        DictionaryMessage::new("Ne plus prendre en charge le ticket actuel"),
    );
    dict.messages.insert(
        "release.ticket_already_taken".to_string(),
        DictionaryMessage::new("Le ticket n'est pris en charge par personne."),
    );
    dict.messages.insert(
        "release.confirmation".to_string(),
        DictionaryMessage::new("Le ticket n’est plus pris en charge par {staff}.\nA cause de **l’API de Discord**, le changement de nom du salon peut prendre jusqu’à **10 minutes**."),
    );
    dict.messages.insert(
        "release.confirmation_rate_limited".to_string(),
        DictionaryMessage::new("Le ticket n’est plus pris en charge par {staff}.\n⚠️ **L’API de Discord** impose une limite de **2** modifications de nom de salon toutes les **10 minutes**. Le changement sera **__automatiquement__** appliqué une fois le délai écoulé."),
    );
    dict.messages.insert(
        "help.rename".to_string(),
        DictionaryMessage::new("Renomme le ticket en ajoutant un libellé personnalisé. Le nom du joueur et le statut restent visibles. Utilisez `!rename <libellé>` ou `!rn <libellé>`. Utilisez `!rename` sans argument pour supprimer le libellé."),
    );
    dict.messages.insert(
        "rename.confirmation".to_string(),
        DictionaryMessage::new("Le ticket a été renommé en **{label}**.\nA cause de **l’API de Discord**, le changement de nom du salon peut prendre jusqu’à **10 minutes**."),
    );
    dict.messages.insert(
        "rename.confirmation_rate_limited".to_string(),
        DictionaryMessage::new("Le ticket a été renommé en **{label}**.\n⚠️ **L’API de Discord** impose une limite de **2** modifications de nom de salon toutes les **10 minutes**. Le changement sera **__automatiquement__** appliqué une fois le délai écoulé."),
    );
    dict.messages.insert(
        "rename.cleared".to_string(),
        DictionaryMessage::new("Le libellé du ticket a été supprimé.\nA cause de **l’API de Discord**, le changement de nom du salon peut prendre jusqu’à **10 minutes**."),
    );
    dict.messages.insert(
        "rename.cleared_rate_limited".to_string(),
        DictionaryMessage::new("Le libellé du ticket a été supprimé.\n⚠️ **L’API de Discord** impose une limite de **2** modifications de nom de salon toutes les **10 minutes**. Le changement sera **__automatiquement__** appliqué une fois le délai écoulé."),
    );
    dict.messages.insert(
        "slash_command.rename_command_description".to_string(),
        DictionaryMessage::new("Renomme le ticket avec un libellé personnalisé."),
    );
    dict.messages.insert(
        "slash_command.rename_label_option".to_string(),
        DictionaryMessage::new("Le libellé à afficher (laisser vide pour supprimer)."),
    );
    dict.messages.insert(
        "slash_command.help_command_argument_desc".to_string(),
        DictionaryMessage::new("Le nom de la commande pour laquelle vous souhaitez de l'aide"),
    );
    dict.messages.insert(
        "slash_command.ping_command_desc".to_string(),
        DictionaryMessage::new("Afficher la latence actuelle du bot."),
    );
    dict.messages.insert(
        "slash_command.ping_command".to_string(),
        DictionaryMessage::new("## Latence\n\nLatence Gateway : **{gateway_latency}** ms.\nLatence REST minimale (GET /gateway) : **{api_latency}** ms.\nLatence REST (envoi d'un message) : **{message_latency}** ms."),
    );

    dict.messages.insert(
        "slash_command.snippet_command_description".to_string(),
        DictionaryMessage::new("Gérer les snippets/modèles de messages"),
    );
    dict.messages.insert(
        "slash_command.snippet_command_help".to_string(),
        DictionaryMessage::new(
            "Gérer les snippets/modèles de messages\n\n\
            **Sous-commandes :**\n\
            • `/snippet create <clé> <contenu>` - Créer un nouveau snippet\n\
            • `/snippet list` - Lister tous les snippets disponibles\n\
            • `/snippet show <clé>` - Afficher le contenu d'un snippet spécifique\n\
            • `/snippet edit <clé> <contenu>` - Modifier un snippet existant\n\
            • `/snippet delete <clé>` - Supprimer un snippet\n\
            • `/snippet use <clé>` - Utiliser un snippet pour répondre\n\n\
            **Utilisation rapide :**\n\
            • Commande slash : `/snippet use <clé>` ou `/reply snippet:<clé>`\n\
            • Commande texte : `!snippet <clé>` ou `!reply {{clé}}`",
        ),
    );
    dict.messages.insert(
        "slash_command.snippet_create_description".to_string(),
        DictionaryMessage::new("Créer un nouveau snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_list_description".to_string(),
        DictionaryMessage::new("Lister tous les snippets"),
    );
    dict.messages.insert(
        "slash_command.snippet_show_description".to_string(),
        DictionaryMessage::new("Afficher un snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_edit_description".to_string(),
        DictionaryMessage::new("Modifier un snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_delete_description".to_string(),
        DictionaryMessage::new("Supprimer un snippet"),
    );
    dict.messages.insert(
        "slash_command.snippet_use_description".to_string(),
        DictionaryMessage::new("Utiliser un snippet pour répondre dans un ticket"),
    );
    dict.messages.insert(
        "slash_command.snippet_key_argument".to_string(),
        DictionaryMessage::new("Clé du snippet (alphanumériques, tirets, underscores)"),
    );
    dict.messages.insert(
        "slash_command.snippet_content_argument".to_string(),
        DictionaryMessage::new("Contenu du snippet (max 4000 caractères)"),
    );
    dict.messages.insert(
        "slash_command.reply_snippet_argument".to_string(),
        DictionaryMessage::new("Utiliser un snippet au lieu de taper un message"),
    );

    dict.messages.insert(
        "snippet.invalid_key_format".to_string(),
        DictionaryMessage::new("La clé du snippet doit contenir uniquement des caractères alphanumériques, des tirets et des underscores."),
    );
    dict.messages.insert(
        "snippet.content_too_long".to_string(),
        DictionaryMessage::new("Le contenu du snippet doit faire 4000 caractères ou moins."),
    );
    dict.messages.insert(
        "snippet.created".to_string(),
        DictionaryMessage::new("Snippet `{key}` créé avec succès !"),
    );
    dict.messages.insert(
        "snippet.creation_failed".to_string(),
        DictionaryMessage::new("Échec de la création du snippet : {error}"),
    );
    dict.messages.insert(
        "snippet.already_exist".to_string(),
        DictionaryMessage::new("Le snippet `{key}` existe déjà."),
    );
    dict.messages.insert(
        "snippet.updated".to_string(),
        DictionaryMessage::new("Snippet `{key}` modifié avec succès !"),
    );
    dict.messages.insert(
        "snippet.update_failed".to_string(),
        DictionaryMessage::new("Échec de la modification du snippet : {error}"),
    );
    dict.messages.insert(
        "snippet.deleted".to_string(),
        DictionaryMessage::new("Snippet `{key}` supprimé avec succès !"),
    );
    dict.messages.insert(
        "snippet.deletion_failed".to_string(),
        DictionaryMessage::new("Échec de la suppression du snippet : {error}"),
    );
    dict.messages.insert(
        "snippet.not_found".to_string(),
        DictionaryMessage::new("Snippet `{key}` introuvable."),
    );
    dict.messages.insert(
        "snippet.list_empty".to_string(),
        DictionaryMessage::new("Aucun snippet trouvé."),
    );
    dict.messages.insert(
        "snippet.no_snippets_found".to_string(),
        DictionaryMessage::new("Aucun snippet trouvé."),
    );
    dict.messages.insert(
        "snippet.list_title".to_string(),
        DictionaryMessage::new("📝 Snippets disponibles"),
    );
    dict.messages.insert(
        "snippet.list_more".to_string(),
        DictionaryMessage::new("...et {count} de plus"),
    );
    dict.messages.insert(
        "snippet.show_title".to_string(),
        DictionaryMessage::new("📝 Snippet : {key}"),
    );
    dict.messages.insert(
        "snippet.created_by".to_string(),
        DictionaryMessage::new("Créé par"),
    );
    dict.messages.insert(
        "snippet.created_at".to_string(),
        DictionaryMessage::new("Créé le"),
    );
    dict.messages.insert(
        "snippet.unknown_subcommand".to_string(),
        DictionaryMessage::new("Sous-commande inconnue"),
    );
    dict.messages.insert(
        "snippet.text_usage".to_string(),
        DictionaryMessage::new("Usage : `!snippet <create|list|show|edit|delete> [args]`"),
    );
    dict.messages.insert(
        "snippet.text_create_usage".to_string(),
        DictionaryMessage::new("Usage : `!snippet create <clé> <contenu>`"),
    );
    dict.messages.insert(
        "snippet.text_show_usage".to_string(),
        DictionaryMessage::new("Usage : `!snippet show <clé>`"),
    );
    dict.messages.insert(
        "snippet.text_edit_usage".to_string(),
        DictionaryMessage::new("Usage : `!snippet edit <clé> <contenu>`"),
    );
    dict.messages.insert(
        "snippet.text_delete_usage".to_string(),
        DictionaryMessage::new("Usage : `!snippet delete <clé>`"),
    );
    dict.messages.insert(
        "snippet.unknown_text_subcommand".to_string(),
        DictionaryMessage::new(
            "Sous-commande inconnue. Utilisez : `create`, `list`, `show`, `edit`, ou `delete`",
        ),
    );
    dict.messages.insert(
        "snippet.used".to_string(),
        DictionaryMessage::new("Snippet '**{key}**' utilisé avec succès !"),
    );
    dict.messages.insert(
        "audit_log.reason".to_string(),
        DictionaryMessage::new("Raison"),
    );
    dict.messages.insert(
        "audit_log.channel".to_string(),
        DictionaryMessage::new("Salon"),
    );
    dict.messages.insert(
        "audit_log.target".to_string(),
        DictionaryMessage::new("Cible"),
    );
    dict.messages.insert(
        "audit_log.unknown".to_string(),
        DictionaryMessage::new("Inconnu"),
    );
    dict.messages.insert(
        "audit_log.unknown_action".to_string(),
        DictionaryMessage::new("Action Inconnue (code: {code})"),
    );
    dict.messages.insert(
        "audit_log.member.kick".to_string(),
        DictionaryMessage::new("Membre Expulsé"),
    );
    dict.messages.insert(
        "audit_log.member.prune".to_string(),
        DictionaryMessage::new("Membres Purgés"),
    );
    dict.messages.insert(
        "audit_log.member.ban_add".to_string(),
        DictionaryMessage::new("Membre Banni"),
    );
    dict.messages.insert(
        "audit_log.member.ban_remove".to_string(),
        DictionaryMessage::new("Membre Débanni"),
    );
    dict.messages.insert(
        "audit_log.member.update".to_string(),
        DictionaryMessage::new("Membre Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.member.role_update".to_string(),
        DictionaryMessage::new("Rôles du Membre Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.member.move".to_string(),
        DictionaryMessage::new("Membre Déplacé dans un Salon Vocal"),
    );
    dict.messages.insert(
        "audit_log.member.disconnect".to_string(),
        DictionaryMessage::new("Membre Déconnecté d'un Salon Vocal"),
    );
    dict.messages.insert(
        "audit_log.member.member_move".to_string(),
        DictionaryMessage::new("Membre Déplacé"),
    );
    dict.messages.insert(
        "audit_log.member.member_disconnect".to_string(),
        DictionaryMessage::new("Membre Déconnecté"),
    );
    dict.messages.insert(
        "audit_log.member.bot_add".to_string(),
        DictionaryMessage::new("Bot Ajouté"),
    );
    dict.messages.insert(
        "audit_log.member.unknown".to_string(),
        DictionaryMessage::new("Action Membre Inconnue"),
    );
    dict.messages.insert(
        "audit_log.member.pruned_count".to_string(),
        DictionaryMessage::new("{count} membres purgés"),
    );
    dict.messages.insert(
        "audit_log.member.messages_deleted".to_string(),
        DictionaryMessage::new("Messages supprimés : {days} jours"),
    );
    dict.messages.insert(
        "audit_log.member.moved_to".to_string(),
        DictionaryMessage::new("Déplacé vers {channel} ({count} membres)"),
    );
    dict.messages.insert(
        "audit_log.member.disconnected_count".to_string(),
        DictionaryMessage::new("{count} membres déconnectés"),
    );
    dict.messages.insert(
        "audit_log.channel.create".to_string(),
        DictionaryMessage::new("Salon Créé"),
    );
    dict.messages.insert(
        "audit_log.channel.update".to_string(),
        DictionaryMessage::new("Salon Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.channel.delete".to_string(),
        DictionaryMessage::new("Salon Supprimé"),
    );
    dict.messages.insert(
        "audit_log.channel.unknown".to_string(),
        DictionaryMessage::new("Action Salon Inconnue"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.create".to_string(),
        DictionaryMessage::new("Permission Créée"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.update".to_string(),
        DictionaryMessage::new("Permission Mise à Jour"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.delete".to_string(),
        DictionaryMessage::new("Permission Supprimée"),
    );
    dict.messages.insert(
        "audit_log.channel_overwrite.unknown".to_string(),
        DictionaryMessage::new("Action Permission Inconnue"),
    );
    dict.messages.insert(
        "audit_log.role.create".to_string(),
        DictionaryMessage::new("Rôle Créé"),
    );
    dict.messages.insert(
        "audit_log.role.update".to_string(),
        DictionaryMessage::new("Rôle Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.role.delete".to_string(),
        DictionaryMessage::new("Rôle Supprimé"),
    );
    dict.messages.insert(
        "audit_log.role.unknown".to_string(),
        DictionaryMessage::new("Action Rôle Inconnue"),
    );
    dict.messages.insert(
        "audit_log.invite.create".to_string(),
        DictionaryMessage::new("Invitation Créée"),
    );
    dict.messages.insert(
        "audit_log.invite.update".to_string(),
        DictionaryMessage::new("Invitation Mise à Jour"),
    );
    dict.messages.insert(
        "audit_log.invite.delete".to_string(),
        DictionaryMessage::new("Invitation Supprimée"),
    );
    dict.messages.insert(
        "audit_log.invite.unknown".to_string(),
        DictionaryMessage::new("Action Invitation Inconnue"),
    );
    dict.messages.insert(
        "audit_log.webhook.create".to_string(),
        DictionaryMessage::new("Webhook Créé"),
    );
    dict.messages.insert(
        "audit_log.webhook.update".to_string(),
        DictionaryMessage::new("Webhook Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.webhook.delete".to_string(),
        DictionaryMessage::new("Webhook Supprimé"),
    );
    dict.messages.insert(
        "audit_log.webhook.unknown".to_string(),
        DictionaryMessage::new("Action Webhook Inconnue"),
    );
    dict.messages.insert(
        "audit_log.emoji.create".to_string(),
        DictionaryMessage::new("Emoji Créé"),
    );
    dict.messages.insert(
        "audit_log.emoji.update".to_string(),
        DictionaryMessage::new("Emoji Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.emoji.delete".to_string(),
        DictionaryMessage::new("Emoji Supprimé"),
    );
    dict.messages.insert(
        "audit_log.emoji.unknown".to_string(),
        DictionaryMessage::new("Action Emoji Inconnue"),
    );
    dict.messages.insert(
        "audit_log.message.delete".to_string(),
        DictionaryMessage::new("Message Supprimé"),
    );
    dict.messages.insert(
        "audit_log.message.bulk_delete".to_string(),
        DictionaryMessage::new("Messages Supprimés en Masse"),
    );
    dict.messages.insert(
        "audit_log.message.pin".to_string(),
        DictionaryMessage::new("Message Épinglé"),
    );
    dict.messages.insert(
        "audit_log.message.unpin".to_string(),
        DictionaryMessage::new("Message Désépinglé"),
    );
    dict.messages.insert(
        "audit_log.message.unknown".to_string(),
        DictionaryMessage::new("Action Message Inconnue"),
    );
    dict.messages.insert(
        "audit_log.guild.update".to_string(),
        DictionaryMessage::new("Serveur Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.integration.create".to_string(),
        DictionaryMessage::new("Intégration Créée"),
    );
    dict.messages.insert(
        "audit_log.integration.update".to_string(),
        DictionaryMessage::new("Intégration Mise à Jour"),
    );
    dict.messages.insert(
        "audit_log.integration.delete".to_string(),
        DictionaryMessage::new("Intégration Supprimée"),
    );
    dict.messages.insert(
        "audit_log.integration.unknown".to_string(),
        DictionaryMessage::new("Action Intégration Inconnue"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.create".to_string(),
        DictionaryMessage::new("Instance Stage Créée"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.update".to_string(),
        DictionaryMessage::new("Instance Stage Mise à Jour"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.delete".to_string(),
        DictionaryMessage::new("Instance Stage Supprimée"),
    );
    dict.messages.insert(
        "audit_log.stage_instance.unknown".to_string(),
        DictionaryMessage::new("Action Stage Inconnue"),
    );
    dict.messages.insert(
        "audit_log.sticker.create".to_string(),
        DictionaryMessage::new("Sticker Créé"),
    );
    dict.messages.insert(
        "audit_log.sticker.update".to_string(),
        DictionaryMessage::new("Sticker Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.sticker.delete".to_string(),
        DictionaryMessage::new("Sticker Supprimé"),
    );
    dict.messages.insert(
        "audit_log.sticker.unknown".to_string(),
        DictionaryMessage::new("Action Sticker Inconnue"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.create".to_string(),
        DictionaryMessage::new("Événement Programmé Créé"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.update".to_string(),
        DictionaryMessage::new("Événement Programmé Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.delete".to_string(),
        DictionaryMessage::new("Événement Programmé Supprimé"),
    );
    dict.messages.insert(
        "audit_log.scheduled_event.unknown".to_string(),
        DictionaryMessage::new("Action Événement Inconnue"),
    );
    dict.messages.insert(
        "audit_log.thread.create".to_string(),
        DictionaryMessage::new("Fil de Discussion Créé"),
    );
    dict.messages.insert(
        "audit_log.thread.update".to_string(),
        DictionaryMessage::new("Fil de Discussion Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.thread.delete".to_string(),
        DictionaryMessage::new("Fil de Discussion Supprimé"),
    );
    dict.messages.insert(
        "audit_log.thread.unknown".to_string(),
        DictionaryMessage::new("Action Thread Inconnue"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_create".to_string(),
        DictionaryMessage::new("Règle AutoMod Créée"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_update".to_string(),
        DictionaryMessage::new("Règle AutoMod Mise à Jour"),
    );
    dict.messages.insert(
        "audit_log.automod.rule_delete".to_string(),
        DictionaryMessage::new("Règle AutoMod Supprimée"),
    );
    dict.messages.insert(
        "audit_log.automod.block_message".to_string(),
        DictionaryMessage::new("Message Bloqué par AutoMod"),
    );
    dict.messages.insert(
        "audit_log.automod.send_alert_message".to_string(),
        DictionaryMessage::new("Alerte Envoyée par AutoMod"),
    );
    dict.messages.insert(
        "audit_log.automod.user_communication_disabled".to_string(),
        DictionaryMessage::new("Utilisateur en Timeout par AutoMod"),
    );
    dict.messages.insert(
        "audit_log.automod.unknown".to_string(),
        DictionaryMessage::new("Action AutoMod Inconnue"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.request_created".to_string(),
        DictionaryMessage::new("Demande de Monétisation Créée"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.terms_accepted".to_string(),
        DictionaryMessage::new("Conditions de Monétisation Acceptées"),
    );
    dict.messages.insert(
        "audit_log.creator_monetization.unknown".to_string(),
        DictionaryMessage::new("Action Monétisation Inconnue"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.update".to_string(),
        DictionaryMessage::new("Statut Vocal Mis à Jour"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.delete".to_string(),
        DictionaryMessage::new("Statut Vocal Supprimé"),
    );
    dict.messages.insert(
        "audit_log.voice_channel_status.unknown".to_string(),
        DictionaryMessage::new("Action Statut Vocal Inconnue"),
    );
    dict.messages.insert(
        "audit_log.change.afk_channel".to_string(),
        DictionaryMessage::new("Salon AFK"),
    );
    dict.messages.insert(
        "audit_log.change.afk_timeout".to_string(),
        DictionaryMessage::new("Délai AFK"),
    );
    dict.messages.insert(
        "audit_log.change.permissions_allow".to_string(),
        DictionaryMessage::new("Permissions Autorisées"),
    );
    dict.messages.insert(
        "audit_log.change.application".to_string(),
        DictionaryMessage::new("ID Application"),
    );
    dict.messages.insert(
        "audit_log.change.archived".to_string(),
        DictionaryMessage::new("Archivé"),
    );
    dict.messages.insert(
        "audit_log.change.asset".to_string(),
        DictionaryMessage::new("Asset"),
    );
    dict.messages.insert(
        "audit_log.change.auto_archive_duration".to_string(),
        DictionaryMessage::new("Durée d'Archivage Auto"),
    );
    dict.messages.insert(
        "audit_log.change.available".to_string(),
        DictionaryMessage::new("Disponible"),
    );
    dict.messages.insert(
        "audit_log.change.avatar".to_string(),
        DictionaryMessage::new("Avatar"),
    );
    dict.messages.insert(
        "audit_log.change.banner".to_string(),
        DictionaryMessage::new("Bannière"),
    );
    dict.messages.insert(
        "audit_log.change.bitrate".to_string(),
        DictionaryMessage::new("Débit"),
    );
    dict.messages.insert(
        "audit_log.change.channel".to_string(),
        DictionaryMessage::new("Salon"),
    );
    dict.messages.insert(
        "audit_log.change.invite_code".to_string(),
        DictionaryMessage::new("Code d'Invitation"),
    );
    dict.messages.insert(
        "audit_log.change.color".to_string(),
        DictionaryMessage::new("Couleur"),
    );
    dict.messages.insert(
        "audit_log.change.timeout".to_string(),
        DictionaryMessage::new("Timeout"),
    );
    dict.messages.insert(
        "audit_log.change.deaf".to_string(),
        DictionaryMessage::new("Sourd"),
    );
    dict.messages.insert(
        "audit_log.change.default_auto_archive".to_string(),
        DictionaryMessage::new("Archivage Auto par Défaut"),
    );
    dict.messages.insert(
        "audit_log.change.default_notifications".to_string(),
        DictionaryMessage::new("Notifications par Défaut"),
    );
    dict.messages.insert(
        "audit_log.change.permissions_deny".to_string(),
        DictionaryMessage::new("Permissions Refusées"),
    );
    dict.messages.insert(
        "audit_log.change.description".to_string(),
        DictionaryMessage::new("Description"),
    );
    dict.messages.insert(
        "audit_log.change.discovery_splash".to_string(),
        DictionaryMessage::new("Image Discovery"),
    );
    dict.messages.insert(
        "audit_log.change.enable_emoticons".to_string(),
        DictionaryMessage::new("Activer Emoticônes"),
    );
    dict.messages.insert(
        "audit_log.change.entity_type".to_string(),
        DictionaryMessage::new("Type d'Entité"),
    );
    dict.messages.insert(
        "audit_log.change.expire_behavior".to_string(),
        DictionaryMessage::new("Comportement d'Expiration"),
    );
    dict.messages.insert(
        "audit_log.change.expire_grace_period".to_string(),
        DictionaryMessage::new("Période de Grâce"),
    );
    dict.messages.insert(
        "audit_log.change.explicit_content_filter".to_string(),
        DictionaryMessage::new("Filtre Contenu Explicite"),
    );
    dict.messages.insert(
        "audit_log.change.format_type".to_string(),
        DictionaryMessage::new("Type de Format"),
    );
    dict.messages.insert(
        "audit_log.change.guild".to_string(),
        DictionaryMessage::new("ID Serveur"),
    );
    dict.messages.insert(
        "audit_log.change.hoist".to_string(),
        DictionaryMessage::new("Affiché Séparément"),
    );
    dict.messages.insert(
        "audit_log.change.icon".to_string(),
        DictionaryMessage::new("Icône"),
    );
    dict.messages.insert(
        "audit_log.change.id".to_string(),
        DictionaryMessage::new("ID"),
    );
    dict.messages.insert(
        "audit_log.change.image".to_string(),
        DictionaryMessage::new("Image"),
    );
    dict.messages.insert(
        "audit_log.change.invitable".to_string(),
        DictionaryMessage::new("Invitable"),
    );
    dict.messages.insert(
        "audit_log.change.inviter".to_string(),
        DictionaryMessage::new("Inviteur"),
    );
    dict.messages.insert(
        "audit_log.change.location".to_string(),
        DictionaryMessage::new("Emplacement"),
    );
    dict.messages.insert(
        "audit_log.change.locked".to_string(),
        DictionaryMessage::new("Verrouillé"),
    );
    dict.messages.insert(
        "audit_log.change.max_age".to_string(),
        DictionaryMessage::new("Durée Max"),
    );
    dict.messages.insert(
        "audit_log.change.max_uses".to_string(),
        DictionaryMessage::new("Utilisations Max"),
    );
    dict.messages.insert(
        "audit_log.change.mentionable".to_string(),
        DictionaryMessage::new("Mentionnable"),
    );
    dict.messages.insert(
        "audit_log.change.mfa_level".to_string(),
        DictionaryMessage::new("Niveau MFA"),
    );
    dict.messages.insert(
        "audit_log.change.mute".to_string(),
        DictionaryMessage::new("Muet"),
    );
    dict.messages.insert(
        "audit_log.change.name".to_string(),
        DictionaryMessage::new("Nom"),
    );
    dict.messages.insert(
        "audit_log.change.nickname".to_string(),
        DictionaryMessage::new("Pseudo"),
    );
    dict.messages.insert(
        "audit_log.change.nsfw".to_string(),
        DictionaryMessage::new("NSFW"),
    );
    dict.messages.insert(
        "audit_log.change.owner".to_string(),
        DictionaryMessage::new("Propriétaire"),
    );
    dict.messages.insert(
        "audit_log.change.permission_overwrites".to_string(),
        DictionaryMessage::new("Permissions Spécifiques"),
    );
    dict.messages.insert(
        "audit_log.change.permissions".to_string(),
        DictionaryMessage::new("Permissions"),
    );
    dict.messages.insert(
        "audit_log.change.position".to_string(),
        DictionaryMessage::new("Position"),
    );
    dict.messages.insert(
        "audit_log.change.preferred_locale".to_string(),
        DictionaryMessage::new("Langue Préférée"),
    );
    dict.messages.insert(
        "audit_log.change.privacy_level".to_string(),
        DictionaryMessage::new("Niveau de Confidentialité"),
    );
    dict.messages.insert(
        "audit_log.change.prune_delete_days".to_string(),
        DictionaryMessage::new("Jours de Purge"),
    );
    dict.messages.insert(
        "audit_log.change.public_updates_channel".to_string(),
        DictionaryMessage::new("Salon Mises à Jour Publiques"),
    );
    dict.messages.insert(
        "audit_log.change.slowmode".to_string(),
        DictionaryMessage::new("Mode Lent"),
    );
    dict.messages.insert(
        "audit_log.change.region".to_string(),
        DictionaryMessage::new("Région"),
    );
    dict.messages.insert(
        "audit_log.change.roles_added".to_string(),
        DictionaryMessage::new("Rôles Ajoutés"),
    );
    dict.messages.insert(
        "audit_log.change.roles_removed".to_string(),
        DictionaryMessage::new("Rôles Retirés"),
    );
    dict.messages.insert(
        "audit_log.change.rules_channel".to_string(),
        DictionaryMessage::new("Salon des Règles"),
    );
    dict.messages.insert(
        "audit_log.change.splash".to_string(),
        DictionaryMessage::new("Image de Fond"),
    );
    dict.messages.insert(
        "audit_log.change.status".to_string(),
        DictionaryMessage::new("Statut"),
    );
    dict.messages.insert(
        "audit_log.change.system_channel".to_string(),
        DictionaryMessage::new("Salon Système"),
    );
    dict.messages.insert(
        "audit_log.change.tags".to_string(),
        DictionaryMessage::new("Tags"),
    );
    dict.messages.insert(
        "audit_log.change.temporary".to_string(),
        DictionaryMessage::new("Temporaire"),
    );
    dict.messages.insert(
        "audit_log.change.topic".to_string(),
        DictionaryMessage::new("Sujet"),
    );
    dict.messages.insert(
        "audit_log.change.type".to_string(),
        DictionaryMessage::new("Type"),
    );
    dict.messages.insert(
        "audit_log.change.unicode_emoji".to_string(),
        DictionaryMessage::new("Emoji Unicode"),
    );
    dict.messages.insert(
        "audit_log.change.user_limit".to_string(),
        DictionaryMessage::new("Limite d'Utilisateurs"),
    );
    dict.messages.insert(
        "audit_log.change.uses".to_string(),
        DictionaryMessage::new("Utilisations"),
    );
    dict.messages.insert(
        "audit_log.change.vanity_url".to_string(),
        DictionaryMessage::new("URL Personnalisée"),
    );
    dict.messages.insert(
        "audit_log.change.verification_level".to_string(),
        DictionaryMessage::new("Niveau de Vérification"),
    );
    dict.messages.insert(
        "audit_log.change.widget_channel".to_string(),
        DictionaryMessage::new("Salon Widget"),
    );
    dict.messages.insert(
        "audit_log.change.widget_enabled".to_string(),
        DictionaryMessage::new("Widget Activé"),
    );
    dict.messages.insert(
        "audit_log.change.system_channel_flags".to_string(),
        DictionaryMessage::new("Options du Salon Système"),
    );
    dict.messages.insert(
        "slash_command.status_command_help".to_string(),
        DictionaryMessage::new(
            "Permet de changer le status du bot.\n

            Vous pouvez choisir le status entre :\n
            - En ligne (`online`)\n
            - Inactif (`idle`)\n
            - Ne pas deranger (`dnd`)\n
            - Invisible (`invisible`)\n
            - Maintenance (`maintenance`) (uniquement par un admin de rustmail)\n
            
            Pour changer le status du bot : `!status <mode>` or `/status`",
        ),
    );
    dict.messages.insert(
        "slash_command.status_command_description".to_string(),
        DictionaryMessage::new("Changer le status du bot (online, idle, dnd, invisible)"),
    );
    dict.messages.insert(
        "slash_command.mode_arg_description".to_string(),
        DictionaryMessage::new("Le mode dans lequel définir le bot"),
    );
    dict.messages.insert(
        "slash_command.online_status_mode".to_string(),
        DictionaryMessage::new("En ligne"),
    );
    dict.messages.insert(
        "slash_command.idle_status_mode".to_string(),
        DictionaryMessage::new("Inactif"),
    );
    dict.messages.insert(
        "slash_command.do_not_disturb_status_mode".to_string(),
        DictionaryMessage::new("Ne pas déranger"),
    );
    dict.messages.insert(
        "status.status_is_missing".to_string(),
        DictionaryMessage::new("Le status est manquant. `!help status` pour plus d'informations."),
    );
    dict.messages.insert(
        "status.invalid_status".to_string(),
        DictionaryMessage::new(
            "Le status est invalide. Utilisez `!help status` pour plus d'informations.",
        ),
    );
    dict.messages.insert(
        "status.status_online".to_string(),
        DictionaryMessage::new("Le bot est maintenant en ligne 🟢."),
    );
    dict.messages.insert(
        "status.status_idle".to_string(),
        DictionaryMessage::new("Le bot est maintenant inactif 🌙."),
    );
    dict.messages.insert(
        "status.status_dnd".to_string(),
        DictionaryMessage::new("Le bot est maintenant en mode `Ne pas déranger` ⛔."),
    );
    dict.messages.insert(
        "status.status_invisible".to_string(),
        DictionaryMessage::new("Le bot est maintenant invisible ⚫."),
    );
    dict.messages.insert(
        "status.status_maintenance".to_string(),
        DictionaryMessage::new("Le bot est maintenant en maintenance 🚧."),
    );
    dict.messages.insert(
        "status.maintenance_mode_active".to_string(),
        DictionaryMessage::new(
            "🔧 Le bot est actuellement en mode maintenance. Veuillez réessayer plus tard.",
        ),
    );
    dict.messages.insert(
        "status.maintenance_mode_active_user".to_string(),
        DictionaryMessage::new("🔧 Le système de support est actuellement en maintenance. Votre message n'a pas pu être traité. Veuillez réessayer plus tard."),
    );
    dict.messages.insert(
        "status.maintenance_not_allowed".to_string(),
        DictionaryMessage::new("Seuls les administrateurs peuvent activer le mode maintenance."),
    );
    dict.messages.insert(
        "status.maintenance_activity".to_string(),
        DictionaryMessage::new("🔧 Maintenance en cours"),
    );
    dict.messages.insert(
        "category.prompt_title".to_string(),
        DictionaryMessage::new("Choisissez une catégorie"),
    );
    dict.messages.insert(
        "category.prompt_message".to_string(),
        DictionaryMessage::new("Merci de choisir une catégorie pour votre ticket. Si vous ne choisissez pas dans les {timeout_minutes} minutes, votre ticket sera créé dans la catégorie par défaut."),
    );
    dict.messages.insert(
        "category.default_button_label".to_string(),
        DictionaryMessage::new("Général"),
    );
    dict.messages.insert(
        "category.selection_expired".to_string(),
        DictionaryMessage::new(
            "Délai de sélection expiré, votre ticket a été créé dans la catégorie par défaut.",
        ),
    );
    dict.messages.insert(
        "category.ticket_opened_in".to_string(),
        DictionaryMessage::new("Votre ticket a été ouvert dans **{category}**."),
    );
    dict.messages.insert(
        "category.too_many_enabled".to_string(),
        DictionaryMessage::new("Trop de catégories activées. Le maximum est {max}."),
    );
    dict.messages.insert(
        "category.not_found".to_string(),
        DictionaryMessage::new("Catégorie introuvable."),
    );
    dict.messages.insert(
        "category.already_exists".to_string(),
        DictionaryMessage::new("Une catégorie avec ce nom existe déjà."),
    );
    dict.messages.insert(
        "category.invalid_emoji".to_string(),
        DictionaryMessage::new("Emoji invalide."),
    );
    dict.messages.insert(
        "category.invalid_discord_category".to_string(),
        DictionaryMessage::new("ID de catégorie Discord invalide."),
    );
    dict.messages.insert(
        "category.created".to_string(),
        DictionaryMessage::new("Catégorie **{name}** créée."),
    );
    dict.messages.insert(
        "category.deleted".to_string(),
        DictionaryMessage::new("Catégorie **{name}** supprimée."),
    );
    dict.messages.insert(
        "category.renamed".to_string(),
        DictionaryMessage::new("Catégorie renommée en **{name}**."),
    );
    dict.messages.insert(
        "category.moved".to_string(),
        DictionaryMessage::new("Catégorie **{name}** déplacée en position {position}."),
    );
    dict.messages.insert(
        "category.timeout_updated".to_string(),
        DictionaryMessage::new("Délai de sélection réglé à {seconds} secondes."),
    );
    dict.messages.insert(
        "category.feature_enabled".to_string(),
        DictionaryMessage::new("Fonctionnalité de sélection de catégorie activée."),
    );
    dict.messages.insert(
        "category.feature_disabled".to_string(),
        DictionaryMessage::new("Fonctionnalité de sélection de catégorie désactivée."),
    );
    dict.messages.insert(
        "category.enabled_one".to_string(),
        DictionaryMessage::new("Catégorie **{name}** activée."),
    );
    dict.messages.insert(
        "category.disabled_one".to_string(),
        DictionaryMessage::new("Catégorie **{name}** désactivée."),
    );
    dict.messages.insert(
        "category.list_header".to_string(),
        DictionaryMessage::new("Catégories de ticket"),
    );
    dict.messages.insert(
        "category.list_empty".to_string(),
        DictionaryMessage::new("Aucune catégorie définie."),
    );
    dict.messages.insert(
        "category.list_item".to_string(),
        DictionaryMessage::new("`{position}` {emoji} **{name}** — {state}"),
    );
    dict.messages.insert(
        "category.state_enabled".to_string(),
        DictionaryMessage::new("activée"),
    );
    dict.messages.insert(
        "category.state_disabled".to_string(),
        DictionaryMessage::new("désactivée"),
    );
    dict.messages.insert(
        "category.unknown_subcommand".to_string(),
        DictionaryMessage::new("Sous-commande inconnue. Utilisez : create, list, rename, move, delete, enable, disable, timeout, on, off."),
    );
    dict.messages.insert(
        "category.text_usage".to_string(),
        DictionaryMessage::new("Utilisation : `{prefix}category <create|list|rename|move|delete|enable|disable|timeout|on|off> ...`"),
    );
    dict.messages.insert(
        "category.create_usage".to_string(),
        DictionaryMessage::new("Utilisation : `{prefix}category create <discord_category_id> <nom> [| description] [| emoji]`"),
    );
}
