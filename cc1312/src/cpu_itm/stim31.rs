#[doc = "Reader of register STIM31"]
pub type R = crate::R<u32, super::STIM31>;
#[doc = "Writer for register STIM31"]
pub type W = crate::W<u32, super::STIM31>;
#[doc = "Register STIM31 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM31`"]
pub type STIM31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM31`"]
pub struct STIM31_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM31"]
    #[inline(always)]
    pub fn stim31(&self) -> STIM31_R {
        STIM31_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM31"]
    #[inline(always)]
    pub fn stim31(&mut self) -> STIM31_W {
        STIM31_W { w: self }
    }
}
