from hycol_py import convert_srgb_hycol, convert_hycol_srgb
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from matplotlib.patches import Circle

fig,ax = plt.subplots(figsize=(10,10))
assert isinstance(ax,plt.Axes)

gamut_round = np.array([
    [1,0,0],
    [1,1,0],
    [0,1,0],
    [0,1,1],
    [0,0,1],
    [1,0,1]
])
ramps = np.linspace(gamut_round,np.roll(gamut_round,1,axis=0),64).swapaxes(0,1)

hramps = convert_srgb_hycol([tuple(c) for c in ramps.reshape((-1,3))])
rampsxyz = np.zeros_like(ramps)

rampsxyz[...,2] = np.array([h[0] for h in hramps]).reshape(ramps.shape[:2])
rampsxyz[...,0] = np.array([np.real(h[1]) for h in hramps]).reshape(ramps.shape[:2])
rampsxyz[...,1] = np.array([np.imag(h[1]) for h in hramps]).reshape(ramps.shape[:2])

for ramp in rampsxyz:
    ax.plot(*ramp.T, c='k')

for pole,ramp in zip(gamut_round,rampsxyz):
    x,y = ramp[0,:2]
    ax.add_patch(Circle((x,y),radius=0.03,facecolor=pole,edgecolor='k',linewidth=1.0))


size = 0.7
def slice_buffer(sliceL):
    grud = np.linspace(-size,size,256)
    xx,yy = np.meshgrid(grud,grud)
    ww = xx+1j*yy
    surf = np.array(convert_hycol_srgb([(sliceL,w) for w in ww.flatten()])).reshape(xx.shape+(3,))
    surf_a = np.zeros(xx.shape + (4,))
    surf_a[...,:3] =  np.clip(surf,0.,1.)
    surf_a[...,3] = 1.0
    surf_a[surf.sum(axis=-1) < 0.01,3] = 0
    return np.flip(surf_a,axis=0)

def plot_slice(ax:plt.Axes,sliceL):
    ax.imshow(slice_buffer(sliceL),extent=[-size,size,-size,size])

plot_slice(ax,55)
plot_slice(ax,68)
plot_slice(ax,85)
plot_slice(ax,95)







ax.set_xlim(-1.1,1.1)
ax.set_ylim(-1.1,1.1)



ax.add_patch(Circle((0,0),1,fill=None,color='k'))

ax.set_axis_off()
ax.set_aspect('equal')

fig.savefig('assets/gamuts.png',bbox_inches='tight')