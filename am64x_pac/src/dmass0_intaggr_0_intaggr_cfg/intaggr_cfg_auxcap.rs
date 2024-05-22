#[doc = "Register `INTAGGR_CFG_AUXCAP` reader"]
pub type R = crate::R<IntaggrCfgAuxcapSpec>;
#[doc = "Register `INTAGGR_CFG_AUXCAP` writer"]
pub type W = crate::W<IntaggrCfgAuxcapSpec>;
#[doc = "Field `GEVI_CNT` reader - 15:0\\]
Number of event counting registers"]
pub type GeviCntR = crate::FieldReader<u16>;
#[doc = "Field `GEVI_CNT` writer - 15:0\\]
Number of event counting registers"]
pub type GeviCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LEVI_CNT` reader - 31:16\\]
Local input events for local to global translation"]
pub type LeviCntR = crate::FieldReader<u16>;
#[doc = "Field `LEVI_CNT` writer - 31:16\\]
Local input events for local to global translation"]
pub type LeviCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEVI_CNT` reader - 47:32\\]
Number of multicast event registers"]
pub type MeviCntR = crate::FieldReader<u16>;
#[doc = "Field `MEVI_CNT` writer - 47:32\\]
Number of multicast event registers"]
pub type MeviCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UNMAP_CNT` reader - 63:48\\]
Number of multicast event registers. Not all registers in the range are necessarily valid."]
pub type UnmapCntR = crate::FieldReader<u16>;
#[doc = "Field `UNMAP_CNT` writer - 63:48\\]
Number of multicast event registers. Not all registers in the range are necessarily valid."]
pub type UnmapCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of event counting registers"]
    #[inline(always)]
    pub fn gevi_cnt(&self) -> GeviCntR {
        GeviCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Local input events for local to global translation"]
    #[inline(always)]
    pub fn levi_cnt(&self) -> LeviCntR {
        LeviCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 32:47 - 47:32\\]
Number of multicast event registers"]
    #[inline(always)]
    pub fn mevi_cnt(&self) -> MeviCntR {
        MeviCntR::new(((self.bits >> 32) & 0xffff) as u16)
    }
    #[doc = "Bits 48:63 - 63:48\\]
Number of multicast event registers. Not all registers in the range are necessarily valid."]
    #[inline(always)]
    pub fn unmap_cnt(&self) -> UnmapCntR {
        UnmapCntR::new(((self.bits >> 48) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of event counting registers"]
    #[inline(always)]
    #[must_use]
    pub fn gevi_cnt(&mut self) -> GeviCntW<IntaggrCfgAuxcapSpec> {
        GeviCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Local input events for local to global translation"]
    #[inline(always)]
    #[must_use]
    pub fn levi_cnt(&mut self) -> LeviCntW<IntaggrCfgAuxcapSpec> {
        LeviCntW::new(self, 16)
    }
    #[doc = "Bits 32:47 - 47:32\\]
Number of multicast event registers"]
    #[inline(always)]
    #[must_use]
    pub fn mevi_cnt(&mut self) -> MeviCntW<IntaggrCfgAuxcapSpec> {
        MeviCntW::new(self, 32)
    }
    #[doc = "Bits 48:63 - 63:48\\]
Number of multicast event registers. Not all registers in the range are necessarily valid."]
    #[inline(always)]
    #[must_use]
    pub fn unmap_cnt(&mut self) -> UnmapCntW<IntaggrCfgAuxcapSpec> {
        UnmapCntW::new(self, 48)
    }
}
#[doc = "The AuxCap Register contains information on additional capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_cfg_auxcap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_cfg_auxcap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrCfgAuxcapSpec;
impl crate::RegisterSpec for IntaggrCfgAuxcapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_cfg_auxcap::R`](R) reader structure"]
impl crate::Readable for IntaggrCfgAuxcapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_cfg_auxcap::W`](W) writer structure"]
impl crate::Writable for IntaggrCfgAuxcapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_CFG_AUXCAP to value 0x0128_0032_0000"]
impl crate::Resettable for IntaggrCfgAuxcapSpec {
    const RESET_VALUE: u64 = 0x0128_0032_0000;
}
