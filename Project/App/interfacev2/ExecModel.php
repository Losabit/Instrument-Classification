<?php
echo "<br>";
$output = shell_exec("python /var/www/interface/script.py ".$_GET['dirName'].$_GET['fileName']);
echo "Output : " . $output;