<?php
$dir = "Models/";

// Ouvre un dossier bien connu, et liste tous les fichiers
if (is_dir($dir)) {
    if ($dh = opendir($dir)) {
        while (($file = readdir($dh)) !== false) {
            echo  "Model <a href='http://interface.saveyourfood.fr/ExecModel.php?fileName=$file&dirName=$dir'>  $file </a>";
           echo  "<br>";
        }
        closedir($dh);
    }
}