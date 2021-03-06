#[doc = "Reader of register EFUSEADDR"]
pub type R = crate::R<u32, super::EFUSEADDR>;
#[doc = "Writer for register EFUSEADDR"]
pub type W = crate::W<u32, super::EFUSEADDR>;
#[doc = "Register EFUSEADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCK`"]
pub type BLOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLOCK`"]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `ROW`"]
pub type ROW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ROW`"]
pub struct ROW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - BLOCK"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 0:10 - ROW"]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:15 - BLOCK"]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
    #[doc = "Bits 0:10 - ROW"]
    #[inline(always)]
    pub fn row(&mut self) -> ROW_W {
        ROW_W { w: self }
    }
}
