#[doc = "Register `RINGACC_CFG_SIZE` reader"]
pub type R = crate::R<RingaccCfgSizeSpec>;
#[doc = "Register `RINGACC_CFG_SIZE` writer"]
pub type W = crate::W<RingaccCfgSizeSpec>;
#[doc = "Field `SIZE` reader - 19:0\\]
Tx Ring element count. This field configures the size of the ring in elements. For rings in CREDENTIALS or QM modes the size must be an even number."]
pub type SizeR = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - 19:0\\]
Tx Ring element count. This field configures the size of the ring in elements. For rings in CREDENTIALS or QM modes the size must be an even number."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ELSIZE` reader - 26:24\\]
Ring element size. This field is encoded as follows: 0 = 4 bytes 1 = 8 bytes 2 = 16 bytes 3 = 32 bytes 4 = 64 bytes 5 = 128 bytes 6 = 256 bytes 7 = RESERVED"]
pub type ElsizeR = crate::FieldReader;
#[doc = "Field `ELSIZE` writer - 26:24\\]
Ring element size. This field is encoded as follows: 0 = 4 bytes 1 = 8 bytes 2 = 16 bytes 3 = 32 bytes 4 = 64 bytes 5 = 128 bytes 6 = 256 bytes 7 = RESERVED"]
pub type ElsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `QMODE` reader - 31:30\\]
Defines the mode for this ring or queue."]
pub type QmodeR = crate::FieldReader;
#[doc = "Field `QMODE` writer - 31:30\\]
Defines the mode for this ring or queue."]
pub type QmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Tx Ring element count. This field configures the size of the ring in elements. For rings in CREDENTIALS or QM modes the size must be an even number."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Ring element size. This field is encoded as follows: 0 = 4 bytes 1 = 8 bytes 2 = 16 bytes 3 = 32 bytes 4 = 64 bytes 5 = 128 bytes 6 = 256 bytes 7 = RESERVED"]
    #[inline(always)]
    pub fn elsize(&self) -> ElsizeR {
        ElsizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Defines the mode for this ring or queue."]
    #[inline(always)]
    pub fn qmode(&self) -> QmodeR {
        QmodeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Tx Ring element count. This field configures the size of the ring in elements. For rings in CREDENTIALS or QM modes the size must be an even number."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<RingaccCfgSizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Ring element size. This field is encoded as follows: 0 = 4 bytes 1 = 8 bytes 2 = 16 bytes 3 = 32 bytes 4 = 64 bytes 5 = 128 bytes 6 = 256 bytes 7 = RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn elsize(&mut self) -> ElsizeW<RingaccCfgSizeSpec> {
        ElsizeW::new(self, 24)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Defines the mode for this ring or queue."]
    #[inline(always)]
    #[must_use]
    pub fn qmode(&mut self) -> QmodeW<RingaccCfgSizeSpec> {
        QmodeW::new(self, 30)
    }
}
#[doc = "The Tx Ring Size Register contains the element size and element counts for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccCfgSizeSpec;
impl crate::RegisterSpec for RingaccCfgSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_cfg_size::R`](R) reader structure"]
impl crate::Readable for RingaccCfgSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_cfg_size::W`](W) writer structure"]
impl crate::Writable for RingaccCfgSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_CFG_SIZE to value 0"]
impl crate::Resettable for RingaccCfgSizeSpec {
    const RESET_VALUE: u32 = 0;
}
