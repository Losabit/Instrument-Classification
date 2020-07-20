<!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" href="https://templates.pingendo.com/assets/Pingendo_favicon.ico">
    <title>App Neon - Pingendo template</title>
    <meta name="description" content="Free Bootstrap 4 Pingendo Neon template made for app and softwares.">
    <meta name="keywords" content="Pingendo app neon free template bootstrap 4">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css" type="text/css">
    <link rel="stylesheet" href="neon.css">
    <script src="js/navbar-ontop.js"></script>
    <script src="js/animate-in.js"></script>
</head>

<body>
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
                                <form method="POST" action="upload_model.php" class="form-group"
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
        </div>
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
                </div>
            </div>
        </div>
    </div>
</body>
</html>