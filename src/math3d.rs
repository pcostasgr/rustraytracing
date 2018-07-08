extern crate rand;
use rand::{thread_rng,Rng};
use std::{ptr,fmt};

const MAX_FLOAT:f32=9999999999999999999999999.0;



#[derive(Debug,Copy,Clone)]
pub struct Vec3D<T> {
    pub x:T,
    pub y:T,
    pub z:T,
}

type v3f=Vec3D<f32>;
type v3i=Vec3D<i32>;



pub struct HitRecord{
    pub t:f32,
    pub p:v3f,
    pub normal:v3f,
    pub object_index:usize,
}

impl HitRecord{
    pub fn new()->Self{
        HitRecord{
            t:0.0,
            p:v3f{x:0.0,y:0.0,z:0.0},
            normal:v3f{x:0.0,y:0.0,z:0.0},
            object_index:0
        }
    }
}

pub trait Hitable{
    fn Hit(&self,r:Ray,t_min:f32,t_max:f32,rec:& mut  HitRecord)->bool;
    fn GetMaterial(&self)->& Box<Material>;
}

impl Vec3D<f32>{
    pub fn Length(&self)-> f32{
        let mut value:f32=self.x*self.x+self.y*self.y+self.z*self.z;
        value.sqrt()
    }
    
    pub fn SquaredLength(&self)-> f32{
        let mut value:f32=self.x*self.x+self.y*self.y+self.z*self.z;
        value
    }

