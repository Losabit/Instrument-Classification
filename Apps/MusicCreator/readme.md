# SoundAnalyzer

## Description
Application ayant pour but de former un dataset audio rapidement en proposant de manipuler un grand nombre de fichier .wav et en faisant des opérations dessus.
Les opérations sont les suivantes :
  - Découpage d'une partie d'un morceau en un ou plusieurs fichiers
  - Analyse de la fréquence audio et de fourier du fichier
  - Split des canaux du fichier
  - Transformation des fichiers au format .csv
  - Changement de la fréquence
 
A la base, il s'agissait d'une application expérimentale en console pour voir comment fonctionne un fichier .wav et comment on pourrait lui appliquer des "transformations". Si vous prenez le temps de voir le code vous verez que tout hormis fourier et le changement de fréquence ont été fait en manipulant les bytes du fichier .wav.
Par la suite il a été décidé de la changer en une application Winforms et l'adapter à la manipulation de fichiers en grosses quantité pour pouvoir faire aisément des transformations et ainsi créer notre dataset.

## Lancer l'application
L'éxécutable du projet se trouve dans SoundAnalyzer/bin
Attention ! : Le fait qu'il s'agisse d'une application Winform a rendu impossible l'utilisation de Mono. De ce fait, l'application est disponible seuleument sur Windows.
