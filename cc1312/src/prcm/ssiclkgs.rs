#[doc = "Reader of register SSICLKGS"]
pub type R = crate::R<u32, super::SSICLKGS>;
#[doc = "Writer for register SSICLKGS"]
pub type W = crate::W<u32, super::SSICLKGS>;
#[doc = "Register SSICLKGS `reset()`'s with value 0"]
impl crate::ResetValue for super::SSICLKGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CLK_EN"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLK_EN"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}
