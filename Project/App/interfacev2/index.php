<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <!-- PAGE settings -->
  <link rel="icon" href="https://templates.pingendo.com/assets/Pingendo_favicon.ico">
  <title>App Neon - Pingendo template</title>
  <meta name="description" content="Free Bootstrap 4 Pingendo Neon template made for app and softwares.">
  <meta name="keywords" content="Pingendo app neon free template bootstrap 4">
  <!-- CSS dependencies -->
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css" type="text/css">
  <link rel="stylesheet" href="neon.css">
  <!-- Script: Make my navbar transparent when the document is scrolled to top -->
  <script src="js/navbar-ontop.js"></script>
  <!-- Script: Animated entrance -->
  <script src="js/animate-in.js"></script>
</head>

<body>
  <!-- Navbar -->
  <!-- Cover -->
  <!-- Article style section -->
  <!-- Features -->
  <!-- Features -->
  <!-- Carousel reviews -->
  <!-- Call to action -->
  <!-- Footer -->
  <!-- JavaScript dependencies -->
  <div class="">
    <div class="container">
      <div class="row">
        <div class="col-md-12">
          <h1 class="display-4 text-center pb-5">Find instruments</h1>
        </div>
      </div>
    </div>
  </div>
  <div class="py-0" style="">
    <div class="container">
      <div class="row h-75">
        <div class="col-md-6">
          <div class="row">
            <br>
            <br>
            <div class="row" style="margin-left: 30%; margin-top: 20%;">
                <div class="card">
                  <div class="card-body">
                    <form method="POST" action="upload_photo.php" class="form-group"
                          enctype="multipart/form-data">
                      <div class="form-group">
                        <label for="titre" class="form-label">Titre du fichier</label>
                        <input type="text" class="form-control" name="titre" placeholder="Titre du fichier"
                               id="titre">
                      </div>
                      <br>

                      <div class="form-group" style="text-align: center;">
                        <label for="photo" class="form-label">Votre fichier</label> <br>
                        <input class="btn btn-primary" type="file" name="photo" id="photo">
                          <br>
                          <label for="model" class="form-labael"> Le model </label><br>
                          <select name="Model" size="1">
                          <?php
                          $dir = "Models/";
                          if (is_dir($dir)) {
                              if ($dh = opendir($dir)) {
                                  while (($file = readdir($dh)) !== false) {
                                      echo  "<OPTION value='$file'> $file </OPTION>";
                                  }
                                  closedir($dh);
                              }
                          }
                          ?>
                          </select>
                      </div>
                      <br>
                      <button  style="margin-right: 20%" class="btn btn-sucess" type="submit">Envoyer

                    </form>
                  </div>
                </div>
            </div>
          </div>
          <div class="row">
            <div class="col-md-12" id="root" >
              <div class="row">
                <div class="col-md-5" style=""></div>
                <div class="col-md-6 mr-0" style=""></div>
              </div>
            </div>
          </div>
          <div class="col-md-12 mt-3">

            </div>
        </div>
        <div class="col-md-6" style="">
          <div class="row h-25">
            <div class="col-md-12 h-100" style="">
              <h1 class="text-center my-4">Your result</h1>
            </div>
          </div>
          <div class="row h-75">
            <div class="col-md-12 h-100">
              <div class="table-responsive">
                <table class="table">
                  <thead>
                    <tr>
                      <th>#</th>
                      <th>Instrument</th>
                      <th>Is present</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr>
                      <td style=""><br><i class="fa fa-camera-retro fa-lg" style=""></i></td>
                      <td>Saxophone</td>
                      <td>No</td>
                    </tr>
                    <tr>
                      <td>2</td>
                      <td>Piano</td>
                      <td>No</td>
                    </tr>
                    <tr>
                      <td>3</td>
                      <td>Guitare</td>
                      <td>No</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
<!--    </div>  <a href="ListModel.php"> <button  style="margin-right: 20%" class="btn btn-sucess" >Voir les modèls </button></a>-->


  </div>

  <div class="pt-5">
    <div class="container">
      <div class="row">
        <div class="col-md-12 text-center d-md-flex align-items-center">
          <ul class="nav mx-md-auto d-flex justify-content-center">
            <li class="nav-item"> <a class="nav-link active" href="#">Home</a> </li>
            <li class="nav-item"> <a class="nav-link" href="#">Features</a> </li>
            <li class="nav-item"> <a class="nav-link" href="#">Pricing</a> </li>
            <li class="nav-item"> <a class="nav-link" href="#">About</a> </li>
          </ul>
          <div class="row">
            <div class="col-md-12 d-flex align-items-center justify-content-md-between justify-content-center my-2"> <a href="#">
                <i class="d-block fa fa-facebook-official text-muted fa-lg mx-2"></i>
              </a> <a href="#">
                <i class="d-block fa fa-instagram text-muted fa-lg mx-2"></i>
              </a> <a href="#">
                <i class="d-block fa fa-twitter text-muted fa-lg ml-2"></i>
              </a> </div>
          </div>
        </div>
      </div>
      <div class="row">
        <div class="col-md-12 text-center">
          <p class="mt-2 mb-0">© 2014-2018 Pingendo. All rights reserved</p>
        </div>
      </div>
    </div>
  </div>
 </body>
</html>