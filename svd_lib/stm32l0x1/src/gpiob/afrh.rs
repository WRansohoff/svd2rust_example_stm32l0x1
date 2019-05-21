#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRH {
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
pub struct AFSEL15R {
    bits: u8,
}
impl AFSEL15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL14R {
    bits: u8,
}
impl AFSEL14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL13R {
    bits: u8,
}
impl AFSEL13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL12R {
    bits: u8,
}
impl AFSEL12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL11R {
    bits: u8,
}
impl AFSEL11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL10R {
    bits: u8,
}
impl AFSEL10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL9R {
    bits: u8,
}
impl AFSEL9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFSEL8R {
    bits: u8,
}
impl AFSEL8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AFSEL15W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL15W<'a> {
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
pub struct _AFSEL14W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL14W<'a> {
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
pub struct _AFSEL13W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL13W<'a> {
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
pub struct _AFSEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL12W<'a> {
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
pub struct _AFSEL11W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL11W<'a> {
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
pub struct _AFSEL10W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL10W<'a> {
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
pub struct _AFSEL9W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL9W<'a> {
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
pub struct _AFSEL8W<'a> {
    w: &'a mut W,
}
impl<'a> _AFSEL8W<'a> {
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
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel15(&self) -> AFSEL15R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL15R { bits }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel14(&self) -> AFSEL14R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL14R { bits }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel13(&self) -> AFSEL13R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL13R { bits }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel12(&self) -> AFSEL12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL12R { bits }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel11(&self) -> AFSEL11R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL11R { bits }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel10(&self) -> AFSEL10R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL10R { bits }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel9(&self) -> AFSEL9R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL9R { bits }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel8(&self) -> AFSEL8R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFSEL8R { bits }
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
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel15(&mut self) -> _AFSEL15W {
        _AFSEL15W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel14(&mut self) -> _AFSEL14W {
        _AFSEL14W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel13(&mut self) -> _AFSEL13W {
        _AFSEL13W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel12(&mut self) -> _AFSEL12W {
        _AFSEL12W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel11(&mut self) -> _AFSEL11W {
        _AFSEL11W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel10(&mut self) -> _AFSEL10W {
        _AFSEL10W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel9(&mut self) -> _AFSEL9W {
        _AFSEL9W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 8..15)"]
    #[inline]
    pub fn afsel8(&mut self) -> _AFSEL8W {
        _AFSEL8W { w: self }
    }
}
