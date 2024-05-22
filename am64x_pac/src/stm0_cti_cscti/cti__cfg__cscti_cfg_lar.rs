#[doc = "Register `CTI__CFG__CSCTI_CFG_LAR` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgLarSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_LAR` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgLarSpec>;
#[doc = "Field `ACCESS_W` reader - 31:0\\]
A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
pub type AccessWR = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_W` writer - 31:0\\]
A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
pub type AccessWW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
    #[inline(always)]
    pub fn access_w(&self) -> AccessWR {
        AccessWR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
    #[inline(always)]
    #[must_use]
    pub fn access_w(&mut self) -> AccessWW<Cti_Cfg_CsctiCfgLarSpec> {
        AccessWW::new(self, 0)
    }
}
#[doc = "This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_lar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_lar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgLarSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgLarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_lar::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgLarSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_lar::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgLarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_LAR to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgLarSpec {
    const RESET_VALUE: u32 = 0;
}
