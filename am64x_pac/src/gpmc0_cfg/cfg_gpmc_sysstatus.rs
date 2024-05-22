#[doc = "Register `CFG_GPMC_SYSSTATUS` reader"]
pub type R = crate::R<CfgGpmcSysstatusSpec>;
#[doc = "Register `CFG_GPMC_SYSSTATUS` writer"]
pub type W = crate::W<CfgGpmcSysstatusSpec>;
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal reset monitoring"]
pub type ResetdoneR = crate::BitReader;
#[doc = "Field `RESETDONE` writer - 0:0\\]
Internal reset monitoring"]
pub type ResetdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal reset monitoring"]
    #[inline(always)]
    pub fn resetdone(&self) -> ResetdoneR {
        ResetdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal reset monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn resetdone(&mut self) -> ResetdoneW<CfgGpmcSysstatusSpec> {
        ResetdoneW::new(self, 0)
    }
}
#[doc = "This register provides status information about the module, excluding the interrupt status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_sysstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_sysstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcSysstatusSpec;
impl crate::RegisterSpec for CfgGpmcSysstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_sysstatus::R`](R) reader structure"]
impl crate::Readable for CfgGpmcSysstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_sysstatus::W`](W) writer structure"]
impl crate::Writable for CfgGpmcSysstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_SYSSTATUS to value 0"]
impl crate::Resettable for CfgGpmcSysstatusSpec {
    const RESET_VALUE: u32 = 0;
}
