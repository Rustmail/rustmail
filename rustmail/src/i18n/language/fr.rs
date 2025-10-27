use crate::errors::dictionary::DictionaryMessage;
use crate::errors::dictionary::ErrorDictionary;

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
        "discord.user_is_a_bot".to_string(),
        DictionaryMessage::new("L'utilisateur spécifié est un rustmail"),
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
        DictionaryMessage::new("✅ Thread déplacé vers la catégorie '{category}' par {staff}")
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
        DictionaryMessage::new("⏰ Rappel enregistré pour {time} ({remaining_time}) !"),
    );
    dict.messages.insert(
        "reminder.registered_with_content".to_string(),
        DictionaryMessage::new(
            "⏰ Rappel enregistré pour {time} ({remaining_time}) !\n\n{content}",
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
        DictionaryMessage::new("Configure un rappel à une heure spécifique. Pour ce faire, faites `!remind <HH:MM> <contenu du rappel>` ou `!rem <HH:MM> <contenu du rappel>`. Si l'heure est déjà passée aujourd'hui, le rappel sera programmé pour demain."),
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
        "help.help".to_string(),
        DictionaryMessage::new("Affiche une liste de toutes les commandes disponibles avec une brève description. Pour afficher le message d'aide, faites `!help`."),
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
        DictionaryMessage::new("## Commandes :\n\n**Toutes les commandes** disponibles sont également utilisables via des **__commandes slash__** portant le __même nom__.\n\n"),
    );
    dict.messages.insert(
        "add_reminder.helper".to_string(),
        DictionaryMessage::new("Format incorrect. Utilisation : `{prefix}remind ou {prefix}rem <HH:MM> [contenu du rappel]`"),
    );
}
