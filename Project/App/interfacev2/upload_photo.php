<?php
$error = null; // Détermine le type d'erreur

// On vérifie que le fichier a bien été envoyé et sans aucune erreur

    if (isset($_FILES['photo']) && $_FILES['photo']['error'] == 0) {
            // On retire les accents
            $photoName = strtr($_FILES['photo']['name'],
                'ÀÁÂÃÄÅÇÈÉÊËÌÍÎÏÒÓÔÕÖÙÚÛÜÝàáâãäåçèéêëìíîïðòóôõöùúûüýÿ',
                'AAAAAACEEEEIIIIOOOOOUUUUYaaaaaaceeeeiiiioooooouuuuyy');

            // On vérifie qu'il s'agit bien d'une image
            $infosfichier = pathinfo($_FILES['photo']['name']);
        $extension_upload = $infosfichier['extension'];
            $extensions_autorisees = ['mp3','wav','mp4'];
			$extensions_trans = ['mp3','mp4'];
            $name = $_POST['titre'];
            $model = $_POST['model'];
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
                        $dirName = "Models/".$_POST['model'];
                        if($_POST['model'] == "mlp.txt"){
                            $type = 1;
                        }
                        elseif ($_POST['model'] == "RBF"){
                            $type = 2;
                        }
                        $photoName = "uploads/".$photoName;
                        $pathFile = $photoName;
                        echo  '<br>';
                        exec("python3 /var/www/interface/script.py $photoName $type $dirName ",$output,$error_code);
                        print_r($output);
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

echo "<meta http-equiv='refresh' content='5;index.php?info=true&piano=$output[0]&saxo=$output[1]&guitare=$output[2]'>"
?>
<div class="row">
    <img src="img/loading.gif" style="display: block; margin-left: auto; margin-right: auto;">
</div>
