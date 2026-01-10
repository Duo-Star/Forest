#![allow(dead_code)]

use super::super::conic::x_line::XLine;

pub struct DXLine {
    xl1: XLine,
    xl2: XLine,
}

impl DXLine {
    pub fn new(xl1: XLine, xl2: XLine) -> Self {
        DXLine { xl1, xl2 }
    }
    pub fn ang_b(&self) -> DXLine {
        DXLine::new(
            XLine::new(
                self.xl1.p,
                self.xl1.u.angle_bisector(self.xl1.v),
                self.xl1.u.angle_bisector(-self.xl1.v)
            ),
            XLine::new(
                self.xl2.p,
                self.xl2.u.angle_bisector(self.xl2.v),
                self.xl2.u.angle_bisector(-self.xl2.v)
            ),
        )
    }
}
