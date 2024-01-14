# Convertion d'images PNG en JPEG 2000

Petit projet personnel suite à la [vidéo sur les ondelette](https://www.youtube.com/watch?v=vpmlGMZSpvQ) de EL_jj où j'essaye de convertir une image sous format PNG en JPEG 2000

## Approche Générale

### Manipulation d'image

- import d'image : :white_check_mark:
- convertion d'espace colorimétrique (RGBa <-> RGB <-> YCbCr <-> YUV <-> RGBa) : :x:
  - *JPEG 2000 utilise l'espace colorimétrique YUV au de YCbCr pour JPEG*
- séparation de l'image selon les différents channels : :white_check_mark:
- reconstruction de l'image  : :white_check_mark:

### Décodage des images sous format PNG

- apprendre l'encodage PNG : :x:

### Compression à l'aide d'ondelettes

- processus de compression : :x:
- utilisation de différentes ondelettes : :x:

### Encodage sous format JPEG *2000*

- apprendre l'encodage JEPG 2000 : :x:

## TODOs

- implémenter `from_yuv` et `to_yuv` pour tout les types de pixel
- gérer les convertion `u8` <-> `i8` pour YUV
- gérer les possibles différents type de sortie de `channels` et entrées de `from_channels`
- implémenter un découpage en YUV pour une image PNG
- faire la compression