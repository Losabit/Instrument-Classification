<?php
$error = null; // Détermine le type d'erreur

// On vérifie que le fichier a bien été envoyé et sans aucune erreur

if (isset($_FILES['photo']) && $_FILES['photo']['error'] == 0) {
    // On retire les accents
    $photoName = strtr($_FILES['photo']['name'],
        'ÀÁÂÃÄÅÇÈÉÊËÌÍÎÏÒÓÔÕÖÙÚÛÜÝàáâãäåçèéêëìíîïðòóôõöùúûüýÿ',
        'AAAAAACEEEEIIIIOOOOOUUUUYaaaaaaceeeeiiiioooooouuuuyy');

    $infosfichier = pathinfo($_FILES['photo']['name']);
    $extension_upload = $infosfichier['extension'];
    $extensions_autorisees = ['txt','mp3','wav','mp4'];
    $name = $_POST['titre'];
    $model = $_POST['model'];
    if (in_array($extension_upload,$extensions_autorisees))
    {
        if (!move_uploaded_file($_FILES['photo']['tmp_name'], 'Models/'.$photoName )) {
            $error = 'phase-3-move-file';
        }
        // Gestion des erreurs du processus d'envoi
        switch ($error) {
            case 'phase-3-move-file':
                $message = 'Erreur lors du déplacement du fichier en phase 3';
                break;
        }
        if ($error === null) { ?>
            <p style="color:green; font-size: 20px; font-weight: bold; text-align: center;">Envoyé avec succès !</p>
            <br>
            <p style="font-weight: bold; text-align: center;">Vous allez être redirigé d'ici un instant..

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
        echo "Extension invalide, merci de nous fournir un .txt ou un fichier texte quelconques";



    }

}
else
{
    echo 'Il ny a pas de fichier';
}

echo "<meta http-equiv='refresh' content='5;index.php'>"
?>
<div class="row">
    <img src="img/loading.gif" style="display: block; margin-left: auto; margin-right: auto;">
</div>
