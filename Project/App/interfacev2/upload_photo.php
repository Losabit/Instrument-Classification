<?php ;
$error = null; // Détermine le type d'erreur

// On vérifie que le fichier a bien été envoyé et sans aucune erreur
echo  $_FILES['photo']['error'] == 0;
echo "<br>";
echo $_FILES['photo']['error'];
echo "<br>";
    if (isset($_FILES['photo']) && $_FILES['photo']['error'] == 0) {
            // On retire les accents
            $photoName = strtr($_FILES['photo']['name'],
                'ÀÁÂÃÄÅÇÈÉÊËÌÍÎÏÒÓÔÕÖÙÚÛÜÝàáâãäåçèéêëìíîïðòóôõöùúûüýÿ',
                'AAAAAACEEEEIIIIOOOOOUUUUYaaaaaaceeeeiiiioooooouuuuyy');
        echo "<br>";

        echo $photoName;
        echo "<br>";

            // On vérifie qu'il s'agit bien d'une image
            $infosfichier = pathinfo($_FILES['photo']['name']);
        echo "<br>";

        $extension_upload = $infosfichier['extension'];
            $extensions_autorisees = ['mp3','wav','mp4'];
			$extensions_trans = ['mp3','mp4'];
            $name = $_POST['titre'];
            $model = $_POST['model'];
            echo "ext : " . $infosfichier['extension'];
        echo "<br>";


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
					exec("lame –decode uploads/$photoName.mp3 uploads/$photoName.wav");



				}

                if ($error === null) { ?>
                    <p style="color:green; font-size: 20px; font-weight: bold; text-align: center;">Envoyé avec succès !</p>
                    <br>
                    <p style="font-weight: bold; text-align: center;">Vous allez être redirigé d'ici un instant..
                        <?php
                        $pathFile = $_GET['dirName']. $_GET['fileName'];
                        $output = shell_exec("python /var/www/interface/script.py $pathFile ");
                        echo 'ouut :'$output;
                        ?>
                    </p>
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

<div class="row">
    <img src="img/loading.gif" style="display: block; margin-left: auto; margin-right: auto;">
</div>