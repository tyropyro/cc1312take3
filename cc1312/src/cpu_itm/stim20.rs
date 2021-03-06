#[doc = "Reader of register STIM20"]
pub type R = crate::R<u32, super::STIM20>;
#[doc = "Writer for register STIM20"]
pub type W = crate::W<u32, super::STIM20>;
#[doc = "Register STIM20 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM20`"]
pub type STIM20_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM20`"]
pub struct STIM20_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM20"]
    #[inline(always)]
    pub fn stim20(&self) -> STIM20_R {
        STIM20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM20"]
    #[inline(always)]
    pub fn stim20(&mut self) -> STIM20_W {
        STIM20_W { w: self }
    }
}
