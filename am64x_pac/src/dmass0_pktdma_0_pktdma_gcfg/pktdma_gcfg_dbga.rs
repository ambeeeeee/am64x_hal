#[doc = "Register `PKTDMA_GCFG_DBGA` reader"]
pub type R = crate::R<PktdmaGcfgDbgaSpec>;
#[doc = "Register `PKTDMA_GCFG_DBGA` writer"]
pub type W = crate::W<PktdmaGcfgDbgaSpec>;
#[doc = "Field `DBG_ADDR` reader - 7:0\\]
Selects offset within unit to access seperate debug registers"]
pub type DbgAddrR = crate::FieldReader;
#[doc = "Field `DBG_ADDR` writer - 7:0\\]
Selects offset within unit to access seperate debug registers"]
pub type DbgAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_UNIT` reader - 15:8\\]
Selects which unit to read debug information from"]
pub type DbgUnitR = crate::FieldReader;
#[doc = "Field `DBG_UNIT` writer - 15:8\\]
Selects which unit to read debug information from"]
pub type DbgUnitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_EN` reader - 31:31\\]
Debug enable"]
pub type DbgEnR = crate::BitReader;
#[doc = "Field `DBG_EN` writer - 31:31\\]
Debug enable"]
pub type DbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Selects offset within unit to access seperate debug registers"]
    #[inline(always)]
    pub fn dbg_addr(&self) -> DbgAddrR {
        DbgAddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Selects which unit to read debug information from"]
    #[inline(always)]
    pub fn dbg_unit(&self) -> DbgUnitR {
        DbgUnitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Debug enable"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DbgEnR {
        DbgEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Selects offset within unit to access seperate debug registers"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_addr(&mut self) -> DbgAddrW<PktdmaGcfgDbgaSpec> {
        DbgAddrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Selects which unit to read debug information from"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_unit(&mut self) -> DbgUnitW<PktdmaGcfgDbgaSpec> {
        DbgUnitW::new(self, 8)
    }
    #[doc = "Bit 31 - 31:31\\]
Debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_en(&mut self) -> DbgEnW<PktdmaGcfgDbgaSpec> {
        DbgEnW::new(self, 31)
    }
}
#[doc = "This register provides a writable address which allows debug information to be read from the Debug Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_dbga::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_dbga::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgDbgaSpec;
impl crate::RegisterSpec for PktdmaGcfgDbgaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_dbga::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgDbgaSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_dbga::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgDbgaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_DBGA to value 0"]
impl crate::Resettable for PktdmaGcfgDbgaSpec {
    const RESET_VALUE: u32 = 0;
}
