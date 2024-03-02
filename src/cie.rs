#[derive(Debug,Copy,Clone)]
pub struct SRGB {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl PartialEq for SRGB{
    fn eq(&self, other: &Self) -> bool {
        (self.r,self.g,self.b) == (other.r,other.g,other.b)
    }
}

impl SRGB {
    pub fn to_u8(self)->[u8;3]{
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        ]
    }

    pub fn in_gamut(self)->bool{
        let gamut = 0.0..=1.0;
        gamut.contains(&self.r) & gamut.contains(&self.g) & gamut.contains(&self.b)
    }

    pub fn sub_gamut(self)->bool{
        (self.r < 0.0) | (self.g < 0.0) | (self.b < 0.0)
    }

    pub fn super_gamut(self)->bool{
        (self.r > 1.0) | (self.g > 1.0) | (self.b > 1.0)
    }


    pub const BLACK : SRGB = SRGB{r:0.,g:0.,b:0.};
    pub const RED : SRGB = SRGB{r:1.,g:0.,b:0.};
    pub const GREEN : SRGB = SRGB{r:0.,g:1.,b:0.};
    pub const BLUE : SRGB = SRGB{r:0.,g:0.,b:1.};
    pub const CYAN : SRGB = SRGB{r:0.,g:1.,b:1.};
    pub const MAGENTA : SRGB = SRGB{r:1.,g:0.,b:1.};
    pub const YELLOW : SRGB = SRGB{r:1.,g:1.,b:0.};

    pub const GAMUT_POLES : [SRGB;6] = [
        Self::RED,Self::YELLOW,Self::GREEN,
        Self::CYAN,Self::BLUE,Self::MAGENTA
    ];
}

impl From<[u8;3]> for SRGB{
    fn from(rgb8: [u8;3]) -> Self {
        SRGB{
            r: (rgb8[0] as f64)/255.,
            g: (rgb8[1] as f64)/255.,
            b: (rgb8[2] as f64)/255.,
        }
    }
}



#[derive(Debug,Copy,Clone)]
pub struct CIEXYZ{
    x:f64,
    y:f64,
    z:f64
}
#[derive(Debug,Copy,Clone)]
pub struct CIELAB{
    pub l_star:f64,
    pub a_star:f64,
    pub b_star:f64
}


#[inline]
fn s2lin(cgamma:f64)->f64{
    if cgamma <= 0.04045
    {
        cgamma / 12.92
    }
    else {
        ((cgamma+0.055)/1.055).powf(2.4)
    }
}

#[inline]
fn lin2s(clin:f64)->f64{
    if clin <= 0.0031308 {
        12.92*clin
    }
    else{
        1.055*(clin.powf(1./2.4))-0.055
    }
}


fn rgb2XYZ(r:f64,g:f64,b:f64) -> CIEXYZ {
    let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
    let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
    let z = r * 0.0193 + g * 0.1192 + b * 0.9505;
    CIEXYZ{x,y,z}
}

fn XYZ2rgb(xyz:CIEXYZ)->(f64,f64,f64){
    let (x,y,z) = (xyz.x,xyz.y,xyz.z);
    let r = x*3.2404542 +y*-1.5371385 +z*-0.4985314;
    let g = x*-0.9692660  +y*1.8760108 + z*0.0415560;
    let b = x*0.0556434 +y*-0.2040259  +z*1.0572252;
    (r,g,b)
}


impl CIEXYZ {
    pub fn planckian_locus(temperature:f64, luma_y : f64) -> Self{
        let tau = 1000./temperature;
        let tau2 = tau*tau;
        let tau3 = tau2*tau;

        let x_c = if temperature < 4000.{
            -0.2661239 *tau3 - 0.2343589 *tau2 + 0.8776956 *tau + 0.179910
        } else{
            -3.0258469 *tau3 + 2.1070379 *tau2 + 0.2226347 *tau + 0.240390
        };

        let x_c2 = x_c *x_c;
        let x_c3 = x_c2*x_c;

        let y_c = if temperature < 2222.{
            -1.1063814 *x_c3 - 1.34811020*x_c2 + 2.18555832*x_c - 0.20219683
        }else if temperature < 4000.{
            -0.9549476 *x_c3 - 1.37418593 *x_c2 + 2.09137015 *x_c - 0.16748867
        }else{
            3.0817580 *x_c3 - 5.87338670 *x_c2 + 3.75112997 *x_c - 0.37001483
        };

        let big_x = luma_y / y_c * x_c;
        let big_z = luma_y / y_c *(1.-x_c-y_c);

        CIEXYZ{
            x : big_x,
            y : luma_y,
            z : big_z
        }
    }

