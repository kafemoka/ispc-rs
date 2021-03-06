# ddvol

An example of a scivis style volume renderer, hopefully in the future supporting
rendering of distributed data. Currently it's just a simple volume raycaster which
can load and render RAW
volume files and colormap them with some default provided transfer functions,
`grayscale`, `cool_warm` and `jet` or by loading a transfer function exported
from [ParaView](http://www.paraview.org/). Examples of the scene format can be
found in [`scenes/`](scenes/).

Below are two example images of datasets from [OSPRay's downloadable demos](http://www.ospray.org/demos.html),
the magnetic reconnection 512^3 data set and the CSAFE 302^3 dataset,
colormapped with transfer functions exported from ParaView. The scene files for
these are included in [`scenes/`](scenes/) though you'll need to download the volume
data from the OSPRay webpage.

Magnetic Reconnection, via the OSPRay demos page courtesy of Bill Daughton (LANL) and
Berk Geveci (KitWare) from [this paper](http://arxiv.org/abs/1405.4040)
![Magnetic Reconnection](http://i.imgur.com/3tPHx2S.png)

`ddvol` can also render implicit isosurfaces, set with the `-i` or `--isovalue` argument.
![Magnetic Reconnection with Isosurface](http://i.imgur.com/6Duu3da.png)

CSAFE, via the OSPRay demos page courtesy of of the Center for the Simulation of
Accidental Fires and Explosions (CSAFE) at the Scientific Computing and Imaging
Institute (SCI), University of Utah.
![CSAFE](http://i.imgur.com/zdX2ZF5.png)

