#[derive(PartialEq, Eq, Hash)]
pub struct Cursor {
    pub x : i32,
    pub y : i32,
}

pub struct Bounds {
    left : i32, 
    top : i32,
    right : i32,
    bottom : i32,
}


impl Cursor {
    pub fn up(&self) -> Cursor{
        return Cursor {x : self.x +1, y : self.y};
    }
    
    pub fn down(&self) -> Cursor{
        return Cursor {x : self.x-1, y : self.y};
    }

    pub fn left(&self) -> Cursor {
        return Cursor {y : self.y -1, x : self.x};
    }

    pub fn right (&self) -> Cursor {
        return Cursor {y : self.y + 1, x : self.x};
    }

    pub fn in_bounds(&self, bounds : Bounds) -> bool {
        return self.x >= bounds.left && self.x <= bounds.right 
            && self.y >= bounds.top && self.y <= bounds.bottom;
    }
}
