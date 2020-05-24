#[doc = "Reader of register STIM23"]
pub type R = crate::R<u32, super::STIM23>;
#[doc = "Writer for register STIM23"]
pub type W = crate::W<u32, super::STIM23>;
#[doc = "Register STIM23 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM23`"]
pub type STIM23_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM23`"]
pub struct STIM23_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM23"]
    #[inline(always)]
    pub fn stim23(&self) -> STIM23_R {
        STIM23_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM23"]
    #[inline(always)]
    pub fn stim23(&mut self) -> STIM23_W {
        STIM23_W { w: self }
    }
}
