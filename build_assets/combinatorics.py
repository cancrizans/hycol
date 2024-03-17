import numpy as np
import matplotlib.pyplot as plt
from matplotlib import patches
import roman
from matplotlib import collections  as mc
from scipy.spatial import ConvexHull

def draw_wheel(ax:plt.Axes):
    ax.add_patch(patches.Circle((0,0),1,fill=None))
    larr = []
    for k in range(14):
        theeta = np.pi*2*k/14.
        r1 = 0.9
        r2 = 1.1
        larr.append(
            [(r1*np.cos(theeta),r1*np.sin(theeta)),(r2*np.cos(theeta),r2*np.sin(theeta))]
            )
    larr = np.array(larr)
    lc = mc.LineCollection(larr,colors='k')
    lc.set_snap(False)
    ax.add_collection(lc)
    ax.set_xlim(-1.15,1.15)
    ax.set_ylim(-1.15,1.15)
    ax.set_aspect('equal')
    ax.set_axis_off()


ns = np.arange(2**7,dtype=np.uint8)[:,np.newaxis]

configs = np.unpackbits(ns,axis=-1)[:,1:]
configs.shape

def rot(seq):
    carry = seq[0]
    return seq[1:] + (1-carry,)

class SymConf:
    def __init__(self, conf:tuple):
        self._conf = conf
    
    def all_rots(self):
        rott = self._conf
        rots = [rott]
        for _ in range(14):
            rott = rot(rott)
            rots.append(rott)

        return frozenset(rots)
        
    def __eq__(self, __value: object) -> bool:
        if not isinstance(__value,SymConf):
            return False
        return self.all_rots() == __value.all_rots()
    def __hash__(self) -> int:
        
        return self.all_rots().__hash__()

configs_tups = [tuple(conf) for conf in configs]
syms = [SymConf(l) for l in configs_tups]
elsymset = set(syms)
final_confs = tuple(sym._conf for sym in elsymset)



fig,axs = plt.subplots(2,5,figsize=(12,4))

for i,c in enumerate(final_confs):
    ax : plt.Axes = axs[i//5,i%5]

    draw_wheel(ax)
    
    # ax.set_title(roman.toRoman(i))
    ax.annotate(roman.toRoman(i+1),(0,0),ha = 'center',va='center')
    

    

    dots = []
    for k in range(7):
        bitset = c[k]
        thet = np.pi * (k + (7 if bitset else 0)) / 7
        dot = (np.cos(thet),np.sin(thet))
        ax.add_patch(patches.Circle(dot,radius=0.1,color='k'))
        dots.append(dot)

    dots = np.array(dots)
    hull = ConvexHull(dots)
    for simplex in hull.simplices:
        ax.plot(dots[simplex, 0], dots[simplex, 1], 'k')

fig.savefig('assets/combinatoric_modes.png')


## triads



triads = [
    dict(name="Ultra-narrow",base=(6,8)),
    dict(name="Left Narrow",base=(6,9)),
    dict(name="Right Narrow",base=(5,8)),
    dict(name="Wide",base=(5,9))
]

fig,axs = plt.subplots(1,len(triads),figsize=(8,3))

for triad,ax in zip(triads,axs):
    draw_wheel(ax)
    ax.set_title(triad['name'])

    ks = np.array((0,) + triad['base'])
    hue = np.pi * ks/7.
    xys = np.array([np.cos(hue),np.sin(hue)]).T

    crosses = []
    for xy in xys:
        ax.add_patch(patches.Circle(xy,radius=0.1,color='k'))
        ax.add_patch(patches.Circle(-xy,radius=0.05,color='k',fill=None))
        crosses.append(-xy)

    ax.add_patch(patches.Arc((1,0),
                             width=2*0.17,
                             height=2*0.17,
                             theta1=90,
                             theta2=270,
                             color='k',fill=None,lw=1))    
    # ax.scatter(*np.array(crosses).T,marker='o',c='k')
    
    lsegs = [
        (xy,xyp)
        for xy,xyp in zip(xys,np.roll(xys,1,axis=0))
    ]

    ax.add_collection(mc.LineCollection(lsegs,colors='k'))

    # ax.arrow(0.5,0,0.3,0,facecolor='k',head_width=0.07)

fig.tight_layout()
fig.savefig('assets/triads.png')