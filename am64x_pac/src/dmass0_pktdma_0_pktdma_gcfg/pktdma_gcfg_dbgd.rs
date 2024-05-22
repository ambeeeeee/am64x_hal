#[doc = "Register `PKTDMA_GCFG_DBGD` reader"]
pub type R = crate::R<PktdmaGcfgDbgdSpec>;
#[doc = "Register `PKTDMA_GCFG_DBGD` writer"]
pub type W = crate::W<PktdmaGcfgDbgdSpec>;
#[doc = "Field `DBG_DATA` reader - 31:0\\]
Provides debug information from various internal units. The value which is read back depends on which unit and register are selected in the Debug Address Register"]
pub type DbgDataR = crate::FieldReader<u32>;
#[doc = "Field `DBG_DATA` writer - 31:0\\]
Provides debug information from various internal units. The value which is read back depends on which unit and register are selected in the Debug Address Register"]
pub type DbgDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Provides debug information from various internal units. The value which is read back depends on which unit and register are selected in the Debug Address Register"]
    #[inline(always)]
    pub fn dbg_data(&self) -> DbgDataR {
        DbgDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Provides debug information from various internal units. The value which is read back depends on which unit and register are selected in the Debug Address Register"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_data(&mut self) -> DbgDataW<PktdmaGcfgDbgdSpec> {
        DbgDataW::new(self, 0)
    }
}
#[doc = "This register provides read only debug data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_dbgd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_dbgd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgDbgdSpec;
impl crate::RegisterSpec for PktdmaGcfgDbgdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_dbgd::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgDbgdSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_dbgd::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgDbgdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_DBGD to value 0"]
impl crate::Resettable for PktdmaGcfgDbgdSpec {
    const RESET_VALUE: u32 = 0;
}