    pub fn ToI32(&self)->Vec3D<i32>{
        Vec3D{x:self.x as i32,y:self.y as i32,z:self.z as i32}
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Ray{
    pub origin:v3f,
    pub direction:v3f,
}

impl Ray{
    pub fn new()->Self{
        Ray{
            origin:v3f{x:0.0,y:0.0,z:0.0},
            direction:v3f{x:0.0,y:0.0,z:0.0},            
        }
    }

    pub fn PointAtParameter(&self,t:f32) -> v3f{
        let v=VecMul3D(&self.direction,t);
        let r=VecAdd3D(&self.origin,&v);
        r
    }
}


pub fn VecDot3D(v1:&Vec3D<f32>,v2:&Vec3D<f32>)-> f32{
    (v1.x*v2.x+v1.y*v2.y+v1.z*v2.z)
}

pub fn VecCross3D(v1:&Vec3D<f32>,v2:&Vec3D<f32>)-> Vec3D<f32>{
    let x=v1.y*v2.z-v1.z*v2.y;
    let y=-(v1.x*v2.z)-v1.z*v2.x;
    let z=v1.x*v2.y-v1.y*v2.x;

    Vec3D{x:x,y:y,z:z}
}

pub fn VecAdd3D(v1:&Vec3D<f32>,v2:&Vec3D<f32>)-> Vec3D<f32>{
   Vec3D{x:v1.x+v2.x,y:v1.y+v2.y,z:v1.z+v2.z}
}

pub fn VecSub3D(v1:&Vec3D<f32>,v2:&Vec3D<f32>)-> Vec3D<f32>{
    Vec3D{x:v1.x-v2.x,y:v1.y-v2.y,z:v1.z-v2.z}
}

pub fn VecMul3D(v:&Vec3D<f32>,f:f32)->Vec3D<f32>{
    Vec3D{x:v.x*f,y:v.y*f,z:v.z*f}
}

pub fn VecMulVec3D(v1:&Vec3D<f32>,v2:&Vec3D<f32>)->Vec3D<f32>{
    Vec3D{x:v1.x*v2.x,y:v1.y*v2.y,z:v1.z*v2.z}
}

pub fn VecDiv3D(v:&v3f,f:f32)->v3f{

    //this sucks I'll use an epsilon next time I promise
    if f!=0.0 {
        
        Vec3D{x:v.x/f,y:v.y/f,z:v.z/f}
    }else{
        Vec3D{x:0.0,y:0.0,z:0.0}
    }
}

pub fn VecNorm3D(v:&v3f)->v3f{
    let length=v.Length();
    VecDiv3D(v,length)
}

pub fn Refrect(v:&v3f,n:&v3f)->v3f{
    let result=VecDot3D(&v,&n)*2.0;
    let nn=VecMul3D(&n,result);
    VecSub3D(&v,&nn)
}

//#[derive(Debug,Copy,Clone)]
pub struct Sphere {
    pub center:v3f,
    pub radius:f32,
    pub  material: Box<Material >,
}

impl  Hitable for Sphere{ 

    fn Hit(&self,r:Ray,t_min:f32,t_max:f32,rec:& mut HitRecord)->bool{

        
        let oc=VecSub3D(&r.origin,&self.center);
        let a=VecDot3D(&r.direction,&r.direction);
        let b=VecDot3D(&oc,&r.direction);
        let c=VecDot3D(&oc,&oc)-(self.radius*self.radius);
        let discriminant=(b*b)-(a*c);
        
        if discriminant>0.0 {
            let sq:f32=(b*b)-(a*c);
            let mut temp:f32=(-b-sq.sqrt())/a;
            
            if temp <t_max && temp>t_min{
                rec.t=temp;
                rec.p=r.PointAtParameter(rec.t);
                rec.normal=VecSub3D(&rec.p,&self.center);
                rec.normal=VecDiv3D(&rec.normal,self.radius);
                return true;
            }
            
            temp=(-b+sq.sqrt())/a;
            if temp<t_max && temp>t_min{
                rec.t=temp;
                rec.p=r.PointAtParameter(rec.t);
                rec.normal=VecSub3D(&rec.p,&self.center);
                rec.normal=VecDiv3D(&rec.normal,self.radius);
                return true;
            }
        }
        false
    }

     fn GetMaterial(&self)->& Box<Material>{
        return &self.material;
     }

}


    //r lifetime for rec must be smaler than a and m
    pub fn GetColor(mut ray:Ray,world:& World,depth:i16)->v3f{
    
        let b:bool;
        
        let mut tmp_rec=HitRecord::new();        
        let result=world.Hit(ray,0.001,MAX_FLOAT,& mut tmp_rec);
        
        if result {
            let mut scattered=Ray::new();
            let mut attenuation=v3f{x:0.0,y:0.0,z:0.0};

            let object=&world.objects[tmp_rec.object_index];
            let material=object.GetMaterial();

            if depth<50 {
                let is_scattered=material.Scatter(&ray,&tmp_rec,& mut attenuation,& mut scattered);


                if is_scattered {
                    let depth_=depth+1;

                    let mut v=GetColor(scattered,world,depth_);
                    
                    //println!("debugging {:?} ",v);

                    v=VecMulVec3D(&v,&attenuation);
                    return v; 
                }else {
                    return v3f{x:0.0,y:0.0,z:0.0};
                }

            } 

            return v3f{x:0.0,y:0.0,z:0.0};
           
        }else{
            let mut unit_direction=VecNorm3D(&ray.direction);
        
            if depth>0 {
             //   println!("depth {0} debugging {1} {2} {3}",depth,ray.direction.x,ray.direction.y,ray.direction.z);
            }

            let t=0.5*(unit_direction.y+1.0);
            let v1=v3f{x:(1.0-t),y:(1.0-t),z:(1.0-t)};
            let v2=v3f{x:0.5*t,y:0.7*t,z:1.0*t};
            let v3=VecAdd3D(&v1,&v2);
        
            if depth>0 {
               // println!("depth:{0} debugging {1} {2} {3} ",depth,v3.x,v3.y,v3.z);
            }
            return v3;
        }


    }

pub fn HitSphere(center:v3f,radius:f32,r:Ray)->f32 {
    let oc=VecSub3D(&r.origin,&center);
    let a=VecDot3D(&r.direction,&r.direction);
    let b=2.0*VecDot3D(&oc,&r.direction);
    let c=VecDot3D(&oc,&oc)-(radius*radius);
    let discriminant=b*b-(4.0*a*c);
    
    if discriminant<0.0{
        return -1.0;    
    }
    
    (-b-discriminant.sqrt())/(2.0*a)

}


pub struct World{
    pub  objects: Vec<Box<Hitable> >,
}

impl World{
    pub fn new()->Self {
        World{
            objects:Vec::new(),
        }
    }
}

impl  World{

        fn Hit(&self,r:Ray,t_min:f32,t_max:f32,rec:& mut HitRecord)->bool{

            let mut hit_anything:bool=false;

            let mut temp_rec=HitRecord::new();

            let mut closest_so_far=t_max;
            let mut hit:bool=false;
            
            let mut index=0;
            for obj in &self.objects {
                    let hit=obj.Hit(r,t_min,closest_so_far,&mut temp_rec);
                    
                    if hit{
                        hit_anything=true;
                        closest_so_far=temp_rec.t;
                        rec.t=temp_rec.t;
                        rec.p=temp_rec.p.clone();
                        rec.normal=temp_rec.normal.clone();
                        rec.object_index=index;
                    }

                index+=1;
            }
            
            if hit_anything{
                return true
            }else{
                return false;
            }

            //return None;
        }

}


pub struct Camera{
    pub lower_left_corner:v3f,
    pub horizontal:v3f,
    pub vertical:v3f,
    pub origin:v3f,
}

impl Camera{
    pub fn new()->Self{
        Camera{
            lower_left_corner:v3f{x:-2.0,y:-1.0,z:-1.0},
            horizontal:v3f{x:4.0,y:0.0,z:0.0},
            vertical:v3f{x:0.0,y:2.0,z:0.0},
            origin:v3f{x:0.0,y:0.0,z:0.0},

        }
    }

    pub fn GetRay(&self,u:f32,v:f32)->Ray{
            let mut v1=VecMul3D(&self.horizontal,u);            
            let v2=VecMul3D(&self.vertical,v);
            
            v1=VecAdd3D(&self.lower_left_corner,&v1);
            v1=VecAdd3D(&v1,&v2);

            let r:Ray=Ray{
                origin:self.origin,
                direction:v1
            };
            
            r
    }
}

pub fn RandomInUnitSphere()->v3f{
    let mut p=v3f{x:0.0,y:0.0,z:0.0};
    loop {
        let r1=thread_rng().gen_range(0.0,1.0);
        let r2=thread_rng().gen_range(0.0,1.0);
        let r3=thread_rng().gen_range(0.0,1.0);

        p=v3f{x:2.0*r1-1.0,y:2.0*r2-1.0,z:2.0*r3-1.0};
        if p.SquaredLength()<1.0 {  break; }
    }
    p
}
 
pub trait Material{
    fn Scatter(&self,r_in:&Ray,rec:&HitRecord,attenuation:&mut v3f,scattered:&mut Ray)->bool;
}

pub struct Lambertian {
    pub albedo:v3f,
}

impl Lambertian{
    pub fn new(a:v3f)->Self{
        Lambertian{
            albedo:a
        }
    }

}



impl  Material for Lambertian{
       fn Scatter(&self,r_in:&Ray,rec:&HitRecord,attenuation:&mut v3f,scattered:&mut Ray)->bool{
        let random_point=RandomInUnitSphere();

        let mut target=VecAdd3D(&rec.p,&rec.normal);

        target=VecAdd3D(&target,&random_point);

        let dir=VecSub3D(&target,&rec.p);

        scattered.origin=rec.p.clone();
        scattered.direction=dir.clone();

        attenuation.x=self.albedo.x;
        attenuation.y=self.albedo.y;
        attenuation.z=self.albedo.z;

        return true
       }
}

pub struct Metal{
    pub albedo:v3f,
    pub fuzz:f32,
}

impl Metal{
    pub fn new(a:v3f,f:f32)->Self{
        let fuzz_=if f<1.0 { f } else { 1.0 };
        Metal{
            albedo:a,
            fuzz:fuzz_
        }
    }
}

impl  Material for Metal{
       fn Scatter(&self,r_in:&Ray,rec:&HitRecord,attenuation:&mut v3f,scattered:&mut Ray)->bool{
           let unit_vector=VecNorm3D(&r_in.direction);

            let reflected=Reflect(&unit_vector,&rec.normal);
            let random_point=RandomInUnitSphere();
            let fuzz_random_point=VecMul3D(&random_point,self.fuzz);
            let fuzz_reflected=VecAdd3D(&reflected,&fuzz_random_point);

            scattered.origin=rec.p.clone();
            scattered.direction=fuzz_reflected.clone();

            attenuation.x=self.albedo.x;
            attenuation.y=self.albedo.y;
            attenuation.z=self.albedo.z;

            let result=VecDot3D(&scattered.direction,&rec.normal);
            if result>0.0 {
                return true;
            }

            false
        }
}

pub fn Reflect(v:&v3f,n:&v3f)->v3f{
    let dot=2.0*VecDot3D(v,n);
    let t=VecMul3D(n,dot);
    VecSub3D(v,&t)
}



