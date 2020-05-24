#[doc = "Reader of register STIM5"]
pub type R = crate::R<u32, super::STIM5>;
#[doc = "Writer for register STIM5"]
pub type W = crate::W<u32, super::STIM5>;
#[doc = "Register STIM5 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM5`"]
pub type STIM5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM5`"]
pub struct STIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM5"]
    #[inline(always)]
    pub fn stim5(&self) -> STIM5_R {
        STIM5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM5"]
    #[inline(always)]
    pub fn stim5(&mut self) -> STIM5_W {
        STIM5_W { w: self }
    }
}
