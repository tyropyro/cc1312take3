#[doc = "Reader of register STIM4"]
pub type R = crate::R<u32, super::STIM4>;
#[doc = "Writer for register STIM4"]
pub type W = crate::W<u32, super::STIM4>;
#[doc = "Register STIM4 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM4`"]
pub type STIM4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM4`"]
pub struct STIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM4"]
    #[inline(always)]
    pub fn stim4(&self) -> STIM4_R {
        STIM4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM4"]
    #[inline(always)]
    pub fn stim4(&mut self) -> STIM4_W {
        STIM4_W { w: self }
    }
}
