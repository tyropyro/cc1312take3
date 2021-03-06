#[doc = "Reader of register HASHDIGESTC"]
pub type R = crate::R<u32, super::HASHDIGESTC>;
#[doc = "Writer for register HASHDIGESTC"]
pub type W = crate::W<u32, super::HASHDIGESTC>;
#[doc = "Register HASHDIGESTC `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHDIGESTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HASH_DIGEST`"]
pub type HASH_DIGEST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HASH_DIGEST`"]
pub struct HASH_DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_DIGEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HASH_DIGEST"]
    #[inline(always)]
    pub fn hash_digest(&self) -> HASH_DIGEST_R {
        HASH_DIGEST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HASH_DIGEST"]
    #[inline(always)]
    pub fn hash_digest(&mut self) -> HASH_DIGEST_W {
        HASH_DIGEST_W { w: self }
    }
}
