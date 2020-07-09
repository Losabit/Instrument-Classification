<?php ;
/**
 * upload_photo.php
 * ----------------
 *
 * Permet de gérer l'upload des fichiers du site
 *
 * Phase 1
 * -------
 * A l'enregistrement, id_photo = ?
 * -> On ne peut pas se contenter de lire le dernier id dans la base de données car si jamais quelqu'un envoie
 * sa photo au même momemnt il risque de lire également le même id maxiumum, causant ainsi un conflit
 * Exemple:
 * - user1 envoie sa photo à 12:00:00 et 0ms -> il lit que le dernier id est 49 par exemple
 * - user2 envoie sa photo à 12:00:00 et 1ms -> il aussi que le dernier id est 49 car user1 n'a pas encore
 * terminé le traitement de son image
 * Conclusion: les deux photo auront id = 50 dans la base de données => conflit
 *
 * -> On enregistre avec titre_photo par exemple à 'superphoto_%id%.jpg'
 *
 * Phase 2
 * -------
 * Après l'enregistrement, on connait l'id réel dans la colonne id_photo
 * -> Met à jour titre_photo avec l'id_photo en remplaçant %id% par id_photo
 *
 * Phase 3
 * -------
 * Déplacement du fichier.
 *
 * @var $bd PDO
 */


$error = null; // Détermine le type d'erreur

// On vérifie que le fichier a bien été envoyé et sans aucune erreur
    if (isset($_FILES['photo']) && $_FILES['photo']['error'] == 0) {
            // On retire les accents
            $photoName = strtr($_FILES['photo']['name'],
                'ÀÁÂÃÄÅÇÈÉÊËÌÍÎÏÒÓÔÕÖÙÚÛÜÝàáâãäåçèéêëìíîïðòóôõöùúûüýÿ',
                'AAAAAACEEEEIIIIOOOOOUUUUYaaaaaaceeeeiiiioooooouuuuyy');
            echo $photoName;
            // On vérifie qu'il s'agit bien d'une image
            $infosfichier = pathinfo($_FILES['photo']['name']);
            $extension_upload = $infosfichier['extension'];
            $extensions_autorisees = ['mp3','wave','mp4'];
			$extensions_trans = ['mp3','mp4'];
            $description = $_POST['description'];
            $name = $_POST['titre'];

            if (in_array($extension_upload,$extensions_autorisees)) // Il faut que les 2 variables valent 1 (vrai) pour rentrer dans le IF
            {

                // On peut déplacer le fichier du rep temporaire et le stocker définitivement dans le dossier "uploads"
                if (!move_uploaded_file($_FILES['photo']['tmp_name'], 'uploads/'.$photoName )) {
                    $error = 'phase-3-move-file';
                }
        // Gestion des erreurs du processus d'envoi
                switch ($error) {
                    case 'phase-3-move-file':
                        $message = 'Erreur lors du déplacement du fichier en phase 3';
                        break;
                }
				
				if (in_array($extension_upload,$extensions_trans)){
					exec("lame –decode uploads/".$photoName.mp3" uploads/".$photoName .".wav");
					echo `script.py`

					
				}

                if ($error === null) { ?>
                    <p style="color:green; font-size: 20px; font-weight: bold; text-align: center;">Envoyé avec succès !</p>
                    <br>
                    <p style="font-weight: bold; text-align: center;">Vous allez être redirigé d'ici un instant..</p>
                <?php } else { ?>
                    <p style="color:red; font-size: 30px; font-weight: bold; text-align: center;">
                        Erreur, échec de l'envoi !<br>
                        Code d'erreur: <?php echo $message; ?>
                    </p>
                    <br>
                    <p style="color:orange; font-size: 20px; font-weight: bold; text-align: center;">
                        Pas de panique, le fichier est peut-être incorrecte, ou alors vous n'avez pas bien renseigné les champs du formulaire.
                    </p>
                    <br>
                    <p style="font-weight: bold; text-align: center;">Vous allez être redirigé d'ici un instant..</p>';
                <?php } }else
            {
                echo "Extension invalide, merci de nous fournir un .wave, mp3 , mp4, ou un fichier audio quelconques";



            }

    }
    else
    {
        echo 'Il ny a pas de fichier';
    }
?>

<meta http-equiv="refresh" content="35;index.php">

<div class="row">
    <img src="img/loading.gif" style="display: block; margin-left: auto; margin-right: auto;">
</div>