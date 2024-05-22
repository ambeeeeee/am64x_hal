#[doc = "Register `RINGACC_CFG_BA_HI` reader"]
pub type R = crate::R<RingaccCfgBaHiSpec>;
#[doc = "Register `RINGACC_CFG_BA_HI` writer"]
pub type W = crate::W<RingaccCfgBaHiSpec>;
#[doc = "Field `ADDR_HI` reader - 15:0\\]
Tx Ring base address (MSBs)"]
pub type AddrHiR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_HI` writer - 15:0\\]
Tx Ring base address (MSBs)"]
pub type AddrHiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Tx Ring base address (MSBs)"]
    #[inline(always)]
    pub fn addr_hi(&self) -> AddrHiR {
        AddrHiR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Tx Ring base address (MSBs)"]
    #[inline(always)]
    #[must_use]
    pub fn addr_hi(&mut self) -> AddrHiW<RingaccCfgBaHiSpec> {
        AddrHiW::new(self, 0)
    }
}
#[doc = "The Tx Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to the element size of the ring, or to double the element size of the ring if the qmode is CREDENTIALS or QM modes. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_ba_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_ba_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccCfgBaHiSpec;
impl crate::RegisterSpec for RingaccCfgBaHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_cfg_ba_hi::R`](R) reader structure"]
impl crate::Readable for RingaccCfgBaHiSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_cfg_ba_hi::W`](W) writer structure"]
impl crate::Writable for RingaccCfgBaHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_CFG_BA_HI to value 0"]
impl crate::Resettable for RingaccCfgBaHiSpec {
    const RESET_VALUE: u32 = 0;
}