    pub fn cct_mccamy(&self) -> f64{

        let (xe,ye) : (f64,f64) = (0.3320, 0.1858);
        
        let n = (self.x-xe)/(self.y-ye);
        let n2 = n*n;
        let n3 = n*n2;

        - 449.*n3 + 3525.*n2 - 6823.3*n + 5520.33

    }
}



impl From<SRGB> for CIEXYZ {
    fn from(srgb: SRGB) -> Self {
        let lin_r = s2lin(srgb.r);
        let lin_g = s2lin(srgb.g);
        let lin_b = s2lin(srgb.b);
        rgb2XYZ(lin_r, lin_g, lin_b)
    }
}

impl From<CIEXYZ> for SRGB {
    fn from(xyz: CIEXYZ) -> Self {
        let (linr,ling,linb) = XYZ2rgb(xyz);
        let r = lin2s(linr);
        let g = lin2s(ling);
        let b = lin2s(linb);
        SRGB{r,g,b}
    }
}





#[inline]
fn labf(t:f64)->f64{
    const delta : f64 = 6./29.;
    if t > delta*delta*delta{
        t.powf(1./3.)
    }
    else{
        t / (delta*delta*3.0) + 4./29.
    }
}

#[inline]
fn labinvf(t:f64)->f64{
    const delta : f64 = 6./29.;
    if t > delta{
        t*t*t
    }
    else{
        3.*delta*delta*(t - 4./29.)
    }
}

const XN : f64 = 0.950489;
const YN : f64 = 1.;
const ZN : f64 = 1.088840;

impl From<CIEXYZ> for CIELAB {
    fn from(xyz:CIEXYZ) -> Self{    
        let fy = labf(xyz.y/YN);
        let fx = labf(xyz.x/XN);
        let fz = labf(xyz.z/ZN);
    
        let l_star = 116. * fy - 16.;
        let a_star = 500. * (fx-fy);
        let b_star = 200. * (fy-fz);
    
        CIELAB{l_star,a_star,b_star}
    }
}

impl From<CIELAB> for CIEXYZ{
    fn from(lab: CIELAB) -> Self {
        let ltilde = (lab.l_star+16.)/116.;
        CIEXYZ{
            x : XN * labinvf(ltilde + lab.a_star/500.),
            y : YN * labinvf(ltilde),
            z : ZN * labinvf(ltilde - lab.b_star/200.)
        }
    }
}


impl From<SRGB> for CIELAB {
    fn from(srgb: SRGB) -> Self {
        let xyz : CIEXYZ = srgb.into();
        xyz.into()
    }
}
impl From<CIELAB> for SRGB {
    fn from(lab: CIELAB) -> Self {
        let xyz : CIEXYZ = lab.into();
        xyz.into()
    }
}





#[cfg(test)]
mod tests {

    use super::*;
    use assert_float_eq::assert_f64_near;

    #[test]
    fn lumas(){
        let white = SRGB{r:1.,g:1.,b:1.};
        assert_f64_near!(1.,lin2s(1.));
        assert_f64_near!(1.,s2lin(1.));

        let white_xyz : CIEXYZ = white.into();

        assert_f64_near!(1.0,white_xyz.y);

        let white_lab : CIELAB = white_xyz.into();

        assert_f64_near!(white_lab.l_star,100.,4);

        // let grey : CIELAB = SRGB{r:0.5,g:0.5,b:0.5}.into();
        // assert_eq!(grey.a_star,0.0);
        // assert_eq!(grey.b_star,0.0);
    }

    #[test]
    fn cie_roundtrips() {
        let testsrgb = SRGB{r:0.3,g:0.01,b:0.8};
        let testsrgb_xyz: CIEXYZ = testsrgb.into();
        let testsrgb_rt : SRGB = testsrgb_xyz.into();
        assert_eq!(testsrgb.to_u8(), testsrgb_rt.to_u8());


        for ti in 0..20{
            let t : f64 = (ti as f64)/20.0;

            assert_f64_near!(labinvf(labf(t)), t)
        }

        let testsrgb_lab : CIELAB = testsrgb.into();
        let testsrgb_lab_rt : SRGB = testsrgb_lab.into();
        assert_eq!(testsrgb_lab_rt.to_u8(),testsrgb.to_u8());

    }
}