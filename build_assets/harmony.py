from hycol_py import color_wheel, color_wheel_extremes,from_polar, convert_hycol_srgb,Constants
import numpy as np
import matplotlib.pyplot as plt
from matplotlib import patches
import matplotlib.collections as cm

## Prepare palette

SKIN_CENTER = Constants.skin_center()
skinbg,skinbgdark = convert_hycol_srgb([(92,SKIN_CENTER),(50,SKIN_CENTER)])

hues = np.pi*np.arange(14)/7
loomas = 70 + 8*np.abs(np.cos( (hues + np.deg2rad(-170))/2. ) ) #+0*np.sin(hues+np.deg2rad(-50))
#chroomas = 1.13 - 0.4*np.sin(hues + np.deg2rad(40)) + 0.00*np.abs(np.cos(hues/2.)+ np.deg2rad(-140))

chroomas = 0.8 + 0.45*np.power(np.abs(np.cos((hues +np.deg2rad(-220))/2.)),1.5)

polars = [
    (loomas[i],hues[i],chroomas[i]) for i in range(14)
]

reptones_h = from_polar(SKIN_CENTER,polars)
reptones = convert_hycol_srgb(reptones_h)

names = list("vcohyxgtnblmpr")


## Star diagram

fig,axs = plt.subplots(figsize=(6,6))
ax = axs
assert isinstance(ax,plt.Axes)
idx = np.arange(14)
ang = np.pi*idx/7
xy = np.array([np.cos(ang),np.sin(ang)]).T

for i in idx:
    tone = reptones[i]
    xys = xy[i]
    ax.add_patch(patches.Circle(xys,0.15,color=tone,zorder=0))
    ax.annotate(names[i],xys,ha='center',va='center',fontsize=20)
    ax.plot(*np.array([xys,(xy[(i+8)%14])]).T, c='k', ls = ':' if i%2 == 0 else '-', zorder=-2)
ax.set_xlim(-1.3,1.3)
ax.set_ylim(-1.3,1.3)
ax.set_aspect('equal')
ax.set_axis_off()
fig.tight_layout()
fig.savefig('assets/star.png')


## Lattice

fig,ax = plt.subplots(figsize=(9,9))
assert isinstance(ax,plt.Axes)
ax.set_aspect('equal')

crad = 0.3
segs = []
lsts = []
for ii in range(8):
    for j in range(8):
        i = ii-j//2
        x = i + .5*(j)
        y = j * np.sqrt(3)/2

        k = (i*3 + j*8)%14

        col = reptones[k]
        if k%2 == 0:
            ax.add_patch(patches.Circle((x,y),crad,color=col))
        else:
            sqz = crad*0.8
            ax.add_patch(patches.Rectangle((x-sqz,y-sqz),sqz*2,sqz*2,color=col))
        ax.annotate(names[k],(x,y),ha='center',va='center',fontsize=20)

        segs.append(((x,y),(x+1,y)))
        lsts.append(':')
        segs.append(((x,y),(x+.5,y+np.sqrt(3)/2)))
        lsts.append('-')
        segs.append(((x,y),(x-.5,y+np.sqrt(3)/2)))
        lsts.append('--')

        dy = 0.1
        arrof = 0.15
        # ax.arrow(x,y-crad-arrof-dy,0,dy, head_width=0.04,color='#666')
        # ax.arrow(x,y+crad+arrof+dy,0,-dy, head_width=0.04,color='#666')

ax.add_collection(cm.LineCollection(segs,colors='k',zorder=-3,linestyles=lsts))

ax.set_xlim(0.3,6.3)
ax.set_ylim(0.5,6.5)
ax.set_axis_off()
ax.add_patch(patches.Rectangle((-5,-5),20,20,color=skinbgdark,zorder = -500))
fig.tight_layout()
fig.savefig('assets/lattice.png')


## SCSS

scss_tonelist = ["$tones:","["]

scss_tones = []

for i in range(14):
    rgbf = reptones[i]
    r,g,b = (int(c*255.) for c in rgbf)
    hexa = f"#{r:02x}{g:02x}{b:02x}"
    scss_tonelist.append(
        hexa
    )
    scss_tones.append(f"$reptone-{i}: {hexa};")
scss_tonelist.append("]")

_ = " ".join(scss_tonelist) 
_ = "---\n---\n" 
scss =  "//Auto-generated\n"+ "\n" + "\n".join(scss_tones)

with open('_sass/_tones.scss','w') as f:
    f.write(scss)