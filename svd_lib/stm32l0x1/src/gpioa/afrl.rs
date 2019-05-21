#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL7R {
    bits: u8,
}
impl AFSEL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL6R {
    bits: u8,
}
impl AFSEL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL5R {
    bits: u8,
}
impl AFSEL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL4R {
    bits: u8,
}
impl AFSEL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL3R {
    bits: u8,
}
impl AFSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL2R {
    bits: u8,
}
impl AFSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL1R {
    bits: u8,
}
impl AFSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL0R {
    bits: u8,
}
impl AFSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel7(&self) -> AFSEL7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL7R { bits }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel6(&self) -> AFSEL6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL6R { bits }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel5(&self) -> AFSEL5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL5R { bits }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel4(&self) -> AFSEL4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL4R { bits }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel3(&self) -> AFSEL3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL3R { bits }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel2(&self) -> AFSEL2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL2R { bits }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel1(&self) -> AFSEL1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL1R { bits }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel0(&self) -> AFSEL0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel7(&mut self) -> _AFSEL7W {
        _AFSEL7W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel6(&mut self) -> _AFSEL6W {
        _AFSEL6W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel5(&mut self) -> _AFSEL5W {
        _AFSEL5W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel4(&mut self) -> _AFSEL4W {
        _AFSEL4W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel3(&mut self) -> _AFSEL3W {
        _AFSEL3W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel2(&mut self) -> _AFSEL2W {
        _AFSEL2W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel1(&mut self) -> _AFSEL1W {
        _AFSEL1W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 0..7)"]
    #[inline]
    pub fn afsel0(&mut self) -> _AFSEL0W {
        _AFSEL0W { w: self }
    }
}
