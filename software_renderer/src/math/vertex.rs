
use std::ops;

#[derive(Debug)]
pub struct Vertex2f
{
    pub x : f32,
    pub y : f32
}

impl ops::Add<Vertex2f> for Vertex2f
{
    type Output = Vertex2f;

    fn add(self, _rhs: Vertex2f)->Vertex2f
    {
        Vertex2f
        {
            x : self.x + _rhs.x,  
            y : self.y + _rhs.y
        }
    }
}

#[derive(Debug)]
pub struct Vertex3f
{
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl ops::Add<Vertex3f> for Vertex3f
{
    type Output = Vertex3f;

    fn add(self, _rhs : Vertex3f)->Vertex3f
    {
        Vertex3f
        {
            x : self.x + _rhs.x, 
            y : self.y + _rhs.y, 
            z : self.z + _rhs.z
        }
    }
}

#[derive(Debug)]
pub struct Vertex4f
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32
}

impl ops::Add<Vertex4f> for Vertex4f
{
    type Output = Vertex4f;
    fn add(self, _rhs : Vertex4f) -> Vertex4f
    {
        Vertex4f{
            x : self.x + _rhs.x, 
            y : self.y + _rhs.y, 
            z : self.z + _rhs.z, 
            w : self.w + _rhs.w
        }  
    }
}
