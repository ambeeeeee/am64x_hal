#[doc = "Register `MCRC64_REGS_MCRC_BUS_SEL` reader"]
pub type R = crate::R<Mcrc64RegsMcrcBusSelSpec>;
#[doc = "Register `MCRC64_REGS_MCRC_BUS_SEL` writer"]
pub type W = crate::W<Mcrc64RegsMcrcBusSelSpec>;
#[doc = "Field `ITC_MEN` reader - 0:0\\]
Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled Please refer the description of CPU Data trace at page 1-21 for the priority between different data buses."]
pub type ItcMenR = crate::BitReader;
#[doc = "Field `ITC_MEN` writer - 0:0\\]
Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled Please refer the description of CPU Data trace at page 1-21 for the priority between different data buses."]
pub type ItcMenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTC_MEN` reader - 1:1\\]
Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
pub type DtcMenR = crate::BitReader;
#[doc = "Field `DTC_MEN` writer - 1:1\\]
Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
pub type DtcMenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEN` reader - 2:2\\]
Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
pub type MenR = crate::BitReader;
#[doc = "Field `MEN` writer - 2:2\\]
Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
pub type MenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled Please refer the description of CPU Data trace at page 1-21 for the priority between different data buses."]
    #[inline(always)]
    pub fn itc_men(&self) -> ItcMenR {
        ItcMenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
    #[inline(always)]
    pub fn dtc_men(&self) -> DtcMenR {
        DtcMenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
    #[inline(always)]
    pub fn men(&self) -> MenR {
        MenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable/disables the tracing of instruction TCM 0: Tracing of ITCM bus has been disabled 1: Tracing of ITCM bus has been enabled Please refer the description of CPU Data trace at page 1-21 for the priority between different data buses."]
    #[inline(always)]
    #[must_use]
    pub fn itc_men(&mut self) -> ItcMenW<Mcrc64RegsMcrcBusSelSpec> {
        ItcMenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable/disables the tracing of data TCM 0: Tracing of DTCM_ODD and DTCM_EVEN buses have been disabled 1: Tracing of DTCM_ODD and DTCM_EVEN buses have been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dtc_men(&mut self) -> DtcMenW<Mcrc64RegsMcrcBusSelSpec> {
        DtcMenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable/disables the tracing of VBUSM 0: Tracing of VBUSM master bus has been disabled 1: Tracing of VBUSM master bus has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn men(&mut self) -> MenW<Mcrc64RegsMcrcBusSelSpec> {
        MenW::new(self, 2)
    }
}
#[doc = "Data bus tracing selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_mcrc_bus_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_mcrc_bus_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsMcrcBusSelSpec;
impl crate::RegisterSpec for Mcrc64RegsMcrcBusSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_mcrc_bus_sel::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsMcrcBusSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_mcrc_bus_sel::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsMcrcBusSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_MCRC_BUS_SEL to value 0x07"]
impl crate::Resettable for Mcrc64RegsMcrcBusSelSpec {
    const RESET_VALUE: u32 = 0x07;
}
